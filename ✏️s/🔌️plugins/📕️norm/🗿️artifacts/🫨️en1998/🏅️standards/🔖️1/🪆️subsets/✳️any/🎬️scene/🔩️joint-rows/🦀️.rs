//! 🔩️ Joint rows in 3D: one marker per `joints[]` row at its plan position below the slab of its storey, coloured by its capacity design check `ΣM_Rc / (1.3 ΣM_Rb)` (EN 1998-1 4.4.2.3), with grips on the plan position.

use super::handles::Index;
use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::schema::joints as rows;
use super::SceneInput;

/// 🔩️ The joint markers of the document; a row whose building or storey does not exist draws nothing (the integrity check names it).
pub(crate) fn project(input: &SceneInput, layout: &Layout, handles: &Index, out: &mut Out) {
    let d = input.document;
    for j in &d.joints {
        if !out.own("joints", &j.id) {
            continue;
        }
        let Some(b) = d.buildings.iter().find(|b| b.id == j.building_id) else { continue };
        let Some(top) = rows::elevation_m(b, j) else { continue };
        let (o, path, check) = (layout.origin(&b.id), path_of("joints", &j.id), format!("en1998.1.{}.capacityDesign.joint.{}", b.id, j.id));
        let size = 1.5 * super::buildings::column_size(b);
        let at = v3(o.x + j.x_m - size / 2.0, o.y + j.y_m - size / 2.0, o.z + top - SLAB_THICKNESS - size);
        let ratio = rows::ratio(j);
        let name = loc(&format!("Joint {}: ΣM_Rc = {:.0} kNm, 1.3 ΣM_Rb = {:.0} kNm", j.id, j.column_moment_rd_nm / 1e3, rows::demand_nm(j) / 1e3), &format!("Knoten {}: ΣM_Rc = {:.0} kNm, 1,3 ΣM_Rb = {:.0} kNm", j.id, j.column_moment_rd_nm / 1e3, rows::demand_nm(j) / 1e3));
        let mut spec = Spec::new(EntityKind::Solid, j.id.clone(), path.clone(), box_solid(at.clone(), v3(size, size, size)), MaterialRole::Neutral, name, LAYER_JOINTS);
        spec.binding = binding_exact(input.report, &check);
        spec.opacity = 0.8;
        spec.handles = handles.of(&j.id);
        spec.detail = loc("Capacity design joint: the sum of the column resistances is at least 1.3 times the sum of the beam resistances", "Kapazit\u{e4}tsbemessungsknoten: Summe der St\u{fc}tzenwiderst\u{e4}nde mindestens 1,3-fache Summe der Riegelwiderst\u{e4}nde");
        let (binding, centre) = (spec.binding.clone(), v3(at.x + size / 2.0, at.y + size / 2.0, at.z + size));
        let text = if ratio.is_finite() { format!("{ratio:.2}") } else { "\u{221e}".to_owned() };
        out.push(spec, &[at.clone(), centre.clone()]);
        out.annotate(Annotation::Label { id: format!("{}:ratio", j.id), anchor: centre.clone(), text: loc(&format!("ΣM_Rc / (1.3 ΣM_Rb) = {text}"), &format!("ΣM_Rc / (1,3 ΣM_Rb) = {text}")), tone: tone_of(&binding) }, &centre);
        if let Some(clause) = binding.as_ref().and_then(|x| clause_of(input.report, &x.check_id)) {
            out.annotate(Annotation::Clause { id: format!("{}:clause", j.id), anchor: entity_ref(EntityKind::Solid, &j.id, &path), clause }, &centre);
        }
    }
}
