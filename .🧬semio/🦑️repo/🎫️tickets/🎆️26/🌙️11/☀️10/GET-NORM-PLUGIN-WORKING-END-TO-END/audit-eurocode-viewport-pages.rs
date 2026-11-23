use pack::snapshot::ArtifactDsl;
use semio_plugin::{BuiltNode, World3dSceneLane};
use semio_s_plugin_norm_app_surface::viewport3d::{config::Lang, lowering, toolbar::{lowered_viewport, ViewportView}};
use semio_ui_contract::{Component, UI_DOCUMENT_NODES};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[path = "eurocode-ui-arena.rs"]
mod arena;
#[path = "../../../../../../../✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/🧊️viewport3d/🖼️window/🦀️.rs"]
mod window;

macro_rules! family {
    ($($family:literal => $snapshot:ident),+) => {$(
        #[cfg(feature = $family)]
        use audited_model::$snapshot as Snapshot;
    )+};
}

family!("en1990" => En1990Snapshot, "en1991" => En1991Snapshot, "en1992" => En1992Snapshot, "en1993" => En1993Snapshot,
    "en1994" => En1994Snapshot, "en1995" => En1995Snapshot, "en1996" => En1996Snapshot, "en1997" => En1997Snapshot,
    "en1998" => En1998Snapshot, "en1999" => En1999Snapshot);

fn census(node: &BuiltNode) -> usize {
    1 + node.children.iter().map(census).sum::<usize>()
}

fn carrier_text(node: &BuiltNode, payload: &mut String) {
    if let Component::Text(text) = &node.component {
        payload.push_str(&text.value.to_string());
        if let Some(attributes) = &text.data_attributes {
            let fields: BTreeMap<_, _> = attributes.iter().map(|(key, value)| (key.as_str(), value.as_str())).collect();
            for value in fields.values() { payload.push_str(value); }
        }
    }
    for child in &node.children { carrier_text(child, payload); }
}

fn records(node: &BuiltNode, lane: World3dSceneLane) -> Vec<Value> {
    let root = node.children.iter().find(|child| child.key.as_str() == lane.body_key()).expect("typed lane");
    let mut payload = String::new();
    carrier_text(root, &mut payload);
    serde_json::from_str(&payload).expect("independent JSON decoder")
}

fn id(value: &Value) -> &str {
    value["id"].as_str().expect("record identity")
}

fn mesh_triangles(mesh: &Value) -> Vec<[Vec<Value>; 3]> {
    let data = &mesh["data"];
    let indices = data["indices"].as_array().expect("triangle indices");
    let mut output = Vec::with_capacity(indices.len());
    for index in indices {
        let index = index.as_u64().expect("vertex index") as usize;
        output.push([("positions", 3), ("normals", 3), ("colors", 4)].map(|(lane, stride)| {
            data[lane].as_array().expect("vertex data")[index * stride..(index + 1) * stride].to_vec()
        }));
    }
    output
}

fn portable(node: &BuiltNode, key: &str) -> Value {
    serde_json::json!({
        "key": key, "component": node.component, "layout": node.layout, "style": node.style,
        "accessibility": node.accessibility, "children": node.children.iter().map(|child| portable(child, child.key.as_str())).collect::<Vec<_>>(),
    })
}

fn main() {
    let arena = arena::register();
    let mut args = std::env::args().skip(1).peekable();
    let chrome = args.next().expect("chrome nodes").parse::<usize>().expect("numeric chrome nodes");
    let emit = args.next_if(|argument| argument == "--emit").is_some();
    #[cfg(feature = "en1992")]
    let geometry_only = args.next_if(|argument| argument == "--geometry-only").is_some();
    for path in args {
        let document = Snapshot::parse_dsl(&std::fs::read_to_string(&path).expect("asset")).expect("model");
        #[cfg(feature = "en1992")]
        let report = if geometry_only { Default::default() } else {
            audited_report::live::responsive(&document, semio_s_plugin_norm_app_surface::PAINT_BUDGET).0
        };
        #[cfg(not(feature = "en1992"))]
        let report = audited_report::live::report(&document);
        #[cfg(feature = "en1990")]
        let scene = audited_scene::live_scene_of(&document, &report, &Default::default());
        #[cfg(not(feature = "en1990"))]
        let scene = audited_scene::live_scene_of(&document, &report);
        for lang in [Lang::En, Lang::De] {
            let authored = lowered_viewport(&scene, &ViewportView::default(), &[], lang);
            let source = window::world3d_scene_of(&authored, "norm.fixture", false);
            let expected: Vec<Value> = serde_json::from_str(&source.instances_json).expect("source instances");
            let expected_meshes: Vec<Value> = serde_json::from_str(&source.meshes_json).expect("source meshes");
            let expected_meshes: BTreeMap<_, _> = expected_meshes.iter().map(|mesh| (id(mesh), mesh)).collect();
            let mut seen = Vec::new();
            let mut fragmented = BTreeMap::<String, Vec<[Vec<Value>; 3]>>::new();
            let mut page = 0;
            let mut largest = 0;
            let mut most_items = 0;
            let smallest = usize::from(!expected.is_empty());
            loop {
                let (node, actual, count) = window::render_viewport_page("norm.fixture.viewport", &authored, "norm.fixture", false, page, chrome, "Viewport").expect("bounded scene");
                assert_eq!(actual, page);
                let nodes = census(&node) + chrome;
                largest = largest.max(nodes);
                assert!(nodes <= UI_DOCUMENT_NODES);
                let items = semio_ui_runtime::surface_subtree_items(&node).expect("real reconciler census");
                most_items = most_items.max(items);
                assert!(1 + items + chrome * 32 <= semio_ui_runtime::SURFACE_RECONCILE_MAX_ITEMS,
                    "{path} {lang:?} page={page} surface_items={items} chrome={chrome}");
                let instances = records(&node, World3dSceneLane::Instances);
                let meshes = records(&node, World3dSceneLane::Meshes);
                assert!(instances.len() >= smallest);
                let references = instances.iter().map(|item| item["meshId"].as_str().expect("mesh ref")).collect::<BTreeSet<_>>();
                assert_eq!(meshes.iter().map(id).collect::<BTreeSet<_>>(), references);
                let page_meshes: BTreeMap<_, _> = meshes.iter().map(|mesh| (id(mesh), mesh)).collect();
                for instance in instances {
                    let mesh = &page_meshes[instance["meshId"].as_str().expect("mesh ref")];
                    if let Some((original_id, part)) = id(&instance).rsplit_once("#mesh-page=") {
                        let original = expected.iter().find(|item| id(item) == original_id).expect("original exact instance");
                        assert_eq!(instance["interactionId"].as_str(),
                            original["interactionId"].as_str().or(Some(original_id)));
                        let position = part.parse::<usize>().expect("part ordinal");
                        if position == 0 { seen.push(original.clone()); }
                        fragmented.entry(original_id.to_owned()).or_default().extend(mesh_triangles(mesh));
                    } else {
                        assert_eq!(expected_meshes[id(mesh)], *mesh);
                        seen.push(instance);
                    }
                }
                if emit && matches!(lang, Lang::En) && [0, count / 2, count - 1].contains(&page) {
                    let key = format!("norm.fixture.page-{page}.model");
                    println!("[DEBUG] retained {}", portable(&node, &key));
                }
                drop(node);
                arena::drain();
                page += 1;
                if page == count { break; }
            }
            assert_eq!(seen, expected);
            for (instance, triangles) in &fragmented {
                let original = expected.iter().find(|item| id(item) == instance).unwrap();
                assert_eq!(*triangles, mesh_triangles(expected_meshes[original["meshId"].as_str().unwrap()]));
            }
            let (last, actual, count) = window::render_viewport_page("norm.fixture.viewport", &authored, "norm.fixture", false, usize::MAX, chrome, "Viewport").expect("clamped page");
            assert_eq!((actual, count), (page - 1, page));
            drop(last);
            arena::drain();
            println!("[DEBUG] OK {path} {lang:?} pages={page} max_nodes={largest} instances={} surface_items={most_items} fragmented={}", seen.len(), fragmented.len());
        }
    }
    arena::close(arena);
}
