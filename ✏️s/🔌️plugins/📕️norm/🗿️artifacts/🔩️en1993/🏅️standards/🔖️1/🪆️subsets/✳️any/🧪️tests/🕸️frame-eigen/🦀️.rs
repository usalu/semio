//! 🦀️ EN 1993 general planar frame alpha_cr — the Rust SUBJECT adapter. The mechanics-kernel eigenvalue of `🕸️frame-eigen` (and the stability-function closed form of `🧮️clauses/🌐️global`) is compared by the feature's relative tolerance with the independent numpy/scipy oracle running in the oracle role. Step parsing and plumbing are the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_artifact_norm_en1993_checks::clauses::global::alpha_cr_portal_fixed;
    use semio_s_artifact_norm_en1993_frame_eigen::{alpha_cr, solve_second_order};
    use semio_s_artifact_norm_en1993_model::En1993Snapshot;
    use semio_s_plugin_norm_mechanics::member_modes::{buckling, Ends, Props};

    fn portal(load_factor: f64) -> En1993Snapshot {
        let mut document = En1993Snapshot::frame_portal_showcase();
        document.frame_nodes.iter_mut().for_each(|node| {
            node.load_x = 0.0;
            node.load_z *= load_factor;
        });
        document
    }

    fn cantilever() -> En1993Snapshot {
        let mut document = portal(1.0);
        document.frame_nodes.retain(|node| node.id == "node-a" || node.id == "node-b");
        document.frame_bars.retain(|bar| bar.id == "bar-1");
        if let Some(top) = document.frame_nodes.iter_mut().find(|node| node.id == "node-b") {
            top.load_z = -100_000.0;
        }
        document
    }

    fn member(ends: Ends) -> Result<f64, String> {
        let props = Props { length: 5.0, e: 210e9, g: 80.77e9, a: 0.01, iy: 8e-5, iz: 8e-5, j: 1e-6 };
        buckling(&props, ends).map(|(load, _)| load).map_err(|fault| format!("{fault:?}"))
    }

    fn solved(document: &En1993Snapshot) -> Result<f64, String> {
        alpha_cr(document).map_err(|fault| fault.en)
    }

    fn course(name: &str) -> Result<f64, String> {
        let (bar, rest) = if let Some(rest) = name.strip_prefix("column") { ("bar-1", rest) } else { ("bar-2", name.strip_prefix("beam").ok_or("not a bar course")?) };
        let forces = solve_second_order(&En1993Snapshot::frame_portal_showcase()).map_err(|fault| fault.en)?;
        let first = forces.bar(bar).ok_or("bar missing")?.first;
        let (quantity, _) = rest.split_once("PyNite").or_else(|| rest.split_once("Anastruct")).ok_or("no solver suffix")?;
        match quantity {
            "Moment" => Ok(first.m),
            "Shear" => Ok(first.v),
            "Axial" => Ok(first.n.abs()),
            other => Err(format!("EN 1993 frame course has no quantity {other}")),
        }
    }

    fn case_course(name: &str) -> Result<f64, String> {
        use semio_s_artifact_norm_en1993_checks::frame_envelope;
        use semio_s_artifact_norm_en1993_frame_eigen::{FrameLoads, Imperfections};
        let rest = name.strip_prefix("case").ok_or("not a load case course")?;
        let (case, rest) = ["Combo", "G", "Q", "W"].iter().find_map(|c| rest.strip_prefix(*c).map(|r| (*c, r))).ok_or("no load case in the quantity")?;
        let bar = if rest.starts_with("Column") { "bar-1" } else { "bar-2" };
        let document = En1993Snapshot::frame_cases_showcase();
        let of = |id: &str| FrameLoads::of_case(&document, id).map_err(|fault| fault.en);
        let loads = match case {
            "G" => of("g-permanent")?,
            "Q" => of("q-imposed")?,
            "W" => of("w-wind")?,
            _ => {
                let mut loads = of("g-permanent")?;
                loads.add(&of("q-imposed")?.scaled(0.5));
                loads.add(&of("w-wind")?.scaled(0.25));
                loads
            }
        };
        let result = frame_envelope::second_order(&document, &loads, &Imperfections::default()).map_err(|fault| fault.en)?;
        Ok(result.bar(bar).ok_or("bar missing")?.first.m)
    }

    fn sway_course(name: &str) -> Result<f64, String> {
        use semio_s_artifact_norm_en1993_checks::frame_envelope;
        use semio_s_artifact_norm_en1993_frame_eigen::Imperfections;
        let with_h = |h: f64| {
            let mut document = En1993Snapshot::frame_portal_showcase();
            document.frame_nodes.iter_mut().filter(|n| n.id == "node-b").for_each(|n| n.load_x = h);
            document
        };
        let sway_of = |h: f64| -> Result<frame_envelope::Sway, String> {
            let document = with_h(h);
            let loads = frame_envelope::load_sets(&document).map_err(|fault| fault.en)?.remove(0).loads;
            let base = frame_envelope::second_order(&document, &loads, &Imperfections::default()).map_err(|fault| fault.en)?;
            Ok(frame_envelope::sway_imperfection(&document, &loads, &base))
        };
        match name {
            "swayPhi" => Ok(sway_of(20_000.0)?.phi),
            "swayForce" => Ok(sway_of(20_000.0)?.forces.first().ok_or("no sway force")?.1),
            "swayNodesH20" => Ok(sway_of(20_000.0)?.forces.len() as f64),
            "swayNodesH59" => Ok(sway_of(59_000.0)?.forces.len() as f64),
            "swayNodesH61" => Ok(sway_of(61_000.0)?.forces.len() as f64),
            other => {
                let bar = if other.starts_with("swayColumn") { "bar-1" } else { "bar-2" };
                let results = frame_envelope::combination_results(&with_h(20_000.0)).map_err(|fault| fault.en)?;
                Ok(results[0].result.bar(bar).ok_or("bar missing")?.first.m)
            }
        }
    }

    pub fn quantity(name: &str) -> Result<f64, String> {
        if name.starts_with("sway") {
            return sway_course(name);
        }
        if name.starts_with("case") {
            return case_course(name);
        }
        if name.starts_with("column") || name.starts_with("beam") {
            return course(name);
        }
        Ok(match name {
            "portalLowest" => solved(&portal(1.0))?,
            "portalLoaded20" => solved(&portal(20.0))?,
            "portalClosedForm" => {
                let document = portal(1.0);
                alpha_cr_portal_fixed(document.materials[0].e_modulus, document.sections[0].iy, document.sections[0].iy, 4.0, 6.0, 200_000.0).ok_or("portal closed form undefined")?
            }
            "cantileverLowest" => solved(&cantilever())?,
            "cantileverEuler" => {
                let document = cantilever();
                std::f64::consts::PI.powi(2) * document.materials[0].e_modulus * document.sections[0].iy / (4.0 * 16.0) / 100_000.0
            }
            "ncrPinned" => member(Ends::Pinned)?,
            "ncrCantilever" => member(Ends::Cantilever)?,
            "ncrFixed" => member(Ends::Fixed)?,
            other => return Err(format!("EN 1993 frame vectors have no quantity {other}")),
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
