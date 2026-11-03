//! 🧱️ Wall rows of a building in 3D at their plan positions: one oriented body per wall row and storey, the stiffness centre that the positioned walls give and the grips on the wall start, end and direction.

use super::handles::Index;
use super::kit::*;
use super::model::{En1998Building, En1998Member};
use super::response::Response;
use super::scene::*;
use super::schema::wall_plan;
use super::SceneInput;

/// 🧱 Id of the body of the wall row `member` in the storey row `storey_id`.
pub fn body_id(building_id: &str, member: &str, storey_id: &str) -> String {
    format!("{building_id}:wall:{member}:{storey_id}")
}

/// 🧱 The wall rows with their storey indices `(member, storey index)`: a wall without a storey reference stands in every storey.
pub fn rows_of(b: &En1998Building) -> Vec<(&En1998Member, usize)> {
    b.members.iter().filter(|m| wall_plan::is_wall_row(m)).flat_map(move |m| b.storeys.iter().enumerate().filter(move |(_, s)| m.wall_storey_id.is_empty() || m.wall_storey_id == s.id).map(move |(index, _)| (m, index))).collect()
}

fn role_of(member: &En1998Member) -> MaterialRole {
    if member.material == semio_s_artifact_norm_en1998_model::StructuralMaterial::Masonry { MaterialRole::Masonry } else { MaterialRole::Concrete }
}

/// 🧱 One body per wall row and storey from the wall start along its direction for its length, as thick as the row, as high as the storey below the slab; masonry walls take the colour of the `masonry.simple` check.
pub fn wall_bodies(input: &SceneInput, handles: &Index, b: &En1998Building, r: &Response, o: &Vec3, bp: &str, out: &mut Out) {
    let binding = binding_where(input.report, |c| c.subject.entity_id == b.id && c.id.ends_with(".masonry.simple"));
    for (m, index) in rows_of(b) {
        let storey = &b.storeys[index];
        let (bottom, top) = (if index == 0 { 0.0 } else { r.elevations[index - 1] }, r.elevations[index]);
        let (s, c) = m.wall_direction_rad.sin_cos();
        let origin = v3(o.x + m.wall_start_x_m + 0.5 * m.wall_thickness_m * s, o.y + m.wall_start_y_m - 0.5 * m.wall_thickness_m * c, o.z + bottom);
        let size = v3(m.wall_length_m, m.wall_thickness_m, (top - bottom - SLAB_THICKNESS).max(0.1));
        let id = body_id(&b.id, &m.id, &storey.id);
        let masonry = m.material == semio_s_artifact_norm_en1998_model::StructuralMaterial::Masonry;
        let name = loc(
            &format!("{} wall {} at storey {}: l = {:.2} m, t = {:.2} m", if masonry { "Masonry" } else { "Reinforced concrete" }, m.id, storey.id, m.wall_length_m, m.wall_thickness_m),
            &format!("{}wand {} im Geschoss {}: l = {:.2} m, t = {:.2} m", if masonry { "Mauerwerks" } else { "Stahlbeton" }, m.id, storey.id, m.wall_length_m, m.wall_thickness_m),
        );
        let mut spec = Spec::new(EntityKind::Solid, id.clone(), format!("{bp}.members[id={}]", m.id), yaw_box_solid(origin.clone(), m.wall_direction_rad, size.clone()), role_of(m), name, LAYER_STOREYS);
        spec.binding = if masonry { binding.clone() } else { None };
        spec.opacity = 0.8;
        spec.handles = handles.of(&id);
        let end = v3(o.x + m.wall_start_x_m + m.wall_length_m * c, o.y + m.wall_start_y_m + m.wall_length_m * s, o.z + bottom + size.z);
        out.push(spec, &[origin, end]);
    }
}

/// 🎯 The stiffness centre that the positioned walls give, as a labelled marker per storey with the torsional radii; nothing without a wall plan.
pub fn stiffness_centre(b: &En1998Building, o: &Vec3, top: f64, storey_index: usize, out: &mut Out) {
    let storey = &b.storeys[storey_index];
    let Some(plan) = wall_plan::wall_plan_of_storey(&wall_plan::placed_walls_of(&b.members), storey) else { return };
    let at = v3(o.x + plan.centre_of_stiffness_x_m, o.y + plan.centre_of_stiffness_y_m, o.z + top);
    let text = loc(
        &format!("Walls of {}: centre of stiffness ({:.2}, {:.2}) m, r_x = {:.2} m, r_y = {:.2} m, stiffness {}", storey.id, plan.centre_of_stiffness_x_m, plan.centre_of_stiffness_y_m, plan.radius_x_m, plan.radius_y_m, plan.stiffness_factor_text()),
        &format!("Wände von {}: Steifigkeitsmittelpunkt ({:.2}, {:.2}) m, r_x = {:.2} m, r_y = {:.2} m, stiffness {}", storey.id, plan.centre_of_stiffness_x_m, plan.centre_of_stiffness_y_m, plan.radius_x_m, plan.radius_y_m, plan.stiffness_factor_text()),
    );
    out.annotate(Annotation::Label { id: format!("{}:{}:walls-cos", b.id, storey.id), anchor: at.clone(), text, tone: Tone::Info }, &at);
}
