//! 🧪️ Actual parent member events remove only the selected reference; child artboards require qualified commands.
use super::*;
#[test]fn page_drawing_parent_events_preserve_genuine_member_references(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let transport:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/🧬️mutations/🖊️set-page-background-drawing/🧪️tests/🔣️transport.json")).unwrap();let member:semio_s_artifact_layout_layout_model::LayoutDrawingMember=semio_framework_pack_json::from_json_str(&transport["cases"][0]["input"]["SetPageBackgroundDrawing"]["drawing"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let mut doc=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();doc.background_drawing=Some(member.clone());doc.pages[0].background_drawing=Some(member);let original=doc.clone();
 for row in fixture["commands"].as_array().unwrap(){let mutation=page_field_mutation(&doc,&doc.pages[0],row["field"].as_str().unwrap(),row["value"].as_str().unwrap()).unwrap();let LayoutMutation::SetPageBackgroundDrawing(payload)=mutation else{panic!("one actual parent reference event")};assert_eq!(payload.page_id,doc.pages[0].id);assert!(payload.drawing.is_none());assert_eq!(doc,original)}
 for field in fixture["qualifiedChildCommands"].as_array().unwrap().iter().chain(fixture["unavailableCapabilities"].as_array().unwrap()){assert!(page_field_mutation(&doc,&doc.pages[0],field.as_str().unwrap(),"300").is_none())}
}
