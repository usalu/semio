# Grouped IO Caller Custody

Literal current Rust grouped imports requiring direct defining OS IO ownership, retaining all other General imports.

{
  "symbols": [
    {
      "name": "io_dialects_for",
      "owner": "dialects_for"
    },
    {
      "name": "format_accept_filter",
      "owner": "format_accept_filter"
    },
    {
      "name": "format_descriptor",
      "owner": "format_descriptor"
    },
    {
      "name": "formats_csv",
      "owner": "formats_csv"
    },
    {
      "name": "io_compose_via",
      "owner": "io_compose_via"
    },
    {
      "name": "io_dispatch",
      "owner": "io_dispatch"
    },
    {
      "name": "io_keys_for",
      "owner": "io_keys_for"
    },
    {
      "name": "list_composer_entries",
      "owner": "list_composer_entries"
    },
    {
      "name": "normalize_format_kind",
      "owner": "normalize_format_kind"
    },
    {
      "name": "preflight_composer_entry_refs",
      "owner": "preflight_composer_entry_refs"
    },
    {
      "name": "preflight_format_descriptors",
      "owner": "preflight_format_descriptors"
    },
    {
      "name": "preflight_subset_validators",
      "owner": "preflight_subset_validators"
    },
    {
      "name": "register_composer_entries",
      "owner": "register_composer_entries"
    },
    {
      "name": "register_composer_entry_refs",
      "owner": "register_composer_entry_refs"
    },
    {
      "name": "register_format_descriptors",
      "owner": "register_format_descriptors"
    },
    {
      "name": "register_subset_validator",
      "owner": "register_subset_validator"
    },
    {
      "name": "register_subset_validators",
      "owner": "register_subset_validators"
    },
    {
      "name": "io_resolve",
      "owner": "resolve"
    },
    {
      "name": "set_io_fallback_dispatcher",
      "owner": "set_io_fallback_dispatcher"
    },
    {
      "name": "subset_validator_entry_of",
      "owner": "subset_validator_entry_of"
    },
    {
      "name": "wire_artifact_compose",
      "owner": "wire_artifact_compose"
    },
    {
      "name": "wire_decode_composed_artifact",
      "owner": "wire_decode_composed_artifact"
    },
    {
      "name": "wire_list_composer_entries",
      "owner": "wire_list_composer_entries"
    },
    {
      "name": "Analysis",
      "owner": "Analysis"
    },
    {
      "name": "AnalyzeSource",
      "owner": "AnalyzeSource"
    },
    {
      "name": "AsyncComposeFn",
      "owner": "AsyncComposeFn"
    },
    {
      "name": "ComposeError",
      "owner": "ComposeError"
    },
    {
      "name": "ComposeSource",
      "owner": "ComposeSource"
    },
    {
      "name": "ComposedArtifact",
      "owner": "ComposedArtifact"
    },
    {
      "name": "ComposerEntry",
      "owner": "ComposerEntry"
    },
    {
      "name": "Composition",
      "owner": "Composition"
    },
    {
      "name": "IoConfidence",
      "owner": "Confidence"
    },
    {
      "name": "ErasedComposeSource",
      "owner": "ErasedComposeSource"
    },
    {
      "name": "FormatDescriptor",
      "owner": "FormatDescriptor"
    },
    {
      "name": "FormatRegistryError",
      "owner": "FormatRegistryError"
    },
    {
      "name": "IoDirection",
      "owner": "IoDirection"
    },
    {
      "name": "IoFallback",
      "owner": "IoFallback"
    },
    {
      "name": "IoFallbackDispatcher",
      "owner": "IoFallbackDispatcher"
    },
    {
      "name": "IoKey",
      "owner": "IoKey"
    },
    {
      "name": "IoPayload",
      "owner": "IoPayload"
    },
    {
      "name": "IoResolveError",
      "owner": "IoResolveError"
    },
    {
      "name": "SubsetValidator",
      "owner": "SubsetValidator"
    },
    {
      "name": "SubsetValidatorEntry",
      "owner": "SubsetValidatorEntry"
    },
    {
      "name": "WireComposeSource",
      "owner": "WireComposeSource"
    },
    {
      "name": "WireComposedArtifact",
      "owner": "WireComposedArtifact"
    }
  ],
  "roster": [
    {
      "path": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs",
      "statement": "use semio_framework::{AppDefinition, MediaClass, MediaType, ConfigSpec,\n// Terminology, Locale, …}` lines need the FULL framework crate's surface, which this wasm-safe\n// kernel crate cannot depend on without an actual `semio-framework` → `semio-framework-os-kernel`\n// →(back to)→ `semio-framework` cargo dependency CYCLE (`semio-framework` already depends on this\n// crate — see its Cargo.toml). It is mounted in `🧰️framework/📦️packages/🦀️rust/🦀️.rs`\n// (the `semio-framework` crate) instead, where all of those symbols already live — see that\n// file's own `os_workflow` mount for the real fix, and the run crate's glue.rs for the matching\n// `extern crate semio_framework as workflow;` alias change.\n\n// 🚪️ `io`'s FULL registry file (`ComposerEntry`/`IoKey`/`io_dispatch`/`SubsetValidator`/…) is\n// still mounted independently here AND in `semio-framework`'s own glue (as `io`) — that half of\n// the double-mount is recorded debt D2 (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM),\n// cleaned up wholesale at W6 alongside the old registry itself. This mount exists solely so\n// `store::ArtifactEnvelope` can carry a persisted `dialect`/`migrated_from` coordinate (26/08/10\n// D4 evolution slice); a kernel-side dependency on the full `semio-framework` crate (to reuse ITS\n// `io` mount instead) would be circular — see the `os_workflow`/`workflow` comment above.\n#[path = \"../../../../🔨️modules/🚪️io/🦀️.rs\"]\npub mod os_io;\n\n// 🪡 Thunk macros resolve their types and dispatch through this canonical kernel I/O owner.\npub use crate::os_io as io;\npub use crate::os_io::{ComposeFuture, ErasedComposeSource};"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs",
      "statement": "use semio_framework::{ErasedComposeSource, IoDirection, IoKey, IoPayload};"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs",
      "statement": "use semio_framework::{io::{self}, io_schema::{IoError, IoOutcome, IoPayload}, sqlite_snapshot::{self, SnapshotEncoding, SqliteSnapshotPhase}};"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs",
      "statement": "use semio_framework::{io::{self}, io_schema::{IoError, IoOutcome, IoPayload}, sqlite_snapshot::{self, SqliteSnapshotPhase}};"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs",
      "statement": "use semio_framework::{\n    io, kernel, AppDefinition, AssetDeclaration, ComposerEntryDescriptor, ContributedInferenceMetadata, ContributionSet, ExecutionProtocol, FileTypeContribution, IoEntryDescriptor, IoEntryDirection, MediaClass, MediaForm, MediaType,\n    PackageDescriptor, PackageHashes, PackageRole, PanelTabDefinition, PluginManifest,\n};"
    }
  ]
}
