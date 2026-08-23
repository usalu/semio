use semio_repo_test_host::parse_json;
use semio_s_plugin_stdio_test_oracle::artifacts::bmp::standards::v_v3::subsets::any::{oracle_apply_mutation, oracle_undo_mutation, project_bmp_mutation};

const CASES: &[(&str, &str)] = &[
    ("no-mutation", "{}"),
    ("set-snapshot", r#"{"width":3,"height":2,"fill":[64,128,192,255]}"#),
    ("set-header-fields", r#"{"row_order":"top-down"}"#),
    ("insert-palette-entry", r#"{"index":0,"entry":{"b":10,"g":20,"r":30,"reserved":0}}"#),
    ("remove-palette-entry", r#"{"index":0}"#),
    ("set-palette-entry", r#"{"index":0,"entry":{"b":1,"g":2,"r":3,"reserved":0}}"#),
    ("set-pixel-data", r#"{"fill":[200,40,40,255]}"#),
];

fn main() {
    let input = std::fs::read("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🧫️fixtures/🖼️rathaus-ahlen-grundriss.bmp").expect("read fixture");
    println!("input: {} bytes", input.len());

    let original_projection = project_bmp_mutation(&input).expect("project original");
    println!("original projection: {}", original_projection.to_string());

    for (kind, params) in CASES {
        let spec_text = format!(r#"{{"kind": "{kind}", "params": {params}}}"#);
        let spec = parse_json(&spec_text).expect("parse spec");

        let mutated = oracle_apply_mutation(&input, &spec).unwrap_or_else(|error| panic!("apply_mutation({kind}) failed: {error}"));
        let mutated_projection = project_bmp_mutation(&mutated).unwrap_or_else(|error| panic!("project(mutated {kind}) failed: {error}"));
        println!("mutate-{kind}: {} bytes -> projection {}", mutated.len(), mutated_projection.to_string());

        let undone = oracle_undo_mutation(&input, &spec).unwrap_or_else(|error| panic!("undo_mutation({kind}) failed: {error}"));
        let undone_projection = project_bmp_mutation(&undone).unwrap_or_else(|error| panic!("project(undone {kind}) failed: {error}"));
        println!("inverse-{kind}: matches original = {}", undone_projection == original_projection);
    }

    println!("all cases ran without error");
}
