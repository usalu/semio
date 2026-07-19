//! 🖼️ Image / spatial entropy over plain pixel slices (no image-decoding dependency): global
//! grayscale histogram entropy and gray-level co-occurrence matrix (GLCM) texture entropy.

use crate::numeric::x_ln_x;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖Config
/// 🖼️ Which spatial-entropy computation [`entropy_2d`] performs.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SpatialMethod {
    /// 🖼️ Global histogram entropy over all pixel values.
    Global,
    /// 🖼️ Gray-level co-occurrence matrix entropy for the pixel offset `(dx, dy)`.
    Glcm { dx: i32, dy: i32 },
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SpatialConfig {
    pub method: SpatialMethod,
    pub bins: usize,
}

impl SpatialConfig {
    pub fn new(method: SpatialMethod, bins: usize) -> Result<Self, EntropyError> {
        if bins < 2 {
            return Err(EntropyError::InvalidConfig { field: "bins", reason: "must be at least 2" });
        }
        Ok(Self { method, bins })
    }
}
// #endregion 🔖Config

// #region 🔖Binning
fn bin_pixels(pixels: &[f64], bins: usize) -> Result<Vec<usize>, EntropyError> {
    let (min, max) = pixels.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    if !(max > min) {
        return Err(EntropyError::DegenerateInput { what: "constant image has zero dynamic range" });
    }
    Ok(pixels
        .iter()
        .map(|&v| (((v - min) / (max - min) * bins as f64).floor() as usize).min(bins - 1))
        .collect())
}
// #endregion 🔖Binning

// #region 🔖Dispatch
/// 🖼️ Computes a spatial entropy measure over a row-major `width x height` pixel grid.
pub fn entropy_2d(pixels: &[f64], width: usize, height: usize, cfg: SpatialConfig) -> Result<Estimate, EntropyError> {
    if pixels.is_empty() {
        return Err(EntropyError::EmptyInput { what: "pixels" });
    }
    if pixels.len() != width * height {
        return Err(EntropyError::ShapeMismatch { what: "pixels", expected: width * height, actual: pixels.len() });
    }
    for (i, &v) in pixels.iter().enumerate() {
        if !v.is_finite() {
            return Err(EntropyError::NonFinite { what: "pixels", index: i });
        }
    }
    let levels = bin_pixels(pixels, cfg.bins)?;

    let (nats, method, diagnostics) = match cfg.method {
        SpatialMethod::Global => {
            let mut counts = vec![0.0_f64; cfg.bins];
            for &l in &levels {
                counts[l] += 1.0;
            }
            let n = pixels.len() as f64;
            let nats = -counts.iter().map(|&c| x_ln_x(c / n)).sum::<f64>();
            (nats, "global_histogram", vec![("bins", cfg.bins as f64)])
        }
        SpatialMethod::Glcm { dx, dy } => {
            let mut joint = vec![0.0_f64; cfg.bins * cfg.bins];
            let mut pairs = 0.0_f64;
            for y in 0..height as i32 {
                for x in 0..width as i32 {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
                        continue;
                    }
                    let a = levels[(y as usize) * width + x as usize];
                    let b = levels[(ny as usize) * width + nx as usize];
                    joint[a * cfg.bins + b] += 1.0;
                    joint[b * cfg.bins + a] += 1.0; // 🖼️ standard GLCM symmetrization
                    pairs += 2.0;
                }
            }
            if pairs <= 0.0 {
                return Err(EntropyError::InvalidConfig { field: "dx/dy", reason: "offset produces no valid pixel pairs" });
            }
            let nats = -joint.iter().map(|&c| x_ln_x(c / pairs)).sum::<f64>();
            (nats, "glcm", vec![("bins", cfg.bins as f64), ("dx", dx as f64), ("dy", dy as f64)])
        }
    };

    let mut warnings = Vec::new();
    if pixels.len() < 10 * cfg.bins {
        warnings.push(Warning::SmallSample { n: pixels.len(), recommended: 10 * cfg.bins });
    }

    Ok(Estimate {
        value: LogBase::Nats.from_nats(nats),
        base: LogBase::Nats,
        method,
        n: pixels.len(),
        n_effective: pixels.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics,
    })
}
// #endregion 🔖Dispatch

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_image_is_rejected() {
        let pixels = vec![5.0; 16];
        let cfg = SpatialConfig::new(SpatialMethod::Global, 4).unwrap();
        assert!(matches!(entropy_2d(&pixels, 4, 4, cfg), Err(EntropyError::DegenerateInput { .. })));
    }

    #[test]
    fn checkerboard_has_higher_glcm_entropy_than_flat_gradient() {
        let width = 8;
        let height = 8;
        let checker: Vec<f64> = (0..width * height).map(|i| (((i / width) + (i % width)) % 2) as f64).collect();
        let gradient: Vec<f64> = (0..width * height).map(|i| (i % width) as f64).collect();
        let cfg = SpatialConfig::new(SpatialMethod::Glcm { dx: 1, dy: 0 }, 4).unwrap();
        let h_checker = entropy_2d(&checker, width, height, cfg).unwrap().value;
        let h_gradient = entropy_2d(&gradient, width, height, cfg).unwrap().value;
        assert!(h_checker > h_gradient, "checker={h_checker} gradient={h_gradient}");
    }

    #[test]
    fn global_entropy_of_uniform_random_image_is_near_max() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let width = 32;
        let height = 32;
        let pixels: Vec<f64> = (0..width * height).map(|_| rng.next_f64()).collect();
        let cfg = SpatialConfig::new(SpatialMethod::Global, 8).unwrap();
        let est = entropy_2d(&pixels, width, height, cfg).unwrap();
        assert!(est.value > 0.8 * 8.0_f64.ln(), "got {}", est.value);
    }

    #[test]
    fn shape_mismatch_is_rejected() {
        let pixels = vec![1.0, 2.0, 3.0];
        let cfg = SpatialConfig::new(SpatialMethod::Global, 4).unwrap();
        assert!(matches!(entropy_2d(&pixels, 2, 2, cfg), Err(EntropyError::ShapeMismatch { .. })));
    }

    #[test]
    fn spatial_config_rejects_small_bins() {
        assert!(SpatialConfig::new(SpatialMethod::Global, 1).is_err());
    }
}
// #endregion 🔖Tests
