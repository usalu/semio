use super::*;

const CANONICAL_QUERY: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = 'core'\nRETURN a.name, b.name";

#[semio_framework_async_macros::async_test]
async fn parses_full_jack_ast_shape() {
    let root = parse_jack_ast(CANONICAL_QUERY);
    assert_eq!(root.kind, "query");
    assert_eq!(root.children.len(), 3);
    assert_eq!(root.children[0].kind, "match");
    assert_eq!(root.children[1].kind, "where");
    assert_eq!(root.children[2].kind, "return");
    let pattern = &root.children[0].children[0];
    assert_eq!(pattern.kind, "pattern");
    assert_eq!(pattern.children.len(), 3);
    assert_eq!(pattern.children[1].kind, "edge");
}

#[semio_framework_async_macros::async_test]
async fn selection_maps_to_smallest_containing_ast_node() {
    let root = parse_jack_ast(CANONICAL_QUERY);
    let a_offset = CANONICAL_QUERY.find("a:Piece").unwrap();
    let node = jack_ast_node_for_selection(&root, a_offset, a_offset + 1).expect("node");
    assert_eq!(node.kind, "var");
}

#[semio_framework_async_macros::async_test]
async fn selectable_spans_include_var_label_and_property_access() {
    let text = "MATCH (a1:Piece) RETURN a1.name";
    let tokens = tokenize_language(text, "jack");
    let spans = selectable_spans_for_jack(text, &tokens);
    assert!(spans.iter().any(|s| s.kind == "varLabel" && s.start == 7 && s.end == 15));
    assert!(spans.iter().any(|s| s.kind == "propertyAccess" && s.start == 24 && s.end == 31));
}

#[semio_framework_async_macros::async_test]
async fn symbol_occurrences_find_bound_variable_uses() {
    let symbol = jack_symbol_at_offset(CANONICAL_QUERY, CANONICAL_QUERY.find("a.name").unwrap()).expect("symbol");
    assert_eq!(symbol.kind, JackSymbolKind::Variable);
    assert_eq!(symbol.occurrences.len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn symbol_at_label_position_is_node_kind_not_variable() {
    let symbol = jack_symbol_at_offset(CANONICAL_QUERY, CANONICAL_QUERY.find("Piece").unwrap() + 1).expect("symbol");
    assert_eq!(symbol.kind, JackSymbolKind::NodeKind);
}

#[semio_framework_async_macros::async_test]
async fn placeholders_suggest_expr_after_and() {
    let text = "MATCH (a:Piece) WHERE a.name = 'x' AND ";
    let placeholders = jack_editor_placeholders(text, text.len());
    assert!(placeholders.iter().any(|p| p.label == "expr"));
}

#[semio_framework_async_macros::async_test]
async fn placeholders_suggest_label_after_colon() {
    let text = "MATCH (a:";
    let placeholders = jack_editor_placeholders(text, text.len());
    assert!(placeholders.iter().any(|p| p.label == "Label"));
}

#[semio_framework_async_macros::async_test]
async fn newline_gates_allow_after_match_close_paren() {
    let text = "MATCH (a:Piece)";
    let gates = jack_newline_gate_offsets(text);
    assert!(gates.contains(&text.len()));
}

#[semio_framework_async_macros::async_test]
async fn newline_gates_disallow_inside_token() {
    let text = "MATCH (a:Piece)";
    let inside = text.find("Piece").unwrap() + 2;
    assert!(!jack_newline_allowed_at(text, inside));
}

#[semio_framework_async_macros::async_test]
async fn newline_gates_disallow_before_dot() {
    let text = "RETURN a.name";
    let before_dot = text.find('.').unwrap();
    assert!(!jack_newline_allowed_at(text, before_dot));
}
