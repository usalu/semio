import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
if(process.argv[2]==="mount"){
  const pairs=JSON.parse(readFileSync(join(import.meta.dir,"held-pairs.json"),"utf8"));
  for(const pair of pairs)assert.equal(readFileSync(pair.path,"utf8"),pair.before,"Concurrent Curation retained scene consumer change: "+pair.path);
  for(const pair of pairs)writeFileSync(pair.path,pair.after);
  console.log("[DEBUG] Curation typed retained lane consumers mounted paths="+pairs.length+" original_ui_predicates_preserved=true");
}else{
const root="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any";
const pairs:{path:string,before:string,after:string}[]=[];
const change=(tail:string,apply:(source:string)=>string)=>{const path=join(root,tail),before=readFileSync(path,"utf8"),after=apply(before);assert.notEqual(after,before,tail);pairs.push({path,before,after})};
change("✏️editor/🧪️tests/🔬️unit/🦀️.rs",source=>{
  source=source.replace('serde_json::to_string(&app.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render").root).expect("render json")','semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render")).expect("bounded retained fixture projection")');
  const start=source.indexOf('    pub fn table_scene_of('),end=source.indexOf('\n    /// 📊️ Renders `body_key`',start);assert(start>=0&&end>start);
  return source.slice(0,start)+String.raw`    pub fn table_scene_of(node: &semio_framework_plugin::BuiltNode) -> Option<semio_framework_plugin::TableScene> {
        if let semio_framework_plugin::Component::Surface(props) = &node.component {
            if props.kind == semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::Table {
                let spine: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(props).expect("typed table spine");
                assert!(node.rejected_children.is_empty(), "exact retained table has no rejected children");
                assert_eq!(spine.lanes.len(), 2, "complete table lane declarations");
                assert_eq!(node.children.len(), spine.lanes.len(), "complete retained table carriers");
                for (name, key) in [("columns", semio_framework_ui_scene::TABLE_COLUMNS_LANE_KEY), ("rows", semio_framework_ui_scene::TABLE_ROWS_LANE_KEY)] {
                    let declared: Vec<_> = spine.lanes.iter().filter(|lane| lane.lane == name).collect();
                    assert_eq!(declared.len(), 1, "one exact declared table lane {name}");
                    let carriers: Vec<_> = node.children.iter().filter(|child| child.key.as_str() == key).collect();
                    assert_eq!(carriers.len(), 1, "one exact retained table carrier {key}");
                    let payload = semio_framework_plugin::artifact_app_laws::built_carrier_text(carriers[0]);
                    assert_eq!(payload.len(), declared[0].bytes as usize, "complete lane UTF-8 bytes");
                    assert_eq!(semio_framework_ui_scene::scene_lane_hash(&payload), declared[0].hash, "complete lane digest");
                }
                return Some(semio_framework_plugin::artifact_app_laws::built_surface_scene(node).expect("complete first-party retained table scene"));
            }
        }
        node.children.iter().find_map(table_scene_of)
    }
`+source.slice(end);
});
change("✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs",source=>source.replace('let scene: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(props).expect("table scene");','let scene = crate::editor::sourcing::unit_tests::context::table_scene_of(node.children.get(1).expect("table surface")).expect("complete retained table scene");'));
change("✏️editor/🎭️modes/✏️edit/🪟️windows/🧺️curated/🧪️tests/🔬️unit/🦀️.rs",source=>source.replace('let scene: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(props).expect("table scene");','let scene = crate::editor::sourcing::unit_tests::context::table_scene_of(&node).expect("complete retained table scene");'));
change("👁️viewer/🎭️modes/👁️view/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs",source=>source.replace('serde_json::to_string(&render(&document).expect("bounded table")).expect("render json")','semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&document).expect("bounded table"))).expect("bounded retained fixture projection")'));
writeFileSync(join(import.meta.dir,"held-pairs.json"),JSON.stringify(pairs,null,2)+"\n");console.log("[DEBUG] Curation retained consumer family held paths="+pairs.length+" production_mutations=0");
}
