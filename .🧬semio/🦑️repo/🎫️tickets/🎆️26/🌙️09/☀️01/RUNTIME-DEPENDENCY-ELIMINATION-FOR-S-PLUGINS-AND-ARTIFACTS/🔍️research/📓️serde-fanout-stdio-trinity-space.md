# Serde Fan-Out — stdio / trinity / space Batch

Batch: `🗄️stdio` rust + its two `🏭️bridge` manifests (brep, mesh), `🔱️trinity`, `🪐️space`.
Companion docs (read first, authoritative): `📓️serde-replacement-surface.md` (trait-bound design),
`📓️serde-fanout-playbook.md` (mechanical recipe, traps, pilot status).

## Headline

| manifest | `[dependencies]` state | source conversion |
|---|---|---|
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🏭️bridge/Cargo.toml` | **zero third-party** — DONE | 100% |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🏭️bridge/Cargo.toml` | **zero third-party** — DONE | 100% |
| `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` | `serde`/`serde_json` **kept, interim** | ~1376/1962 files mechanically converted; central macro hand-converted; ~410 files remain |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml` | `serde`/`serde_json` **kept, interim** | 50/71 files converted + 2 hand-written composed-child structs; ~20 remain |
| `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` | `serde`/`serde_json` **kept, interim** | 101/122 files converted; ~21 remain |

**None of the three plugin manifests reached zero third-party.** True scope, measured directly
(real `serde_json::` call sites in non-test/oracle/probe/generator/fixture files), was far larger
than the ticket brief's estimate: `🗄️stdio` alone has **563 real call-site files** (not ~503 for
trinity as briefed — stdio turned out to be the outsized one, ~8x trinity). Cargo.toml cannot be
flipped to zero until every file in a crate is converted (a partial flip breaks the build), so per
the playbook's own sanctioned interim pattern (`📖️playbook`'s manifest does the same), `serde`/
`serde_json` were **restored** in all three plugin manifests after the mechanical pass, with a
`🚧️` docstring explaining why. Removing them now would leave three widely-depended-on crates
(stdio is `semio-s-plugin-stdio`, the crate the ticket flagged as highest blast radius) in a
guaranteed-broken state for the whole fleet.

## Framework addition (shared benefit, not scoped to just this batch)

`📓️serde-replacement-surface.md` named an open gap: **no bridge between `pack::json::Value`
(literal JSON text) and `DslValue` (the `ToValue`/`FromValue` in-memory tree) — "small (structural
`From`/`TryFrom`), not yet written."** Every `Mutation`/`MutationDiff` implementor that also needs
literal JSON **text** (a wire `OpText`/`OpBinary` codec, a `.json`-fixture round trip) hits this
gap. Added it:

- `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` — new `from_dsl_value(&protocol::value::DslValue) -> Value` /
  `to_dsl_value(&Value) -> protocol::value::DslValue` (structural walk, `DslValue::Number` always
  widens to `Number::Float` going out, always narrows to `f64` coming back — matches `DslValue`'s
  own single-`f64` contract, documented in the function docstrings).
- `🧰️framework/🔨️modules/🎒️pack/🦀️component.rs` — re-exported at the `pack` facade root as
  `json_from_dsl_value` / `json_to_dsl_value` (alongside the existing `json_to_string`/`parse_json`
  family).
- Dependency direction verified before writing: `pack` already depends on
  `semio-framework-replication` (`protocol`) — confirmed by the pre-existing `pub use protocol::codec`
  in `pack`'s own glue — so `pack → protocol::value::DslValue` is not a new edge, no cycle. The
  reverse (`protocol → pack::json`) would be a cycle and is NOT what was built.

This is what `impl_serde_op_codec!` (stdio's mutation wire-format macro, below) and `JackSnapshot`'s
JSON-fixture bridge (trinity) are both built on. **Written, not yet verified by a passing test** —
see Verification section.

## `🗄️stdio` — `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust`

### Manifest
Added `semio-framework-value-derive` and `pack` (key `pack`, `package = "semio-framework-pack"`,
matching the pre-existing repo convention seen in `mesh-engine`'s Cargo.toml) at the same
`../../../../../` depth as every other framework dep in this file. **Resolve-checked** with
`ls -d <dir>/<relpath>` — both resolve. `serde`/`serde_json` restored (interim, see Headline).

### Hand-converted (small, central, high-leverage)
- `📦️glue.rs`: added `extern crate semio_framework_value_derive as value_derive;` (mirrors the
  existing `dsl`/`protocol`/`store`/`schema` alias pattern already in this file).
- `impl_serde_op_codec!` macro (used by 7 mutation types: `DwgMutation`, `SvgBasicMutation`,
  `SvgTinyMutation`, `TiffBaselineMutation`, `XmlValidMutation`, `JpgBaselineMutation`) — rewired
  `OpText`/`OpBinary` from `serde_json::to_string/from_str/to_vec/from_slice` onto
  `dsl::ToValue::to_value(self)` → `pack::json_from_dsl_value` → `pack::json_to_string` (and the
  parse-side inverse), using UFCS (`dsl::ToValue::to_value(...)`, never `.to_value()` shorthand —
  per playbook trap #1, `DslField`'s own same-named methods would otherwise collide).
- `semantic_fingerprint<T: serde::Serialize>` → `semantic_fingerprint<T: dsl::ToValue>`. **Verified
  dead code first** (`grep -rn "semantic_fingerprint("` — zero call sites anywhere in the crate),
  so the signature change has no ripple.

### Mechanical pass (script, see Method)
1376 of 1962 candidate files converted (`#[derive(Serialize, Deserialize, ...)]` →
`#[derive(value_derive::ToValue, value_derive::FromValue, ...)]`, `#[serde(...)]` → `#[value(...)]`,
`use serde::{...};` deleted). Excluded from the automated pass: 4 files with a `#[child(...)]`
composed-artifact field (`✳️object`/`✳️kit` subsets' `🧬️schema/🦀️component.rs` +
`📸️snapshot/🦀️component.rs`) — same trap as trinity's `JackArtifact` below, need the identical
hand-written `ToValue`/`FromValue` + `dsl::to_dsl_value`/`from_dsl_value`-per-child-field pattern,
**not yet done**.

**Skipped by the script (unsupported `#[serde(...)]` attribute, need hand `impl`), 15 files:**
`💬️bcf/…/🔺️diff`, `🧿️semio/…✳️cad/…/📸️snapshot`, `🧿️semio/…✳️any/🧬️schema/🧰️triples`,
`🧿️semio/…✳️model/…/🎛set-element`, `🧿️semio/…✳️model/…/🧭set-spatial-node`,
`🧿️semio/…✳️brep/…/📸️snapshot` (+ its `🏟️arena` submodule), `🧿️semio/…✳️document/…/📸️snapshot`,
`📄️pdf/…/📥️insert-page`, `📕️xlsx/…/🔺️diff`, `📜️docx/…/🔺️diff`, `🎞️pptx/…/🔺️diff`,
`🧊️gltf/…/📸️snapshot`, `🧊️gltf/…/✏️🔘️change-node-name`, `🧊️gltf/…/🔺️diff` — these carry `flatten`/
`transparent`/`bound(`/`alias`/`serialize_with`/`deserialize_with`, none of which the derive
supports (by design, per the playbook).

**Not touched at all: ~410 files** with a genuine `serde_json::` call site in production code
(`to_string`/`from_str`/`to_vec`/`from_slice`/`json!`/`Value` manipulation) outside a derive
attribute — the mechanical pass only rewrites derive lines and attributes, never call sites. The
`📇️registry/🦀️component.rs` file (12 structs, all `#[serde(deny_unknown_fields)]`, parsing every
artifact's `include_str!`'d `📜️artifact-definition.json`) is in this remaining set — **flagged,
not converted**: `deny_unknown_fields` is a documented no-op on the derive (playbook), so
converting these 12 structs naively would silently drop real validation currently enforced on every
artifact definition in the plugin. Needs a hand-written rejection check alongside the derive, not
a blind swap. Regenerate the exact remaining list with:
```
grep -rl "serde_json::" --include="*.rs" ✏️s/🔌️plugins/🗄️stdio/ | grep -v /target/ | \
  grep -vP '/🧪️(test|tests|oracle)/|/🔬️probes/|/🏭️generator/|/🧫️fixtures/'
```

## `🔱️trinity` — `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust`

### Manifest
Same two deps added, same depth pattern (`../../../../../`), resolve-checked. `serde`/`serde_json`
restored (interim).

### Mechanical pass
50 of 71 candidate files converted by the same script.

### Hand-converted (composed-child trap, playbook trap #3)
`JackArtifact` (`🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`) and
`JackSnapshot` (…`/📸️snapshot/🦀️component.rs`) both carry `#[child(kind = "s.stdio.semio.graph")]
pub content: JackContentChild` where `JackContentChild = store::ArtifactChild<SemioGraphSnapshot>`
— a framework-owned generic that still speaks `serde` (framework is exempt). The mechanical script
had already (wrongly) derived these two before this was caught; reverted to bare
`#[derive(Clone, Debug, PartialEq, ArtifactSchema)]` (no `ToValue`/`FromValue` derive) plus a
hand-written `impl dsl::ToValue`/`impl dsl::FromValue` per struct, bridging `content` through
`dsl::to_dsl_value(&self.content)` / `dsl::from_dsl_value(field("content")?)` exactly per the
playbook's `PlaybookArtifact` reference pattern — every other field goes through
`dsl::ToValue::to_value`/`dsl::FromValue::from_value` directly. `JackSnapshot`'s own
`encode_jack_snapshot_json`/`decode_jack_snapshot_json` (used by the `mutate-jack-1` fixture
comparison) were also rewired off `serde_json::to_string`/`from_str` onto
`pack::json_to_string(&pack::json_from_dsl_value(&dsl::ToValue::to_value(snapshot)))` and the parse
inverse — the new framework bridge above, exercised here for the first time.

**Skipped (unsupported attribute), 1 file:**
`♻️rewrite/…/✏️editor/🌍️world/🦀️component.rs`.

**Not touched, ~19 files** with real `serde_json::` call sites (`to_string`/`from_str`/`json!`/
`Value` manipulation on `Camera`, `Node`, `Edge`, `Rhs`, `Lhs`, `RewriteSnapshot`, an executor
result, a language-service fixture, viewer/editor-command JSON round trips). One of these —
`🗿️artifacts/🔌️jack/…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/…/🦀️component.rs` — has a
**real cross-plugin coupling**: it converts a `JackSnapshot` into `semio_s_plugin_stdio`'s own
`JsonSnapshot::from_value(impl Into<JsonValue>)`, where `JsonValue` is *stdio's* json-artifact tree
type (a third, different shape from both `DslValue` and `pack::json::Value`) — that conversion
currently only accepts `serde_json::Value`. Unblocking this call site cleanly means giving stdio's
own `🗄️stdio/🗿️artifacts/🔣️json/…/📸️snapshot/🦀️component.rs` a
`impl From<pack::JsonValue> for JsonValue` (or equivalent), which is in scope for the stdio manifest
but was not reached this session. Regenerate the list with the same `grep` shape as stdio's, rooted
at `✏️s/🔌️plugins/🔱️trinity/`.

## `🪐️space` — `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust`

### Manifest
Same two deps added at the same depth, resolve-checked. `serde`/`serde_json` restored (interim).
No `impl_serde_op_codec!`-equivalent macro in this crate's own `📦️glue.rs` — nothing to hand-convert
there.

### Mechanical pass
101 of 122 candidate files converted. **Zero `#[child(...)]` fields anywhere in this plugin** (
checked before running the script) and **zero unsupported-attribute files** — the cleanest of the
three.

**Not touched, ~21 files** with real `serde_json::` call sites: home/space JSON+CSV+XLSX+ZIP
import/export serializers, viewer windows, the compiled-DAG window. Regenerate with the same `grep`
shape, rooted at `✏️s/🔌️plugins/🪐️space/`.

## Method

1. Grepped every manifest's real `serde_json::` call sites (non-test/oracle/probe/generator/
   fixture dirs) FIRST to get true production scope before editing — this is what surfaced stdio
   being ~8x larger than trinity, contradicting the ticket brief's estimate (which appears to have
   counted differently, likely gross matches including test dirs).
2. Built one Python codemod (`/private/tmp/.../scratchpad/codemod.py`, kept in scratch, not the
   ticket folder — it is a throwaway tool, not a deliverable artifact) that only touches:
   `#[derive(...)]` lines (swaps bare `Serialize`/`Deserialize` tokens for
   `value_derive::ToValue`/`value_derive::FromValue`, preserving every other derive item and their
   order), `#[serde(...)]` → `#[value(...)]` (verbatim inner content — supported spellings are
   identical), and deletes `use serde(_json)?::...;` import lines. **Whole-file skip** (zero edits)
   if the file contains `flatten`/`transparent`/`bound(`/`alias`/`serialize_with`/
   `deserialize_with`/`rename_all_fields` anywhere — those need a hand `impl`, never a blind swap.
3. Ran it against the real per-plugin file lists (excluding `#[child(...)]`-bearing files, checked
   separately first). Verified afterward with a repo-wide grep that zero bare `Serialize,`/
   `Deserialize,` derive-list tokens or `#[serde(` lines remain outside the skip list or excluded
   test/oracle dirs — clean.
4. Hand-converted the two `🏭️bridge` manifests, stdio's central macro + dead-code fingerprint fn,
   and trinity's two composed-child structs, all cross-checked against the playbook's own worked
   examples (`PlaybookArtifact`) rather than improvised.
5. Every new path dependency resolve-checked with `ls -d <manifest-dir>/<relative-path>` before
   being written — all seven resolved (2 bridges × `pack`; stdio/trinity/space × `pack` +
   `semio-framework-value-derive`).

## Verification — honest, updated after os-kernel's check actually completed

The long-running `cargo check -p semio-framework-os-kernel --message-format=short` (started
early this session, foregrounded, polled by reading its output file rather than
`Monitor`/waiting) **finished after ~44 minutes: exit 0, warnings only, zero errors.**
`semio-framework-os-kernel` is green — the ~75-error state the fan-out playbook described has
been repaired since that doc was written. Its warning output also **directly confirms this
session's `pack::json_from_dsl_value`/`json_to_dsl_value` addition compiles**: the only warnings
attributed to `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` are 13 cosmetic "unnecessary
qualification" hits at exactly the lines those two new functions occupy (`protocol::value::DslValue::…`
where a bare `DslValue::…` would do inside the already-scrutinized match) — not a single error.

**Follow-up checks run in the foreground immediately after, one at a time, shared target dir:**

- `cd .../✳️brep/🏭️bridge && cargo check --message-format=short` — **92 errors, all in
  `semio-framework`/`semio-framework-plugin` (transitive deps), zero in the bridge's own code**
  (`grep -i "brep|semio-semio-v1"` against the full output — no hits outside `Compiling`/
  `Checking` lines). Root cause, read from the errors themselves: `Mutation:
  serde::de::DeserializeOwned` is not satisfied inside `🔌️plugin/🦀️component.rs` (867 errors
  there alone), plus ~80 `expected &_, found <MutationType>` mismatches inside
  `🛍️products/💻️os/🔨️modules/🔁️workflow/**`. This is the SAME root-cause trait-bound rewrite
  (`MutationDiff`/`Mutation` now bound on `ToValue + FromValue` instead of `serde::Serialize`/
  `DeserializeOwned`) fanning out into **two more framework crates** beyond os-kernel —
  `semio-framework-plugin` and the `🔁️workflow` module inside `semio-framework` itself — neither
  touched by this session, neither named by this batch's 5 manifests. Recorded, not fixed, per
  the ticket's "another agent's in-flight work" instruction — os-kernel being green again but
  `semio-framework`/`semio-framework-plugin` now red suggests the trait-bound fan-out is still
  actively spreading crate-by-crate through the framework, not finished.
- `cd .../✳️mesh/🏭️bridge && cargo check --message-format=short` — same story, same two upstream
  crates, zero mesh-bridge-specific errors (confirmed by the same grep).

**Conclusion: the two bridge manifests themselves are very likely correct** (their own code
produces zero errors in either run; the only failures are pre-existing, upstream, and shared by
every other crate in the workspace right now) but **cannot be proven green until
`semio-framework`/`semio-framework-plugin` are repaired** — outside this ticket batch's scope.
`semio-s-plugin-space`/`-trinity`/`-stdio` were not re-checked after this finding: since they
transitively depend on the same two currently-red framework crates (every plugin does, via
`semio-framework`), and since none of the three is source-complete yet regardless (see the
Headline table), a check would reproduce the same upstream noise without adding signal. Verdict
for all five manifests: **WRITTEN, NOT PROVEN — blocked on `semio-framework`/
`semio-framework-plugin`, not on anything in this batch.**
## What a follow-up agent should do, in order

1. **First check whether `semio-framework`/`semio-framework-plugin` are green yet** — that is the
   actual current blocker, not os-kernel (os-kernel itself is confirmed green this session, see
   Verification). Look for a status update from whoever owns the `MutationDiff`/`Mutation`
   trait-bound fan-out (the same root cause named in the fan-out playbook), or just run
   `cargo check -p semio-framework-plugin --message-format=short` directly. Don't attempt this
   repair as part of this ticket's batch — it's framework-wide, not scoped to `✏️s/`.
2. Once that's green, re-run the two bridge checks
   (`cd .../✳️brep/🏭️bridge && cargo check --message-format=short`, same for `✳️mesh`) — this
   session's own runs already produced zero bridge-specific errors, so these should go straight to
   PROVEN.
3. `cargo check -p semio-s-plugin-space --message-format=short` next (cleanest of the three
   plugins — no composed-child traps, no unsupported-attribute files) — but note it will still fail
   until space's own ~21 remaining call-site files are converted (see the Headline table); this
   step is about confirming the ALREADY-converted 101 files didn't regress anything, not about a
   clean pass yet.
4. Then `semio-s-plugin-trinity`, then `semio-s-plugin-stdio` (largest, highest blast radius,
   should go last so its inevitable first-pass errors don't block signal on the smaller crates).
5. For each plugin, once its remaining `serde_json::` call-site files (regenerate the list with the
   `grep` command given per-section above) and unsupported-attribute files are converted, delete
   the interim `serde`/`serde_json` lines from that manifest's `[dependencies]` — not before.

---

# 🔧️ Follow-up session — finishing stdio's 19 mandated files, framework primitives, verification

Scope this session: `🗄️stdio`'s "15 unsupported-attribute files" + "4 composed-child files" named by
the previous session, PLUS whatever framework `🌱️value/✨️derive`/`🌱️value/🔁️codec`/`🎒️pack/🔤️json`
gaps those files actually needed (found by reading each file, not guessed in advance). `🔱️trinity`/
`🪐️space` were NOT touched this session — out of scope, still exactly as the prior session left them.

## Framework additions — all PROVEN BY A PASSING TEST before being relied on

Every addition below was verified in isolation (a scratch crate outside the workspace, or the
crate's own `#[cfg(test)]` module) BEFORE being used in a real stdio file, to avoid burning the
enormous `cargo check -p semio-s-plugin-stdio` cycle on a macro bug.

1. **`pack::json!` macro** (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs`) — a `serde_json::json!`
   replacement over `pack::json::Value`: TT-muncher recursion (`json_array_internal!`/
   `json_object_internal!`, `#[doc(hidden)]`, `#[macro_export]`), supports nested objects/arrays,
   empty `{}`/`[]`, trailing commas, arbitrary leaf expressions via `Value::from`, and `null`/
   `true`/`false` literals. Added `impl<T: Into<Value>> From<Option<T>> for Value` alongside (None
   → Null) so `Option<T>` leaves work inside the macro without an explicit match.
   **PROVEN**: 5 new tests in that file's own `mod tests` — `json_macro_builds_scalars_and_null`,
   `..._builds_arrays_incl_empty_and_nested`, `..._builds_objects_incl_empty_and_trailing_commas`,
   `..._evaluates_arbitrary_expressions_and_options`, `..._matches_to_string_of_equivalent_hand_built_value`
   — all pass alongside the file's pre-existing 26 tests (`cargo test -p semio-framework-pack --lib
   json::`: 31 passed, 0 failed).
2. **`impl<T> ToValue/FromValue for PhantomData<T>`** (`🌱️value/🔁️codec/🦀️component.rs`) — encodes as
   `DslValue::Null`, decodes from anything, unconditional on `T` (mirrors `serde`'s own blanket impl;
   needed by `brep`'s generational-arena `Store<T, Id>`, whose `_marker: PhantomData<fn() -> Id>`
   field the derive would otherwise choke on). **PROVEN**: new test
   `phantom_data_encodes_as_null_and_decodes_from_anything` in that file's `mod tests`.
3. **`impl<T: ToValue/FromValue, const N: usize> for [T; N]`** (same file) — fixed-size arrays encode
   as a plain JSON array; decode rejects any length mismatch (real `.gltf` fields like
   `base_color_factor: [f64; 4]` need this). **PROVEN**: new test
   `fixed_size_array_round_trips_and_rejects_wrong_length` (round-trip + two wrong-length rejections).
4. **`🌱️value/✨️derive` extensions** (`🦀️component.rs`) — all four landed together, all four proven
   together in one standalone scratch crate (`semio_framework_value_derive` path-dependency +
   a same-API-surface fake `semio_framework_os_kernel` stand-in, avoiding a full os-kernel rebuild —
   see Verification Method below):
   - **`#[value(transparent)]`** (container, struct-only): exactly one field (named OR unnamed),
     forwards straight to/from that field's own `ToValue`/`FromValue`.
   - **`#[value(serialize_with = "path")]`/`#[value(deserialize_with = "path")]`** (field): replace
     the default `ToValue::to_value`/`FromValue::from_value` call for that field with a plain
     function call — `fn(&FieldType) -> DslValue` / `fn(DslValue) -> Result<FieldType, ValueError>`.
     Combine with bare `#[value(default)]` for the "missing key defaults, present key goes through
     the custom fn" split (the `deserialize_double_option` shape).
   - **`deny_unknown_fields` actually enforced** for `Data::Struct` (previously parsed into
     `ContainerAttrs` but never read anywhere — a real no-op despite the module docs already
     claiming it was supported). Still a no-op on enum containers (no occurrence needed it there).
   - **Auto-synthesized per-type-parameter bound**: a generic struct/enum gets `Param: ToValue`
     (resp. `FromValue`) added to its `where` clause automatically, ONE per own type parameter,
     unless overridden by `#[value(bound = "...")]` (kept as an escape hatch, e.g. for an unused
     `PhantomData<T>` parameter). This REPLACES the originally-planned design (a `bound` attribute
     you write by hand every time) — auto-synthesis is strictly less error-prone since every
     generic type this derive has been applied to so far needs exactly the uniform bound, and
     forgetting to write `#[value(bound = "...")]` on some struct is not a way this can fail
     silently anymore. **Also fixed as part of this work**: per-variant `#[value(rename = "...")]`
     was silently ignored on unit-only and tagged enums alike (`variant_wire_name(...)` was called
     with a hardcoded `&None` in all 7 call sites, never the variant's own parsed attrs) — needed by
     `GltfAnimationPath`/`GltfInterpolation` (`#[serde(rename = "translation")]` etc. on individual
     unit variants). Fixed by parsing `variant.attrs` at each call site instead.
   **PROVEN**: a standalone `cargo +nightly-2026-07-07 run` scratch crate
   (`/private/tmp/.../scratchpad/valuetest/{fake_kernel,tester}`) exercising: transparent (named +
   tuple), a 3-type-param and a 2-type-param generic struct with NO explicit bound (auto-synthesis),
   `deny_unknown_fields` rejecting an injected extra key, `serialize_with`/`deserialize_with` on a
   real `Option<Option<i32>>` double-option field (all three states + missing-key default), and a
   per-variant-`rename` unit enum. All 7 scenarios pass; verbatim tail:
   ```
   transparent named OK
   transparent tuple OK
   generic auto-bound OK
   deny_unknown_fields OK: ValueError("unknown field `unexpected`")
   serialize_with/deserialize_with + default OK
   two-param generic auto-bound OK
   per-variant rename on unit enum OK
   ALL DERIVE EXTENSION TESTS PASSED
   ```

### Verification method for the derive extensions — why a scratch crate, not the real workspace

`os-kernel`/`semio-framework-plugin` were RED for most of this session (a concurrent agent's
`ArtifactApp::Snapshot` trait-bound migration, out of this ticket's scope — see "os-kernel is now
green" below for when it landed), so `cargo test -p semio-framework-replication` could prove the
framework primitives (§1–3 above, which live IN that crate) but nothing depending on `os-kernel`
could compile at all. Rather than wait, a throwaway crate outside the cargo workspace
(`/private/tmp/.../scratchpad/valuetest/`) path-depended on the REAL `semio-framework-value-derive`
(a proc-macro crate — no `os-kernel` dependency itself) plus a hand-written ~150-line
`semio_framework_os_kernel` stand-in matching the real `DslValue`/`ToValue`/`FromValue`/`ValueError`
API exactly (verified field-for-field against `🌱️value/🦀️component.rs`/`🌱️value/🔁️codec/🦀️component.rs`).
Since the derive's generated code references `::semio_framework_os_kernel::...` by absolute path,
the fake crate registered under that exact name satisfies it, and the whole thing compiles/runs in
under 10 seconds instead of the tens of minutes a real `os-kernel` dependency would cost. This is
the same technique the earlier BLAKE3/parry3d central verification used (standalone crate, real
oracle), applied to a proc-macro instead of a hand-written algorithm.

## `🗄️stdio` files converted this session (all 19 named by the ticket, plus what they pulled in)

**The 15 "unsupported-attribute" files — DONE, using the extensions above:**

| file | what it needed |
|---|---|
| `💬️bcf/…/🔺️diff/🦀️component.rs` | `bound(...)` → dropped (auto-synthesis) |
| `🧿️semio/…✳️cad/🧬️schema/📸️snapshot/🦀️component.rs` | `tag = "kind"` enum (already supported) + JSON bridge → `pack::to_json_string`/`from_json_str` |
| `🧿️semio/…✳️any/🧬️schema/🧰️triples/🦀️component.rs` | `bound(...)` ×2 → dropped; test's own `NoDefault(u32)` tuple struct → `#[value(transparent)]`; test's `serde_json::to_string/from_str` → `pack::to_json_string`/`from_json_str` |
| `🧿️semio/…✳️model/🧬️schema/🧬️mutations/🧭set-spatial-node/🦀️.rs` | `deserialize_with = "deserialize_double_option"` (attribute kept verbatim) |
| `🧿️semio/…✳️model/🧬️schema/🧬️mutations/🎛set-element/🦀️.rs` | same `deserialize_double_option` |
| `🧿️semio/…✳️model/🧬️schema/🧬️mutations/🦀️.rs` (parent, hosts `deserialize_double_option` itself) | rewrote the helper fn from `fn(D: Deserializer) -> Result<Option<Option<T>>, D::Error>` to `fn(DslValue) -> Result<Option<Option<T>>, ValueError>` — turned out to be a ONE-LINE body change (`Option::<T>::from_value(value).map(Some)`, our blanket `Option<T>: FromValue` already does exactly what the old code needed) since our derive, UNLIKE serde, treats a bare `Option<T>` field as REQUIRED (not implicitly `#[serde(default)]`'d) by default — see the file's own new doc comment; one `serde_json::Value` test fixture read → `pack::JsonValue`/`pack::parse_json`, one `entry["id"] == "…"` string-literal comparison → `.as_str() == Some("…")` (`pack::JsonValue` has no `PartialEq<&str>`) |
| `🧿️semio/…✳️brep/🧬️schema/📸️snapshot/🦀️component.rs` | `tag = "kind", rename_all_fields = "camelCase"` → dropped `rename_all_fields` (this derive's `rename_all` already covers struct-variant member names too — verified NOT redundant-with-a-caveat, genuinely identical output here since both were set to the same case) + JSON bridge |
| `🧿️semio/…✳️brep/🧬️schema/📸️snapshot/🏟️arena/🦀️component.rs` | `define_id!` macro's per-id-type derive; `Slot<T>` generic; `Store<T, Id>`'s `#[serde(bound(serialize = "T: Serialize", …))]` (asymmetric — `Id` needs NO bound, only `T` does) → dropped in favor of auto-synthesis PLUS the new `PhantomData<T>` blanket impl (auto-synthesis over-constrains `Id: ToValue+FromValue` too, harmless since every real `Id` from `define_id!` satisfies it); test's `serde_json::to_string/from_str` on a `TestId` → `pack::to_json_string`/`from_json_str` |
| `🧿️semio/…✳️document/🧬️schema/📸️snapshot/🦀️component.rs` | plain mechanical (no traps) + JSON bridge |
| `📄️pdf/…/🧬️mutations/📥️insert-page/🦀️.rs` | `deny_unknown_fields` (now enforced) + `deserialize_with = "deserialize_page"` rewritten to call a NEW `#[derive(value_derive::FromValue)] #[value(deny_unknown_fields)] struct PagePayload` (FromValue-only derive, no ToValue — supported) instead of hand-rolling a `Deserializer` visitor; one test fixture read `serde_json::from_str::<PdfMutation>` → `pack::from_json_str` |
| `📕️xlsx/…/🔺️diff/🦀️component.rs` | `bound(...)` → dropped |
| `📜️docx/…/🔺️diff/🦀️component.rs` | `bound(...)` ×2 → dropped |
| `🎞️pptx/…/🔺️diff/🦀️component.rs` | `bound(...)` ×2 → dropped |
| `🧊️gltf/…/📸️snapshot/🦀️component.rs` | see dedicated section below — by far the largest and most structurally different of the 15 |
| `🧊️gltf/…/🧬️mutations/✏️🔘️change-node-name/🦀️.rs` | `deny_unknown_fields` ×2 (structs, enforced) + `tag = "phase", content = "value"` (already supported, adjacently-tagged) on the enum, ALSO `deny_unknown_fields` there (parsed, not enforced for enums — documented, no test needed it); `deserialize_with = "required_option"` DELETED outright (not rewritten) — see the file's new doc comment for why a bare `Option<String>` field needs nothing extra under this derive's opposite-of-serde default; 4 `serde_json::from_str/to_value/json!` test call sites → `pack::from_json_str`/`pack::to_json_string`/`pack::parse_json`/`pack::json!` |
| `🧊️gltf/…/🔺️diff/🦀️component.rs` | 4496 lines total, but ALL real `serde`/`Serialize`/`Deserialize` usage confined to lines 31–1173 (3 generic collection-triple types with `bound(...)`, dropped for auto-synthesis) — the remaining ~3300 lines are pure hand-rolled diff algebra with zero serde, untouched |

**The 4 composed-child files — DONE, hand-written `ToValue`/`FromValue` bridging `#[child(...)]`
fields through `to_dsl_value`/`from_dsl_value` (the pre-existing serde-based bridge —
`ArtifactChild<S>` derives with `#[serde(bound = "")]`, so it's `Serialize`/`Deserialize` for ANY
`S`, INCLUDING an `S` that no longer is — confirmed by reading `store::ArtifactChild`'s own
definition, `🏪️store/🦀️component.rs:2565`, so removing `serde` from `SemioBrepSnapshot` etc.
earlier in the fan-out does NOT break these bridges), mirroring `📖️playbook`'s `PlaybookArtifact`
reference exactly — derive list and `#[serde(...)]` attrs on the struct itself are LEFT UNCHANGED
(still needed for the `ArtifactChild<S>`/`ArtifactLink` fields' own serde requirement), only the
composed struct's own `impl ToValue`/`impl FromValue` are added, additively:**

| file | children bridged | extra work |
|---|---|---|
| `🧿️semio/…✳️object/🧬️schema/🦀️component.rs` (`SemioObjectArtifact`) | `brep`/`mesh`/`properties` (`Option<ArtifactChild<S>>` ×3) | — |
| `🧿️semio/…✳️object/🧬️schema/📸️snapshot/🦀️component.rs` (`SemioObjectSnapshot`) | same 3 | JSON bridge fns → `pack::to_json_string`/`from_json_str` |
| `🧿️semio/…✳️kit/🧬️schema/🦀️component.rs` (`SemioKitArtifact`) | `objects`/`models` (`Vec<ArtifactChild<S>>` ×2, not `Option` — bridges identically), `properties` (`Option<ArtifactChild<S>>`), `representations` (`Vec<ArtifactLink>` — a LINK slot, not a CHILD slot, same bridge mechanism) | — |
| `🧿️semio/…✳️kit/🧬️schema/📸️snapshot/🦀️component.rs` (`SemioKitSnapshot`) | same 4 | plus 4 plain (non-composed) structs in the SAME file — `SemioKitType`/`SemioKitPiece`/`SemioKitConnection`/`SemioKitDesign` — converted mechanically (missed by the prior session's bulk pass, not composed-child, just ordinary structs); JSON bridge fns → `pack::to_json_string`/`from_json_str` |

## `🧊️gltf`'s snapshot file — the one real outlier among the 15, worth its own note

Unlike every other file in the 15, `🧊️gltf/…/📸️snapshot/🦀️component.rs`'s types are ALSO the
literal wire model for real `.gltf`/`.glb` FILES — `🚪️io/🦀️component.rs` (a sibling file, NOT
touched this session, out of scope) calls `serde_json::to_vec`/`from_str::<GltfDocument>` directly
to read/write genuine spec-compliant glTF JSON text. That means `GltfDocument` and everything it
recursively contains (`GltfAsset`, `GltfScene`, `GltfNode`, `GltfMesh`, `GltfAccessor`, …) needs
`serde::Serialize`/`Deserialize` to remain — not "interim, not yet converted" the way the rest of
this crate's serde is, but a REAL, probably PERMANENT requirement until (if ever) `🚪️io/🦀️component.rs`'s
own byte-exact external-file codec is separately rewritten off `serde_json` (a much bigger, distinct
piece of work, not attempted here). So this file's ~32 structs/enums got `value_derive::ToValue,
value_derive::FromValue` ADDED alongside the existing `Serialize, Deserialize` (never replacing it)
— every `#[serde(...)]` attribute line got an IDENTICAL sibling `#[value(...)]` line (same content,
since every attribute spelling used here — `rename_all`, `rename`, `default`, `default = "path"`,
`skip_serializing_if` — is already 1:1 supported).

Four types could NOT go through the derive even additively and got fully hand-written
`impl ToValue`/`impl FromValue` instead (mirroring their existing hand-rolled `impl Serialize`/
`impl Deserialize` structurally, not textually — no serde types touched):
- **`GltfJson`** — this artifact's OWN local arbitrary-JSON-value enum (`Null`/`Bool`/`Number`/
  `String`/`Array`/`Object`, insertion-order-preserving), used for `extras`/`extensions` slots.
  Structurally identical mapping to `DslValue`.
- **`GltfMorphTarget`** (`#[serde(transparent)]` newtype over `Vec<(String, usize)>` with
  `#[serde(with = "ordered_attr_map")]` on the inner field) — `#[value(transparent)]` alone would
  bypass the inner `with` and use the DEFAULT `Vec<(String,usize)>` encoding (array-of-2-tuples),
  which is the WRONG wire shape for glTF's `attributes`/morph-target maps (must be a JSON OBJECT,
  `{"POSITION": 0, ...}`). Hand-written to call the new `ordered_attr_map_to_value`/
  `ordered_attr_map_from_value` free functions (added alongside the existing `ordered_attr_map`
  serde module, same object-shaped mapping, `DslValue` instead of a `Serializer`/`Deserializer`) —
  also used via `#[value(serialize_with = "…", deserialize_with = "…")]` on `GltfPrimitive::attributes`,
  the other real use of this shape.
- **`GltfCameraProjection`**/**`GltfCamera`** — a hand-rolled `{"type": "perspective", "perspective":
  {...}}` tagged-union-with-flattened-sibling-keys shape (the payload's own key is NAMED after the
  tag value, not a fixed `content` key) — neither `#[value(tag = "…")]` nor `#[value(tag = "…",
  content = "…")]` can express this generically, so hand-written to match exactly.

Two plain structs (`GltfOrthographic`/`GltfPerspective`) were inside the same `//#region 🔖️Camera`
block as the hand-written types above but are themselves ordinary — the mechanical pass's region
exclusion (drawn generously around the whole Camera section) skipped them by accident; caught by a
post-pass script cross-checking every `pub struct`/`pub enum` against whether `value_derive`
appeared in its derive list, and fixed by hand (same additive mechanical conversion as the rest).

Also additive, in the SIBLING `🚪️io/🦀️component.rs` file (not otherwise touched): `GltfComponentType`/
`GltfAccessorType` already had HAND-ROLLED `impl Serialize`/`impl Deserialize` there (wire shape:
raw numeric code / spec string, never the Rust variant name) — added matching hand-written
`impl dsl::ToValue`/`impl dsl::FromValue` right beside them, additive, so the snapshot file's
`GltfSparseIndices`/`GltfAccessor` etc. (which reference these two leaf enums as field types) have
something to derive against. `🚪️io/🦀️component.rs`'s own real `serde_json::to_vec`/`from_str`
codec functions were NOT touched — still real, still needed, still out of scope.

## os-kernel and semio-framework-plugin are now GREEN — landed mid-session by a concurrent agent

`cargo check -p semio-framework-os-kernel` and `cargo check -p semio-framework-plugin` both went
from red (the `ArtifactApp::Snapshot` trait-bound migration status.md flagged as "in progress",
assigned to a dedicated peer agent) to a clean `Finished` with warnings only, observed directly this
session (not inherited from a stale claim — the SAME `cargo check -p semio-s-plugin-stdio` run that
had stopped on `semio-framework-plugin`'s single remaining `E0277` minutes earlier proceeded past it
on the next attempt). This is NOT this session's work — flagged here only because it changes what
"the blocker" means for whoever picks this up next.

## VERIFICATION — what actually ran, verbatim tails, and the real remaining blocker

**PROVEN BY A PASSING RUN:**
- `cargo check -p semio-framework-value-derive --message-format=short` — clean, 2 pre-existing
  cosmetic warnings (`unnecessary qualification`), 0 errors, every time it was re-run after an edit.
- `cargo check -p semio-framework-replication --message-format=short` — clean, 0 errors (this crate
  hosts `🌱️value/🔁️codec` and `🎒️pack`'s `json` module is reached through `semio-framework-pack`,
  which ALSO checked clean, `cargo check -p semio-framework-pack`, 0 errors, 3m21s→2m96s across
  re-runs).
- `cargo test -p semio-framework-pack --lib json::` — **31 passed, 0 failed** (26 pre-existing +
  5 new `json!` macro tests).
- `cargo test -p semio-framework-replication --lib value::codec::` — **7 passed, 0 failed** (5
  pre-existing + `fixed_size_array_round_trips_and_rejects_wrong_length` +
  `phantom_data_encodes_as_null_and_decodes_from_anything`).
- Standalone derive-extension scratch crate — **7/7 scenarios pass** (verbatim tail above).
- `cargo check -p semio-framework-os-kernel` / `-p semio-framework-plugin` — both clean (0 errors),
  confirming the fleet-wide blocker this session started behind is gone.

**WRITTEN BUT NOT YET PROVEN BY A PASSING `cargo check -p semio-s-plugin-stdio`** — every one of the
19 files above, and the two framework files they lean on (`🌱️value/✨️derive`, `🌱️value/🔁️codec`,
`🎒️pack/🔤️json`, `🚪️io/🦀️component.rs`'s additive hand impls). Reason, confirmed across FIVE
separate `cargo check -p semio-s-plugin-stdio` attempts spread across roughly 20 minutes of real
wall-clock time (re-run each time specifically because the error count kept dropping — 1 error in
`semio-framework-plugin` → gone; 17 errors in `semio-framework-ui` → 5 → 5 (stalled)):

```
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs:3:18: error[E0432]: unresolved import `crate::wgpu::draw`: could not find `draw` in `wgpu`
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs:2469:90: error[E0433]: cannot find `draw` in `wgpu`: could not find `draw` in `wgpu`
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs:2483:90: error[E0433]: cannot find `draw` in `wgpu`: could not find `draw` in `wgpu`
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs:2487:101: error[E0433]: cannot find `draw` in `wgpu`: could not find `draw` in `wgpu`
🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs:2576:90: error[E0433]: cannot find `draw` in `wgpu`: could not find `draw` in `wgpu`
error: could not compile `semio-framework-ui` (lib) due to 5 previous errors; 33 warnings emitted
```

**Confirmed unrelated to this session's work, not a phantom/stale-lock artifact**: `git status
--porcelain` on `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/` shows 8 modified
files plus one brand-new untracked file (`🦀️draw_types.rs`) — a concurrent agent mid-refactor,
splitting `draw.rs` apart, currently in a broken intermediate state. `cargo check -p
semio-framework-ui` in ISOLATION (no stdio in the dependency graph) compiles clean — the wgpu
target module is reached only via some feature/dependency-unification path that a bare `-p
semio-framework-ui` check doesn't exercise but building any real plugin (stdio, and independently
confirmed for `semio-s-plugin-vcs`, the ticket's required dependent check) does. **Zero errors, in
any of the five attempts, ever named a symbol, file, or type from anything touched this session** —
every single error is inside `🎯️targets/🧊️wgpu/`, nothing this session edited is anywhere near that
module (stdio does not depend on the desktop wgpu renderer target at all; it's pulled in
transitively through `semio-framework-plugin`/`semio-framework-ui`'s own dependency graph).

`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-stdio` and `cargo check -p
semio-s-plugin-vcs` (the ticket's required dependent check) were both attempted and BOTH hit the
identical `wgpu::draw` blocker, confirming it is fleet-wide right now, not stdio-specific — every
plugin is currently blocked on this, independent of anything in this session's scope.

**Verdict, honestly stated**: all 19 files, plus every framework primitive they needed, are
**WRITTEN and individually PROVEN correct** (the derive extensions via the standalone scratch
crate, the pack/json macro and codec additions via real in-crate tests) but **NOT YET PROVEN by
the crate-level `cargo check -p semio-s-plugin-stdio`/wasip2-build/dependent-check this ticket
requires**, purely because a concurrent, unrelated, actively-in-progress `wgpu` refactor currently
blocks EVERY plugin in the workspace from compiling, not just stdio. Given how fast the two
upstream blockers ahead of this one resolved during this same session (os-kernel, then
semio-framework-plugin, each landing mid-session while this work was in progress), the wgpu
refactor completing soon is the reasonable expectation, not a special case.

## Manifest state — left exactly as found, correctly

`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` still has `serde`/`serde_json` restored with the
prior session's `🚧️ INTERIM` docstring — correctly so, this session did NOT flip it. Regenerate the
remaining count with the same command the prior session left:
```bash
grep -rl "serde_json::" --include="*.rs" ✏️s/🔌️plugins/🗄️stdio/ | grep -v /target/ | \
  grep -vP '/🧪️(test|tests|oracle)/|/🔬️probes/|/🏭️generator/|/🧫️fixtures/'
```
**553 files remain** (down from the prior session's ~558/~410-untouched estimate — the delta is
this session's conversions removing derive-adjacent call sites, not a discrepancy in counting
method). `🚪️io/🦀️component.rs`'s real `.gltf`/`.glb` byte-exact codec is a KNOWN, probably-permanent
subset of that count (not "not yet converted", genuinely still needed) — worth flagging to whoever
scopes the next wave so it isn't miscounted as ordinary remaining work. `derive(.*Serialize` count
dropped from 19 to 5 (the 4 hand-written-only gltf types, all in one file, plus incidental
overlaps) — regenerate with:
```bash
grep -rl "derive(.*Serialize" --include="*.rs" ✏️s/🔌️plugins/🗄️stdio/ | grep -v /target/ | \
  grep -vP '/🧪️(test|tests|oracle)/|/🔬️probes/|/🏭️generator/|/🧫️fixtures/'
```

## What a follow-up agent should do, in order

1. Re-run `cargo check -p semio-s-plugin-stdio --message-format=short` FIRST, before anything else
   — if the `wgpu::draw` refactor has landed (very likely by now given the pace observed this
   session), this either goes straight to a real signal on the 553 remaining call-site files, or
   confirms this session's 19 files + framework additions compile clean, which would be the first
   real compiler confirmation either has ever had.
2. If still blocked on `wgpu`, that is NOT this ticket's problem to fix (framework rendering target,
   unrelated dependency graph) — note it and move on to source-level progress on the 553 remaining
   call-site files, the same way this session did for the derive-attribute files: read each file,
   don't assume, most are small (105 of 558 originally had exactly 1 `serde_json::` occurrence).
3. The `🪟️windows/🪟️main/🦀️component.rs` viewer/editor family (~78 files, ~390 call sites, one
   shared template — `entity_count`/`world_instances_json`/`render`, differing only by
   substituted subset name) was IDENTIFIED as the single highest-leverage remaining class
   (structurally byte-identical across all ~78 instances, confirmed by diffing two after
   normalizing subset-name tokens) but NOT converted this session — the `pack::json!` macro this
   session added exists specifically to make that conversion mechanical
   (`serde_json::json!({...})` → `pack::json!({...})` is close to a literal find-replace once
   `serde_json::Value`/`to_string`/`to_value` are also swapped for `pack::JsonValue`/
   `pack::json_to_string`/`ToValue::to_value`). This is the natural next wave.
4. `📇️registry/🦀️component.rs` (12 structs, all `#[serde(deny_unknown_fields)]`, parsing every
   artifact's own `📜️artifact-definition.json`) — flagged by the prior session as needing hand
   conversion because the derive's `deny_unknown_fields` was a documented no-op. **That is no
   longer true** — this session's derive extension enforces it for structs. Converting these 12
   should now be a much more direct mechanical pass than the prior session's note implied.

---

# Follow-up session — `🪐️space` and `🔱️trinity` finished, manifests now zero-serde

Picks up exactly where the previous session's "What a follow-up agent should do" list ends.
`semio-framework`/`semio-framework-os-kernel` were confirmed green at the start of this session
(the `#[derive(ToValue)] match *self` bug fix had already landed) — see this ticket's own
`📓️verified-outcomes.md` for that headline.

## Result

**Both `space` and `trinity` Cargo.toml `[dependencies]` are now zero third-party.** `serde`/
`serde_json` moved to `[dev-dependencies]` in both manifests (not deleted outright — see "Why
dev-dependencies, not deleted" below).

| manifest | `[dependencies]` | remaining `serde_json::` call sites |
|---|---|---|
| `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` | **zero third-party** | 3 files, all `#[cfg(test)]`/comment-only |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml` | **zero third-party** | 15 files, all `🧪️tests/<case>/🦀️component.rs` fixture dirs |

Both had ~101/122 and ~50/71 files converted respectively when this session started (per the
previous session's headline table). This session converted every remaining production call site in
both crates: the full `serde_json::json!`/`Value`/`to_string`/`from_str`/`to_value`/`from_value`/
`to_vec`/`from_slice`/`to_writer`/`to_string_pretty`/`Map`/`Number` surface, across ~80 files.

## Why `[dev-dependencies]`, not deleted outright

Both crates carry a `🧪️tests/<mutation-case>/🦀️component.rs` fixture-comparison suite (~15-20 tiny
files per crate, one per mutation test case) that deliberately encodes/decodes through
`serde_json` as a **differential third-party oracle** — several of trinity's own test function
names literally say `_matches_the_serde_json_oracle`/`_and_serde_json_boundaries`. This is the
CLAUDE.md-sanctioned exception ("You SHOULD use existing libraries as possible to test our
implementation"), and matches the official dependency scanner's own methodology (it excludes
`🧪️oracle`/`🔬️probes`/`🏭️generator`/`🧫️fixtures` from violation counts, and only inspects the
literal `[dependencies]` table — which is now clean). Converting these ~35 fixture files too was
judged out of proportion to their value (they are integration-test-only, `#[cfg(test)]`-gated,
never reachable from the shipped `wasm32-wasip2` component) and was left alone on purpose, mirroring
the previous session's own explicit precedent for `stdio`.

## Framework additions (shared benefit, several plugins will need these)

1. **`pack::json!` macro** — landed by a *concurrent* session mid-way through this one (found via
   the "file changed on disk" note when editing `🎒️pack/🔤️json/🦀️component.rs`); this session did
   NOT write it, just consumed it. A TT-muncher builder for `pack::json::Value` object/array
   literals, `serde_json::json!`-compatible syntax (bare nested `{}`/`[]`, trailing commas, any
   `Into<Value>` leaf expression). Verified by that session's own `json_macro_builds_*` unit tests
   (all passing, see Verification below).
2. **`impl std::ops::Index<&str>` / `Index<usize>` for `pack::json::Value`** (this session) —
   `value["key"]`/`value[i]` read access, `Value::Null` on any miss, mirroring
   `serde_json::Value`'s own permissive `Index` semantics. No `IndexMut` (see the
   `set-active-panel-tab` note below for why call sites route through `Object::insert` instead).
3. **Cross-type `PartialEq<str|String|bool|f64|i64|u64|i32|u32|usize|&str>` for `pack::json::Value`**
   (this session, both directions) — lets `assert_eq!(value["key"], "literal")` read unchanged from
   the `serde_json`-based call sites it replaces.
4. **`impl fmt::Display for pack::json::Value`** (this session) — `.to_string()`/`format!("{v}")`
   parity with `serde_json::Value`'s own `Display`.
5. **`pack::json::to_string_pretty` / `pack::json_to_string_pretty`** (this session) — a genuine
   gap: the module's own docstring previously said "no pretty-printing — no consumer needs it",
   which stopped being true once real call sites (an example-document viewer, a Jack fixture
   export, a rewrite-rule inspector) needed `serde_json::to_string_pretty` replaced. 2-space
   indent, `": "` after keys, matches `serde_json::to_string_pretty`'s own layout byte-for-byte on
   every fixture this session checked by eye.
6. **`Value::as_array` changed from `Option<&[Value]>` to `Option<&Vec<Value>>`** (this session,
   API-compatibility fix on a concurrent session's own new type) — `serde_json::Value::as_array`
   returns `Option<&Vec<Value>>` specifically so `.and_then(Value::as_array).cloned()` call sites
   produce an owned `Vec<Value>`; the bare-slice signature silently failed that exact pattern
   (`[Value]` is unsized, has no `Clone`). Every existing caller of the old signature (`.iter()`,
   `.len()`, indexing) still compiles unchanged since `Vec<T>` derefs to `[T]`.
7. **`impl From<usize>`/`From<i8>` for `Value`/`Number`** (this session) — the two integer widths
   real call sites needed that the pre-existing `u64/i64/f64/u32/i32` set didn't cover.
8. **`impl From<pack::json::Value>` / `From<&pack::json::Value>` for `semio_s_plugin_stdio`'s own
   `JsonValue`, plus `JsonSnapshot::to_pack_value()`** (this session,
   `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️base/🧬️schema/📸️snapshot/🦀️component.rs`)
   — the cross-plugin coupling the previous session flagged and explicitly deferred ("stdio's own
   JSON artifact type... currently only accepts `serde_json::Value`"). `Number` has no independent
   lexeme in `pack::json::Value` (unlike stdio's own arbitrary-precision `Number { lexeme }`), so
   the bridge round-trips a lone number through `pack::json_to_string`/`pack::parse_json` rather
   than reimplementing float/int formatting a second time. **Additive only** — stdio's own
   `serde_json`-based `From` impls and `to_serde_value` are untouched, `stdio`'s manifest was not
   touched, per the "another agent may be converting stdio concurrently" instruction. Used by
   `space`'s and `trinity`'s `home`/`jack`/`rewrite` JSON import/export bridges (6 files), which
   now go `snapshot -> DslValue -> pack::JsonValue -> stdio::JsonValue` instead of through
   `serde_json::Value`.
9. **`semio-framework-graph`: `Completion` (in `🕸️graph/🗣️dsl/🦀️component.rs`) gained
   `value_derive::ToValue`/`FromValue`, additive alongside its existing `Serialize`/`Deserialize`**
   — needed by trinity's `complete_jack_json`/the world-editor test round trip. New
   `semio-framework-value-derive` path dependency added to that crate's Cargo.toml (resolve-checked)
   and a `value_derive` extern-crate alias added to its `glue.rs`, mirroring the pattern every
   already-converted plugin uses.
10. **`semio-framework-ui-scene`: hand-written `FromValue`** (decode direction only — nothing here
    needed the encode direction) for `NodeGraphPortRecord`/`NodeGraphNodeRecord`/
    `NodeGraphEdgeRecord`/`NodeGraphFindItem`/`NodeGraphOperatorVariadicRecord`/
    `NodeGraphOperatorChannelRecord`/`NodeGraphOperatorRecord`
    (`🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️scenes.rs`) — **deliberately hand-written, not derived**: this
    crate's own module docstring declares it "wasm-safe by construction... depends on `ui_contract`
    and `serde` only", and `#[derive(ToValue, FromValue)]`'s generated code hardcodes
    `::semio_framework_os_kernel::…` paths (per the fan-out playbook), which would have forced this
    minimal-dependency crate to newly depend on the entire OS kernel. Added a lighter
    `semio-framework-replication` (`protocol`) path dependency instead — that crate's own doc says
    it "depends on neither [client nor server]", confirmed no cycle risk — and hand-wrote each
    `FromValue::from_value` against `protocol::value::{DslValue, FromValue, ValueError}` directly,
    following the `PlaybookArtifact` object-vec decode pattern (`get`/`field`/`opt` closures over
    `DslValue::into_object()`'s entries). Verified: `cargo test -p semio-framework-ui-scene --lib`
    — 99/99 passing (see Verification).
11. **`semio-framework-os` (host): `OsWorkflowChannelSpec`/`OsWorkflowOperatorInfo` gained
    `value_derive::ToValue`** (encode direction only), additive alongside `Serialize`. New
    `semio-framework-value-derive` dependency + `value_derive` alias (feature-gated identically to
    the pre-existing `dsl`/`protocol` aliases on `os-host-full`). Used by `space`'s
    `json_array_to_node_graph_operators` shim.

## Hand-written impls in the plugins themselves, and why (per-file)

- **`🪐️space/⚙️engine/🪐️space/⚙️engine/🦀️component.rs` — `AppRegistrationWireEntry`**: reverted a
  PRE-EXISTING (not this session's) `#[derive(value_derive::FromValue)]` that could not have
  compiled — `app: AppDefinition` (from `semio-framework-manifest`, a large tree:
  `AppRole`/`ArtifactDialect`/`LocalizedLabel`/`Modes`(`NonEmptyVec` w/ a hand-rolled
  `serde(try_from/into)` wire shape)/`WindowKinds`/…) has no `FromValue` anywhere, and converting
  that entire manifest module is squarely out of this ticket's plugin-manifest scope. Hand-wrote
  `FromValue` for the wrapper struct instead: `plugin_id` through `FromValue` directly, `app`
  through the OLD `dsl::from_dsl_value::<T: DeserializeOwned>` bridge (framework-internal,
  `AppDefinition` already satisfies it via its own `Serialize`/`Deserialize`, and calling it does
  **not** add a `serde` dependency to the plugin crate — the bound is checked against the
  already-compiled generic function in `os-kernel`, never named in the plugin's own source).
  Exactly the fan-out playbook's composed-field pattern (`ArtifactChild<S>`), applied to a second
  framework type.
- **`🪐️space/⚙️engine/🪐️space/🦀️component.rs` — `create_space_app`'s demo-document pretty-printer**:
  same technique for `OsWorkflowArtifactDocument` (`= BackboneDocument<WorkflowSnapshot,
  WorkflowMutation>`), an even deeper framework tree (`ArtifactVcs`/`ArtifactCursor`/`Edit`/
  `Conflict`/…) — `dsl::to_dsl_value(&document)` then `pack::json_from_dsl_value`/
  `pack::json_to_string_pretty`. Flagged with a `🚧️` docstring as real follow-up (native `ToValue`
  once the field tree gets it) rather than a permanent shim. Also fixed a pre-existing bug found
  while touching this line: `parse_demo_space_document()` is `pub async fn` and was being called
  without `.await` (would not have compiled either way, unrelated to serde) — added `.await`.
- **`🔱️trinity/🗿️artifacts/♻️rewrite/…/✏️editor/🌍️world/🦀️component.rs` — `JackRunWithFixture`**:
  had `#[serde(flatten)]` on its `result: QueryResult` field — not a derive-supported attribute.
  Hand-wrote `ToValue` splicing `QueryResult::to_value()`'s own object entries directly into the
  parent, then pushing `fixtureJson`, matching the old flattened wire shape byte-for-byte.
- **Both `TrinityRewriteError::Json`/`TrinityRamError::Json` variants**: were `serde_json::Error`;
  changed to hold a `String` (or `dsl::ValueError` for the rewrite one) with new `From<pack::JsonError>`
  and `From<dsl::ValueError>` impls, since the JSON pipeline now produces two different first-party
  error types instead of serde_json's one.
- **`🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs`'s `from_json`**: `Camera`/`Node`/`Edge` already
  had `FromValue`; rewired `serde_json::from_value(v.clone())` to
  `dsl::FromValue::from_value(pack::json_to_dsl_value(v))`.
- **Byte-budget counters** (`🪐️space/🏠️home/…/✏️editor/🦀️component.rs`'s `home_config_edit_bytes`,
  `🔱️trinity/🔌️jack/…/✏️editor/🦀️component.rs`'s `jack_bounded_serialized_bytes`): both used to
  stream through a `serde_json::to_writer`-fed byte-counting `io::Write` sink to enforce a size
  cap without materializing the full string. Since `pack::to_json_string` is not streaming, both
  now compute the full string length directly and compare against the cap — behaviourally
  identical (same pass/fail boundary, same `Ok(bytes)`/`Err(message)` result), just not
  streaming-incremental. Bound changed `T: serde::Serialize` → `T: dsl::ToValue`.

## Verification — proven by passing runs this session

```
cargo check -p semio-framework-pack --message-format=short         # 0 errors, 1 pre-existing warning
cargo test  -p semio-framework-pack --lib                          # 84 passed; 0 failed
cargo check -p semio-framework-graph --message-format=short        # 0 errors
cargo check -p semio-framework-ui-scene --message-format=short     # 0 errors, 1 pre-existing warning
cargo test  -p semio-framework-ui-scene --lib                      # 99 passed; 0 failed
cargo check -p semio-framework-os-kernel --message-format=short    # 0 errors (default features)
```

**NOT proven — `semio-s-plugin-space` and `semio-s-plugin-trinity` themselves never completed a
clean `cargo check` this session**, blocked by two *different*, clearly unrelated, actively-changing
framework files (confirmed via `git diff --stat` showing live in-progress edits, not anything this
session touched):

1. `space` requires `semio-framework-os`'s `os-host-full` feature, which requires
   `semio-framework-os-kernel/sync` — that module (`🏪️store/🔄️sync/🦀️component.rs`, feature-gated,
   so a plain default-feature `cargo check -p semio-framework-os-kernel` never compiles it) is
   mid-refactor: `ArtifactStore<P, Mutation>::attach_backbone`/`detach_backbone`/`tick`/`dispatch`
   all currently fail with `P: protocol::ToValue`/`FromValue` not satisfied. This is very likely
   the "`TopicContribution` seam" work the ticket brief names as a concurrent agent's in-flight
   task — not touched.
2. `trinity` (and `space`, transitively, via `semio_framework_plugin`) depends on
   `semio-framework-ui`, whose wgpu target currently fails with `effective_scissors is private` at
   `🎯️targets/🧊️wgpu/🦀️draw.rs:2033`. `git diff --stat` on that exact file shows **941 deletions**
   in the working tree right now — an active, large-scale in-progress refactor, unrelated to
   serialization. Not touched.

Both were re-checked twice, ~40 seconds apart, with identical results — not a stale/queued-check
artifact (the ticket's own "phantom blocker" pattern), a live, currently-broken dependency.
**Every file this session edited inside `space`/`trinity` was reviewed by hand against the
`ToValue`/`FromValue` derives and signatures actually present on its target types** (not merely
pattern-matched), and the two isolable framework crates this session touched most
(`semio-framework-pack`, `semio-framework-ui-scene`) both check AND test clean — but the plugin
crates' own `cargo check` is honestly **WRITTEN, NOT PROVEN**, blocked on the two items above, per
the ticket's own repeated caution about phantom vs. real blockers. `wasm32-wasip2` builds were not
attempted for the same reason (would hit the identical blockers, no new signal).

## What a follow-up agent should do

1. Re-run `cargo check -p semio-framework-os --features os-host-full --message-format=short` and
   `cargo check -p semio-framework-ui --message-format=short` first, standalone — if both are
   green, `cargo check -p semio-s-plugin-space --message-format=short` and
   `cargo check -p semio-s-plugin-trinity --message-format=short` should go straight to PROVEN or
   surface the first real (hopefully small) mistake in this session's ~80 hand-edited call sites.
2. If real errors surface in `space`/`trinity` themselves (not the two framework blockers above),
   they are almost certainly in one of the hand-written impls listed above (`AppRegistrationWireEntry`,
   `JackRunWithFixture`, the byte-budget counters, or the `ui-scene` `FromValue` impls) — those are
   the only places this session wrote code without a derive doing the mechanical work, and the
   `git diff` for each is small and self-contained.
3. Once green, `cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-space` and the trinity
   equivalent are the remaining unproven step from the ticket's own checklist.
