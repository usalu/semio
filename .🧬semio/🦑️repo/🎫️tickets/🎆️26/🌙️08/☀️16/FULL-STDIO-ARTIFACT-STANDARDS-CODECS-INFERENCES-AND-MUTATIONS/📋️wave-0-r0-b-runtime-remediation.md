# Wave 0 R0-B Runtime Remediation

## Scope

Updated only:

- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- this report

No plugin, stdio, command-trait, script, AGENTS, or git file was modified by this shard.

## Landed Contract

- Codec contexts own finite read, write, work, allocation, and recursion budgets plus shared cancellation. Streaming and random-access reads pass through host-owned bounded wrappers; decode and encode both carry a `ResourceResolver` seam.
- `SourceSpan`, owned `AnchoredSyntax`, `OpaqueExtension`, and `ArtifactCodecResult` validate lossless records. Canonical finalization sorts and verifies the retained records; it does not claim that a concrete format codec exists.
- Composer, subset-validator, format, child-store factory, document-codec, dialect-migration, fallback-dispatcher, and touched wire/query paths return typed availability or conflict errors. Fallible public registrations use `#[must_use]`.
- Preflight plus atomic batch registration is available for format descriptors, subset validators, composer entries, document codecs, and dialect migrations.
- `FormatDescriptor` is plural: `mimes: Vec<String>` and `extensions: Vec<String>`. It has no singular compatibility accessor. Claims are trimmed, case-normalized, sorted, and duplicate normalized MIME, extension, or alias inputs reject with `FormatRegistryConflict::Invalid`; cross-descriptor MIME and extension collisions reject.
- `ArtifactStore::new` is fallible. It validates authoritative edit/change/checkpoint/alternative identities and references; a missing cursor folds all authoritative edits in deterministic history order. Reset and remote snapshots use the same validation before mutation.
- Mutation validation occurs before inverse construction, apply, replay, remote ingest, and remote snapshot merge. Remote records with an existing but inequivalent identity reject before merge.
- Projection freshness has explicit Replay, PolicyChange, ExternalResourceChange, Checkpoint, and PruneDrafts causes. Remote mutation and snapshot paths record RemoteIngest; checkpoint pinning and explicit checkpoint selection return typed results and invalidate through the shared generation/stamp seam.
- Empty checkpointing rejects with `VcsError::ValidationFailed`; draft pruning rejects with `VcsError::ValidationFailed`; branching an empty history returns `VcsError::NoCheckpoint`. None reports a misleading success.

## Verification

Run from repository root on 2026-08-16:

```text
rustfmt --edition 2021 --check \
  🧰️framework/🔨️modules/🚪️io/🦀️component.rs \
  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
```

Result: passed.

```text
cargo check -p semio-framework-os-kernel --lib
```

Result: passed, with pre-existing warnings in DSL/SPR paths.

```text
cargo test -p semio-framework-os-kernel --lib \
  os_io::tests::format_registry_allows_an_unregistered_mime_and_rejects_duplicate_claims \
  -- --exact
```

Result: did not reach the selected test. The combined test build currently fails outside this lease in `📡️spr/🎮️command/🦀️component.rs` (`dsl_derive::CompositeMutation` missing, unresolved `delta` formatting placeholders, and missing `MutationKind` implementation) and `📡️spr/🧪️testkit/🦀️component.rs` (`MutationOrigin` not re-exported through `os_spr`). The two direct store `origin` metadata initializers exposed by that build were migrated in this shard; no owned library error remains under `cargo check --lib`.

Earlier focused runtime checks passed before the combined test tree was broken by concurrent SPR changes:

```text
cargo test -p semio-framework-os-kernel --lib \
  os_io::tests::codec_context_bounds_streaming_random_access_recursion_and_resolved_resources \
  -- --exact
cargo test -p semio-framework-os-kernel --lib \
  os_store::component::tests::dialect_migration_preflight_and_batch_commit_are_conflict_free_or_noop \
  -- --exact
cargo test -p semio-framework-os-kernel --lib \
  os_store::component::tests::projection_result_gate_rejects_results_after_every_invalidating_store_transition \
  -- --exact
```

Result: passed at that point. They are not evidence for the later combined tree; rerun after the SPR test blockers clear.

## External Constructor Inventory

The new authoritative constructor is `ArtifactStore::new(envelope) -> Result<Self, VcsError>`. The following complete inventory is the current output of:

```text
rg -n -P 'ArtifactStore::new\(' --glob '*.rs' |
  rg -v '^🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:'
```

Count: 69 external calls. They deliberately remain for the global propagation shard.

```text
🧰️framework/🛍️products/💻️os/🦀️component.rs:621
🧰️framework/🛍️products/💻️os/🦀️component.rs:1333
🧰️framework/🛍️products/💻️os/🦀️component.rs:1629
🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:624
🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:1341
🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:1637
🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs:1765
🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs:2396
🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs:508
✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:58
✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:55
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2298
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2308
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2708
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2891
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2903
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2939
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2955
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2986
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2992
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3019
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3090
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3191
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3204
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3227
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3263
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:8110
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:12075
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:1227
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:49
🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs:148
✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:63
✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:50
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:61
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:96
✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:111
✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs:1040
✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:45
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1996/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:46
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:46
✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:53
✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:55
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📙️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:46
✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:54
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:45
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:65
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📕️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:42
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:46
✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:124
✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:233
✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs:885
✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs:903
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:46
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:46
✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:55
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:46
✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:72
✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:55
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:106
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:44
✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:60
✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:47
✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:183
✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️component.rs:50
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:60
✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:84
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs:397
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️component.rs:444
✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:43
```

## Plural-Format and Registry Handoff

Fresh `rg` inventory has one external `FormatDescriptor` literal and 30 external format-query callers. The literal is `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️component.rs:281`; it still constructs `mimes` from singular `representation.mime` and `extensions` from singular `representation.extension`, and must move to representation-owned plural claims.

The 30 query call sites are in these path families: `🧰️framework/🛍️products/💻️os/🦀️component.rs` (12), `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` (12), `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` (1), `✏️s/🔌️plugins/🏭️process/.../🚪️io/🦀️component.rs` (1), and `✏️s/🔌️plugins/🪐️space/...` media commands (3). They must propagate `FormatRegistryError` and choose representation claims explicitly; no primary MIME/extension replacement is available.

`document_codec` has four live external callers in plugin-host and store-sync (plus four documentation-only text matches), and `set_checkpoint_composition_pins` has one plugin caller. They must propagate the new typed errors.

## Remaining Blockers

- The combined test target is blocked by concurrent SPR derive/re-export/test changes described above; rerun all scoped tests after that owner repairs it.
- The 69 constructor calls and all plural-format/document-query call sites need the designated global propagation shard. This shard intentionally added no compatibility constructor or singular format accessor.
- The complete plugin atomic plan still needs the schema and native-inference registry preflight/batch APIs, outside R0-B ownership. Assembly cannot honestly claim all-registry atomicity until those land.
- No format-specific parser/serializer was added or represented as implemented. This is the executable host framework contract only.
