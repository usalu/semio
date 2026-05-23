from pathlib import Path

p = Path("elements/client/lib/board/rs/lib.rs")
t = p.read_text(encoding="utf-8")
old_hover = """\t\tfn hovered_style_kind(&self, id: &str) -> Option<BoardElementStyleKind> {
\t\t\t(self.hovered_id.as_deref() == Some(id)).then_some(BoardElementStyleKind::Hovered)
\t\t}"""
new_hover = """\t\tfn hovered_style_kind(&self, id: &str) -> Option<BoardElementStyleKind> {
\t\t\tif self.is_preselect_active() {
\t\t\t\treturn None;
\t\t\t}
\t\t\t(self.hovered_id.as_deref() == Some(id)).then_some(BoardElementStyleKind::Hovered)
\t\t}"""
if old_hover in t:
    t = t.replace(old_hover, new_hover)
test = """
\t#[test]
\tfn board_host_area_select_from_empty_keeps_selection_until_commit() {
\t\tlet mut h = BoardHost::new();
\t\th.set_size(800, 600, 1.0);
\t\tset_detail_lod(&mut h);
\t\tlet mut desc = sample_scene();
\t\tdesc.nodes.push(NodeDescJson {
\t\t\tid: \"b\".into(),
\t\t\tx: 300.0,
\t\t\ty: 0.0,
\t\t\tdraggable: Some(true),
\t\t\tselected: None,
\t\t\tstyle: None,
\t\t\ttext: None,
\t\t\ticon_kind: None,
\t\t\tnode_kind: None,
\t\t\tuser_data: None,
\t\t\tvisible: None,
\t\t\troot: None,
\t\t\tshape: Some(\"circle\".into()),
\t\t\tradius: Some(40.0),
\t\t\twidth: None,
\t\t\theight: None,
\t\t\tscale: None,
\t\t});
\t\th.sync_descriptor(&desc).unwrap();
\t\th.set_selection_ids(&[]);
\t\tlet _ = h.drain_events_json();
\t\tlet w_down = Point::new(350.0, -50.0);
\t\tlet w_mid = Point::new(270.0, 50.0);
\t\tlet s_down = h.world_to_screen(w_down);
\t\tlet s_mid = h.world_to_screen(w_mid);
\t\th.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
\t\th.pointer_move_screen(s_mid.x, s_mid.y, false, false);
\t\tlet _ = h.drain_events_json();
\t\tassert!(h.is_dragging_area_select());
\t\tassert!(h.preselect.contains(\"b\"));
\t\tassert!(h.preselect_removed.is_empty());
\t\tassert!(h.selection.is_empty());
\t\th.pointer_up_screen(s_mid.x, s_mid.y, false, false);
\t\tlet _ = h.drain_events_json();
\t\tassert!(h.selection.contains(\"b\"));
\t\tassert!(h.preselect.is_empty());
\t}
"""
if "board_host_area_select_from_empty_keeps_selection_until_commit" not in t:
    t = t.replace(
        "\t#[test]\n\tfn board_host_minimap_preselect_matches_selected_chrome() {",
        test + "\n\t#[test]\n\tfn board_host_minimap_preselect_matches_selected_chrome() {",
    )
p.write_text(t, encoding="utf-8")
print("patched rust")
