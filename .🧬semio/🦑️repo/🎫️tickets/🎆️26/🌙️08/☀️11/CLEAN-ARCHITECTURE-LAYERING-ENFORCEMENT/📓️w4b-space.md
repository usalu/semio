# W4b — Space Module Schema-Id Rename (`s.` → `os.`)

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs`

## Renames applied

1. `s.space` → `os.space` (schema id for `SpaceSnapshot`) — 4 literal sites:
   - `pub const S_SPACE_SCHEMA: &str = "os.space";` (line 84)
   - `#[dsl(id = "os.space")]` on `SpaceSnapshot` (line 103)
   - `assert_eq!(id, "os.space");` in dsl-id test (line 1783)
   - `kind_id: "os.space".into()` fixture in the real-store zip round-trip test (line 2282)
   - All other `S_SPACE_SCHEMA` constant references (lines 133, 1737, 2269, 2283) pick up the new
     value automatically since they reference the const, not a literal.

2. `s.collection` → `os.collection` (schema id for `CollectionSnapshot`) — 3 literal sites:
   - `pub const S_COLLECTION_SCHEMA: &str = "os.collection";` (line 484)
   - `#[dsl(id = "os.collection")]` on `CollectionSnapshot` (line 593)
   - `assert_eq!(id, "os.collection");` in dsl-id test (line 1773)
   - Doc comment on `CollectionRef` referencing the `` `s.collection` `` id (line 87) updated to
     `` `os.collection` `` for accuracy.
   - `S_COLLECTION_SCHEMA` constant reference (line 607) picks up new value automatically.

3. `s.puzzle2d` → `test.puzzle2d` (arbitrary test-fixture document schema id, unrelated to the
   layering fix but sharing the `s.` prefix by coincidence) — 12 literal sites across several test
   functions (lines 1720, 1890, 1907, 2013, 2060, 2061, 2079, 2089, 2107, 2136, 2143, 2172). Same
   replacement id used consistently everywhere.

`s.stdio.*` — confirmed absent from this file (grepped before and after); nothing touched there,
per instructions (separate in-progress work).

Net diff: 40 lines changed (20 replaced lines) in exactly this one file — confirmed via
`git diff --stat` that no other file was touched.

## Verification

- Crate identification: the `🪐️space/🦀️component.rs` module is **not** mounted into
  `semio-framework-os-kernel`'s `📦️packages/🦀️rust/📦️glue.rs` (that glue file's header notes
  "Infinite/flow component files exist under 🔨️modules/ but are unwired pending dep-DAG cleanup").
  It is instead path-mounted as `pub mod space` in
  `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📦️glue.rs`, gated behind the
  `os-host-full` cargo feature. The owning crate is `semio-framework-os` (package name in
  `🖥️host/📦️packages/🦀️rust/Cargo.toml`), NOT `semio-framework-os-kernel`.

- `cargo check -p semio-framework-os` (no feature flag): succeeds, 0 errors (only pre-existing
  warnings — unused-extern-crate, unnecessary-qualification, unused `#![feature(linkage)]`). Note:
  since `space` is `#[cfg(feature = "os-host-full")]`-gated, this pass does NOT compile my file at
  all — it's a baseline sanity check only.

- `cargo check -p semio-framework-os --features os-host-full` (the pass that actually compiles the
  space module): **15 pre-existing errors**, all located in
  `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` (`host_core`, mounted via
  `#[path = "../../🦀️component.rs"] mod host_core;`) — e.g. `AppDefinition`/`OsAppRegistration`
  missing/renamed `document` field, `ArtifactEnvelope` missing `dialect`/`migrated_from` fields,
  duplicate `label` field. Grepped the full error log for `🪐️space` — zero matches. All error
  spans are in `host_core component.rs`, a file I did not touch and am not assigned. This matches
  the documented "known unrelated concurrent document module churn" (a document-module refactor
  in flight elsewhere in the shared tree). **My change adds no new error class** — none of the 15
  errors reference `S_SPACE_SCHEMA`, `S_COLLECTION_SCHEMA`, `os.space`, `os.collection`,
  `test.puzzle2d`, or anything in the space module file.

- Could not run this file's own `#[test]` fns standalone: the whole `os-host-full` feature build
  must succeed first (space's tests live in the same compilation unit as the broken `host_core`
  code), and I'm not touching `host_core component.rs` per the "only edit assigned files" rule.
  `cargo test -p semio-framework-os space::` was attempted without the feature flag first (module
  not compiled at all under default features, hence "0 tests" — a false-negative, not a real
  pass) and then with `--features os-host-full` (blocked by the same 15 unrelated errors above).

## Status

Rename complete and self-consistent within the assigned file. Full test execution is blocked by
unrelated concurrent churn in `host_core component.rs`; not in scope to fix per assignment
boundaries. Flagging for whoever owns that document-module refactor / for a follow-up check once
it lands.
