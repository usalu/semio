"""Compile the checked-in Flow channel boundary with a compact synthetic DAG screen."""
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ticket = Path(__file__).resolve().parent
root = Path.cwd()
modules = root / "🧰️framework/🛍️products/💻️os/🔨️modules"
board = modules / "♾️infinite/🎲️board"
directed = board / "🔌️ports/➡️directed"
graph = directed / "🕸️dag/🕸️graph"
host = modules / "🌊️flow/🖥️host"
fixture = json.loads((host / "🧪️tests/🧫️fixtures/🎯️directional-picks.json").read_text())
version = "nightly-2026-07-20"


def functions(file, *names):
    text = file.read_text()
    pieces = []
    for name in names:
        match = re.search(r"^( *)(?:(?:pub(?:\([^)]*\))?) )?fn " + name + r"\(", text, re.M)
        indent = match[1]
        first = text[match.start():text.index("\n", match.start())]
        if not first.rstrip().endswith("}"):
            first = text[match.start():text.index("\n" + indent + "}", match.end()) + len(indent) + 2]
        pieces.append(first.replace("pub(super) ", ""))
    return "\n".join(pieces)


schema = (board / "🧬️schema/🪟️dag-views-traits-and-facts/🦀️.rs").read_text()
types = "\n".join(
    re.sub(r",Serialize,Deserialize|^#\[serde[^\n]*\n", "", match[0], flags=re.M)
    for name in ("DagChannelDirection", "DagChannelRef", "DagChannelView")
    for match in [re.search(r"^#\[derive[^\n]*(?:\n#\[[^\n]*)*\npub (?:enum|struct) " + name + r"[^\n]*", schema, re.M)]
)
hits = functions(graph / "📓️graph-state-journal-and-hits/🦀️.rs", "handle_id_for_port", "handle_id_for_channel", "handle_id_for_port_side", "port_row_handle_hit")
views = functions(graph / "🎚️graph-state-interaction-impl/🦀️.rs", "channel_view", "decode_channel_ref", "hovered_channel", "selected_channels", "is_preselect_active", "set_hover_ports", "clear_hover_ports", "handle_interaction_chrome")
viewport = functions(graph / "🔭️viewport-lod-grid-and-camera/🦀️.rs", "set_hover_channel", "set_hover_channel_direction", "set_selected_channels")
picker = functions(graph / "🖱️hover-rect-and-pick-targets/🦀️.rs", "dag_pick_targets_at_screen_json")
row = functions(graph / "🧮️world-snap-engine-sync-and-hit-tests/🦀️.rs", "channel_row_handle_hit")
host_boundary = functions(
    host / "🌳️host-evaluation-history/🎯️selection-hover-and-preview-toggles/🦀️.rs",
    "pick_targets_at_screen_json", "set_hover_channel_direction", "set_selected_channels", "selected_channels",
)
lod = functions(directed / "🧬️schema/🖼️presentation/🦀️.rs", "uses_channel_row_pick", "allows_connection_hit_picking")
scaffold = r"""
use std::collections::{BTreeSet, HashMap};
use semio_framework_value_derive::{ToValue, FromValue};

type HandleId = u64;

#[derive(Clone, Copy, PartialEq)]
enum HandleRole { Target, Source, Other }
#[derive(Clone, Copy, PartialEq)]
enum DagDrawLod { Detail, Micro, Normal, Minimap }
struct DagViewportState { lod: DagDrawLod }
#[derive(Clone, Copy)]
struct Point { x: f64, y: f64 }
struct Target { domain: String, id: u64, generality: u32 }
struct Port { id: String }
struct Node { id: String, inputs: Vec<Port>, outputs: Vec<Port> }
struct Handle { node_id: u64, role: HandleRole }
struct Snapshot { nodes: Vec<Node> }
#[derive(Default)]
struct Selection { node_ids: BTreeSet<u64>, edge_ids: BTreeSet<u64>, handle_ids: BTreeSet<u64> }
#[derive(Default)]
enum InteractionMode { #[default] Idle, SelectionPending {}, AreaSelect {} }
#[derive(Default)]
struct Engine {
    interaction: InteractionMode,
    handles: HashMap<u64, Handle>,
    hover: Option<u64>,
    selection: Selection,
    preselect: Selection,
    preselect_removed: Selection,
}
struct DagGraphState {
    host_snapshot: Snapshot,
    engine: Engine,
    hover_ports: Vec<HandleId>,
    handle_key_map: HashMap<u64, String>,
    node_id_map: HashMap<u64, usize>,
    edge_id_map: HashMap<u64, String>,
}
struct Dag { viewport: DagViewportState, graph: DagGraphState }
struct FlowHost { dag: Dag }

mod semio_os_infinite {
    pub mod board { pub mod schema { pub mod dag_input { pub use crate::{DagChannelDirection, DagChannelRef}; } } }
}
mod dag { pub use crate::set_hover_channel_direction; }

impl Node {
    fn inputs(&self) -> &[Port] { &self.inputs }
    fn outputs(&self) -> &[Port] { &self.outputs }
}
impl Engine {
    fn hit_test_pick_targets(&self, point: Point) -> Vec<Target> {
        let mut targets = vec![
            Target { domain: "node".into(), id: 1, generality: 0 },
            Target { domain: "edge".into(), id: 7, generality: 1 },
        ];
        if point.y == 0. {
            for (x, id) in [(-50., 10), (50., 11)] {
                if (x - point.x).abs() < 5. {
                    targets.push(Target { domain: "handle".into(), id, generality: 2 });
                }
            }
        }
        targets
    }
}
impl DagGraphState {
    fn widget_id_for_node_id(&self, id: u64) -> Option<String> {
        Some(self.host_snapshot.nodes[*self.node_id_map.get(&id)?].id.clone())
    }
    fn set_hover(&mut self, widget: Option<&str>) {
        self.clear_hover_ports();
        self.engine.hover = self.node_id_map.iter().find(|(_, index)| {
            Some(self.host_snapshot.nodes[**index].id.as_str()) == widget
        }).map(|(&id, _)| id);
    }
}
fn draw_lod_for_frame(viewport: &DagViewportState, _: &DagGraphState) -> DagDrawLod { viewport.lod }
fn screen_to_world_point(_: &DagViewportState, _: &DagGraphState, x: f64, y: f64) -> Point { Point { x, y } }
fn input_port_row_hit_bounds(node: &Node, i: usize) -> Option<(f64, f64, f64, f64)> {
    (i < node.inputs.len()).then_some((-50., -5., -1., 5.))
}
fn output_port_row_hit_bounds(node: &Node, i: usize) -> Option<(f64, f64, f64, f64)> {
    (i < node.outputs.len()).then_some((1., -5., 50., 5.))
}
fn point_in_rect(x: f64, y: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
    x0 <= x && x <= x1 && y0 <= y && y <= y1
}

fn check(port: &str, lod: DagDrawLod, x: f64, direction: DagChannelDirection) {
    let id = "variable";
    let nodes = vec![
        Node { id: id.into(), inputs: vec![Port { id: port.into() }], outputs: vec![Port { id: port.into() }] },
        Node { id: format!("{id}@nested"), inputs: vec![], outputs: vec![] },
    ];
    let mut engine = Engine {
        handles: HashMap::from([
            (10, Handle { node_id: 1, role: HandleRole::Target }),
            (11, Handle { node_id: 1, role: HandleRole::Source }),
            (12, Handle { node_id: 2, role: HandleRole::Source }),
            (13, Handle { node_id: 1, role: HandleRole::Other }),
        ]),
        ..Default::default()
    };
    engine.selection.node_ids.insert(1);
    engine.selection.edge_ids.insert(7);
    let graph = DagGraphState {
        host_snapshot: Snapshot { nodes },
        handle_key_map: HashMap::from([
            (10, format!("{id}@{port}")), (11, format!("{id}@{port}")),
            (12, format!("{id}@nested@out")), (13, format!("{id}@other")),
        ]),
        node_id_map: HashMap::from([(1, 0), (2, 1)]),
        edge_id_map: HashMap::from([(7, "wire".into())]),
        hover_ports: Vec::new(),
        engine,
    };
    let mut host = FlowHost { dag: Dag { viewport: DagViewportState { lod }, graph } };
    println!("{}", host.pick_targets_at_screen_json(x, 0.));
    host.set_hover_channel_direction(Some(id), Some(port), Some(direction));
    let channel = DagChannelRef { widget_id: id.into(), port: port.into(), direction };
    assert_eq!(host.dag.graph.hovered_channel().as_ref(), Some(&channel));
    let opposite = DagChannelRef {
        widget_id: id.into(), port: port.into(),
        direction: if direction == DagChannelDirection::In { DagChannelDirection::Out } else { DagChannelDirection::In },
    };
    let collision = DagChannelRef { widget_id: id.into(), port: "nested@out".into(), direction: DagChannelDirection::Out };
    host.dag.graph.set_hover_ports(&[channel.clone(), opposite.clone(), collision.clone()]);
    assert!(host.dag.graph.handle_interaction_chrome(10).2);
    assert!(host.dag.graph.handle_interaction_chrome(11).2);
    assert!(!host.dag.graph.handle_interaction_chrome(12).2);
    host.dag.graph.set_hover_ports(&[opposite.clone()]);
    assert_eq!(host.dag.graph.hover_ports, vec![if direction == DagChannelDirection::In { 11 } else { 10 }]);
    host.set_hover_channel_direction(Some(id), Some(port), Some(direction));
    assert!(host.dag.graph.hover_ports.is_empty());
    host.set_selected_channels(&[channel.clone(), opposite.clone(), collision]);
    let selected = host.selected_channels();
    assert_eq!(selected.len(), 2);
    assert!(selected.contains(&channel) && selected.contains(&opposite));
    host.set_selected_channels(&[]);
    assert!(host.selected_channels().is_empty());
    assert_eq!(host.dag.graph.engine.selection.node_ids, BTreeSet::from([1]));
    assert_eq!(host.dag.graph.engine.selection.edge_ids, BTreeSet::from([7]));
}
"""

port = fixture["widget"]["name"]
cases = fixture["cases"]
calls = []
for case in cases:
    direction = case["direction"]
    x = (25 if case["probe"] == "row" else 50) * (1 if direction == "out" else -1)
    calls.append(f'check({json.dumps(port)}, DagDrawLod::{case["lod"].title()}, {x}., DagChannelDirection::{direction.title()});')

capacity_root = modules / "🌊️flow/🕸️wasm"
capacity_source = capacity_root / "🎬️actions-a/🖱️computing-hover-and-preview/🦀️.rs"
capacity_fixture = (capacity_root / "🧪️tests/🧫️fixtures/🎯️hover-group-capacity.json").resolve()
abi_limits = root / "🧰️framework/🔨️modules/🌉️abi/🪪️limits-and-identifiers/🦀️.rs"
capacity = functions(capacity_source, "decode_hover_ports", "full_hover_roster_fits_the_shared_abi_body")
capacity = capacity.replace('include_str!("../../🧪️tests/🧫️fixtures/🎯️hover-group-capacity.json")', f'include_str!({json.dumps(str(capacity_fixture), ensure_ascii=False)})')
capacity = capacity.replace("semio_os_infinite::board::schema::dag_input::DagChannelDirection", "DagChannelDirection")
capacity_types = """
struct FlowFailure;
enum AbiErrorCode { MalformedTag }
impl FlowFailure { fn new(_: AbiErrorCode, _: &str) -> Self { Self } }
"""
capacity_limit = re.search(r"^pub const ABI_MAX_BODY_BYTES.*;$", abi_limits.read_text(encoding="utf8"), re.M)[0]

source = "\n".join([
    scaffold, types, capacity_types, capacity_limit, capacity, "impl DagGraphState {", hits, views, "}", viewport, picker, row,
    "impl DagDrawLod {", lod, "}", "impl FlowHost {", host_boundary, "}", "fn main() {", "full_hover_roster_fits_the_shared_abi_body();", *calls, "}",
])

lane = Path(sys.argv[1]) if len(sys.argv) > 1 else root / ".cache/b/r/m/u/l/p1c4772/prototype/build"
artifact_dirs = sorted(lane.glob("*/*/out"))
libraries = {item.stem.removeprefix("lib"): item for directory in artifact_dirs for item in directory.glob("lib*") if item.suffix in {".rlib", ".dylib"}}
pack_key = "semio_framework_pack_json-5262b28961bbf9e7"
pack = libraries[pack_key]
lineage = dict(json.loads(pack.with_suffix(".rmeta.semio-deps").read_text())["dependencies"])
direct = {"semio_framework_pack_json": pack}
for name in ("semio_framework_value", "semio_framework_value_derive"):
    direct[name] = libraries[next(key for key in lineage if key.rsplit("-", 1)[0] == name)]
compiler = ["repo", "tools", "exec", "--light", "rustup", "--", "run", version, "rustc"]
generated = ticket / "🗑️generated"
created = not generated.exists()
generated.mkdir(exist_ok=True)
try:
    with tempfile.TemporaryDirectory(prefix="flow-proof-", dir=generated) as temp:
        input_path = Path(temp) / "main.rs"
        input_path.write_text(source, encoding="utf8")
        binary = Path(temp) / ("flow.exe" if os.name == "nt" else "flow")
        args = [str(input_path), "--edition=2024", "-Awarnings", "-C", "debuginfo=0", "-o", str(binary)]
        for directory in artifact_dirs:
            args.extend(["-L", f"dependency={directory}"])
        for name, library in direct.items():
            args.extend(["--extern", f"{name}={library}"])
            if library.with_suffix(".rmeta").is_file():
                args.extend(["--extern", f"{name}={library.with_suffix('.rmeta')}"])
        result = subprocess.run(compiler + args)
        if result.returncode:
            raise SystemExit(result.returncode)
        exec_file = ["repo", "tools", "exec", "--light", "python", "--", "-c", "import os,sys; os.execv(sys.argv[1],sys.argv[1:])"]
        lines = subprocess.check_output(exec_file + [str(binary)], text=True).splitlines()
        assert len(lines) == len(cases), lines
        for case, line in zip(cases, lines):
            rows = json.loads(line)
            handles = [row for row in rows if row["domain"] == "handle"]
            expected = {
                "domain": "handle", "id": f"variable@{port}", "generality": 2, "label": f"variable · {port}",
                "channel": {"widgetId": "variable", "port": port, "direction": case["direction"]},
            }
            assert handles == [expected], (case, rows)
            assert [row for row in rows if row["domain"] != "handle"] == [
                {"domain": "node", "id": "variable", "generality": 0},
                {"domain": "edge", "id": "wire", "generality": 1},
            ], rows
        print("[DEBUG] Five native Flow boundary cases matched the independent Python JSON oracle. Full 1024-channel admission passed.")
finally:
    if created:
        try:
            generated.rmdir()
        except OSError:
            pass
