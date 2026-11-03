//! 🦀️ EN 1993 catalogue-gap checks — the Rust SUBJECT adapter. The torsional buckling resistance, the block tearing resistance, the flange curling, the fatigue limit and the LTB resistance in fire of the checks in `🧱️member-checks/🧾️vid-checks` are compared by the feature's relative tolerance with the independent numpy and `sectionproperties` oracle in the oracle role. Step parsing and plumbing are the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_artifact_norm_en1993_checks::{check_full_steel_structure, clauses, fire_ltb_summary, torsional_summary};
    use semio_s_artifact_norm_en1993_model::En1993Snapshot;

    fn limit_of(document: &En1993Snapshot, id: &str) -> Result<f64, String> {
        check_full_steel_structure(document).checks.iter().find(|c| c.id == id).map(|c| c.limit.value).ok_or(format!("no check {id}"))
    }

    pub fn quantity(name: &str) -> Result<f64, String> {
        let heb240 = En1993Snapshot::compliant_heb240_frame();
        let showcase = En1993Snapshot::special_parts_showcase();
        let torsion = || torsional_summary(&heb240, "member-b1").ok_or("no torsional summary".to_string());
        Ok(match name {
            "ncrTCatalogue" | "ncrTSectionproperties" => torsion()?.n_cr_t,
            "torsionConstantSectionproperties" => heb240.sections[0].it,
            "warpingConstantSectionproperties" => heb240.sections[0].iw,
            "blockTearing" => limit_of(&heb240, "en1993.1-8.3.10.block.joint-j1")?,
            "flangeCurling" => check_full_steel_structure(&showcase).checks.iter().find(|c| c.id == "en1993.1-3.5.4.curl.cf-1").map(|c| c.computed.value).ok_or("no curling check")?,
            "fatigueLimitCategory71" => clauses::fatigue_fire::delta_sigma_d(71.0e6),
            "ltbFire560" => fire_ltb_summary(&heb240, "fire-1").and_then(|s| s.curve.iter().find(|(theta, _)| (*theta - 560.0).abs() < 1e-9).map(|(_, m)| *m)).ok_or("no fire LTB curve point")?,
            other => return Err(format!("EN 1993 catalogue-gap vectors have no quantity {other}")),
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
