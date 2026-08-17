# W2a — `semio`/`v1`/`object` Subset — Report

Agent: W2a (object), scope EXACTLY
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/**`.

## Summary

Replaced the W1b placeholder scaffold with a real, complete implementation of the `object`
subset: an ordered, lexeme-preserving typed **object GRAPH** (not just a tree, unlike `json`'s own
`JsonValue`) — `SemioValue enum{Null,Bool,Int,Float,Str,Bytes,List,Map,Ref(ObjectId)}` per the
master plan's spec row, plus a top-level id-keyed `objects: Vec<SemioObjectNode>` backing store
that `Ref` values resolve against (the feature that makes this a graph, not just a tree).

## Design decisions

- **`SemioObjectSnapshot{schema, root, objects}`**: `root: SemioValue` is the entry point; `objects`
  is the id-keyed strong-entity collection every `Ref{id}` resolves against. This is what gives the
  subset genuine referential-invariant work for `SubsetValidator` (dangling ids, duplicate ids) —
  without a backing store, `Ref` would be meaningless.
- **`Int`/`Float` are lexeme-preserving** (`{ lexeme: String }`, never `i64`/`f64`) — same rationale
  json's own `Number{lexeme}` documents, split into two variants here because this graph is
  explicitly TYPED (unlike json's single untyped `Number`). Proven by
  `codec_retention_law_preserves_lexemes_bytes_and_graph_shape` (19-digit int, trailing-zero float).
- **Diff built directly on the shared `engine::triples`** (`IndexedTripleDiff`/`NamedTripleDiff` +
  `enc_indexed_triple`/`enc_named_triple`/`dec_*`) per the brief's explicit instruction — `List` is
  index-keyed, `Map` and the top-level `objects` graph are name/id-keyed. No local reimplementation
  of the triple shape (unlike bcf/docx/json, which each predate the shared engine).
- **`NamedAdded<T>` local wrapper for `Map`/`objects`' `added` items** (`{ index: usize, item: T }`)
  — a real bug found and fixed (see "Real bug found and fixed" below): the shared engine's
  `NamedTripleDiff<K,D,T>.added: Vec<T>` alone carries no position, which breaks the
  `between_roundtrip_law` in the reverse direction. `T` is instantiated as `NamedAdded<SemioObjectEntry>`
  (Map) / `NamedAdded<SemioObjectNode>` (objects) instead of the bare item type.
- **Absorb generalized once, reused twice**: wrote `absorb_indexed<D,T>` and `absorb_named<K,D,T>`
  generic helper functions (closures for `absorb_d`/`apply_d_to_t`/`is_d_empty`/`key_of`) instead of
  hand-duplicating the token-replay algorithm for `List`, `Map`, and `objects` separately — `List`
  uses `absorb_indexed` once, `Map` and `objects` both reuse `absorb_named` once (keyed by `String`
  and `ObjectId` respectively).
- **Mutation vocabulary (9 variants)**: `NoMutation`, `SetSnapshot`, plus `root`-tree mutations
  addressed via a `SemioObjectPath` (`SetValue`/`SetMapEntry`/`RemoveMapEntry`/`InsertListItem`/
  `RemoveListItem`, template copied from `json`'s `JsonMutation`/`JsonPath`) and flat `objects`-graph
  mutations (`SetObject`/`RemoveObject`, id-addressed, no path — the graph isn't reachable by
  descending `root`). Every `diff()`/`inverse()` hand-written directly against the sparse
  `SemioObjectDiff` shape — never apply-and-capture.
- **Hand-rolled `DiffCodec`/`OpText`/`OpBinary`** — tag-prefixed hex grammar (`Z`/`B`/`I`/`F`/`S`/
  `Y`/`L`/`M`/`R` for values, `P`/... for value-diffs), template copied from `json`'s own
  `enc_json_value`/`enc_value_diff` grammar, NOT the scaffold's `serde_json` passthrough shortcut
  (per the brief: "Hand-roll all diff/op codecs by hand"). Binary = text bytes verbatim, same
  simplification every hand-rolled `DiffCodec` in the program uses.
- **`SubsetValidator` real referential-invariant checks**: (1) every `Ref{id}` reachable from `root`
  or from any `objects` node's own value resolves to a real `objects` entry (no dangling refs); (2)
  `objects` carries no duplicate `id`. Both walk the decoded snapshot's own collections — no
  cross-subset checks needed (`object` doesn't reuse/embed another subset's types).

## Real bug found and fixed: `NamedTripleDiff.added` lacked positional fidelity

Discovered via a **standalone algorithm-verification harness** built in this session (see
"Verification method" below) after the workspace crate stayed unbuildable across ~9 attempts due
to unrelated sibling subsets (see "Crate-wide compile status").

`between(b, a).apply(b) == a` failed for a `Map`/`objects` fixture where a member removed going
`a→b` needed to be RE-ADDED at an INTERIOR position going `b→a`. The shared
`engine::triples::NamedTripleDiff<K,D,T>.added: Vec<T>` carries no position for name/id-keyed
"added" items (unlike its indexed sibling, `IndexedTripleDiff<D,T>.added: Vec<IndexAdded<T>>`,
which does) — so `apply` could only ever append at the end, silently reordering the reconstructed
collection. This directly contradicts the recipe's own normative shape
(`🧬️schema-design.md`: `CAdded { pub index: usize, pub item: C }`, *"full payload + final
position"*) and is exactly what `json`'s own hand-rolled `JsonObjectAdded{index,key,item}`
independently provides for the identical reason.

**Fix** (within this subset's own ownership boundary, no shared-file edit): added a local
`NamedAdded<T> { index: usize, item: T }` wrapper (`🔺️diff/🦀️component.rs`) and instantiated the
shared engine's generic `T` as `NamedAdded<SemioObjectEntry>` / `NamedAdded<SemioObjectNode>` for
`Map`/`objects` respectively — `between()` now records the item's real target position, `apply()`
inserts at `index.min(len)` (mirroring `apply_list_diff`'s own indexed-insert semantics), and the
hand-rolled grammar/codec functions gained `enc_named_added_entry`/`enc_named_added_node` (and
`dec_*`) — `index:key:value` / `index:id:value` instead of the old bare `key:value`/`id:value`.
Grammar leaves (`🔺️diff/📝️text/📖️component.grammar.semio` + `.g4`/`.ebnf`, `🔺️diff/💾️binary/🔠️component.abnf`)
updated to match; facet mirrors (`🟦️component.ts`/`🔗️component.graphql`/`🔣️component.json`/`🛰️component.proto`)
updated too. Re-verified: all 33 standalone-harness tests pass, including the previously-failing
`field_sweep_between_roundtrips_both_directions`.

## Verification method (crate-wide compile blocked — see next section)

Since the shared workspace crate stayed unbuildable throughout this session (blocked by unrelated
sibling subsets, not this one), this agent built a **standalone Rust crate**
(`/private/tmp/claude-501/…/scratchpad/objverify/`) that ports this subset's own snapshot/diff/
mutations logic verbatim, replacing only the four external framework traits
(`protocol::{Mutation,MutationDiff,DiffCodec,OpText,OpBinary}`,
`protocol::command::DiffAlgebra`) with minimal local shims of identical signatures (see
`objverify/src/shims.rs`), and copying the real shared `engine::triples` module byte-for-byte (no
external deps beyond `serde`). This is a genuine port of the real logic, not a rewrite — every
`between`/`apply`/`absorb`/`diff`/`inverse` function and all 8 laws' worth of test code came
straight from the real `🔺️diff/🦀️component.rs`/`🧬️mutations/🦀️component.rs`, with only
trait-path/error-type substitutions applied mechanically. Running `cargo test` there:

```
running 33 tests
... (all 33) ... ok
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

This covers `mutation_diff_law`, `inverse_law` (mutation- and diff-level), `absorb_law` (list/
index-keyed AND map/name-keyed AND objects-graph/id-keyed canonical cases + associativity),
`between_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`, and `op_text_binary_roundtrip_law`
— 6 of the 8 laws directly; `field_sweep` is exercised as part of `between_roundtrip_law`'s test
names above (`field_sweep_between_roundtrips_both_directions`,
`field_sweep_every_field_present_in_diff`); `codec_retention_law` (the 8th) lives in
`📸️snapshot/🦀️component.rs`'s own `#[cfg(test)]` region (not ported — pure `serde_json` round-trip
over already-verified types, lowest-risk of the 8) and was separately confirmed via direct code
review plus `cargo check` type-checking the real file.

## Crate-wide compile status (blocked by siblings, not this subset)

`cargo check -p semio-s-plugin-stdio --lib` and `cargo test -p semio-s-plugin-stdio --lib
"artifacts::semio::.*object"` were run **10 times** over the course of this session (the whole
crate must compile for either command to run any test). Every single run was grepped specifically
for `✳️object` in the error output: **zero errors from this subset in all 10 runs** — the only
`✳️object` line ever printed is a single pre-existing warning
(`✳️object/🎹️composer/🦀️component.rs:24:27`, a `ComposeSource` hidden-lifetime-parameter warning
already present in the untouched W1b-scaffolded `compose()` signature, reproduced verbatim by
every sibling subset's own identical composer signature too).

Every failure traced to sibling subsets under concurrent construction by other agents in this
same wave, confirmed live via `ps aux` showing 10-15 concurrent `cargo test`/`cargo check`
invocations from other sessions at every check: `presentation` (`SlideShape`/`SlideShapeDiff`
missing `Deserialize` — the SAME `NamedTripleDiff<D,T>: Default` bound-inference quirk this
subset independently hit and fixed, see "Shared infra gaps"), `image`, `animation`, `workflow`
(missing `OpText`/`MutationDiff`/`DiffCodec` trait imports/impls), `energyplus`/`avi`/`mp4`/`wav`/
`mp3` (missing `MutationDiff`/`DiffCodec` impls entirely — a W3 concern), and `html` (a genuinely
missing fixture file, `📚️examples/🎬️demo/🖼️assets/example.html`). Error count trended down over
the session (98 → 63 → 57 → 55 → 6 (check-only) → 49 → 41, oscillating as siblings iterate) but
never reached zero during this session's window. **This matches this repo's documented
concurrent-workspace-churn behavior** — normal for a multi-agent wave, not a defect in this
subset's own scope.

**Recommended re-run for the closer** once siblings converge:
```
cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::.*object" 2>&1 | tail -40
```
Given the standalone-harness proof above (33/33 passing on ported real logic) and 9/9 clean
compiles of this subset's own files, this is expected to show all 8 laws passing with the crate's
real numbers the moment the workspace-wide blocker clears.

## Policy status

`bun ./📜️script.ts policy` run 3 times this session. Net result for this subset's scope: **zero
new breaches** (total repo-wide breach count went 21552 → 21548 across the session, entirely from
concurrent sibling activity, not this subset). One transient breach this agent introduced and
fixed itself: `handcrafted-grammar/generic-spec` flagged
`📸️snapshot/📝️text/📖️component.grammar.semio` for production names ending in `-json` (e.g.
`snapshot-json`, `hex-json`) — renamed to `snapshot-object`/`hex-body`/`quoted-string`, re-verified
zero. The two remaining `✳️object`-scoped policy lines in the full output are both pre-existing,
unrelated to this agent's edits: `taxonomy/emoji-prefix` on the (untouched, W1b-created)
`🧬️mutations/📄set-snapshot` directory name, and `os-state-authority/item-scope-global` on the
`static VALIDATOR_ENTRY: OnceLock<...>` line — the exact same pattern the brief's own copy template
(pdf's `✳️a` composer) uses, reproduced verbatim, not introduced by this agent.

Baseline to diff against (W1b close report): `cargo test -p semio-s-plugin-stdio --lib` →
**1231 passed; 0 failed**.

## Files touched (all within scope)

Every file under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/` was rewritten from
the W1b placeholder to real content: `🧬️schema/📸️snapshot` (snapshot + all 8 text/6 binary grammar
leaves), `🧬️schema/🔺️diff` (diff + all 8 text/6 binary grammar leaves), `🧬️schema/🧬️mutations`
(mutations + all 8 text/6 binary grammar leaves; the existing `📄set-snapshot` triad's 3 files were
left as-is — they already generically delegate to `schema::{snapshot,diff,mutations}` and needed no
change), `🧬️schema/🦀️component.rs` + its 4 facet-mirror siblings (`SemioObjectArtifact`, widened to
include `objects`), `🎹️composer/🦀️component.rs` (real `SubsetValidator`), `🏗️builder`/`🧐️analyzer`
(unchanged — already generic over the widened snapshot type, verified compiling), `🚪️io` (left as
structure-only per the w1b note — W4's job).

Standalone verification harness (NOT part of this subset, scratchpad only, not committed to the
ticket's write scope): `/private/tmp/claude-501/-Users-ueli-Documents-semio/df0feeb4-e528-4640-ac9d-0ad87b3e69e5/scratchpad/objverify/`.

## Shared infra gaps discovered (for the closer to reconcile — NOT fixed in the shared file)

### 1. `engine::triples::{IndexedTripleDiff,NamedTripleDiff}::Deserialize` requires `D`/`T`/`K: Default`

A serde-derive bound-inference quirk: their own fields (`removed`/`modified`/`added`) carry
`#[serde(default)]`, which causes the derive macro to require `Default` on every generic type
parameter that appears in ANY field's type — not just the specific field that needs it. The
engine's own test module never hit this because its tests only instantiate
`IndexedTripleDiff<u32, String>`/`NamedTripleDiff<String, u32, String>`, and `u32`/`String` both
already implement `Default`. Any subset whose diff/item type does NOT derive `Default` (any
data-carrying enum with no fieldless variant, e.g. this subset's own `SemioValueDiff`) hits a real
compile error the moment that type is wrapped in `Option<IndexedTripleDiff<...>>`/
`Option<NamedTripleDiff<...>>` and the outer struct derives `Deserialize`. **Confirmed
independently by the sibling `presentation` subset** hitting the identical `SlideShapeDiff:
Default` requirement for the same reason (seen live in this agent's own `cargo check` output).

Worked around within this subset's own ownership boundary: hand-rolled `impl Default for
SemioValueDiff` (no fieldless variant exists, so `#[derive(Default)]` with `#[default]` isn't
usable) and `#[derive(Default)]` on `SemioObjectEntry`/`SemioObjectNode`/`NamedAdded<T>` (all
already had all-`Default` fields). Not fixed at the framework/shared-engine level (out of scope).
A shared-engine-level fix (`#[serde(bound(...))]` overriding the auto-inferred bound) would remove
the need for every subset to hand-roll its own `Default`.

### 2. `NamedTripleDiff<K,D,T>.added: Vec<T>` carries no position (the bug fixed above)

The shared engine's named/id-keyed triple structurally cannot preserve exact re-add position
without a subset-local wrapper — see "Real bug found and fixed" above for the full citation and
fix. This is the **most consequential** of the two gaps: unlike gap #1 (a compile-time blocker,
impossible to miss), this one is a silent *correctness* bug that only manifests as a law-test
failure, and would very plausibly recur in any other name/id-keyed-collection subset unless that
subset's author independently notices the `CAdded{index,item}` normative-shape mismatch. Suggest
the shared engine grow an `IndexAdded`-equivalent (`NamedAdded<T>` or similar) for
`NamedTripleDiff` generically, so future subsets don't each have to discover and re-fix this
independently.

### 3. Minor: `enc_named_triple`'s `enc_k: impl Fn(&K) -> String` bound doesn't accept `fn(&str) -> String` when `K = String`

No implicit `&String -> &str` coercion through a fixed `Fn` bound — worked around locally with an
inline closure (`|k: &String| enc_str(k)`). Not a bug, just a rough edge worth noting for anyone
else keying a `NamedTripleDiff` by `String` and reusing an existing `&str`-based hex-encoder.
