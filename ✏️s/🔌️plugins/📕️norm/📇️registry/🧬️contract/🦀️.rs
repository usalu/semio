//! 🧩 Shared domain and package contracts for independently compiled norm artifacts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

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
    let parsed = semio_framework_pack_json::from_json_str::<PackageSource>(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PackageSchemaError(error.to_string()))?;
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
    Ok(NormArtifactPackage { id: parsed.id, artifact: parsed.artifact, directory: parsed.directory, rust_package: parsed.rust_package, nx_project: parsed.nx_project, dependencies: parsed.dependencies })
}

//#region 💾️PayloadOpBinary
/// 💾️ The binary op frame of a norm mutation aggregate: `format u8` ([`dsl::variants_binary::OP_BINARY_FORMAT`]), the kind's
/// `tag u8` from the subset's `📡️.protocol.semio` (its only source of tags), then the leaf payload
/// ([`protocol::Mutation::payload_value`]) as canonical JSON — decoded back through `Mutation::from_payload_value`, so the
/// codec names no variant and no field.

//#endregion 💾️PayloadOpBinary

//#region 🪢️Mounts
#[path = "🧾️definition/🦀️.rs"]
pub mod definition;
#[path = "🪶️sqlite/🫳️admission/🦀️.rs"]
pub mod sqlite_native;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🪢️Mounts
