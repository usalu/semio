use super::*;
#[test]
fn every_painted_shader_edit_finds_its_anchor_in_the_lit_shader_once() {
    for (anchor, _) in WORLD3D_PAINTED_SHADER_EDITS {
        assert_eq!(WORLD3D_SHADER.matches(anchor).count(), 1, "painted shader anchor: {anchor}");
    }
    let painted = world3d_painted_shader();
    assert!(painted.contains("let lit_color = in.color.rgb * paint_color;") && painted.contains("out.uv = vertex.uv;"), "every painted lane is applied");
}
