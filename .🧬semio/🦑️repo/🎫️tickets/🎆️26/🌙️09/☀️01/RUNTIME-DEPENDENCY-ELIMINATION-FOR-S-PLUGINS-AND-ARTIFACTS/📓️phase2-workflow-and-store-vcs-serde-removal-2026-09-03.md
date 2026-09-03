# Phase 2 — workflow serde removal + store VCS text-codec (2026-09-03)

## ITEM 1 — `🔁️workflow`

Verified the recon claim, then removed serde from every type in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs` and its 23 mutation-leaf files under
`🧬️schema/🧬️mutations/**` (the `🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/**` tree was already
serde-free — a prior pass, not this one).

- Stripped `Serialize, Deserialize` from every `#[derive(...)]` and removed every `#[serde(...)]`
  attribute (struct/enum- and field-level) in the main file and all 23 leaf files (`use
  serde::{Deserialize, Serialize};` removed from each leaf file too).
- Two transitive breaks found by compiling, both fixed in-scope:
  - `🖥️host/🦀️.rs`'s `os_workflow_to_node_graph_payload` embedded `edge.contract` (a
    `MediaContract`) straight into a `serde_json::json!()` literal, which needs `Serialize`. Fixed
    by routing through the permanent `DslValue → serde_json::Value` bridge instead:
    `serde_json::Value::from(edge.contract.to_value())` (🖥️host/🦀️.rs:3864).
  - A **concurrent session** (not me — confirmed via git diff/status, only my own edits were
    staged) landed, mid-session, its own full serde removal for this same file, including
    rewriting the one oracle test that used to call `serde_json::to_value`/`from_value` directly on
    `RunTrigger`/`RunNodeRecord` (`run_payload_json_uses_exact_camel_case_and_rejects_unknown_fields`,
    now built on `dsl::os_pack::json::to_json_string`/`from_json_str` +
    `value_eq_ignoring_object_order`). That superseded my own interim compromise (I had kept
    `Serialize`/`Deserialize` on `RunTrigger`/`RunNodeRecord`/`RunNodeStatus`/`PortFingerprint`/
    `RunOutputArtifact` to satisfy that test) — the peer's version is the cleaner end state and is
    what is on disk now. My own edits to `🖥️host/🦀️.rs` and `🏪️store/🦀️.rs` (ITEM 2, below)
    survived untouched.
- `semio-framework`'s own `Cargo.toml` still carries `serde`/`serde_json` as a regular
  dependency — **not cleared**, because `🛂️manifest` (also `#[path]`-mounted into the same crate)
  still has ~27 in-flight serde-removal errors of its own (confirmed via `cargo check -p
  semio-framework --tests`, all 27 in `🛂️manifest/🦀️.rs`, zero in workflow) — this is a different,
  concurrently-owned item per this ticket's own note. Per the loop rule, the manifest line is
  restored/left alone until that crate compiles clean without it.

## ITEM 2 — os-kernel `store` VCS text-codec

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`'s `print_ops_log` (the op-log encoder
shared by `print_document_text`/`print_document_pack`) had 3 `serde_json::to_string` call sites for
`metadata `/`message `/`conflict ` op-lines (`MutationMeta`, `MutationMessage`,
`crate::os_spr::Conflict`). The **decode** side of the exact same three record kinds
(`parse_document_text`'s `OpsHeaderLine::Metadata`/`Message`/`Conflict` arms) was **already** on
`crate::os_pack::json::from_json_str` — this was a half-finished symmetry gap, not new plumbing.

- Swapped all 3 encode call sites to `crate::os_pack::json::to_json_string(...)` (infallible, so the
  `.map_err(|error| VcsError::Serialize(...))?` wrapper was removed too — matches
  `to_json_string`'s `fn to_json_string<T: ToValue>(value: &T) -> String` signature).
- All three types (`MutationMeta`, `MutationMessage`, `crate::os_spr::Conflict`, defined in
  `semio-framework-replication`) already carry hand-written `ToValue` impls that mirror their
  `#[serde(rename_all/skip_serializing_if/tag)]` shape field-for-field — verified by reading each
  impl, not assumed.
- Added a new differential test,
  `ops_log_records_to_json_string_match_serde_json_byte_for_byte` (🏪️store/🦀️.rs, right after
  `document_text_round_trips_authoritative_metadata_messages_conflicts_and_cursor`), comparing
  `crate::os_pack::json::to_json_string(&x)` to `serde_json::to_string(&x).unwrap()` for all three
  types across sparse (every `Option`/default field omitted) and dense fixtures, including both
  non-default `MutationOrigin` enum-variant-with-fields shapes (`Contributed`/`Transaction`) — the
  exact shape category the ticket flagged as previously bug-prone.
- Did not touch `impl ArtifactPack for serde_json::Value` or the `DslValue ↔ serde_json::Value`
  bridge (left alone per instructions).

## Verification (this session, `iso3` target dir)

```
cargo check -p semio-framework-os-kernel --message-format short   → 0 errors
cargo check -p semio-framework --message-format short             → 0 errors
cargo check -p semio-framework --tests                            → 27 errors, all in 🛂️manifest/🦀️.rs (0 in workflow)
cargo metadata --no-deps --format-version 1; echo $?               → 0
```

`cargo check -p semio-framework-os` (host) and `-p semio-s-plugin-space` were also run: both have
pre-existing errors, **none** touching workflow/host/store — `semio-framework-os`'s 10 errors are
all `AppRef`/`dsl::io_schema::IoPayload` in `🔌️plugin/🖥️host`/`🎚️config` (a different, concurrent
peer session's in-flight serde work — confirmed via `git status`/`git diff --stat`, I made zero
edits there); `semio-s-plugin-space`'s 29 errors are all pre-existing `♾️infinite`/`🎲️board/…/🕸️dag`
issues plus the same `AppRef`/`IoPayload` set. Could not run the new store.rs test standalone —
`cargo test -p semio-framework-os-kernel` pulls in that crate's full plugin dev-dependency list
(every `s/plugin`), which currently fails to build for the same unrelated `♾️infinite`/dag reasons;
verified the test by static cross-check against adjacent pre-existing tests in the same file using
identical `crate::os_spr::*` construction patterns instead.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs` (serde strip; superseded/extended by a
  concurrent session's own pass, see above)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🧬️schema/🧬️mutations/**` (23 leaf files, serde strip)
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs` (`os_workflow_to_node_graph_payload`'s `contract` field)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` (3 encode call sites + new differential test)
