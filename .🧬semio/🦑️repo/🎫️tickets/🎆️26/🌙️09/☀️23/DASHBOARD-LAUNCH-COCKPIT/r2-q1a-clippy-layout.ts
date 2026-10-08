import { patch } from "./r2-q1a-edit.ts";

const edits: Array<[string, string]> = [];
const replace = (from: string, to: string) => edits.push([from, to]);

replace(`#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dimension {
    Auto,
    Cells(u16),
    Weight(u16),
}

impl Default for Dimension {
    fn default() -> Self {
        Dimension::Auto
    }
}
`, `#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Dimension {
    #[default]
    Auto,
    Cells(u16),
    Weight(u16),
}
`);
replace("    if weight_total > 0 {\n        let mut remainders", "    if let Some(weight_total) = std::num::NonZeroU32::new(weight_total) {\n        let mut remainders");
replace("remainders.sort_by(|a, b| b.1.cmp(&a.1));", "remainders.sort_by_key(|remainder| std::cmp::Reverse(remainder.1));");
replace(`    let id = node.window_kind_id.clone();
    with_stack_mut(&mut layout.root, host_window_kind_id, &mut |s| {
        s.children.push(node.clone());
        s.active_window_kind_id = Some(id.clone());
        true
    })`, `    let id = node.window_kind_id.clone();
    let mut node = Some(node);
    with_stack_mut(&mut layout.root, host_window_kind_id, &mut |s| match node.take() {
        Some(node) => {
            s.children.push(node);
            s.active_window_kind_id = Some(id.clone());
            true
        }
        None => false,
    })`);
replace(`    let mut done = false;
    with_stack_mut(&mut layout.root, window_kind_id, &mut |s| {
        if let Some(node) = s.children.iter_mut().find(|c| c.window_kind_id == window_kind_id) {
            node.title = title.clone();`, `    let mut done = false;
    let mut title = Some(title);
    with_stack_mut(&mut layout.root, window_kind_id, &mut |s| {
        if let Some(node) = s.children.iter_mut().find(|c| c.window_kind_id == window_kind_id) {
            node.title = title.take().flatten();`);
patch("⌨️tui/📏️layout/🦀️.rs", edits);
console.log("layout fixed");
