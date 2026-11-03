//! 🧪️ Census tests; scenarios in `🥒️.feature`. The synthetic cases pin the scanner, the tree test is the build gate over the whole norm plugin.

use super::*;

fn scan(src: &str) -> Census {
    let mut c = Census::default();
    scan_source("x.rs", src, &mut c);
    c
}

#[test]
fn complete_pairs_pass() {
    let c = scan(r##"let a = LocalizedLabel::native("Thickness", "Dicke"); let b = LocalizedCopy::new(format!("Member {id}"), format!("Stab {id}")); let c = World3dText::new("t, with comma", r#"Dicke "x""#);"##);
    assert_eq!((c.pairs, c.findings.len()), (3, 0));
}

#[test]
fn missing_german_or_english_is_found() {
    let c = scan(r#"LocalizedLabel::native("Thickness", ""); LocalizedCopy::new("  ", "Dicke"); LocalizedLabel::native(format!("{}", x), format!(" "));"#);
    assert_eq!(c.findings.iter().map(|f| f.rule).collect::<Vec<_>>(), ["empty German", "empty English", "empty German"]);
    assert_eq!(c.findings[0].line, 1);
}

#[test]
fn untranslated_english_prose_is_found_but_names_are_not() {
    let c = scan(r#"LocalizedLabel::native("Design resistance of the section", "Design resistance of the section"); LocalizedLabel::native("EN 1993-1-1", "EN 1993-1-1"); LocalizedLabel::native("S355", "S355");"#);
    assert_eq!(c.findings.len(), 1);
    assert_eq!(c.findings[0].rule, "German equals English");
}

#[test]
fn field_pairs_and_nested_calls_are_parsed() {
    let c = scan(r#"NormFieldChoice { value: "a", label_en: "Yes", label_de: "" }; Foo { en: "A", de: "B" }; LocalizedLabel::native(f(1, (2, 3)), g("x,y"));"#);
    assert_eq!(c.pairs, 3);
    assert_eq!(c.findings.iter().map(|f| f.rule).collect::<Vec<_>>(), ["empty German"]);
    assert_eq!(c.literal_pairs, 2);
}

#[test]
fn identifiers_ending_in_en_are_not_field_pairs() {
    let c = scan(r#"let token: "x", de: "";"#);
    assert_eq!(c.pairs, 0);
}

#[test]
fn every_user_visible_string_of_the_norm_plugin_has_english_and_german() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../..");
    let census = scan_tree(&root);
    assert!(census.files > 100 && census.pairs > 1000, "census scanned {} files and {} pairs", census.files, census.pairs);
    let report: Vec<String> = census.findings.iter().map(|f| format!("{}:{} {} (en {:?}, de {:?})", f.file, f.line, f.rule, f.en, f.de)).collect();
    assert!(report.is_empty(), "{} strings lack English or German:\n{}", report.len(), report.join("\n"));
}
