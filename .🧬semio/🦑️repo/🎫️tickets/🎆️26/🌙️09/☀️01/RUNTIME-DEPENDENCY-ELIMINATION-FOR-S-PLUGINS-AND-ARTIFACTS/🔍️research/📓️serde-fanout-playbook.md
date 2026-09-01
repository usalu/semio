# Serde/serde_json Fan-Out Playbook

Companion to `📓️serde-replacement-surface.md` (read that first for the trait-bound rationale).
This is the mechanical recipe for converting the remaining ~39 `serde`/`serde_json` plugin
manifests. Written from real work done against `📖️playbook` (the pilot) — every rule below was
exercised on at least one real struct/enum in that crate, not theorized.

## Pilot status — read this first

**Plugin piloted: `📖️playbook` (`semio-s-plugin-playbook`).**

**NOT fully green.** What is proven, by a passing `cargo test`:

- `semio-framework-replication --lib`: 225/226 tests pass (the 1 failure is a pre-existing
  taxonomy-fixture path mismatch in a concurrent agent's unrelated work — `payloadSchema`
  string content, nothing to do with serialization; ignore it, don't "fix" it as part of this
  wave).
- `semio-framework-value-derive` (the new proc-macro crate): compiles clean.

**Written but not yet verified by a passing run**: the `📖️playbook` plugin's own `cargo check`.
The trait-bound edit plus the full core-type conversion (`PlaybookStep`/`PlaybookBlock`/
`PlaybookExpr`/`PlaybookVectorField`/`PlaybookBlockOption` in the framework kernel;
`PlaybookDiff`/`PlaybookStringList`/`PlaybookArtifact`/`PlaybookMutation` + its 9 leaf payload
structs in the plugin) were landed and are believed correct by inspection and by the replication
crate's own green run, but the actual `cargo check -p semio-s-plugin-playbook` did not complete
in this session — the machine was saturated by concurrent agents (up to 113 cargo/rustc
processes across 12 sessions), and an isolated `CARGO_TARGET_DIR` I was using to dodge that
contention made it worse (forces a from-scratch dependency rebuild) and was called out and
stopped by the ticket owner mid-session. **Whoever picks this up next: run
`cargo check -p semio-s-plugin-playbook --message-format=short` in the foreground against the
shared target dir, one build at a time, before doing anything else** — it will surface any
mistake in the manual `ToValue`/`FromValue` impls below (the `PlaybookArtifact`/`PlaybookDiff`
hand-written ones are the most likely to have a typo, since they were not derive-generated).

`📖️playbook`'s `Cargo.toml` **still has `serde`/`serde_json`** — do not remove them yet. ~20 of
the plugin's ~30 serde-touching files (editor commands, presence, config, engine snapshot,
inferences, the procedural extension) were surveyed but not converted; they don't implement
`Mutation`/`MutationDiff` so they were not required to prove the trait-bound fix, but the plugin's
manifest cannot go to zero third-party until they are. They all match the mechanical patterns
below — most are the "trivial DslRecord command struct" case, five minutes each.

## The trait-bound fix (already landed, framework-wide)

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`:

```rust
pub trait MutationDiff<P>: Clone + Default + crate::value::ToValue + crate::value::FromValue { .. }
pub trait Mutation<P>: Clone + crate::value::ToValue + crate::value::FromValue { .. }
```

was `Clone [+ Default] + serde::Serialize + serde::de::DeserializeOwned`. This is why every
`Mutation`/`MutationDiff` implementor across every plugin needs `ToValue`/`FromValue` now — not
optional, not per-plugin discretion. **This breaks every plugin that hasn't converted yet** (see
"Blast radius" in the research doc) — expected, per the ticket's own instructions.

## Dependency lines

Delete from a plugin's `Cargo.toml` `[dependencies]` (only once EVERY file in the crate is
converted — see the pilot-status warning above about partial conversion):

```toml
serde.workspace = true
serde_json = { workspace = true, features = ["float_roundtrip"] }   # or whatever feature set
```

Add, if the plugin uses `#[derive(ToValue, FromValue)]` anywhere (it will):

```toml
semio-framework-value-derive = { path = "<count ../ up to 🧰️framework>/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust", package = "semio-framework-value-derive" }
```

Every plugin already depends on `semio-framework-os-kernel` directly — the derive's generated
code is rooted at `::semio_framework_os_kernel::{ToValue, FromValue, DslValue, ValueError,
to_dsl_value, from_dsl_value}` (that crate re-exports the whole `protocol::value` surface at
`os_dsl::schema::{ToValue, FromValue, ValueError, DslValue, to_dsl_value, from_dsl_value}` →
promoted to crate root by `pub use crate::os_dsl::*;`). Do **not** point the derive's generated
paths at `protocol::` or `semio-framework-replication` directly — most plugins do not depend on
that crate directly (only `os-kernel` does), so a bare `protocol::` path will not resolve from
plugin code even though some already-existing plugin source oddly spells it that way (untested,
possibly broken — don't copy that pattern).

If the plugin needs the derive traits themselves in scope for a **hand-written** `impl ToValue`/
`impl FromValue` (see "Composed/child fields" below), import both the derive macros and the
traits — they're in separate Rust namespaces (macro vs. type) so both `use`s coexist under the
same identifiers with zero conflict:

```rust
use semio_framework_os_kernel::{FromValue, ToValue};              // the traits
use semio_framework_value_derive::{FromValue, ToValue};            // the derive macros
```

(Inside the framework crate itself — `os-kernel`'s own files — use `dsl::{FromValue, ToValue}` for
the traits instead, since `dsl` is that crate's own self-alias.)

## The mechanical rewrite table

| serde spelling | first-party spelling |
|---|---|
| `serde_json::Value` | `pack::json::Value` (framework-internal JSON text) **or** `protocol::value::DslValue` (in-memory tree used by `Mutation`/`MutationDiff` — this is what the derive macro targets) |
| `serde_json::from_str::<T>(s)` | `pack::json::parse(s)` → `pack::json::Value`, then hand-decode, **or** if `T: FromValue`: parse to `DslValue` isn't direct — see "JSON text vs DslValue" below |
| `serde_json::to_string(&v)` / `to_string_pretty` | `pack::json::to_string(&value)` where `value: &pack::json::Value` (no pretty-printer exists yet — see research doc's "deliberately not built" note) |
| `serde_json::to_value(&x)` (`x: T`) | `ToValue::to_value(&x)` → `DslValue` directly (no JSON-text round trip needed for in-memory use) |
| `serde_json::from_value(v)` | `FromValue::from_value(v)` |
| `#[derive(Serialize, Deserialize)]` | `#[derive(ToValue, FromValue)]` (from `semio_framework_value_derive`) — add **alongside**, don't just swap the derive list textually; also add the `#[value(...)]` attributes below, they are not inferred from `#[serde(...)]` |
| `#[serde(rename_all = "camelCase")]` | `#[value(rename_all = "camelCase")]` (also supports `"kebab-case"`, `"lowercase"`, `"snake_case"`) |
| `#[serde(rename = "foo")]` | `#[value(rename = "foo")]` |
| `#[serde(default)]` (bare) | `#[value(default)]` (bare) |
| `#[serde(default = "path")]` | `#[value(default = "path")]` |
| `#[serde(skip_serializing_if = "path")]` | `#[value(skip_serializing_if = "path")]` |
| `#[serde(tag = "kind")]` on an enum | `#[value(tag = "kind")]` — internally-tagged |
| `#[serde(tag = "kind", content = "value")]` on an enum | `#[value(tag = "kind", content = "value")]` — adjacently-tagged, supported |
| container `#[serde(default)]` (struct-level, "every field defaults") | container `#[value(default)]` |

### Not supported by `#[derive(ToValue, FromValue)]` — hand-write these

Rare enough in the repo-wide survey (`grep -rhn '#\[serde(' ✏️s` — see the research doc's full
table) that building macro support wasn't worth it for v1:

- ~~`#[serde(tag = "kind", content = "value")]` (adjacently-tagged)~~ **NOW SUPPORTED** (added
  mid-session, after os-kernel repair surfaced ~17 real occurrences using it) —
  `#[value(tag = "kind", content = "value")]` on the enum works exactly like serde's: unit
  variants encode as `{"kind": "..."}` (no content key), newtype/named-field variants encode as
  `{"kind": "...", "value": <payload-or-object>}`. If you hit an enum using `tag`+`content` and
  the derive still rejects it, the macro may be out of date on your branch — re-pull
  `🌱️value/✨️derive/🦀️component.rs` before hand-writing a workaround.
- `#[serde(flatten)]` — hand-write, splice the flattened struct's own entries into the parent's
  `Vec<(String, DslValue)>` directly.
- `#[serde(transparent)]` — hand-write as a 1-line passthrough (`self.0.to_value()`).
- `#[serde(deny_unknown_fields)]` — the derive parses but currently ignores this attribute
  (documented no-op); hand-write if you actually need the rejection.
- `#[serde(alias = "old_name")]` — hand-write (see `PlaybookBlockOption` below for the exact
  pattern: accept either key on decode, always emit the primary key on encode).
- `#[serde(serialize_with = "...")]` / `deserialize_with = "..."` — hand-write.
- `#[serde(bound(...))]` — the derive doesn't emit its own `where` bounds beyond what
  `ToValue`/`FromValue` on field types naturally requires; for a generic struct like
  `ArtifactChild<S>` you may need a hand-written generic impl instead (see below).

### JSON text vs. `DslValue` — pick the right one

- Inside a `Mutation`/`MutationDiff`/`MutationKind` payload, or anywhere the value only ever
  moves in-memory (never touches a file or the wire as literal JSON text): use `ToValue`/
  `FromValue` directly, `DslValue` is the value, no text round trip.
- Where the OLD code literally produced/consumed a JSON **string** (fixture files, a
  `blocksJson`/`contributionsJson`-style stringly-typed field, `include_str!(...)` test
  fixtures): use `pack::json::parse(text) -> pack::json::Value`, then convert
  `pack::json::Value` ↔ your type by hand (there is currently no generic bridge between
  `pack::json::Value` and `DslValue`/`ToValue` — they are sibling trees, not the same type; see
  research doc's open question). For a fixture-only `serde_json::Value` used purely as a test
  oracle assertion tree (`fixture["cases"].as_array()`-style), `pack::json::Value` has the same
  `as_array`/`get`/indexing shape — swap the type and adjust `Number` matching (`as_u64`/`as_f64`
  on `pack::json::Number`, not a bare `f64`).

## Traps actually hit (with the fix)

1. **Method-name collision with `DslField`.** `os_dsl`'s existing `DslField` trait (used by
   `#[derive(DslRecord)]`/`DslScalar`, for the *text/binary DSL grammar*, unrelated to JSON) ALSO
   declares `fn to_value(&self) -> FieldValue` / `fn from_value(value: &FieldValue) -> Result<Self,
   String>`. A type deriving both `DslRecord` and `ToValue`/`FromValue` has TWO same-named methods
   in scope. Fix: the derive macro's generated code always calls fully-qualified
   `::semio_framework_os_kernel::ToValue::to_value(&self.field)` / `FromValue::from_value(...)`,
   never `.to_value()`/`.from_value()` shorthand — copy that style in every hand-written impl too.

2. **Nested `Option<Option<T>>` fields collapse identically to naive serde**, with zero extra
   code — the blanket `impl<T: ToValue> ToValue for Option<T>` recurses naturally
   (`Some(None).to_value()` and `None.to_value()` both produce `DslValue::Null`, matching what
   plain `#[derive(Serialize)]` does for a double-`Option` with no `deserialize_with`). Don't
   add special-case handling unless the original type had `deserialize_with =
   "deserialize_double_option"` (rare — 2 occurrences repo-wide); if it does, you need a real
   presence-preserving encoding and must hand-write both directions.

3. **Composed/child-slot fields (`store::ArtifactChild<S>`) are NOT plain JSON.** They carry a
   `local_owner: Option<Arc<dyn Any>>` field that's `#[serde(skip)]`, and are generic over `S`
   with `#[serde(bound = "")]`. Don't try to derive `ToValue`/`FromValue` on them or route the
   derive macro through them. Instead, hand-write the composing struct's impl and bridge the
   child field(s) through the PRE-EXISTING `protocol::to_dsl_value(&self.child)` /
   `protocol::from_dsl_value::<ArtifactChild<S>>(value)` functions (the `serde`-based bridge —
   `ArtifactChild<S>` already implements `Serialize`/`Deserialize` and that's fine, it's a
   framework type, framework is exempt from the ban). See `PlaybookArtifact`/`PlaybookDiff` in
   `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/{🦀️component.rs,🔺️diff/🦀️component.rs}`
   for the exact pattern — `to_dsl_value`/`from_dsl_value` per composed field, everything else
   through `ToValue`/`FromValue` directly.

4. **Domain types your plugin's mutation payloads reference may live in the FRAMEWORK, not the
   plugin.** `PlaybookStep`/`PlaybookBlock`/`PlaybookExpr` are defined in
   `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs` (the OS kernel), not in the
   plugin crate — only the `Mutation` dispatch enum and its leaf payload structs moved to the
   plugin (see that file's own header comment for why). If your mutation leaf wraps a
   framework-owned domain type, you convert it THERE (framework, allowed to keep `serde`
   alongside — add `ToValue`/`FromValue` additively, `#[derive(Serialize, Deserialize, ToValue,
   FromValue, dsl::DslRecord)]`, don't remove the serde derive). Check with
   `grep -rn "struct <YourDomainType>\b" 🧰️framework/🛍️products/💻️os ✏️s/🔌️plugins/<yours>`
   before assuming it's plugin-local.

5. **A derive macro living in `os-kernel`'s own file needs `os-kernel` to depend on
   `semio-framework-value-derive`.** Added that dependency edge to
   `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`. If you convert a domain type that
   lives in a DIFFERENT framework crate, check whether that crate already has the dependency
   before assuming `#[derive(ToValue, FromValue)]` just works — it needs
   `semio-framework-value-derive` as a direct path dependency, same as `semio-framework-schema`
   already needs `semio-framework-schema-derive`.

6. **The `dsl_value_serde` bridge (`to_dsl_value`/`from_dsl_value`, `T: serde::Serialize +
   DeserializeOwned`) is NOT a permanent second path — flag it, don't lean on it.** It's used in
   trap #3 above as a pragmatic bridge for ONE framework-generic type (`ArtifactChild<S>`) that
   this wave didn't have budget to convert. For it to be deleted outright (CLAUDE.md forbids
   permanent compat layers, so this is real follow-up work, not optional polish): `ArtifactChild<S>`
   needs its own hand-written `impl<S> ToValue for ArtifactChild<S>` / `impl<S> FromValue for
   ArtifactChild<S>` in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (fields:
   `child_id: String`, `target: crate::os_io::ArtifactRef` — `ArtifactRef` itself needs a
   `to_uri()`/`parse_uri()`-based `ToValue`/`FromValue` too, or route through the existing
   `to_uri: String` round trip the way `enc_ref`/`dec_ref` already do by hand in
   `📸️snapshot/🦀️component.rs`). Once every composed-artifact plugin (everything with a
   `#[child(...)]` field — check `grep -rln '#\[child(' ✏️s`) is converted, `to_dsl_value`/
   `from_dsl_value` and the whole `🌱️value/🔀️serde` bridge module become dead code and should be
   deleted, not deprecated.

7. **Bulk attribute duplication is safe to script.** Every `#[serde(default, skip_serializing_if
   = "Option::is_none")]` line in a file needs an identical `#[value(...)]` line immediately
   after it, with zero exceptions in the survey. A small Python pass (insert-after-exact-match,
   idempotent — skip if the next line is already the `#[value(...)]` line) converted 21 fields in
   `PlaybookBlock` in one shot with zero manual edits; the 9 mutation-leaf structs (`AddStep`,
   `RemoveStep`, `MoveStep`, `AddBlock`, `RemoveBlock`, `MoveBlock`, `ReplaceBlock`, `UpdateStep`,
   `ChangeTitle`) were converted by one script pass each doing 4 fixed string replacements. Do
   this for the repetitive 90% of a crate, then hand-review the enum/`tag`/composed-field cases
   individually — those are where the real judgment calls are.

8. **The proc-macro's generated `match` arms must all be `Ok(...)`-wrapped when the closure also
   has an error arm returning `Err(...)`.** (Internal to `semio-framework-value-derive`'s own
   implementation, not something a fan-out agent needs to redo, but if you extend the macro:
   `.map(|variant| match ... { Ok(quote!{..}), .., other => Err(...) }).collect::<syn::Result<Vec<_>>>()`
   — every non-error arm needs `Ok(...)`, a bare `quote!{...}` in one arm and `return Err(...)` in
   another does NOT type-check as the closure's inferred return type.)

## Per-crate verification command

```bash
cargo check -p <crate-name> --message-format=short   # native, fast signal
cargo build --lib --target wasm32-wasip2 -p <crate-name>
cargo test -p <crate-name>
```

Run all three in the **foreground**, against the **shared** `target/` directory (no
`CARGO_TARGET_DIR` override — an isolated one forces a full from-scratch dependency rebuild,
which is what saturated the machine this session), **one crate at a time** — other agents are
building concurrently and the box has a hard concurrency ceiling that was already exceeded once
this session (113 cargo/rustc processes across 12 sessions).

## Verbatim verification output obtained this session

`cargo test -p semio-framework-replication --lib` (full tail, 225 passed / 1 failed):

```
test value::codec::tests::btreemap_round_trips_in_key_order ... ok
test value::codec::tests::scalars_round_trip ... ok
test value::codec::tests::option_collapses_nested_none_like_naive_serde ... ok
test value::codec::tests::vec_round_trips_and_reports_index_on_error ... ok
...
failures:

---- causal::tests::causal_add_fixture_has_exact_required_descriptor stdout ----
thread 'causal::tests::causal_add_fixture_has_exact_required_descriptor' panicked at
🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../🔗️causal/🦀️.rs:856:9:
assertion `left == right` failed
  left:  ... "payloadSchema": String("🛂️schema.json") ...
  right: ... "payloadSchema": String("../🛂️schema/🔣️.json") ...

test result: FAILED. 225 passed; 1 failed; 0 ignored; 0 measured; 158 filtered out; finished in 46.84s
```

The failure is a taxonomy-fixture path string mismatch, unrelated to serialization — it was
already like that before this wave's edits reached that test (a concurrent agent's in-flight
taxonomy change), confirmed by the fact that neither `MutationDiff`/`Mutation`/`ToValue`/
`FromValue` appear anywhere in that assertion's diff.

`cargo check -p semio-framework-value-derive --message-format=short`: clean build, 2 warnings
(`unnecessary qualification`, cosmetic, not fixed — low priority polish for whoever touches that
file next).

`cargo check -p semio-s-plugin-playbook`: **did not complete this session** — see "Pilot status"
above.

## URGENT ADDENDUM — the Diff-struct gap that grew the os-kernel error count

Found mid-session while repairing `semio-framework-os-kernel` after the trait-bound landed: my
first conversion pass (the 116-file bulk script) only touched files whose `#[derive(...)]` line
literally contains `dsl::Mutations` or `dsl::MutationLeaf` — i.e. the `Mutation` dispatch enums and
their leaf payload structs. It **missed every `Diff` type** — the type named in
`#[mutations(snapshot = S, diff = D, schema = "...")]`'s `diff = D` — because those structs are
defined in a SEPARATE file/location and derive `ArtifactSchema` (or nothing dsl-flavored at all),
never `dsl::Mutations`/`dsl::MutationLeaf` themselves. `D: MutationDiff<S>` needs `ToValue +
FromValue` exactly as much as the Mutation enum does — this is why the error count grew instead of
shrinking as more of the crate got type-checked (`Mutation`-derived code compiles, then chokes on
`D::default()`/`.apply()` calls needing `D: ToValue + FromValue`).

**Fixed this session** (added `ToValue, FromValue` to the derive, `#[value(...)]` mirrors, added the
import — same recipe as everywhere else in this doc): `CounterDiff`, `DagDiff`, `DemoDiff`,
`LossyDiff`, `SpaceHistoryDiff`, `DependencyTestDiff`, `FlowDiff`, `WireTestDiff`, `DummyDiff`,
`MiniDiff`, `PublicationPresenceDiff`, `PublicationTransientDiff`, `TxnDiff`, `JobTestDiff`,
`TestDiff`, `SurfaceDiff`, `Identity` + `IdentitySetting` (the latter hand-written — tuple struct,
`#[serde(transparent)]`, not a derive-supported shape), `MergePolicySetting`,
`OpeningPreferences`.

**NOT found / NOT yet converted** — `grep -rl "struct RunDiff\b\|struct WorkflowDiff\b\|struct
Std1AnyDiff\b\|struct Std1StrictDiff\b\|struct Std2AnyDiff\b\|struct TestConfigDiff\b"
🧰️framework` returned nothing; these are referenced by name in `#[mutations(diff = ...)]` but their
struct definitions weren't found by a plain grep in this session — check for a macro-generated
definition (e.g. `#[derive(DslDiff)]` synthesizing the struct, or a `type WorkflowDiff = ...`
alias) before assuming they don't exist.

**For whoever fans this out**: the reliable way to find every type that needs `ToValue`/
`FromValue` is NOT "grep for `dsl::Mutations`/`dsl::MutationLeaf`" (what I did first, incomplete).
It's:

```bash
grep -rhoE '#\[mutations\([^)]*\)\]' --include='*.rs' 🧰️framework ✏️s | grep -oE '(snapshot|diff)\s*=\s*[A-Za-z0-9_:]+'
```

— collect every `snapshot`/`diff` type name, find each struct's definition
(`grep -rn "struct <Name>\b"`), and convert it. The snapshot type (`P` in `MutationDiff<P>`) does
**NOT** need `ToValue`/`FromValue` — only `diff`'s type does (see the trait signature). Also walk
each Diff/Mutation struct's own field types transitively — a field whose type is itself a plain
struct/enum (not a primitive/`Option`/`Vec`/`Box`/`BTreeMap`) needs its own `ToValue`/`FromValue`
too, and THAT search has no shortcut except reading the struct definition.

For a giant shared file (`🏪️store/🦀️component.rs` is 26511 lines, `♾️infinite/…/🕸️dag/🦀️component.rs`
is 9766 lines) — do NOT do a whole-file mechanical pass. Find the exact `struct <Name>` line,
walk backward to its own `#[derive(...)]` line only, and mirror only that struct's own
`#[serde(...)]` attributes (container + its own field span, i.e. up to the struct's matching
closing brace). A file-wide pass on a shared file this size WILL touch unrelated types.

## CONFIRMED GREEN — `semio-framework-os-kernel`

`cargo check -p semio-framework-os-kernel --message-format=short` completed with **0 errors, 33
warnings**, `Finished dev profile [unoptimized] target(s) in 41m 20s`. This is the crate hosting
`🏪️store`, `♾️infinite/…/🕸️dag`, `🔁️workflow`... — wait, see the correction below, `🔁️workflow` is
NOT part of this crate. Everything ELSE in the `#[mutations(diff = ...)]` os-kernel-scoped list
(the authoritative enumeration below) that lives inside `semio-framework-os-kernel` itself is
confirmed compiling.

## The `🔁️workflow` module lives in a DIFFERENT crate than you'd assume

`🔁️workflow/🦀️component.rs`'s own header comment: *"`🔁️workflow/🦀️component.rs` is NOT mounted
here [in `semio-framework-os-kernel`] (tried, reverted — pending dep-DAG cleanup)... It is mounted
in `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` (the `semio-framework` crate) instead."* So
`RunDiff`/`RunMutation`/`WorkflowDiff`/`WorkflowMutation` and everything under `🔁️workflow/**`
compile as part of the **`semio-framework`** crate (the top facade, `🧰️framework/📦️packages/🦀️rust`),
not `semio-framework-os-kernel`. A green `os-kernel` check tells you nothing about whether
`🔁️workflow` compiles — check `semio-framework` (or anything that depends on it, like any plugin)
separately.

`RunDiff`/`RunMutation`/`WorkflowDiff`/`WorkflowMutation` already carry `ToValue, FromValue` in
their derives (someone converted them before this addendum was written) and
`🧰️framework/📦️packages/🦀️rust/Cargo.toml` already lists `semio-framework-value-derive` as a
dependency. A `cargo check -p semio-s-plugin-playbook` run that started around 1:18pm this session
(before that Cargo.toml edit landed) showed 247 "cannot find attribute `value` in this scope"
errors, all in `🔁️workflow/**` — that check had been queued behind lock contention for ~2 hours and
its output reflects the crate-metadata snapshot from when it STARTED, not when it finished; Cargo
does not re-read a manifest mid-build. **Treat any long-queued check's result as dated to its start
time, not its completion time, when judging whether an intervening fix landed in time to matter.**
Re-run fresh (don't trust a check that predates your fix) before concluding something is still
broken.

## Authoritative `#[mutations(diff = ...)]` name list

From `grep -rn '#\[mutations(' --include=*.rs 🧰️framework ✏️s | grep -o 'diff *= *[A-Za-z0-9_:]*' |
sort -u` (180 matches total; framework-only subset shown, ✏️s has ~100 more — one per stdio media
format / norm standard / semio-kit artifact, out of scope for os-kernel's own green-ness but every
one of them needs the identical treatment eventually):

Framework-scoped names and status as of this session's end:; **converted**: `CounterDiff`,
`DagDiff`, `DemoDiff`, `LossyDiff`, `SpaceHistoryDiff`, `DependencyTestDiff`, `FlowDiff`,
`WireTestDiff`, `DummyDiff`, `MiniDiff`, `PublicationPresenceDiff`, `PublicationTransientDiff`,
`TxnDiff`, `JobTestDiff`, `TestDiff`, `SurfaceDiff`, `Identity`+`IdentitySetting` (hand-written,
tuple struct), `MergePolicySetting`, `OpeningPreferences`, `RunDiff`/`RunMutation` (enum, found in
`🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/🧬️mutations/🦀️.rs` + `🔁️workflow/🦀️component.rs:2063` —
NOT a struct, an enum, hence the earlier `grep "struct RunDiff"` false negative),
`WorkflowDiff`/`WorkflowMutation` (same false-negative cause, `🔁️workflow/🦀️component.rs:1333`),
`Std1AnyDiff`/`Std1StrictDiff`/`Std2AnyDiff` (generated by ONE `macro_rules! fixture_channel!` in
`🔌️plugin/🦀️component.rs:27319` — fixed the macro body once, all 3 invocations inherit it),
`TestConfigDiff` (adjacently-tagged enum in
`🔌️plugin/🧪️tests/🧬️test-app-mutations/🎚️config/🦀️.rs`), `Value` (tuple struct in
`🏪️store/👥️presence/♻️retirement/🦀️component.rs:244`, hand-written).

**Root cause of the `struct X` grep false negatives**: several of the "missing" names were never
struct definitions — `RunDiff`/`WorkflowDiff` are enums, `Std1AnyDiff`/`Std1StrictDiff`/
`Std2AnyDiff` are generated by a `macro_rules!` (no literal `struct Std1AnyDiff` text exists
anywhere — it only exists post-expansion). When a name search comes up empty, check for (a) `enum`
not `struct`, (b) a `macro_rules!` generating it, (c) a `type X = Y` alias, before concluding the
type doesn't exist.
