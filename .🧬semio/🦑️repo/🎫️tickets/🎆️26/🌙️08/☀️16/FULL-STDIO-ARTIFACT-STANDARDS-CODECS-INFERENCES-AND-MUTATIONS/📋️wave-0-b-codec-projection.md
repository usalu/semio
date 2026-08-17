# Wave 0-B Codec and Projection Contracts

## Scope

Implemented only the W0-B lease:

- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- this report

No plugin, stdio, glTF, script, launch, AGENTS, or git-worktree path was changed.

## Codec and Resource Contract

`os_io` now owns dependency-free contracts for:

- source-local byte/line/column spans and structured diagnostics/failures;
- shared cancellation tokens, bounded read bytes, work units, logical allocations, and consumption accounting;
- deterministic `Canonical` and `Lossless` decode/encode policies and invocation contexts;
- streaming payload sources/sinks, optional `RandomAccessPayload`, bounded sniff results, and resource resolution;
- layered `PayloadCodec` and typed `ArtifactCodec` traits.

The contracts use only `std` plus local `os_io` domain types. No external runtime interface enters their public signatures.

## Deterministic Registration

- Composer, subset-validator, and format registries use ordered maps and atomically reject typed conflicts without replacing prior owners.
- Format collisions cover identity aliases, extensions, and non-empty MIME claims. `FormatDescriptor::registered_mime()` treats an empty or whitespace MIME as intentionally unregistered.
- This lets `txt` own `text/plain` while EPW has no MIME claim and is routed by extension/sniffing, as coordinated with W0-C.
- The document-codec registry is an ordered map and returns `DocumentCodecRegistryConflict` for every repeated schema; the established codec remains authoritative.

## Projection Boundary

`os_store` now exposes a typed projection/inference seam:

- `ArtifactRevision` and `ArtifactProjectionStamp` carry history identity and generation;
- events carry cause, reconciled snapshot, previous state, explicit cache mode, and caller-defined policy;
- results retain the input stamp and optional semantic-diff proposal;
- `ArtifactStore::accept_projection_result` rejects a stale stamp and returns an accepted result without applying its proposed diff.

`ArtifactCommand::projection_cause` maps apply, remote ingest, undo, redo, and checkout commands. Reset has an explicit projection cause at event construction. Strict `ArtifactDiff` semantic application is deliberately outside this contract boundary.

## Evidence

Commands run from the repository root on 2026-08-16:

```text
rustfmt --edition 2021 --check \
  🧰️framework/🔨️modules/🚪️io/🦀️component.rs \
  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
```

Exit status: `0`.

```text
git diff --check -- \
  🧰️framework/🔨️modules/🚪️io/🦀️component.rs \
  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
```

Exit status: `0`.

```text
cargo test -p semio-framework-os-kernel --lib \
  os_io::tests::io_registry_rejects_a_conflicting_key_without_replacing_the_first -- --exact
cargo test -p semio-framework-os-kernel --lib \
  os_io::tests::format_registry_allows_an_unregistered_mime_and_rejects_duplicate_claims -- --exact
cargo test -p semio-framework-os-kernel --lib \
  os_io::tests::codec_budget_enforces_limits_and_shared_cancellation -- --exact
cargo test -p semio-framework-os-kernel --lib \
  os_store::component::tests::projection_result_gate_rejects_results_after_every_invalidating_store_transition -- --exact
cargo test -p semio-framework-os-kernel --lib \
  os_store::component::tests::register_document_codec_rejects_a_duplicate_schema_without_replacing_the_first -- --exact
cargo test -p semio-framework-os-kernel --lib \
  os_store::component::tests::document_codec_of_round_trips_dsl_and_pack_and_edit_text -- --exact
```

All six focused tests passed. Compilation completed successfully; the crate emitted pre-existing warnings in unrelated DSL/SPR fixture paths.
