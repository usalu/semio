# `PropertyBag`/`PropertyValue` `ToValue`/`FromValue` — already landed; `🕸️dag` converted

## Framework gap: found already closed

The briefing described `PropertyBag` (`BTreeMap<String, graph::manifest::PropertyValue>`,
`🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs:239`) as having no `ToValue`/`FromValue` anywhere,
and `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs` as missing a blanket `BTreeMap` impl. Neither
gap exists in the tree as of this session — both were closed already, apparently by a concurrent
session on this same ticket:

- `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:296-307` already has
  `impl<T: ToValue> ToValue for BTreeMap<String, T>` and
  `impl<T: FromValue> FromValue for BTreeMap<String, T>` (String-keyed, exactly the derive's own
  constraint noted in the briefing).
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs:181-190` already has hand-written
  `impl dsl_core::ToValue for PropertyValue` / `impl dsl_core::FromValue for PropertyValue`
  (necessarily hand-written, not derived — `PropertyValue` is an untagged recursive
  Null/Bool/Number/String/Array/Object enum, doesn't fit the derive's newtype-variant codegen; the
  doc comment above the impls explains this and is dated this same ticket, Phase 2, 26/09/02).

Reasoning was verified end-to-end, not assumed: `dsl_core` in the graph crate is
`extern crate semio_framework_os_kernel as dsl_core` (`🕸️graph/📦️packages/🦀️rust/🦀️.rs:15`);
`semio_framework_os_kernel`'s `DslValue`/`ToValue`/`FromValue`/`ValueError` are
`pub use crate::os_dsl::schema::{...}` (`🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:337`), which in
turn is `pub use protocol::value::{...}` (`🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:444`), and
`protocol::value` is `🌱️value/🦀️.rs` (`📡️replication/📦️packages/🦀️rust/🦀️.rs:33`). One trait
family, four import paths — `PropertyValue` implementing it plus the blanket `BTreeMap<String, T>`
impl together already give `PropertyBag: ToValue + FromValue` for free. No new framework code was
needed or added this session.

## `🕸️dag` conversion (the part still outstanding)

Converted the 3 `serde_json` call sites the briefing pointed at, in
`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs`:

- Deleted the `properties_json_of` helper (old lines 85-94, `serde_json::to_string`) and its stale
  "framework gap" doc comment — its two call sites (`ReplaceNodeProperties`, `ConnectNodes` encode
  arms) now call the file's existing generic `json_of::<T: dsl::ToValue>` directly, same pattern
  already used for `node_json`/`new_kind_json`.
- Both decode-side `serde_json::from_str::<PropertyBag>(...)` calls (`ReplaceNodeProperties`,
  `ConnectNodes` decode arms) became `dsl::json::from_json_str::<PropertyBag>(...)`, matching the
  existing `new_kind_json`/`node_json` decode pattern in the same match.
- Updated the stale `OpText` region doc comment (old line 30) that still said "`serde_json`-encoded
  string field" — now says `dsl::json`-encoded (`ToValue`/`FromValue`, not `serde_json`), since
  `DagNodeSpec`/`DagNodeKind` still round-trip as opaque JSON-string fields (unchanged, out of this
  ticket's 3-refs scope) but via the framework's own codec, not serde.

No behavior change intended: `dsl::json::to_json_string`/`from_json_str`
(`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1418,1424`) are bounded on the identical
`protocol::value::{ToValue, FromValue}` traits, and the `BTreeMap<String, T>` blanket impl's
`from_value` `.collect()`s into a fresh `BTreeMap` (re-sorted by key) regardless of the decoded
object's entry order, so the round trip does not depend on wire insertion order — verified by
reading the impl bodies (`🌱️value/🔁️codec/🦀️.rs:296-307`), not assumed.

## Measurement

`python3 /tmp/prodserde.py ✏️s/🔌️plugins/🕸️dag 40`

- Before this session's edit: **5** (3 in `🚪️io/🧬️mutations/📝️text/🦀️.rs` at lines 93, 127, 130;
  2 in `✳️any/✏️editor/🦀️.rs` at lines 152, 157 — untouched, out of this pass's scope per the
  briefing).
- After: **2** — both remaining refs are the `✏️editor/🦀️.rs` ones, left alone as instructed.

Target (5 → 2) met exactly.

## Files touched this session

Only one file edited:
`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs`
(`git diff --stat`: 1 file changed, 8 insertions(+), 7 deletions(-)).

No framework file (`🕸️graph`, `🌱️value`) needed editing — both were already correct. No `Cargo.toml`
touched. Zero `cargo` commands run, zero sub-agents spawned — every check above was done by reading
source files and running the provided pure-Python measurement script.

`git diff --name-only` at time of writing lists several hundred files repo-wide (many concurrent
sessions on this same ticket touching stdio/procedural/forms/imperative/shooting/cad/etc., all of
which are on the briefing's DO NOT TOUCH list and were never opened this session) — confirmed my
one file is in that list and traced no other unexpected changes attributable to this session.
