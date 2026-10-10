import fs from "node:fs";
let s = fs.readFileSync("overlay.rs", "utf8");
s = s.replace("pub const NEUTRAL: [u8; 3] = [0x5c, 0x5c, 0x5c];", "pub const NEUTRAL: [u8; 3] = [0x80, 0x80, 0x80];");
s = s.replace("/// 🩶 The colour of a surface that states no U-value.", "/// 🩶 The colour of a surface that states no U-value: the no-data grey of the framework heatmap, which no step of the scale reaches.");
s = s.replace("`1.0` the grey mid-point and `2.0`\n//! or more red (poor); a surface without a U-value is [`NEUTRAL`], a dark grey that no step of the ramp reaches.", "`1.0` the light grey mid-point and `2.0`\n//! or more red (poor); a surface without a U-value is [`NEUTRAL`], the mid grey of the framework heatmap that no step of the ramp reaches.");
s = s.replace("//#endregion 🔖️Mesh", `/// 🌡️ The U-value of the surface behind every triangle of \`mesh\`, in triangle order: the values of the framework heatmap that paints the same scale and draws its legend.
pub fn u_values(parts: &[Part<'_>], mesh: &MeshData) -> Vec<Option<f64>> {
    mesh.face_ids.iter().map(|index| parts.get(*index as usize).and_then(|part| part.surface.u_value)).collect()
}
//#endregion 🔖️Mesh`);
fs.writeFileSync("overlay.rs", s);
let t = fs.readFileSync("overlay-tests.rs", "utf8");
t += `
#[test]
fn the_heatmap_values_follow_the_triangles_of_the_mesh() {
    let space = EnvelopeSpace::default();
    let wall = surface("a", Boundary::Exterior, Some(0.3), south_wall());
    let bare = surface("b", Boundary::Exterior, None, l_floor());
    let parts = [Part { space: &space, surface: &wall, placement: SolidPlacement::default() }, Part { space: &space, surface: &bare, placement: SolidPlacement::default() }];
    let mesh = mesh_of(&parts, Mode::UValue);
    let values = u_values(&parts, &mesh);
    assert_eq!(values.len(), 2 + 4);
    assert_eq!(values[..2], [Some(0.3), Some(0.3)]);
    assert!(values[2..].iter().all(Option::is_none));
    assert_eq!(mesh.colors.len(), values.len() * 12);
}
`;
fs.writeFileSync("overlay-tests.rs", t);
