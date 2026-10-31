//! 🧪️ Unit laws of the layout export JSON cursors: the typed writer against an independent serde oracle and the credit bound of the dynamic validator.

use super::json_validation::JSON_INPUT_BYTES_PER_UNIT;
use super::*;

#[test]
fn typed_document_json_matches_serde_and_every_write_is_credit_bounded() {
    use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText as _;

    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.name = "\u{1f642}\n".repeat(MAX_LAYOUT_EXPORT_STRING_BYTES / 5);
    snapshot.background_drawing =
        Some(semio_s_artifact_layout_layout_model::LayoutDrawingMember { handle: store::os_store::artifact_child::ArtifactChild::new("drawing-child".into(), semio_framework_artifact_reference::ArtifactRef { artifact_id: "document".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.draw.drawing".into(), standard: "1".into(), subset: "*".into() } }) });
    snapshot.referenced_model = Some(store::ArtifactLink { target: semio_framework_artifact_reference::ArtifactRef::parse_uri("document!s.stdio.semio@v1/model").expect("model reference"), pin: store::LinkPin::Head, role: "model".into() });
    let expected: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&snapshot)).expect("independent document JSON oracle");
    let mut cursor = TypedJsonCursor::document();
    let mut actual = Vec::new();
    loop {
        let (bytes, done) = cursor.advance(&snapshot).expect("typed JSON step");
        assert!(bytes.len() <= JSON_OUTPUT_BYTES_PER_UNIT);
        actual.extend_from_slice(&bytes);
        if done {
            break;
        }
    }
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&actual).expect("independent bounded JSON oracle"), expected);
}

#[test]
fn dynamic_json_validation_never_scans_more_than_its_input_credit() {
    let text = format!("[{}]", std::iter::repeat_n("null", MAX_LAYOUT_EXPORT_JSON_NODES - 1).collect::<Vec<_>>().join(","));
    let mut cursor = JsonValidationCursor::new(&text, true).expect("validator");
    loop {
        let before = cursor.byte_cursor;
        let done = cursor.advance(&text).expect("bounded validation step");
        assert!(cursor.byte_cursor - before <= JSON_INPUT_BYTES_PER_UNIT);
        if done {
            break;
        }
    }
}

#[test]
fn drawing_member_json_matches_reference_wire_and_preserves_partial_stop_sources(){
 use semio_framework_value::FromValue;
 use semio_s_artifact_layout_layout_model::{LayoutDrawingMember,DrawingSourceAddress,drawing_member_at};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("member.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let source=if row["source"]["kind"]=="document"{DrawingSourceAddress::Document}else{DrawingSourceAddress::Page(row["source"]["index"].as_u64().unwrap()as usize)};
  let member=LayoutDrawingMember::from_value(semio_framework_value::DslValue::from(row["member"].clone())).unwrap();
  let mut snapshot=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
  match source{DrawingSourceAddress::Document=>snapshot.background_drawing=Some(member),DrawingSourceAddress::Page(index)=>snapshot.pages[index].background_drawing=Some(member)}
  let original=snapshot.clone();
  let handle=&drawing_member_at(&snapshot,source).unwrap().unwrap().handle;
  let identities=(handle.child_id.as_ptr(),handle.target.artifact_id.as_ptr());
  for cut in fixture["partialStopAfter"].as_array().unwrap(){
   let mut cursor=ChildJsonCursor::new(source);
   assert!(cursor.owned_bytes()>0);
   for _ in 0..cut.as_u64().unwrap(){let(bytes,done)=cursor.advance(&snapshot).unwrap();assert!(bytes.len()<=JSON_OUTPUT_BYTES_PER_UNIT);if done{break}}
   assert_eq!(snapshot,original);
   drop(cursor);
   assert_eq!(snapshot,original);
  }
  let mut cursor=ChildJsonCursor::new(source);let mut bytes=Vec::new();
  loop{let(chunk,done)=cursor.advance(&snapshot).unwrap();assert!(chunk.len()<=JSON_OUTPUT_BYTES_PER_UNIT);bytes.extend(chunk);if done{break}}
  let actual:serde_json::Value=serde_json::from_slice(&bytes).unwrap();
  let reference=serde_json::to_vec(&row["expectedWire"]).unwrap();
  assert_eq!(actual,serde_json::from_slice::<serde_json::Value>(&reference).unwrap());
  assert!(actual.get("content").is_none());
  let handle=&drawing_member_at(&snapshot,source).unwrap().unwrap().handle;
  assert_eq!((handle.child_id.as_ptr(),handle.target.artifact_id.as_ptr()),identities);
  let expected:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&snapshot)).unwrap();
  let mut document=TypedJsonCursor::document();let mut output=Vec::new();
  loop{let(chunk,done)=document.advance(&snapshot).unwrap();assert!(chunk.len()<=JSON_OUTPUT_BYTES_PER_UNIT);output.extend(chunk);if done{break}}
  assert_eq!(serde_json::from_slice::<serde_json::Value>(&output).unwrap(),expected);
  assert_eq!(snapshot,original);
 }
}
#[test]
fn drawing_member_json_refuses_inline_content_and_unavailable_sources(){
 use semio_framework_value::FromValue;
 use semio_s_artifact_layout_layout_model::{LayoutDrawingMember,DrawingSourceAddress};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("member.json")).unwrap();
 for row in fixture["invalidMembers"].as_array().unwrap(){assert!(LayoutDrawingMember::from_value(semio_framework_value::DslValue::from(row["value"].clone())).is_err());}
 let snapshot=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();let original=snapshot.clone();
 for row in fixture["invalidSources"].as_array().unwrap(){
  let source=if row["kind"]=="document"{DrawingSourceAddress::Document}else{DrawingSourceAddress::Page(row["index"].as_u64().unwrap()as usize)};
  let mut cursor=ChildJsonCursor::new(source);
  loop{match cursor.advance(&snapshot){Err(_)=>break,Ok((bytes,done))=>{assert!(bytes.len()<=JSON_OUTPUT_BYTES_PER_UNIT);assert!(!done,"an unavailable member cannot complete a reference");}}}
  assert_eq!(snapshot,original);
 }
}
