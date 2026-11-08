//! 🦀️ EN 1993 section classes, shear resistance and Annex A — the Rust SUBJECT adapter. Each quantity is evaluated by the clause functions of `🧮️clauses` and compared by the feature's relative tolerance with the independent Python oracle beside this file.

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_artifact_norm_en1993_checks::clauses::{classification, resistance, stability};

    const GEOMETRY: classification::Geometry = classification::Geometry { h: 300.0, b: 150.0, tw: 7.1, tf: 10.7, r: 15.0, weld_throat: 0.0 };
    const AREA: f64 = 5381.0;
    const FY: f64 = 355.0;

    fn web_ct() -> f64 {
        (GEOMETRY.h - 2.0 * GEOMETRY.tf - 2.0 * GEOMETRY.r) / GEOMETRY.tw
    }

    fn flange_ct() -> f64 {
        (GEOMETRY.b - GEOMETRY.tw - 2.0 * GEOMETRY.r) / 2.0 / GEOMETRY.tf
    }

    fn shear_area() -> f64 {
        resistance::av_rolled_i(AREA, GEOMETRY.b, GEOMETRY.tf, GEOMETRY.tw, GEOMETRY.r, GEOMETRY.h - 2.0 * GEOMETRY.tf, resistance::eta_shear(FY))
    }

    fn annex_a(name: &str) -> Result<f64, String> {
        let (wpl_y, wpl_z) = (628.4e3, 125.2e3);
        let (iy, iz, e, length) = (8356e4, 603.8e4, 210000.0, 4000.0);
        let n_rk = AREA * FY;
        let ncr = |i: f64| stability::n_cr(e * i, length);
        let (ncr_y, ncr_z) = (ncr(iy), ncr(iz));
        let (lambda_y, lambda_z) = ((n_rk / ncr_y).sqrt(), (n_rk / ncr_z).sqrt());
        let chi = |lambda: f64, curve: stability::Curve| stability::chi_general(lambda, curve.alpha(), 0.2, 1.0);
        let base = stability::InteractionInput { n_ed: 400e3, my_ed: 100e6, mz_ed: 0.0, n_rk, my_rk: wpl_y * FY, mz_rk: wpl_z * FY, chi_y: chi(lambda_y, stability::Curve::A), chi_z: chi(lambda_z, stability::Curve::B), chi_lt: 1.0, lambda_y, lambda_z, cmy: 0.9, cmz: 0.9, cm_lt: 1.0, class: 1, susceptible_torsion: false, rhs: false, gm1: 1.0, delta_my: 0.0, delta_mz: 0.0 };
        let input = stability::AnnexAInput { base, ncr_y, ncr_z, ncr_t: 1.0e15, ncr_tf: 1.0e15, wel_y: 557e3, wpl_y, wel_z: 80.5e3, wpl_z, mpl_y_rd: wpl_y * FY, mpl_z_rd: wpl_z * FY, it: 20.12e4, iy, a_area: AREA, c1: 1.0, lambda_0: 0.0, cmy0: 0.9, cmz0: 0.9 };
        let r = stability::interaction_annex_a(&input);
        Ok(match name {
            "kyy" => r.kyy,
            "kyz" => r.kyz,
            "kzy" => r.kzy,
            "kzz" => r.kzz,
            "eta_61" => r.eta_61,
            "eta_62" => r.eta_62,
            other => return Err(format!("EN 1993 Annex A has no quantity {other}")),
        })
    }

    pub fn quantity(name: &str) -> Result<f64, String> {
        let eps = classification::epsilon(FY);
        let classed = classification::classify_i_section(&GEOMETRY, FY, 2.0e6, 1.0, true);
        Ok(match name {
            "epsilon_355" => eps,
            "class_web_compression" => classification::internal_part_class(web_ct(), eps, 1.0, 1.0) as f64,
            "class_web_bending" => classification::internal_part_class(web_ct(), eps, -1.0, 0.5) as f64,
            "class_flange" => classification::outstand_class(flange_ct(), eps, 1.0, 1.0, true) as f64,
            "class_plate_outstand_10" => classification::outstand_class(10.0, eps, 1.0, 1.0, true) as f64,
            "class_plate_outstand_12" => classification::outstand_class(12.0, eps, 1.0, 1.0, true) as f64,
            "class_section_compression" => classed.section as f64,
            "class_chs_139_5" => classification::chs_class(139.7 / 5.0, eps) as f64,
            "class_chs_273_63" => classification::chs_class(273.0 / 6.3, eps) as f64,
            "class_chs_508_5" => classification::chs_class(508.0 / 5.0, eps) as f64,
            "class_angle_s235" => classification::angle_class(100.0, 100.0, 10.0, 1.0) as f64,
            "class_angle_s355" => classification::angle_class(100.0, 100.0, 10.0, eps) as f64,
            "a_v" => shear_area(),
            "v_pl_rd" => resistance::v_pl_rd(shear_area(), FY, 1.0),
            other => return annex_a(other),
        })
    }

    pub fn binding() -> Binding {
        let mut bound = Binding::none();
        bound.vector = Box::new(|_, _vector, name| quantity(name));
        bound
    }
}

/// 🧭️ Registration is by scenario family; the oracle role is the Python adapter beside this file.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = steps::norm_adapter(subject::binding());
    }
    built
}
