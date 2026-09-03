# 📓️ Day 2 resume — 2026-09-03

## What the overnight peer work changed
The `ToValue`/`FromValue` migration that blocked all of 2026-09-01 has essentially landed:
`semio-s-plugin-stdio` went from ~2200 errors to zero offenders (`#[value(transparent)]` now covers
newtype structs; `Part21Value::Typed` and `IfcValue::TypedValue` became named-field variants). The
framework store moved from `🏪️store/🦀️component.rs` to `🏪️store/🦀️.rs`, and a repo-wide rename turned
this plugin's artifact from `curate` into `curation`.

## This ticket's work survived the rename intact
Audited file by file. All present under the new spelling: the `🦀️.rs` mutation aggregate and its glue
mount, the three consolidated `🔣️.json` leaf descriptors (binary tags 0/1/2, no `🔣️component.json`
left), `DESCRIPTORS`/`descriptor()` on both hand-written impls,
`SourcingCurationArtifactPreparationFactory`, `SOURCING_CURATION_DOCUMENT_GRANT_BYTES`,
`build_artifact_store_one_item_preparation_factory`, all fourteen ids in
`SOURCING_CURATION_BOUNDED_TOOL_IDS` with no batch-only list, fourteen `Migrated` classifications plus
`setLocale` as `ForbiddenFromUi`, the `SourcingTestApp` guard and single `new_app()`, the
`semio-framework-ui-scene` dependency, `retire_parsed_document` in the relocated store, and the
`replyError` fault serialization.

**And the preparation factories still satisfy the refactored traits exactly** —
`ArtifactStoreOneItemPreparationFactory`, `ArtifactStoreOneItemPreparation`, the request/footprint/
checkpoint/grant/step types, `prepare_one_item`, `Edit` and `MutationMeta` were all re-diffed against
their current definitions. No signature drift.

## One blocker cleared here
`semio-s-plugin-stdio` would not compile: 54 `#[path = "…"]` mounts pointed at directories whose real
on-disk name differs only by a U+FE0F variation selector (`📄txt` vs `📄️txt`). The rename sweep added
VS16 to directory names and missed the path strings; rustc dies at the first one, so it read as a
single error. Repaired by rewriting each string to the exact on-disk spelling — mechanical, and the
script refuses to write unless every mount resolves.

A repo-wide sweep afterwards found **zero** VS16 mounts left anywhere, so peers cleared the rest in
parallel. 44 mounts remain genuinely absent across 24 other files (mostly test files reaching for
`🗄️stdio/🧪️oracle/⚖️law/🦀️.rs`, which does not exist). **Sourcing has none** — its
`mutate-curation-1` test deliberately does not use the shared `⚖️law` module and says so in its own
docstring. So those absences do not touch this ticket.

## Still to do
The served artifacts are stale: `semio_s_plugin_sourcing_component.core.wasm` is still from Sep 1
12:30 and the regenerated `🔣️.json` confirms it — 6 commands `migrated`, 8 still
`batchOnlyPendingRewrite`, i.e. a pre-migration build. Everything in
[✅️end-to-end-checklist.md](✅️end-to-end-checklist.md) from step 1 onward remains open.

Verification now runs against an isolated `CARGO_TARGET_DIR` — peers hold the shared build-directory
lock for long stretches, and three separate attempts stalled on it rather than on anything real.

## 🟢️ 01:35 — the wasm check went green
`cargo check -p semio-s-plugin-sourcing --target wasm32-wasip2` exits 0. Everything this ticket
built — the mutation aggregate, both hand-written `Mutation` impls, the document-lane preparation
factory, all fourteen migrated tool proofs — compiles against the fully refactored framework.

Two blockers were cleared here to get there, both completions of a peer's own half-landed change:
1. **stdio's 54 VS16 `#[path]` mounts** (above).
2. **`CapabilityGrant` missing `ToValue`/`FromValue`.** Its own comment said *"BLOCKED: see
   `Capability` above — must keep pace with it"*. `Capability` had since gained both derives, and
   `CapabilityToken` carries hand-written impls, so the stated blocker was already discharged; the
   derive simply had not followed. One line.

## ⏳️ The window is narrow
Green does not stay green. At 01:35 the check passed; by 01:39 the plugin build hit eighteen fresh
`cannot find derive macro Serialize` errors in `📡️replication` — a peer had removed the import while
derives still referenced it. The test profile separately shows 45 errors there (serde stripped from
`SchemaId`/`PayloadHash`/`MutationId` while `MutationMeta` still derives it). All theirs, all
in-flight.

So verification now runs as a RETRY LOOP against a warm isolated `CARGO_TARGET_DIR`, re-attempting the
plugin build until one lands inside a green window. Two lessons worth keeping:
- The shared `target/` build-directory lock, not compilation, was the dominant cost — three separate
  attempts stalled on it. An isolated target dir with `RUSTC_WRAPPER=""` removes both that lock and
  sccache's serialisation.
- Do not patch a file a peer is typing in. `CapabilityGrant` was safe because it completed an
  instruction they had written down themselves; chasing their moving `📡️replication` edits would not
  have been.
