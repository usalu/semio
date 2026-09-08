//! 🧩 Shared domain and package contracts for independently compiled norm artifacts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "../../⚖️compliance/🦀️.rs"]
#[macro_use]
pub mod document;

#[path = "."]
pub mod config {
    #[path = "../../🎚️config/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🎚️config/🧬️schema/🦀️.rs"]
    pub mod schema;

    #[path = "."]
    pub mod mutations {
        #[path = "../../🎚️config/🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "../../🎚️config/🧬️schema/🧬️mutations/☑️change-selected-check-index/🦀️.rs"]
        pub mod change_selected_check_index;
        #[path = "../../🎚️config/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
        mod text;
        #[path = "../../🎚️config/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
        mod binary;
    }
}

#[path = "../../🖥️app-surface/🦀️.rs"]
pub mod app_surface;

#[path = "../../🗿️artifacts/🦀️.rs"]
pub mod definition;

use semio_framework_value_derive::{FromValue, ToValue};
use std::fmt;

#[derive(Clone, FromValue, ToValue)]
#[value(deny_unknown_fields)]
struct PackageSource {
    definition_version: u8,
    id: String,
    artifact: String,
    directory: String,
    rust_package: String,
    nx_project: String,
    dependencies: Vec<String>,
}

/// 📦 Validated language-neutral package identity for one norm artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NormArtifactPackage {
    pub id: String,
    pub artifact: String,
    pub directory: String,
    pub rust_package: String,
    pub nx_project: String,
    pub dependencies: Vec<String>,
}

/// 🚫 Invalid norm artifact package schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageSchemaError(pub String);

impl fmt::Display for PackageSchemaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for PackageSchemaError {}

/// 🧬 Parses and validates one artifact-local package declaration.
pub fn package_from_schema(source: &str) -> Result<NormArtifactPackage, PackageSchemaError> {
    let parsed = pack::json::from_json_str::<PackageSource>(source).map_err(|error| PackageSchemaError(error.to_string()))?;
    let expected_id = format!("s.norm.{}", parsed.artifact);
    let expected_rust = format!("semio-s-artifact-norm-{}", parsed.artifact);
    let expected_nx = format!("@semio-tech/norm-{}-rs", parsed.artifact);
    if parsed.definition_version != 1 {
        return Err(PackageSchemaError("definition_version must be 1".into()));
    }
    if parsed.id != expected_id {
        return Err(PackageSchemaError(format!("id must be {expected_id}")));
    }
    if parsed.directory.is_empty() {
        return Err(PackageSchemaError("directory must not be empty".into()));
    }
    if parsed.rust_package != expected_rust {
        return Err(PackageSchemaError(format!("rust_package must be {expected_rust}")));
    }
    if parsed.nx_project != expected_nx {
        return Err(PackageSchemaError(format!("nx_project must be {expected_nx}")));
    }
    let mut unique = std::collections::BTreeSet::new();
    for dependency in &parsed.dependencies {
        if dependency == &parsed.rust_package || !unique.insert(dependency) {
            return Err(PackageSchemaError(format!("invalid dependency {dependency}")));
        }
    }
    Ok(NormArtifactPackage {
        id: parsed.id,
        artifact: parsed.artifact,
        directory: parsed.directory,
        rust_package: parsed.rust_package,
        nx_project: parsed.nx_project,
        dependencies: parsed.dependencies,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = include_str!("🧪️fixtures/✅️valid/🔣️.json");
    const INVALID: &str = include_str!("🧪️fixtures/❌️invalid/🔣️.json");

    #[test]
    fn package_schema_matches_third_party_json_oracle() {
        let ours = package_from_schema(VALID).expect("first-party parser");
        let oracle: serde_json::Value = serde_json::from_str(VALID).expect("third-party parser");
        assert_eq!(ours.id, oracle["id"].as_str().expect("oracle id"));
        assert_eq!(ours.rust_package, oracle["rust_package"].as_str().expect("oracle Rust package"));
        assert!(package_from_schema(INVALID).is_err());
        assert!(serde_json::from_str::<serde_json::Value>(INVALID).is_ok());
    }
}
