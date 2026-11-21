use field_projection::accessibility_projection;
use semio_ui_retained_tree::tree::{UiDocumentTree, UiTree};
use serde_json::{json, Value};
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiNodeId, UiNodeRecord, UiRevision};

fn projected_label(fixture: &Value, row: &Value, label: Value, slider: bool) -> Option<String> {
    let component = if slider {
        json!({"type":"slider","value":2,"min":0,"max":10,"step":1})
    } else {
        json!({"type":"input","kind":"number","value":"2"})
    };
    let layout = json!({"kind":"leaf","width":"hug","height":"hug"});
    let records = [
        json!({"id":0,"key":fixture["field"],"component":{"type":"container","role":"field","label":row["label"]},"layout":layout,"style":{},"activity":"idle","accessibility":{},"children":[1]}),
        json!({"id":1,"key":fixture["replacement"],"component":component,"layout":layout,"style":{},"activity":"idle","accessibility":{"label":label},"children":[]}),
    ];
    let header = UiDocumentLeaseHeader { generation:1,surface:SurfaceId::try_from(fixture["surface"].as_str().unwrap()).unwrap(),revision:UiRevision(0),root:UiNodeId(0),layout_epoch:0,node_count:records.len() };
    let mut doc = UiDocumentTree::new(header).unwrap();
    for row in records {
        doc.try_upsert_record(serde_json::from_value::<UiNodeRecord>(row).unwrap()).unwrap();
    }
    let mut tree = UiTree::new();
    tree.publish_document(doc);
    accessibility_projection(&tree).into_iter().find(|node|node.node_id==1).unwrap().label
}

#[test]
fn published_field_names_its_live_control_for_native_assistive_technology() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧫️fixtures/field-association.json")).unwrap();
    for row in fixture["labelRows"].as_array().unwrap() {
        assert_eq!(projected_label(&fixture,row,Value::Null,false).as_deref(),row["label"].as_str());
        assert_eq!(projected_label(&fixture,row,Value::Null,true).as_deref(),row["label"].as_str());
        assert_eq!(projected_label(&fixture,row,row["explicit"].clone(),false).as_deref(),row["explicit"].as_str());
    }
}
