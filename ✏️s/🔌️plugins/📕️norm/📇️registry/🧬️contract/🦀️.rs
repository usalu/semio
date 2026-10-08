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
pub mod results_window_config {
    #[path = "../../🪟️results/🎚️config/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🪟️results/🎚️config/🧬️schema/🦀️.rs"]
    pub mod schema;

    #[path = "../../🪟️results/🎚️config/🧬️schema/🔺️diff/🦀️.rs"]
    pub mod diff;

    #[path = "."]
    pub mod mutations {
        #[path = "../../🪟️results/🎚️config/🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "../../🪟️results/🎚️config/🧬️schema/🧬️mutations/☑️change-selected-check-index/🦀️.rs"]
        pub mod change_selected_check_index;
    }
    #[path = "."]
    pub mod io {
        #[path = "."]
        pub mod binary {
            #[path = "../../🪟️results/🎚️config/🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
            pub mod snapshot;
            #[path = "../../🪟️results/🎚️config/🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
            pub mod mutations;
        }
        #[path = "."]
        pub mod text {
            #[path = "../../🪟️results/🎚️config/🚪️io/📝️text/📸️snapshot/🦀️.rs"]
            pub mod snapshot;
            #[path = "../../🪟️results/🎚️config/🚪️io/📝️text/🧬️mutations/🦀️.rs"]
            pub mod mutations;
        }
    }

}

#[path = "🖥️app-surface/🦀️.rs"]
pub mod app_surface;

#[path = "🪡️list-delta/🦀️.rs"]
pub mod list_delta;

#[path = "🧾️definition/🦀️.rs"]
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
pub mod payload_op_binary {
    use protocol::{Mutation, ProtocolError};

    /// 🚨️ A malformed-op error at `offset`.
    fn malformed(what: &'static str, offset: u64, detail: String) -> ProtocolError {
        ProtocolError::Malformed { what, offset, detail }
    }

    /// 📦️ Encodes `op` under the tag its kind's record declares in `protocol_semio`.
    pub fn encode<S, M: Mutation<S>>(protocol_semio: &str, op: &M) -> Result<Vec<u8>, ProtocolError> {
        let kind = op.descriptor().semantic_kind;
        let tag = dsl::protocol_record::records(protocol_semio).find(|(record, _)| *record == kind).map(|(_, tag)| tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record for '{kind}'")))?;
        let tag = u8::try_from(tag).map_err(|_| malformed("op tag", 1, format!("record '{kind}' tag {tag} exceeds the u8 tag field")))?;
        let mut out = vec![dsl::variants_binary::OP_BINARY_FORMAT, tag];
        out.extend_from_slice(semio_framework_pack_json::to_json_string(&op.payload_value()).as_bytes());
        Ok(out)
    }

    /// 📖️ Decodes the op whose tag names its kind's record in `protocol_semio`; bytes that do not re-encode to themselves
    /// are refused.
    pub fn decode<S, M: Mutation<S>>(protocol_semio: &str, bytes: &[u8]) -> Result<M, ProtocolError> {
        let (&format, rest) = bytes.split_first().ok_or_else(|| malformed("op format", 0, "the op is empty".into()))?;
        if format != dsl::variants_binary::OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("unsupported op format {format}")));
        }
        let (&tag, body) = rest.split_first().ok_or_else(|| malformed("op tag", 1, "the op carries no tag".into()))?;
        let kind = dsl::protocol_record::kind(protocol_semio, u64::from(tag)).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record with tag {tag}")))?;
        let text = std::str::from_utf8(body).map_err(|error| malformed("op payload", 2, error.to_string()))?;
        let payload: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| malformed("op payload", 2, error.to_string()))?;
        let op = M::from_payload_value(kind, payload).map_err(|error| malformed("op payload", 2, error.to_string()))?;
        if encode::<S, M>(protocol_semio, &op)? != bytes {
            return Err(malformed("op encoding", 0, "the op bytes are not canonical".into()));
        }
        Ok(op)
    }
}
//#endregion 💾️PayloadOpBinary

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path="🪶️sqlite/🫳️admission/🦀️.rs"]
pub mod sqlite_native;
