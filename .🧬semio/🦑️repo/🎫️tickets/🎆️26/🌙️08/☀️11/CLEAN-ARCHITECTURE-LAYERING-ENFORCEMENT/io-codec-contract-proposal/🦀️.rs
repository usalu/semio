//! 🧪️ Actual neutral contract ownership and portable behavior corpus.
use super::*;
use semio_framework_io_schema::Confidence;

fn corpus() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("closed codec corpus")
}

async fn charge_budget(budget: &mut CodecBudget, resource: &str, amount: u64) -> Result<(), CodecFailure> {
    match resource { "read" => budget.charge_read(amount).await, "write" => budget.charge_write(amount).await, "work" => budget.charge_work(amount).await, "allocation" => budget.charge_allocation(amount).await, _ => panic!("closed resource") }
}

#[semio_framework_async_macros::async_test]
async fn neutral_codec_has_exactly_one_definition_owner() {
    let owned = syn::parse_file(include_str!("../../🦀️.rs")).expect("actual owned Rust syntax");
    let mut names = owned.items.iter().filter_map(|item| match item {
        syn::Item::Struct(item) if matches!(item.vis, syn::Visibility::Public(_)) => Some(item.ident.to_string()),
        syn::Item::Enum(item) if matches!(item.vis, syn::Visibility::Public(_)) => Some(item.ident.to_string()),
        syn::Item::Trait(item) if matches!(item.vis, syn::Visibility::Public(_)) => Some(item.ident.to_string()),
        syn::Item::Type(item) if matches!(item.vis, syn::Visibility::Public(_)) => Some(item.ident.to_string()),
        _ => None,
    }).collect::<Vec<_>>();
    let data = corpus();
    let mut expected = data["exports"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_owned()).collect::<Vec<_>>();
    names.sort(); expected.sort();
    assert_eq!(names, expected);
    let retired = syn::parse_file(include_str!("../../../🦀️.rs")).expect("actual registry syntax");
    for item in retired.items {
        let name = match item { syn::Item::Struct(item) => Some(item.ident.to_string()), syn::Item::Enum(item) => Some(item.ident.to_string()), syn::Item::Trait(item) => Some(item.ident.to_string()), syn::Item::Type(item) => Some(item.ident.to_string()), _ => None };
        if let Some(name) = name { assert!(!expected.contains(&name), "duplicate definition {name}"); assert_ne!(name, "Confidence"); }
    }
    println!("[DEBUG] independent syn: all 32 contracts have one neutral definition owner; generic registry definitions retired");
}

#[semio_framework_async_macros::async_test]
async fn neutral_source_span_matches_portable_admission_corpus() {
    for row in corpus()["spans"].as_array().unwrap() {
        let value = &row["value"];
        let span = SourceSpan { resource: value["resource"].as_str().unwrap().to_owned(), byte_start: value["byte_start"].as_u64().unwrap(), byte_end: value["byte_end"].as_u64().unwrap(), line: value["line"].as_u64().map(|value| value as u32), column: value["column"].as_u64().map(|value| value as u32) };
        assert_eq!(span.validate().await.is_ok(), row["valid"].as_bool().unwrap(), "{row}");
    }
    println!("[DEBUG] actual neutral SourceSpan admission matches every independent portable vector");
}

#[semio_framework_async_macros::async_test]
async fn neutral_budget_preserves_failed_admission_counters() {
    for row in corpus()["budgets"].as_array().unwrap() {
        let limit = row["limit"].as_u64().unwrap(); let used = row["used"].as_u64().unwrap(); let increment = row["increment"].as_u64().unwrap();
        let mut budget = CodecBudget::new(CodecLimits { max_read_bytes: limit, max_written_bytes: limit, max_work_units: limit, max_allocations: limit, max_recursion_depth: 1 }, CancellationToken::new()).await;
        charge_budget(&mut budget, row["resource"].as_str().unwrap(), used).await.unwrap();
        assert_eq!(charge_budget(&mut budget, row["resource"].as_str().unwrap(), increment).await.is_ok(), row["valid"].as_bool().unwrap(), "{row}");
        let consumption = budget.consumption().await;
        let actual = match row["resource"].as_str().unwrap() { "read" => consumption.read_bytes, "write" => consumption.written_bytes, "work" => consumption.work_units, "allocation" => consumption.allocations, _ => panic!("closed resource") };
        assert_eq!(actual, row["consumed"].as_u64().unwrap());
    }
    println!("[DEBUG] actual neutral budget ceilings agree with portable vectors; refused admission preserves counters");
}

#[semio_framework_async_macros::async_test]
async fn neutral_sniff_confidence_preserves_all_schema_owned_variants() {
    for row in corpus()["confidence"].as_array().unwrap() {
        let confidence = match row["value"].as_str().unwrap() { "None" => Confidence::None, "Low" => Confidence::Low, "Medium" => Confidence::Medium, "High" => Confidence::High, _ => panic!("closed confidence") };
        let sniff = PayloadSniff { media_type: Some("fixture/type".into()), confidence, diagnostics: Vec::new() };
        assert_eq!(sniff.confidence.rank() as u64, row["rank"].as_u64().unwrap());
        assert_eq!(format!("{:?}", sniff.confidence), row["value"].as_str().unwrap());
        assert_eq!(sniff.confidence != Confidence::None, row["identified"].as_bool().unwrap());
    }
    println!("[DEBUG] actual PayloadSniff binds canonical schema Confidence identity/ranks; None represents absent evidence");
}
