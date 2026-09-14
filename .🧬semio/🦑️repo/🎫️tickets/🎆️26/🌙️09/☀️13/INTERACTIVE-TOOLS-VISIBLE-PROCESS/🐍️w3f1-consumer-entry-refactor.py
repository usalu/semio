"""Re-points the W3-F1 reorganize tool modules onto the layout-run consumer entry (layout_run_job, layout_run_entity,
layout_run_overlay_positions)."""
import re
import sys

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/"
TOOL = "/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs"

PLUGINS = {
    "reasoning": dict(path="💡️reasoning/🗿️artifacts/🔌️wires", graph="WiresLayoutGraph", mutation="WiresMutation", id="node_id", x="new_x", y="new_y", fault="reasoning.wires.layout-run",
                      signature="identity: ToolRunIdentity, snapshot: &WiresSnapshot, checkpoint: Option<&[u8]>, provisional: &[WiresMutation]",
                      layout="layout_graph(snapshot)", config="LayoutRunConfig::default()"),
    "dag": dict(path="🕸️dag/🗿️artifacts/🕸️dag", graph="DagLayoutGraph", mutation="DagMutation", id="id", x="x", y="y", fault="dag.layout-run",
                signature="identity: ToolRunIdentity, snapshot: &DagSnapshot, config: &DagConfig, checkpoint: Option<&[u8]>, provisional: &[DagMutation]",
                layout="layout_graph(snapshot, &layered_targets(snapshot, config)?)", config="layout_config()"),
}


def refactor(name: str) -> None:
    spec = PLUGINS[name]
    path = ROOT + spec["path"] + TOOL
    source = open(path, encoding="utf-8").read()
    start = source.index("/// 🪪️ A node's trace and provisional entity")
    end = source.index("}\n", start) + 2
    source = source[:start] + source[end:].lstrip("\n")
    source = source.replace("layout_entity(", "layout_run_entity(")
    start = source.index("/// ⏩️ The current positions a resumed run continues from")
    end = source.index("//#endregion 🔖️Job")
    block = (
        "/// 📍️ The overlay positions a resumed run continues from: the base origins with the provisional moves folded in.\n"
        f"fn overlay_positions(layout: &{spec['graph']}, provisional: &[{spec['mutation']}]) -> Vec<LayoutRunPoint> {{\n"
        "    let index: HashMap<&str, u32> = layout.node_ids.iter().enumerate().map(|(at, id)| (id.as_str(), at as u32)).collect();\n"
        "    let moves = provisional.iter().filter_map(|op| match op {\n"
        f"        {spec['mutation']}::MoveNode(payload) => index.get(payload.{spec['id']}.as_str()).map(|at| (*at, LayoutRunPoint::new(payload.{spec['x']}, payload.{spec['y']}))),\n"
        "        _ => None,\n"
        "    });\n"
        "    layout_run_overlay_positions(&layout.graph, moves)\n"
        "}\n\n"
        "/// 🧵️ Builds the layout run over the run's base; a settings change resumes it from the ledger's checkpoint with the\n"
        "/// provisional moves folded over the base positions (`📓️wave-W3-F.md` §3.3).\n"
        f"pub fn build_job({spec['signature']}) -> Result<ToolRunJob, Fault> {{\n"
        f"    let layout = {spec['layout']};\n"
        "    let resume = checkpoint.map(|checkpoint| LayoutRunResume { checkpoint, positions: overlay_positions(&layout, provisional), provisional_len: provisional.len() as u32 });\n"
        f"    let job = layout_run_job(identity, &layout.graph, {spec['config']}, || move_encoder(layout.node_ids.clone()), resume).map_err(|error| Fault::from(format!(\"{spec['fault']}: {{error:?}}\")))?;\n"
        "    Ok(Box::new(job))\n"
        "}\n"
    )
    source = source[:start] + block + source[end:]
    match = re.search(r"use semio_framework_graph_layout_run::\{([^}]*)\};", source)
    names = {item.strip() for item in match.group(1).split(",")}
    names -= {"LayoutRunJob", "LayoutRunResumeError"}
    names |= {"layout_run_entity", "layout_run_job", "layout_run_overlay_positions", "LayoutRunResume"}
    ordered = sorted(names, key=lambda item: (item[0].isupper(), item))
    source = source[: match.start()] + "use semio_framework_graph_layout_run::{" + ", ".join(ordered) + "};" + source[match.end():]
    open(path, "w", encoding="utf-8").write(source)


for plugin in sys.argv[1:]:
    refactor(plugin)
