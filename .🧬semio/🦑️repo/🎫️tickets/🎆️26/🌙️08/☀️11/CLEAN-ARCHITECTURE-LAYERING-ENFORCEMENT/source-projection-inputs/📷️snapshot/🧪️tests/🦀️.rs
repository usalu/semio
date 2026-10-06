//! 🧫️ Checks the same explicit source projection corpus as TypeScript.
use super::{SourceChange, SourceProjection, SourceProjectionError};

#[test]
fn source_projection_preserves_explicit_deletion_and_exact_predecessors() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("projection corpus");
    for row in fixture["cases"].as_array().expect("cases") {
        let files = &row["files"];
        let read = |path: &str| files[path].as_str();
        let changes: Vec<_> = row["changes"].as_array().expect("changes").iter().map(|change| SourceChange { path: change["path"].as_str().expect("path"), before: change["before"].as_str(), after: change["after"].as_str() }).collect();
        let result = (|| -> Result<serde_json::Value, SourceProjectionError> {
            let projection = SourceProjection::new(&changes, read)?;
            for reference in row["references"].as_array().expect("references") { projection.require(reference["target"].as_str().expect("target"), read)?; }
            let mut projected = serde_json::Map::new();
            for request in row["requests"].as_array().expect("requests") {
                let path = request.as_str().expect("request");
                projected.insert(path.to_owned(), serde_json::json!(projection.resolve(path, read)?));
            }
            let inverse: Vec<_> = projection.inverse().into_iter().map(|change| serde_json::json!({ "path": change.path, "before": change.before, "after": change.after })).collect();
            Ok(serde_json::json!({ "files": projected, "inverse": inverse }))
        })();
        let actual = match result { Ok(value) => value, Err(error) => serde_json::json!({ "refusal": { "code": error.code.as_str(), "path": error.path } }) };
        assert_eq!(actual, row["expected"], "{}", row["id"]);
    }
    println!("[DEBUG] source projection: shared corpus preserved without physical fallback for deletions");
}
