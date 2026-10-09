//! 🧪️ Original Board boundary laws against canonical admitted records.

use super::*;use semio_framework_value::{DslValue,FromValue,ToValue,NativeDecodeControl};
fn fixtures()->serde_json::Value{serde_json::from_str(include_str!("../🔣️.json")).unwrap()}
#[test]fn typed_visibility_agrees_with_independent_serde_json(){for row in fixtures()["visibility"].as_array().unwrap(){let flags=&row["flags"];let typed=board_schema::BoardVisibility{hidden:flags["hidden"].as_bool(),visible:flags["visible"].as_bool(),locked:flags["locked"].as_bool()};assert_eq!(board_schema::board_visible_option(&typed),row["option"].as_bool());assert_eq!(board_schema::board_visible_or_true(&typed),row["visible"].as_bool().unwrap());assert_eq!(board_schema::board_locked_option(&typed),row["locked"].as_bool());}println!("[DEBUG] native Board visibility 5 independent serde cases");}
#[test]fn explicit_palette_io_matches_independent_json_and_all_48_components(){let f=fixtures();for row in f["overlays"].as_array().unwrap(){let source=row.to_string();let mut accepted=|_|true;let mut control=NativeDecodeControl::new(1024*1024,&mut accepted);let overlay=palette_io::decode_board_palette_overlay_json(&source,&mut control).unwrap();assert_eq!(serde_json::Value::from(&overlay.to_value()),*row);if !row.as_object().unwrap().is_empty(){assert!(control.owned_bytes()>0);}let base=board_schema::palette::BoardPalette {
 raster_clear:[10,20,30,255],
 grid_minor_stroke:[10,20,30,255],
 edge_stroke:[10,20,30,255],
 edge_stroke_hovered:[10,20,30,255],
 edge_stroke_selected:[10,20,30,255],
 edge_stroke_selection_exit:[10,20,30,255],
 edge_stroke_disabled:[10,20,30,255],
 node_fill:[10,20,30,255],
 node_stroke:[10,20,30,255],
 node_fill_hovered:[10,20,30,255],
 node_stroke_hovered:[10,20,30,255],
 node_fill_selected:[10,20,30,255],
 node_stroke_selected:[10,20,30,255],
 node_fill_selection_exit:[10,20,30,255],
 node_stroke_selection_exit:[10,20,30,255],
 node_fill_disabled:[10,20,30,255],
 node_stroke_disabled:[10,20,30,255],
 node_stroke_computing:[10,20,30,255],
 node_stroke_stale:[10,20,30,255],
 node_stroke_error:[10,20,30,255],
 node_stroke_blocked:[10,20,30,255],
 indirect_handle_fill:[10,20,30,255],
 indirect_handle_stroke:[10,20,30,255],
 handle_fill:[10,20,30,255],
 handle_stroke:[10,20,30,255],
 handle_fill_hovered:[10,20,30,255],
 handle_stroke_hovered:[10,20,30,255],
 handle_fill_selected:[10,20,30,255],
 handle_stroke_selected:[10,20,30,255],
 handle_fill_selection_exit:[10,20,30,255],
 handle_stroke_selection_exit:[10,20,30,255],
 handle_fill_disabled:[10,20,30,255],
 handle_stroke_disabled:[10,20,30,255],
 wire_stroke:[10,20,30,255],
 wire_stroke_hovered:[10,20,30,255],
 wire_stroke_selected:[10,20,30,255],
 wire_stroke_highlighted:[10,20,30,255],
 wire_stroke_disabled:[10,20,30,255],
 selection_preview_fill:[10,20,30,255],
 selection_preview_stroke:[10,20,30,255],
 label_fill:[10,20,30,255],
 label_fill_hovered:[10,20,30,255],
 label_halo:[10,20,30,255],
 minimap_widget_panel_fill:[10,20,30,255],
 minimap_widget_panel_stroke:[10,20,30,255],
 minimap_widget_viewport_fill:[10,20,30,255],
 minimap_widget_viewport_stroke:[10,20,30,255],
 minimap_widget_viewport_stroke_hovered:[10,20,30,255],
};let result=board_schema::palette::apply_board_palette_overlay(&base,&overlay);
 assert_eq!(result.raster_clear,overlay.raster_clear.unwrap_or(base.raster_clear));
 assert_eq!(result.grid_minor_stroke,overlay.grid_minor_stroke.unwrap_or(base.grid_minor_stroke));
 assert_eq!(result.edge_stroke,overlay.edge_stroke.unwrap_or(base.edge_stroke));
 assert_eq!(result.edge_stroke_hovered,overlay.edge_stroke_hovered.unwrap_or(base.edge_stroke_hovered));
 assert_eq!(result.edge_stroke_selected,overlay.edge_stroke_selected.unwrap_or(base.edge_stroke_selected));
 assert_eq!(result.edge_stroke_selection_exit,overlay.edge_stroke_selection_exit.unwrap_or(base.edge_stroke_selection_exit));
 assert_eq!(result.edge_stroke_disabled,overlay.edge_stroke_disabled.unwrap_or(base.edge_stroke_disabled));
 assert_eq!(result.node_fill,overlay.node_fill.unwrap_or(base.node_fill));
 assert_eq!(result.node_stroke,overlay.node_stroke.unwrap_or(base.node_stroke));
 assert_eq!(result.node_fill_hovered,overlay.node_fill_hovered.unwrap_or(base.node_fill_hovered));
 assert_eq!(result.node_stroke_hovered,overlay.node_stroke_hovered.unwrap_or(base.node_stroke_hovered));
 assert_eq!(result.node_fill_selected,overlay.node_fill_selected.unwrap_or(base.node_fill_selected));
 assert_eq!(result.node_stroke_selected,overlay.node_stroke_selected.unwrap_or(base.node_stroke_selected));
 assert_eq!(result.node_fill_selection_exit,overlay.node_fill_selection_exit.unwrap_or(base.node_fill_selection_exit));
 assert_eq!(result.node_stroke_selection_exit,overlay.node_stroke_selection_exit.unwrap_or(base.node_stroke_selection_exit));
 assert_eq!(result.node_fill_disabled,overlay.node_fill_disabled.unwrap_or(base.node_fill_disabled));
 assert_eq!(result.node_stroke_disabled,overlay.node_stroke_disabled.unwrap_or(base.node_stroke_disabled));
 assert_eq!(result.node_stroke_computing,overlay.node_stroke_computing.unwrap_or(base.node_stroke_computing));
 assert_eq!(result.node_stroke_stale,overlay.node_stroke_stale.unwrap_or(base.node_stroke_stale));
 assert_eq!(result.node_stroke_error,overlay.node_stroke_error.unwrap_or(base.node_stroke_error));
 assert_eq!(result.node_stroke_blocked,overlay.node_stroke_blocked.unwrap_or(base.node_stroke_blocked));
 assert_eq!(result.indirect_handle_fill,overlay.indirect_handle_fill.unwrap_or(base.indirect_handle_fill));
 assert_eq!(result.indirect_handle_stroke,overlay.indirect_handle_stroke.unwrap_or(base.indirect_handle_stroke));
 assert_eq!(result.handle_fill,overlay.handle_fill.unwrap_or(base.handle_fill));
 assert_eq!(result.handle_stroke,overlay.handle_stroke.unwrap_or(base.handle_stroke));
 assert_eq!(result.handle_fill_hovered,overlay.handle_fill_hovered.unwrap_or(base.handle_fill_hovered));
 assert_eq!(result.handle_stroke_hovered,overlay.handle_stroke_hovered.unwrap_or(base.handle_stroke_hovered));
 assert_eq!(result.handle_fill_selected,overlay.handle_fill_selected.unwrap_or(base.handle_fill_selected));
 assert_eq!(result.handle_stroke_selected,overlay.handle_stroke_selected.unwrap_or(base.handle_stroke_selected));
 assert_eq!(result.handle_fill_selection_exit,overlay.handle_fill_selection_exit.unwrap_or(base.handle_fill_selection_exit));
 assert_eq!(result.handle_stroke_selection_exit,overlay.handle_stroke_selection_exit.unwrap_or(base.handle_stroke_selection_exit));
 assert_eq!(result.handle_fill_disabled,overlay.handle_fill_disabled.unwrap_or(base.handle_fill_disabled));
 assert_eq!(result.handle_stroke_disabled,overlay.handle_stroke_disabled.unwrap_or(base.handle_stroke_disabled));
 assert_eq!(result.wire_stroke,overlay.wire_stroke.unwrap_or(base.wire_stroke));
 assert_eq!(result.wire_stroke_hovered,overlay.wire_stroke_hovered.unwrap_or(base.wire_stroke_hovered));
 assert_eq!(result.wire_stroke_selected,overlay.wire_stroke_selected.unwrap_or(base.wire_stroke_selected));
 assert_eq!(result.wire_stroke_highlighted,overlay.wire_stroke_highlighted.unwrap_or(base.wire_stroke_highlighted));
 assert_eq!(result.wire_stroke_disabled,overlay.wire_stroke_disabled.unwrap_or(base.wire_stroke_disabled));
 assert_eq!(result.selection_preview_fill,overlay.selection_preview_fill.unwrap_or(base.selection_preview_fill));
 assert_eq!(result.selection_preview_stroke,overlay.selection_preview_stroke.unwrap_or(base.selection_preview_stroke));
 assert_eq!(result.label_fill,overlay.label_fill.unwrap_or(base.label_fill));
 assert_eq!(result.label_fill_hovered,overlay.label_fill_hovered.unwrap_or(base.label_fill_hovered));
 assert_eq!(result.label_halo,overlay.label_halo.unwrap_or(base.label_halo));
 assert_eq!(result.minimap_widget_panel_fill,overlay.minimap_widget_panel_fill.unwrap_or(base.minimap_widget_panel_fill));
 assert_eq!(result.minimap_widget_panel_stroke,overlay.minimap_widget_panel_stroke.unwrap_or(base.minimap_widget_panel_stroke));
 assert_eq!(result.minimap_widget_viewport_fill,overlay.minimap_widget_viewport_fill.unwrap_or(base.minimap_widget_viewport_fill));
 assert_eq!(result.minimap_widget_viewport_stroke,overlay.minimap_widget_viewport_stroke.unwrap_or(base.minimap_widget_viewport_stroke));
 assert_eq!(result.minimap_widget_viewport_stroke_hovered,overlay.minimap_widget_viewport_stroke_hovered.unwrap_or(base.minimap_widget_viewport_stroke_hovered));
 }println!("[DEBUG] native Board palette 3 overlays actual production operator 48 fields");}
#[test]fn palette_numeric_spelling_matches_independent_json_integer_schema(){for row in fixtures()["paletteSources"].as_array().unwrap(){let mut accepted=|_|true;let mut control=NativeDecodeControl::new(1024*1024,&mut accepted);let source=row["source"].as_str().unwrap();let overlay=palette_io::decode_board_palette_overlay_json(source,&mut control).unwrap();let oracle:serde_json::Value=serde_json::from_str(source).unwrap();assert_eq!(overlay.node_fill.unwrap(),[oracle["nodeFill"][0].as_f64().unwrap()as u8,2,255,0]);}println!("[DEBUG] native Board palette JSON numeric spelling agrees with independent integer semantics");}
#[test]fn palette_json_refusals_and_owned_controls_are_observed(){for row in fixtures()["invalidOverlays"].as_array().unwrap(){let mut accepted=|_|true;let mut control=NativeDecodeControl::new(1024*1024,&mut accepted);assert!(palette_io::decode_board_palette_overlay_json(&row.to_string(),&mut control).is_err());}for source in ["{",r#"{"nodeFill":[0,0,0,255],"nodeFill":[1,2,3,255]}"#]{let mut accepted=|_|true;let mut control=NativeDecodeControl::new(1024*1024,&mut accepted);assert!(palette_io::decode_board_palette_overlay_json(source,&mut control).is_err());}let mut canceled=|_|false;let mut control=NativeDecodeControl::new(1024*1024,&mut canceled);assert!(palette_io::decode_board_palette_overlay_json("{}",&mut control).is_err());let mut accepted=|_|true;let mut control=NativeDecodeControl::new(1,&mut accepted);assert!(palette_io::decode_board_palette_overlay_json(r#"{"nodeFill":[0,0,0,255]}"#,&mut control).is_err());println!("[DEBUG] native Board palette 10 shape refusals malformed duplicate canceled owned ceiling");}
#[test]fn userdata_and_snapshot_rows_remain_first_party_owned(){for payload in fixtures()["userdata"].as_object().unwrap().values(){let node=serde_json::json!({"id":"n","x":1,"y":2,"userData":payload});let node=board_schema::NodeDescriptor::from_value(DslValue::from(node)).unwrap();assert_eq!(serde_json::Value::from(&node.to_value())["userData"],*payload);let handle=port_schema::HandleDescriptor::from_value(DslValue::from(serde_json::json!({"id":"h","nodeId":"n","angle":0,"userData":payload}))).unwrap();assert_eq!(serde_json::Value::from(&handle.to_value())["userData"],*payload);let edge=directed_schema::EdgeDescriptor::from_value(DslValue::from(serde_json::json!({"id":"e","source":"h","target":"j","userData":payload}))).unwrap();assert_eq!(serde_json::Value::from(&edge.to_value())["userData"],*payload);let wire=directed_schema::WireDescriptor::from_value(DslValue::from(serde_json::json!({"id":"w","source":"h","userData":payload}))).unwrap();assert_eq!(serde_json::Value::from(&wire.to_value())["userData"],*payload);}
let intrinsic=DslValue::object([("id".into(),DslValue::String("n".into())),("x".into(),DslValue::float(1.0)),("y".into(),DslValue::float(2.0)),("userData".into(),DslValue::Bytes(vec![0,255]))]);let node=board_schema::NodeDescriptor::from_value(intrinsic).unwrap();assert_eq!(node.user_data,Some(DslValue::Bytes(vec![0,255])));
let source=r#"{"schema":"board.ports.directed.v1","nodes":[{"id":"n","userData":{"large":18446744073709551615}}],"edges":[],"targetRegions":[],"meta":{"caption":"Grüße"}}"#;let snapshot:directed_schema::BoardSnapshot=semio_framework_pack_json::from_json_str(source,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();assert_eq!(snapshot.nodes[0].user_data.as_ref().unwrap().get("large").and_then(DslValue::as_u64),Some(u64::MAX));assert_eq!(serde_json::Value::from(&snapshot.to_value())["meta"],serde_json::json!({"caption":"Grüße"}));assert!(directed_schema::BoardSnapshot::from_value(DslValue::from(serde_json::json!({"schema":"s","nodes":[1],"edges":[]}))).is_err());println!("[DEBUG] native Board userdata 4 families x5 neutral kinds+intrinsic bytes+u64MAX+owned record refusal");}
#[test]fn physical_visibility_normalization_matches_independent_json(){let source=r#"{"nodes":[{"id":"a","x":0,"y":0,"hidden":true,"visible":true},{"id":"b","x":1,"y":2,"hidden":false,"visible":false}],"handles":[{"id":"h","nodeId":"a","angle":0,"hidden":true}],"edges":[{"id":"e","source":"h","target":"j","visible":false}],"wires":[]}"#;let mut accepted=|_|true;let mut control=NativeDecodeControl::new(1024*1024,&mut accepted);let scene=visibility_io::decode_board_scene_json(source,&mut control).unwrap();let oracle:serde_json::Value=serde_json::from_str(source).unwrap();assert_eq!(scene.nodes[0].visible,Some(!oracle["nodes"][0]["hidden"].as_bool().unwrap()));assert_eq!(scene.nodes[1].visible,Some(!oracle["nodes"][1]["hidden"].as_bool().unwrap()));assert_eq!(scene.handles[0].visible,Some(false));assert_eq!(scene.edges[0].visible,Some(false));let mut canceled=|_|false;let mut control=NativeDecodeControl::new(1024*1024,&mut canceled);assert!(visibility_io::decode_board_scene_json(source,&mut control).is_err());println!("[DEBUG] native Board explicit controlled scene IO 4 normalized rows and cancellation agree with independent serde JSON");}
#[test]fn edge_tip_owned_facts_match_independent_json_defaults(){for row in fixtures()["edgeTips"].as_array().unwrap(){let entry=board_schema::edge_tip::EdgeTipCatalogEntry::from_value(DslValue::from(row["entry"].clone())).unwrap();let result=board_schema::edge_tip::EdgeTipDef::from_catalog_entry(&entry).unwrap();let observed=serde_json::json!({"geometry":serde_json::Value::from(&result.geometry.to_value()),"filled":result.filled,"scale":result.scale});assert_eq!(observed["geometry"],row["expected"]["geometry"]);assert_eq!(observed["filled"],row["expected"]["filled"]);assert_eq!(observed["scale"].as_f64(),row["expected"]["scale"].as_f64());}println!("[DEBUG] native Board edge tips 6 owned entries match independent serde JSON defaults");}

#[test]fn controlled_snapshot_io_preserves_exact_owned_rows_and_refuses_before_publication(){let source=r#"{"schema":"board.ports.directed.v1","nodes":[{"id":"n","userData":{"large":18446744073709551615}}],"edges":[],"meta":{"text":"Grüße"}}"#;let mut events=0;let mut accepted=|_|{events+=1;true};let mut control=NativeDecodeControl::new(1024*1024,&mut accepted);let snapshot=snapshot_io::decode_board_snapshot_json(source,&mut control).unwrap();assert_eq!(snapshot.nodes[0].user_data.as_ref().unwrap().get("large").and_then(DslValue::as_u64),Some(u64::MAX));assert_eq!(snapshot.meta.as_ref().unwrap().get("text").unwrap().as_str(),Some("Grüße"));assert!(control.owned_bytes()>0);drop(control);assert!(events>0);for source in ["{",r#"{"schema":"s","nodes":[1],"edges":[]}"#,r#"{"schema":"s","schema":"s","nodes":[],"edges":[]}"#]{let mut accepted=|_|true;let mut control=NativeDecodeControl::new(1024*1024,&mut accepted);assert!(snapshot_io::decode_board_snapshot_json(source,&mut control).is_err());}let mut canceled=|_|false;let mut control=NativeDecodeControl::new(1024*1024,&mut canceled);assert!(snapshot_io::decode_board_snapshot_json(source,&mut control).is_err());let mut accepted=|_|true;let mut control=NativeDecodeControl::new(1,&mut accepted);assert!(snapshot_io::decode_board_snapshot_json(source,&mut control).is_err());println!("[DEBUG] native Board snapshot IO exact u64MAX/UTF8+3 refusals+caller progress/cancellation/owned limit");}
#[test]fn controlled_palette_projection_pays_and_matches_independent_json(){for row in fixtures()["overlays"].as_array().unwrap(){let overlay=board_schema::BoardPaletteOverlay::from_value(DslValue::from(row.clone())).unwrap();let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1024*1024,&mut accepted);assert_eq!(serde_json::Value::from(&overlay.to_value_controlled(&mut control).unwrap()),*row);}let overlay=board_schema::BoardPaletteOverlay{node_fill:Some([1,2,3,255]),..Default::default()};let mut canceled=|_|false;let mut control=semio_framework_value::NativeEncodeControl::new(1024*1024,&mut canceled);assert!(overlay.to_value_controlled(&mut control).is_err());let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1,&mut accepted);assert!(overlay.to_value_controlled(&mut control).is_err());println!("[DEBUG] native Board palette controlled projection 3 independent values+cancel/owned refusal");}

