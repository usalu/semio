//! Standalone compile check for the lopdf 0.44 usage in this ticket's
//! ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs
//! Not part of the repo — the shared oracle crate currently fails to compile due to unrelated
//! in-progress sibling files (png, csv), so this isolates just the lopdf-specific logic.

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(entries) => entries.iter().find(|(name, _)| name == key).map(|(_, value)| value),
            _ => None,
        }
    }
    pub fn str(&self, key: &str) -> String {
        match self.get(key) {
            Some(Json::String(value)) => value.clone(),
            _ => String::new(),
        }
    }
}

//#region 🔖️Spec
fn target_text(spec: &Json) -> String {
    spec.get("params").and_then(|params| params.get("snapshot")).and_then(|snapshot| snapshot.get("page")).map(|page| page.str("text")).unwrap_or_default()
}
//#endregion 🔖️Spec

//#region 🔖️Dispatch
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    match spec.str("kind").as_str() {
        "" => Err("mutation spec carries no `kind`".to_string()),
        "no-mutation" => build_single_page_pdf(&independent_first_text(input)?),
        "set-snapshot" => build_single_page_pdf(&target_text(spec)),
        kind => Err(format!("mutation kind {:?} has no oracle implementation ({} input byte(s))", kind, input.len())),
    }
}
//#endregion 🔖️Dispatch

//#region 🔖️IndependentReader
pub fn independent_first_text(input: &[u8]) -> Result<String, String> {
    use lopdf::{content::Content, Object};

    let document = lopdf::Document::load_mem(input).map_err(|error| format!("independent reader could not parse the document: {}", error))?;
    let pages = document.get_pages();
    let page_id = *pages.get(&1).ok_or("independent reader found no page 1")?;
    let content = document.get_page_content(page_id);
    let decoded = Content::decode(&content).map_err(|error| format!("independent reader could not decode page 1's content stream: {}", error))?;
    for operation in &decoded.operations {
        match operation.operator.as_str() {
            "Tj" => {
                if let Some(Object::String(bytes, _)) = operation.operands.first() {
                    return Ok(String::from_utf8_lossy(bytes).into_owned());
                }
            }
            "TJ" => {
                if let Some(Object::Array(items)) = operation.operands.first() {
                    if let Some(Object::String(bytes, _)) = items.iter().find(|item| matches!(item, Object::String(..))) {
                        return Ok(String::from_utf8_lossy(bytes).into_owned());
                    }
                }
            }
            _ => {}
        }
    }
    Err("independent reader found no text-showing operator on page 1".to_string())
}

pub fn project_pdf_1_4(input: &[u8]) -> Result<Json, String> {
    use lopdf::Object;

    let document = lopdf::Document::load_mem(input).map_err(|error| format!("independent reader could not parse the document: {}", error))?;
    let pages = document.get_pages();
    let page_id = *pages.get(&1).ok_or("independent reader found no page 1")?;
    let dictionary = document.get_dictionary(page_id).map_err(|error| format!("page 1 dictionary unreadable: {}", error))?;
    let number = |object: &Object| -> f64 {
        match object {
            Object::Integer(value) => *value as f64,
            Object::Real(value) => *value as f64,
            _ => 0.0,
        }
    };
    let media_box: Vec<f64> = match dictionary.get(b"MediaBox").ok().and_then(|value| value.as_array().ok()) {
        Some(items) => items.iter().map(number).collect(),
        None => return Err("page 1 carries no MediaBox".to_string()),
    };
    if media_box.len() != 4 {
        return Err(format!("page 1 MediaBox has {} entries, expected 4", media_box.len()));
    }
    let text = independent_first_text(input)?;
    Ok(Json::Object(vec![("width".to_string(), Json::Number(media_box[2] - media_box[0])), ("height".to_string(), Json::Number(media_box[3] - media_box[1])), ("text".to_string(), Json::String(text))]))
}
//#endregion 🔖️IndependentReader

//#region 🔖️IndependentWriter
pub fn build_single_page_pdf(text: &str) -> Result<Vec<u8>, String> {
    use lopdf::{
        content::{Content, Operation},
        dictionary, Document, Object, Stream,
    };

    let mut document = Document::with_version("1.4");
    let pages_id = document.new_object_id();
    let font_id = document.add_object(dictionary! { "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Helvetica" });
    let resources_id = document.add_object(dictionary! { "Font" => dictionary! { "F1" => font_id } });
    let content = Content { operations: vec![Operation::new("BT", vec![]), Operation::new("Tf", vec!["F1".into(), 12.into()]), Operation::new("Td", vec![72.into(), 720.into()]), Operation::new("Tj", vec![Object::string_literal(text)]), Operation::new("ET", vec![])] };
    let content_id = document.add_object(Stream::new(dictionary! {}, content.encode().map_err(|error| format!("independent writer could not encode page content: {}", error))?));
    let page_id = document.add_object(dictionary! { "Type" => "Page", "Parent" => pages_id, "Contents" => content_id, "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()], "Resources" => resources_id });
    document.objects.insert(pages_id, Object::Dictionary(dictionary! { "Type" => "Pages", "Kids" => vec![page_id.into()], "Count" => 1 }));
    let catalog_id = document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
    document.trailer.set("Root", catalog_id);
    let mut out = Vec::new();
    document.save_to(&mut out).map_err(|error| format!("independent writer could not save: {}", error))?;
    Ok(out)
}
//#endregion 🔖️IndependentWriter

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_then_read_back_round_trips_text() {
        let bytes = build_single_page_pdf("Wave seven replaced this page.").expect("build");
        let projection = project_pdf_1_4(&bytes).expect("project");
        assert_eq!(projection.get("text"), Some(&Json::String("Wave seven replaced this page.".to_string())));
        assert_eq!(projection.get("width"), Some(&Json::Number(612.0)));
        assert_eq!(projection.get("height"), Some(&Json::Number(792.0)));
    }

    #[test]
    fn oracle_apply_mutation_dispatches_both_kinds() {
        let source = build_single_page_pdf("SemIO").expect("build source");
        let no_mut = oracle_apply_mutation(&source, &Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string()))])).expect("no-mutation");
        assert_eq!(project_pdf_1_4(&no_mut).unwrap().get("text"), Some(&Json::String("SemIO".to_string())));

        let spec = Json::Object(vec![("kind".to_string(), Json::String("set-snapshot".to_string())), ("params".to_string(), Json::Object(vec![("snapshot".to_string(), Json::Object(vec![("page".to_string(), Json::Object(vec![("text".to_string(), Json::String("new text".to_string()))]))]))]))]);
        let set = oracle_apply_mutation(&source, &spec).expect("set-snapshot");
        assert_eq!(project_pdf_1_4(&set).unwrap().get("text"), Some(&Json::String("new text".to_string())));

        assert!(oracle_apply_mutation(&source, &Json::Object(vec![("kind".to_string(), Json::String("bogus".to_string()))])).is_err());
    }

    #[test]
    fn reads_first_tj_text_from_a_real_pdflatex_document() {
        let path = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf";
        let bytes = std::fs::read(path).expect("read real fixture");
        let text = independent_first_text(&bytes).expect("independent_first_text on the real 65-page thesis");
        assert_eq!(text, "SemIO");
        let projection = project_pdf_1_4(&bytes).expect("project the real fixture");
        assert_eq!(projection.get("text"), Some(&Json::String("SemIO".to_string())));
    }
}
