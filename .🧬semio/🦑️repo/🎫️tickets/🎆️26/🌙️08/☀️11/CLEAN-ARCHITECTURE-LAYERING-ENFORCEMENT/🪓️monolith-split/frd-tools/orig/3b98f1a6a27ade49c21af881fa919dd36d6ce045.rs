//! 🧰️ The catalog file the proctor's own tests play and the catalog loaded from it, shared with the tests of the crates above (`testkit`).

use super::*;

/// 📚️ The catalog file the proctor's own tests play.
pub fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🧫️fixtures/📚️catalog/🔣️.json")
}

/// 📚️ The fixture catalog, loaded.
pub fn fixture() -> LoadedCatalog {
    load_catalog(&fixture_path()).expect("the fixture catalog is valid")
}
