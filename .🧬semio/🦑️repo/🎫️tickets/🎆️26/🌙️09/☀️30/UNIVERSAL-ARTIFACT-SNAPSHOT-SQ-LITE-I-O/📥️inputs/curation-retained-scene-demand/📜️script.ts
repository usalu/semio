import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
const root="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any";
const columns=[{id:"name",label:"Name / Name",sortable:true},{id:"note",label:"Notiz / Note",sortable:false}];
const rows=[{id:"α",name:"Tür 🧱",note:"A retained table publishes its exact row cells in ordered text pages. Every page remains part of the same scene lane, including escaped quotes \"double\", backslashes \\, newlines\n, German umlauts äöüß, and a complete astral scalar 🧱. The receiver reads all packed attribute slices in order before validating the lane's UTF-8 byte count and digest. A retained table publishes its exact row cells in ordered text pages. Every page remains part of the same scene lane, including escaped quotes \"double\", backslashes \\, newlines\n, German umlauts äöüß, and a complete astral scalar 🧱. The receiver reads all packed attribute slices in order before validating the lane's UTF-8 byte count and digest. A retained table publishes its exact row cells in ordered text pages. Every page remains part of the same scene lane, including escaped quotes \"double\", backslashes \\, newlines\n, German umlauts äöüß, and a complete astral scalar 🧱. The receiver reads all packed attribute slices in order before validating the lane's UTF-8 byte count and digest."},{id:"β",name:"Fenster",note:""},{id:"γ",name:"Stair",note:null}];
const hash=(text:string)=>{let value=0xcbf29ce484222325n;for(const byte of Buffer.from(text,"utf8"))value=BigInt.asUintN(64,(value^BigInt(byte))*0x100000001b3n);return value.toString(16).padStart(16,"0")};
const fixture={schemaVersion:1,surfaceSchema:"table@1",columns,rows,columnsJson:JSON.stringify(columns),rowsJson:JSON.stringify(rows),lanes:[{name:"columns",key:"framework.scene.table.columns",bytes:Buffer.byteLength(JSON.stringify(columns)),hash:hash(JSON.stringify(columns))},{name:"rows",key:"framework.scene.table.rows",bytes:Buffer.byteLength(JSON.stringify(rows)),hash:hash(JSON.stringify(rows))}]};
const fixturePath=join(root,"🧫️fixtures/🚚️retained-table/🔣️.json"),schemaPath=join(root,"🧫️fixtures/🚚️retained-table/🧬️schema/🔣️.json"),testPath=join(root,"✏️editor/🧪️tests/🔬️unit/🦀️.rs");
const before=readFileSync(testPath,"utf8");assert(!before.includes("retained_table_lanes_restore_complete_neutral_owner"));
const demand=String.raw`

#[test]
fn retained_table_lanes_restore_complete_neutral_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../\u{1f9eb}\u{fe0f}fixtures/\u{1f69a}\u{fe0f}retained-table/\u{1f523}\u{fe0f}.json")).unwrap();
    let scene = semio_framework_plugin::TableScene::base(fixture["columnsJson"].as_str().unwrap().into(), fixture["rowsJson"].as_str().unwrap().into());
    let node = semio_framework_plugin::scene_surface("neutral-retained-table", semio_framework_plugin::SurfaceKind::Table, &scene).unwrap();
    assert_eq!(node.children.len(), 2);
    assert!(serde_json::to_string(&node).unwrap_err().to_string().contains("BuiltChildren requires retained page transport"));
    let semio_framework_plugin::Component::Surface(props) = &node.component else { panic!("typed table surface") };
    let spine: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(props).unwrap();
    assert_eq!(props.doc_schema.as_str(), fixture["surfaceSchema"].as_str().unwrap());
    assert!(spine.columns_json.is_empty() && spine.rows_json.is_empty());
    assert_eq!(spine.lanes.len(), fixture["lanes"].as_array().unwrap().len());
    for (actual, expected) in spine.lanes.iter().zip(fixture["lanes"].as_array().unwrap()) {
        assert_eq!(actual.lane, expected["name"].as_str().unwrap());
        assert_eq!(u64::from(actual.bytes), expected["bytes"].as_u64().unwrap());
        assert_eq!(actual.hash, expected["hash"].as_str().unwrap());
    }
    let restored = context::table_scene_of(&node).expect("complete retained table");
    assert_eq!(restored.columns_json, scene.columns_json);
    assert_eq!(restored.rows_json, scene.rows_json);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&restored.columns_json).unwrap(), fixture["columns"]);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&restored.rows_json).unwrap(), fixture["rows"]);
    eprintln!("[DEBUG] Curation retained table complete owner lanes=2 rows=3 bytes={} exact_utf8_digest=true", restored.rows_json.len());
}
`;
const pairs=[{path:fixturePath,before:null,after:JSON.stringify(fixture,null,2)+"\n"},{path:schemaPath,before:null,after:JSON.stringify({$schema:"https://json-schema.org/draft/2020-12/schema",const:fixture},null,2)+"\n"},{path:testPath,before,after:before+demand}];
writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify(pairs,null,2)+"\n");assert.equal(readFileSync(testPath,"utf8"),before);
for(const pair of pairs){mkdirSync(join(pair.path,".."),{recursive:true});writeFileSync(pair.path,pair.after)}
console.log("[DEBUG] Curation complete retained table neutral demand mounted paths=3 independent_buffer_utf8_fnv=true");
