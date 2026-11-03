//! 🦀️ EN 1993 storey stability vectors — the Rust SUBJECT adapter. The clause functions of `🧮️clauses/🌐️global` evaluate each quantity and are compared by the feature's relative tolerance with the independent Python oracle running in the oracle role. Step parsing, tolerances and plumbing are the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_artifact_norm_en1993_checks::clauses::global::{alpha_cr_storey, bracing_qd, sway_equivalent_force, sway_phi};

    const STIFFNESS_E: f64 = 210.0e9;
    const COLUMN_I: f64 = 3.923e-5;

    fn alpha(drift: f64) -> Result<f64, String> {
        alpha_cr_storey(2.0e4, 4.0e5, 4.0, drift).ok_or_else(|| format!("storey alpha_cr undefined for drift {drift}"))
    }

    fn portal_drift() -> f64 {
        2.0e4 / (2.0 * 12.0 * STIFFNESS_E * COLUMN_I / 4.0_f64.powi(3))
    }

    pub fn quantity(name: &str) -> Result<f64, String> {
        Ok(match name {
            "alpha_showcase" => alpha(0.002)?,
            "alpha_soft" => alpha(0.1)?,
            "phi_h4_m2" => sway_phi(4.0, 2.0),
            "phi_h4_m2_force" => sway_equivalent_force(sway_phi(4.0, 2.0), 4.0e5),
            "qd_6m" => bracing_qd(1.0e6, 2.0, 6.0, 0.02),
            "alpha_portal_first_order" => alpha(portal_drift())?,
            other => return Err(format!("EN 1993 storey vectors have no quantity {other}")),
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
