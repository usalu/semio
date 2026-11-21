extern crate self as semio_ui_scene;
extern crate self as ui_contract;
use semio_ui_contract_builder::*;
use semio_ui_contract_values::{UiArenaLifetime, close_ui_value_page_with_grant, ui_value_arena_registry};
use std::collections::HashMap;

type UiAssemblyResult<T> = Result<T, &'static str>;
use semio_ui_contract_budget::{UI_BUILT_CHILDREN_MAX, UI_DOCUMENT_NODES};
fn ui_assembly_error(code: &'static str) -> &'static str { code }

type UiNodeId = usize;
struct UiNodeRecord { key: UiText, component: Component, children: Vec<UiNodeId> }
struct UiDocumentTree { records: HashMap<UiNodeId, UiNodeRecord> }
impl UiDocumentTree {
    fn record(&self, id: UiNodeId) -> Option<&UiNodeRecord> { self.records.get(&id) }
    fn flatten(&mut self, node: &BuiltNode, counter: &mut usize) -> UiNodeId {
        let id = *counter;
        *counter += 1;
        let children = node.children.iter().map(|child| self.flatten(child, counter)).collect();
        self.records.insert(id, UiNodeRecord { key: node.key.clone(), component: node.component.credited_clone().unwrap(), children });
        id
    }
}

#[test]
fn real_semantic_image_owner_roundtrips_direct_and_paged_sources() {
    let _arena = scope::UiArenaFixture::new();
    let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(std::env::var("SEMIO_IMAGE_FIXTURE").unwrap()).unwrap()).unwrap();
    for locale in ["en", "de"] {
        let row = &fixture[locale];
        let source = row["steps"][0]["blocks"][0]["src"].as_str().unwrap();
        let alt = row["alt"].as_str().unwrap();
        let build = |source: &str| image_with_source("asset.image", source, Label::try_from(alt).unwrap()).unwrap();
        let image = build(source);
        let Component::Image(props) = &image.component else { panic!("a proper Image component") };
        assert_eq!(props.alt.as_ref().unwrap().0.as_str(), alt);
        assert_eq!(image.accessibility.label.as_ref().unwrap().0.as_str(), alt);
        if source.len() <= UI_TEXT_MAX_BYTES {
            assert_eq!(props.src.as_str(), source);
            assert_eq!(props.source_body_bytes, None);
            assert!(image.children.is_empty());
        } else {
            assert_eq!(props.source_body_bytes, Some(source.len() as u32));
            let key = props.src.as_str();
            let mut document = UiDocumentTree { records: HashMap::new() };
            let id = document.flatten(&image, &mut 1);
            let record = document.record(id).unwrap();
            assert_eq!(retained_text_body(&document, record, key, source.len()).as_deref(), Some(source));
            assert_eq!(retained_text_body(&document, record, key, source.len() + 1), None);
            let alternate = build(&source.replace("image/png", "image/gif"));
            let Component::Image(changed) = &alternate.component else { panic!("replacement image") };
            assert_eq!(changed.source_body_bytes, props.source_body_bytes);
            assert_ne!(changed.src, props.src);
            assert_eq!(retained_text_body(&document, record, changed.src.as_str(), source.len()), None);
        }
    }
}

thread_local! {
    static UI_IMAGE_LAST_URL: std::cell::RefCell<HashMap<String, String>> = Default::default();
    static UI_IMAGE_URL_CACHE: std::cell::RefCell<HashMap<String, String>> = Default::default();
}

#[test]
fn native_raster_identity_requires_the_current_exact_source() {
    let source = "data:image/png;base64,opaque";
    UI_IMAGE_LAST_URL.with(|cache| cache.borrow_mut().insert("image".into(), source.into()));
    UI_IMAGE_URL_CACHE.with(|cache| cache.borrow_mut().insert("image".into(), "loaded-raster".into()));
    assert_eq!(decoded_ui_image_key("image", source).as_deref(), Some("loaded-raster"));
    assert_eq!(decoded_ui_image_key("image", ""), None);
    assert_eq!(decoded_ui_image_key("image", "data:image/png;base64,change"), None);
}
