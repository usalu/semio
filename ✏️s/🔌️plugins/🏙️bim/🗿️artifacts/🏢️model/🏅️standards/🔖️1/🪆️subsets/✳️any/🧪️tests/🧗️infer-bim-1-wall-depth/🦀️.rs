//! 🧗️ BIM inference case `infer-bim-1-wall-depth`, Rust adapter (subject role only). The shapely reproduction lives in `🐍️.py` beside this file; this adapter answers the same scenario from
//! `ModelInference` of the very same committed snapshot: the extremes, the elevation area and (for a free wall of constant thickness without openings) the volume of every wall with an attached top or
//! base, the path length, cross-section, volume and visible surface of every sweep, and the reveal area and lateral frame planes of every opening with an authored reveal. It registers no oracle
//! handler: a subject that re-read the committed expectation would be a self-comparison.
//!
//! The subject half is `sut`-gated because the generated host links this repository's crate only for the subject role.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_bim_model::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;
    use semio_s_artifact_bim_model::standards::v1::subsets::any::schema::inferences::element_solids::ElementSolid;
    use semio_s_artifact_bim_model::{ModelInference, ModelSnapshot, TopConstraint, Wall};

    fn number(value: f64) -> String {
        format!("{value:?}")
    }

    fn attached(wall: &Wall) -> bool {
        matches!(wall.top, TopConstraint::Roof { .. } | TopConstraint::Slab { .. } | TopConstraint::Ceiling { .. }) || wall.base_slab.is_some()
    }

    fn free(snapshot: &ModelSnapshot, id: &str, wall: &Wall) -> bool {
        let ends = |wall: &Wall| match &wall.axis {
            semio_s_artifact_bim_model::Axis::Line { start, end } | semio_s_artifact_bim_model::Axis::Arc { start, end, .. } => [(start.x, start.y), (end.x, end.y)],
        };
        let mine = ends(wall);
        let alone = snapshot.walls.iter().filter(|(other, row)| other.as_str() != id && row.storey == wall.storey).all(|(_, row)| ends(row).iter().all(|a| mine.iter().all(|b| (a.0 - b.0).hypot(a.1 - b.1) > 1e-6)));
        alone && !snapshot.openings.values().any(|opening| opening.host == id)
    }

    fn reveal_area(solid: &ElementSolid) -> f64 {
        let mesh = solid.mesh();
        solid
            .face_groups
            .iter()
            .enumerate()
            .filter(|(_, group)| solid.groups[**group as usize].part == "reveal")
            .map(|(triangle, _)| {
                let [a, b, c] = mesh.triangle(triangle);
                let (u, v) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]]);
                0.5 * ((u[1] * v[2] - u[2] * v[1]).powi(2) + (u[2] * v[0] - u[0] * v[2]).powi(2) + (u[0] * v[1] - u[1] * v[0]).powi(2)).sqrt()
            })
            .sum()
    }

    /// 🧗️ `{walls, sweeps, reveals}` of the committed attic model, in the shape of the shapely table.
    pub fn wall_depth(ctx: &Context) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.contains("📸️snapshot")).ok_or_else(|| "the scenario names no snapshot".to_string())?;
        let snapshot = decode_model_snapshot_json(&String::from_utf8(ctx.input_bytes(&uri)?).map_err(|error| format!("the committed snapshot is not UTF-8: {error}"))?)?;
        let inferred = ModelInference::infer(&snapshot).map_err(|error| error.to_string())?;
        let mut walls = Vec::new();
        for (id, wall) in snapshot.walls.iter().filter(|(_, wall)| attached(wall)) {
            let layout = inferred.wall_layout.get(id).ok_or_else(|| format!("wall {id} has no layout"))?;
            let volume = if free(&snapshot, id, wall) { format!(",\"volume\":{}", number(layout.volume)) } else { String::new() };
            walls.push(format!("\"{id}\":{{\"base_z\":{},\"side_area\":{},\"top_z\":{}{volume}}}", number(layout.base_z), number(layout.side_area), number(layout.top_z)));
        }
        let mut sweeps = Vec::new();
        for id in snapshot.wall_sweeps.keys() {
            let row = inferred.quantities.elements.get(id).ok_or_else(|| format!("sweep {id} has no quantity"))?;
            sweeps.push(format!("\"{id}\":{{\"gross_volume\":{},\"length\":{},\"section_area\":{},\"surface_area\":{}}}", number(row.gross_volume), number(row.length), number(row.gross_area), number(row.surface_area)));
        }
        let mut reveals = Vec::new();
        for (id, opening) in snapshot.openings.iter().filter(|(_, opening)| opening.reveal_depth.is_some()) {
            let host = inferred.element_solids.get(&opening.host).ok_or_else(|| format!("host {} has no solid", opening.host))?;
            let filler = inferred.element_solids.get(id).ok_or_else(|| format!("opening {id} has no solid"))?;
            reveals.push(format!("\"{id}\":{{\"frame_back_y\":{},\"frame_front_y\":{},\"jamb_area\":{}}}", number(filler.bounds.min.y), number(filler.bounds.max.y), number(reveal_area(host))));
        }
        let text = format!("{{\"reveals\":{{{}}},\"sweeps\":{{{}}},\"walls\":{{{}}}}}", reveals.join(","), sweeps.join(","), walls.join(","));
        let projection = parse_json(&text)?;
        Ok(Outcome::with_raw(text.into_bytes(), projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls; the id is the feature's `@id-*` tag.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("wall-depth", subject::wall_depth);
    }
    built
}
//#endregion 🔖️Registration
