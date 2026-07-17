use animate_core::config::OutputFormat;
use animate_core::{AnimateConfig, Scene, SceneRunner};
use std::path::PathBuf;

use crate::cache::PartialMovieCache;
use crate::renderer::VelloRenderer;
use crate::writer::SceneFileWriter;

/// 📦 Paths to encoded artifacts.
#[derive(Clone, Debug, Default)]
pub struct OutputPaths {
    pub mp4: Option<PathBuf>,
    pub gif: Option<PathBuf>,
    pub png_dir: Option<PathBuf>,
    pub last_frame: Option<PathBuf>,
}

/// 🎬 Renders any `Scene` implementation to configured outputs.
pub fn render_scene<S: Scene>(scene: S, config: AnimateConfig) -> Result<OutputPaths, String> {
    let runner = SceneRunner::build(scene, config.clone());
    let frame_count = runner.frame_count();
    let mut renderer = VelloRenderer::new(config.pixel_width, config.pixel_height)?;
    let mut writer = SceneFileWriter::new(&config)?;
    let mut cache = if config.cache_partial_movies {
        Some(PartialMovieCache::open(config.partial_movie_dir())?)
    } else {
        None
    };

    let mut current_hash = String::new();
    let mut current_partial: Option<PathBuf> = None;
    let mut last_pixels: Option<Vec<u8>> = None;

    for frame in 0..frame_count {
        let snapshot = runner.snapshot_at(frame);
        let hash = snapshot.animation_hash();

        if hash != current_hash {
            if let Some(partial) = current_partial.take() {
                let encoded = writer.finalize_partial(&partial)?;
                if let Some(cache) = cache.as_mut() {
                    cache.insert(current_hash.clone(), encoded);
                }
            }
            if let Some(cache) = cache.as_ref() {
                if let Some(cached) = cache.get(&hash) {
                    writer.register_cached_partial(cached);
                    current_hash = hash;
                    current_partial = None;
                    continue;
                }
            }
            current_hash = hash.clone();
            current_partial = Some(writer.begin_partial(&hash, frame)?);
        }

        let pixels = renderer.render_frame(&snapshot)?;
        if let Some(ref partial) = current_partial {
            writer.write_frame_png(partial, &pixels, frame)?;
        }
        last_pixels = Some(pixels);
    }

    if let Some(partial) = current_partial {
        let encoded = writer.finalize_partial(&partial)?;
        if let Some(cache) = cache.as_mut() {
            cache.insert(current_hash, encoded);
            let _ = cache.write_index();
        }
    }

    writer.encode_outputs(last_pixels.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use animate_core::sobject::{Mobility, PaintStyle, Sobject, SobjectId, SobjectShape, StrokeStyle};
    use animate_core::SceneContext;
    use mathematical_geometry::{Affine, Point};
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestScene;

    impl Scene for TestScene {
        fn construct(&mut self, ctx: &mut SceneContext) {
            ctx.add(Sobject {
                id: SobjectId(0),
                shape: SobjectShape::Circle {
                    center: Point::new(0.0, 0.0),
                    radius: 1.0,
                },
                transform: Affine::IDENTITY,
                fill: Some(PaintStyle { color: [0.2, 0.6, 1.0, 1.0] }),
                stroke: Some(StrokeStyle { color: [1.0, 1.0, 1.0, 1.0], width: 0.04 }),
                z_index: 0,
                mobility: Mobility::Static,
            });
            ctx.play(0.5);
        }
    }

    fn temp_config() -> AnimateConfig {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("animate_render_test_{stamp}"));
        let mut config = AnimateConfig {
            pixel_width: 64,
            pixel_height: 64,
            frame_rate: 15.0,
            output_dir: dir.clone(),
            media_dir: dir.join("media"),
            file_stem: "test_scene".into(),
            output_formats: vec![OutputFormat::LastFrame],
            cache_partial_movies: false,
            ..AnimateConfig::default()
        };
        config.apply_quality(animate_core::QualityPreset::Low480p15);
        config
    }

    #[test]
    fn render_scene_writes_last_frame() {
        let config = temp_config();
        let outputs = render_scene(TestScene, config.clone()).expect("render");
        let last = outputs.last_frame.expect("last frame path");
        assert!(last.exists());
    }
}
