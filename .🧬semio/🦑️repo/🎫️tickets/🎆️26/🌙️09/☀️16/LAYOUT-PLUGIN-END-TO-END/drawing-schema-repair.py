from pathlib import Path
import hashlib,json
root=Path.cwd()
base=root/"✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any"
paths=["🧬️schema/🦀️.rs","🧬️schema/📸️snapshot/🦀️.rs","🧬️schema/📸️snapshot/🧪️tests/🔬️round-trip/🦀️.rs","✏️editor/⚙️engine/📤️export/🔣️json/🧪️tests/🔬️unit/🦀️.rs"]
raw={x:(base/x).read_bytes() for x in paths}
texts={x:b.decode("utf-8") for x,b in raw.items()}
s=texts[paths[0]]
start=s.index("/// 🌉️ Layout carries one persisted background child")
end=s.index("/// 📄️ A page visits genuine Drawing references",start)
assert "pub struct LayoutDrawingChild" in s[start:end]
s=s[:start]+s[end:]
texts[paths[0]]=s
s=texts[paths[1]]
start=s.index("/// 📸️ Persisted layout document snapshot")
end=s.index("#[derive(",start)
s=s[:start]+"""/// 📸️ Persists Layout document fields and genuine Drawing member references.
/// Current child heads belong to their independent member stores; authored initial payloads remain in separate Drawing births.
/// Derived child/link slot tables and text/pack codecs share the same reference-only schema.
"""+s[end:]
texts[paths[1]]=s
s=texts[paths[2]]
old='Some(LayoutDrawingChild { handle: store::os_store::artifact_child::ArtifactChild::new("child-drawing-1".to_string(), semio_framework_artifact_reference::ArtifactRef::parse_uri("doc-1!s.stdio.semio@v1/drawing").expect("valid child ref uri")), content: semio_framework_value::DslValue::Null });'
new='Some(LayoutDrawingMember { handle: store::os_store::artifact_child::ArtifactChild::new("child-drawing-1".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "doc-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.draw.drawing".into(), standard: "1".into(), subset: "*".into() } }) });'
assert s.count(old)==1
s=s.replace(old,new,1)
start=s.index("/// 🧪️ Every field on")
end=s.index("#[semio_framework_async_macros::async_test]",start)
s=s[:start]+"""/// 🧪️ Genuine Drawing references and the model link survive both canonical text and binary codecs.
"""+s[end:]
texts[paths[2]]=s
s=texts[paths[3]]
old='Some(semio_s_artifact_layout_layout_model::LayoutDrawingChild { handle: store::os_store::artifact_child::ArtifactChild::new("drawing-child".into(), semio_framework_artifact_reference::ArtifactRef::parse_uri("document!s.stdio.semio@v1/drawing").expect("child reference")), content: scene_content(&Default::default()) });'
new='Some(semio_s_artifact_layout_layout_model::LayoutDrawingMember { handle: store::os_store::artifact_child::ArtifactChild::new("drawing-child".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "document".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.draw.drawing".into(), standard: "1".into(), subset: "*".into() } }) });'
assert s.count(old)==1
texts[paths[3]]=s.replace(old,new,1)
rows=[]
for x in paths:
 if (base/x).read_bytes()!=raw[x]:raise RuntimeError("Source changed before guarded write")
for x in paths:
 output=texts[x].encode("utf-8")
 (base/x).write_bytes(output)
 rows.append({"path":str((base/x).relative_to(root)),"beforeSha256":hashlib.sha256(raw[x]).hexdigest(),"sha256":hashlib.sha256(output).hexdigest()})
ticket=Path(__file__).resolve().parent
(ticket/"🗑️generated/drawing-schema-repair.json").write_text(json.dumps({"sourceAuthored":True,"sourceReady":False,"semanticExecuted":False,"files":rows},ensure_ascii=False,indent=2)+"\n")
print(json.dumps(rows,ensure_ascii=False))
