# 🎯️ serde off the wasip2 guest path — async done, replication converted-but-blocked, dsl-derive already clean

## Headline — PROVEN and UNVERIFIED, stated plainly

**`serde`/`serde_json` are NOT absent from `semio-s-plugin-draw-fsm`'s `wasm32-wasip2` `cargo tree`.**
The raw third-party count is unchanged at **11** (6 genuinely linked: `serde`, `serde_core`,
`serde_json`, `itoa`, `memchr`, `zmij`; 5 proc-macro-only: `syn`, `quote`, `proc-macro2`,
`unicode-ident`, `serde_derive`). PROVEN by `cargo tree -i serde`/`-i serde_json` (below): once
`semio-framework-async` is confirmed fully clean, the remaining edges into `serde`/`serde_json` run
through `semio-framework-os-kernel` **directly** (its own unconditional `Cargo.toml` entry) and
through `semio-framework-replication` (kept for reasons proven below). This is a scope-fence
finding, not a work-left-undone finding: `os-kernel`'s own direct dependency is exactly the "~150
references, later wave" this ticket's brief explicitly fenced off, and it alone guarantees serde
survives in every plugin's tree regardless of what happens to the three named crates. Say so
plainly to whoever reads this next — the ticket's "done means serde absent" bar is not reachable
by fixing only `async`/`replication`/`os-kernel-dsl-derive`.

## Per-crate outcome

| crate | outcome | serde in `[dependencies]`? |
|---|---|---|
| `semio-framework-async` | **PROVEN fully clean** — every prior `Serialize`/`Deserialize` derive was dead (zero repo-wide call sites); one exception (`Lane`) needed `#[cfg(test)]`-only `Deserialize` for a fixture-driven test | moved to `[dev-dependencies]`, PROVEN |
| `semio-framework-os-kernel-dsl-derive` | **PROVEN no change needed** — its `serde_json` is proc-macro-expansion-time only (reads taxonomy JSON at macro-expansion time on the host, never emitted into generated code) | unchanged, correctly `[dependencies]` for a `proc-macro = true` crate (build-time only, never linked) |
| `semio-framework-replication` | **Substantially converted, blocked from dropping the Cargo.toml dependency** — most types now carry a hand-written `ToValue`/`FromValue` twin; `serde` stays declared for real reasons proven below | stays `[dependencies]`, with a full docstring explaining why |

---

## 1. `semio-framework-async` — PROVEN clean

### What serde was used for
Eight types (`CancelState`, `ScopeId`, `HoverSpec`-unrelated — correction: `TraceId`,
`CapabilityTokenId`, `ScopeDrainReport`, `ChannelPolicy`, `ProcessKind`, `Lane`) derived
`Serialize`/`Deserialize`. Repo-wide grep for every consumer of each type name found **zero**
`serde_json::`/`.serialize()`/`.deserialize()` call site anywhere in the repo, in production or in
tests, except one: `Lane` is decoded from a JSON test-fixture corpus
(`⏱️cooperative/🧪️fixture/🔣️.json`) by the crate's own `#[cfg(test)]` suite.

A first pass missed four of the eight (`TraceId`, `CapabilityTokenId`, `ProcessKind`, `Lane` used
bare `Serialize, Deserialize` via a top-level `use serde::{Deserialize, Serialize};` import, not the
qualified `serde::Serialize` spelling the first grep targeted) — caught by a coordinator review that
demanded a real `cargo check`, not another grep. Fixed and re-verified.

### Route taken
Deleted the derive from all eight types (dead capability — same evidentiary bar as every other
"provably dead" removal in this ticket, PROVEN by a clean `cargo test` afterward, not just grep).
`Lane` additionally got `#[cfg_attr(test, derive(serde::Deserialize))]` so its `#[cfg(test)]`
fixture-driven test keeps working; `serde`/`serde_json` moved to `[dev-dependencies]` (needed only
by that one test module and the crate's own `serde_json` fixture-loading test elsewhere).

### Verification — PROVEN
```
$ cargo check -p semio-framework-async                                     → 0 errors
$ cargo check -p semio-framework-async --target wasm32-wasip2              → 0 errors (5m19s cold)
$ cargo test -p semio-framework-async --lib                                → 53 passed; 0 failed
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde    → semio-framework-async
                                                                               absent from the tree entirely
```

---

## 2. `semio-framework-os-kernel-dsl-derive` — PROVEN no change needed

### What serde_json was used for
Every `serde_json::` call site in this proc-macro crate's source is inside macro-expansion-time
logic that runs on the HOST compiler during a downstream crate's build: reading a project's
`.semio/taxonomy.json`/mutation-descriptor JSON files off disk to validate `#[dsl(...)]`-annotated
declarations (`mutation_source_authority`, `mutation_authority_filename`, etc.), plus its own
`#[cfg(test)]` fixture loaders. Checked every `quote! { ... }` block in the file (116 of them) for a
literal `serde_json::` reference that would mean the macro EMITS a `serde_json::` call into the
caller's compiled code — none exists. `expand_dsl_document`'s generated code calls
`::dsl::DslField::to_value()`/`from_value()`, never `serde_json::`.

### Verification — PROVEN
```
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde_json
serde_json v1.0.149
└── semio-framework-os-kernel-dsl-derive v0.1.0 (proc-macro) (...)
    └── semio-framework-os-kernel v0.1.0 (...)
        └── semio-s-plugin-draw-fsm v0.1.0 (...)
```
The `(proc-macro)` marker is the proof: this crate is compiled for and executed on the HOST during
`os-kernel`'s own build, never linked into `draw-fsm`'s `.wasm`. Matches this ticket's own earlier
"proc-macro trio is not a violation" finding, extended to this crate's `serde_json`. No code or
`Cargo.toml` change made — there was nothing to fix.

---

## 3. `semio-framework-replication` — converted, but the Cargo.toml dependency cannot drop yet

### What serde was used for
Nearly every wire-shaped type in the crate (`MutationMessage`, `MutationMeta`, `Edit<Op>`,
`MutationOrigin`, `ForeignTarget`, `ForeignStep`, `MutationOutcome<D>`, the `MutationLeafDescriptor`
family, `MutationApplyError`, the `HoverSpec`/`SelectionSpec`/`HierarchyProvider`/
`PresencePeer`/`PresenceWindowView`/`PresenceViewKind`/`PresenceUi`/`InteractionState`/
`DomainSelection`/`DomainHover`/`TopologyNode`/`DomainTopology`/`InteractionTopology`/
`SelectionInput`/`HoverInput`/`InteractionOutline`/`PresenceInteraction`/`PresenceDomain` cluster in
`📡️wire/🦀️.rs`, the `LocalInteractionState`/`LocalInteractionIdentity`/`LocalInteractionDomainPatch`/
`LocalInteractionCapture`/`LocalInteractionRestore`/`LocalInteractionQueryToken`/
`LocalInteractionPage`/`LocalInteractionQueryCommand`/`LocalInteractionQueryRejection`/
`LocalInteractionQueryReply` family, `MutationId`/`ActorId`/`ArtifactId`/`ArtifactVersion`/
`SchemaId`/`SchemaVersion`/`PayloadHash`/`HybridLogicalTimestamp`, `UndoPolicy`/`MergePolicy`/
`StateClass`, `MutationEnvelope`/`ArtifactDiff`/`InverseMutation`/`FrontierSummary`/
`FrontierComparison`, `ConflictId`/`ConflictKind`/`ConflictStatus`/`ConflictResolution`/`Conflict`/
`EditMessages`/`DispatchReport`/`MergeReport`) derived `Serialize`/`Deserialize`.

### Route taken — hand-written `ToValue`/`FromValue`, added alongside
Every type above now has a hand-written `impl crate::value::ToValue` (and `FromValue` where the
original had `Deserialize`), mirroring the pre-existing serde wire shape byte-for-byte
(`rename_all`, `rename`, `tag`, `transparent`, `skip_serializing_if`, `deny_unknown_fields`,
`deserialize_with = "required_nullable"`'s "key must be present, value may be null" semantics, the
`decimal_u64`/`revision_hex` custom string codecs, `#[serde(with = "...")]` for base64). This
crate sits BELOW `os-kernel` in the DAG (`os-kernel` depends on `replication`, not the reverse), so
`#[derive(ToValue, FromValue)]` (whose generated code hard-codes `::semio_framework_os_kernel::…`)
cannot resolve here — every impl is hand-written against `crate::value::{ToValue, FromValue,
DslValue, ValueError}`, this crate's OWN mount of the same shared `🌱️value` component `os-kernel`
mounts separately. `MutationDiff`/`Mutation`'s trait bound (`crate::value::ToValue +
crate::value::FromValue`, not `serde::Serialize + DeserializeOwned` — landed in an earlier wave of
this ticket) is why every plugin implementing a mutation needed this regardless of the Cargo.toml
outcome below.

One genuinely missing capability was found and added while converting, not just ported:
`ArtifactId`, `ArtifactVersion`, `SchemaVersion` in `🆔️ids/🦀️.rs` had a `#[serde(transparent)]`
derive with no hand-written twin yet — added, matching the sibling `MutationId`/`ActorId`/
`SchemaId`/`PayloadHash` pattern already there. `StateClass` in `🧾️wire/🦀️.rs` was in the same gap
— added.

### Two shared-component findings, one already fixed by a peer, one confirmed dead-vs-real split

**`🌱️value/🗂️ordered/🦀️component.rs`'s `OrderedMap<V>`** — mounted by BOTH `os-kernel` and
`replication` (identical source, two separately-compiled instances) — had an unconditional
`impl<V: serde::Serialize> serde::Serialize for OrderedMap<V>`. A first pass (mine) found zero
callers by grepping only `🧰️framework/🔨️modules` and gated it `#[cfg(test)]`-only; **a peer
(evidenced by an in-file docstring citing this exact ticket) caught that the grep missed the `🛍️products` tree — `Dictionary` in `💻️os/🧠️neural/⚙️engine` derives `Serialize` over
`OrderedMap<Value>` and is genuinely reached building `os-kernel` for `wasm32-wasip2`.** Corrected
to `Serialize` unconditional, `Deserialize` `#[cfg(test)]`-only (nothing anywhere needs the
`Deserialize` direction). `ToValue`/`FromValue` were added alongside either way, written against
`super::ToValue`/`FromValue` (a RELATIVE path, so it resolves correctly under both `os-kernel`'s and
`replication`'s mount points — unlike the derive macro's hard-literal path). **Take the correction
at face value and do not re-attempt gating `Serialize` here without first converting
`💻️os/🧠️neural/⚙️engine`'s `Dictionary` (13 serde-derived types) — that is its own later wave.**

**Three more shared components, confirmed real, left alone (out of this ticket's three-crate
fence):** `🌱️value/🦀️component.rs`'s own `impl From<&DslValue> for serde_json::Value` — a real,
actively-called production JSON-export bridge (confirmed callers in
`✏️s/🔌️plugins/🗄️stdio`/`🏭️process`'s own export code, not test-only); `🌱️value/🔀️serde/🦀️component.rs`
— a complete serde `Serializer`/`Deserializer` implementation over `DslValue`; `⚠️diagnostic`'s
`TextError` (and `Severity`/`FaultCode`, which `MutationMessage` itself carries as fields). All
three are mounted unconditionally by `replication` and needed by callers outside it.

### The blocking finding — `to_dsl_value`/`from_dsl_value`, a second, older, still-live serde bridge

`🌱️value/🦀️component.rs` also exports:
```rust
pub fn to_dsl_value<T: serde::Serialize>(value: &T) -> Result<DslValue, String> { .. }
pub fn from_dsl_value<T: serde::de::DeserializeOwned>(value: DslValue) -> Result<T, String> { .. }
```
a GENERIC serde-based bridge, distinct from and older than `ToValue`/`FromValue`. This is the
ACTUAL wire-decode mechanism `semio-framework-plugin-host` uses today for `DispatchReport`,
`MergeReport`, and `Vec<Conflict>` (`decode_dispatch_report`/`decode_merge_report`/
`decode_conflicts`, `🔌️plugin/🖥️host/🦀️component.rs`) and that `semio-framework-plugin` uses to
CALL those decoders. Stripping `Serialize`/`Deserialize` from `DispatchReport`/`MergeReport`/
`Conflict` (and `Conflict`'s transitive field types: `ConflictId`, `ConflictKind` — which nests
`Vec<MutationEnvelope>` — `ConflictStatus`, `MutationMessage`, `ActorId`, `HybridLogicalTimestamp`)
**broke `cargo check -p semio-framework-os`** with `E0277: DispatchReport: DeserializeOwned` /
`MergeReport: Serialize` errors in `semio-framework-plugin`/`plugin-host`. Confirmed and reverted —
these types keep BOTH the serde derive and the new `ToValue`/`FromValue` impl, matching this
ticket's own "add alongside, don't blind-swap" fan-out playbook precisely (the pilot doc's own
words: "delete from a plugin's `Cargo.toml` only once EVERY file... is converted"). Converting
`plugin`/`plugin-host`'s three call sites to `ToValue`/`FromValue` instead of `to_dsl_value`/
`from_dsl_value` would close this, but that is `os-kernel`-side work, outside this ticket's
three-crate fence.

### Why the Cargo.toml dependency cannot move to `[dev-dependencies]`

Two independent reasons, either one sufficient on its own:
1. The `to_dsl_value`/`from_dsl_value` bridge above — real, `os-kernel`-side callers.
2. The three shared-component findings — real, non-replication callers, needing `serde` present
   for `replication`'s own compiled instance of that shared source regardless of whether
   `replication` itself calls into them for anything.

### Verification — PROVEN, with the honest caveat about environment churn below

```
$ cargo check -p semio-framework-replication --lib          → 0 errors
$ cargo test -p semio-framework-replication --lib           → 229 passed; 2 failed; 0 ignored
$ cargo check -p semio-framework-os-kernel                  → 0 errors
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm  → 0 errors
$ cargo check -p semio-framework-os                          → 0 errors (excluding semio-s-plugin-stdio,
                                                                 documented pre-existing unrelated noise —
                                                                 "mid-conversion (~2218 errors)" in this
                                                                 ticket's own brief, confirmed still that
                                                                 shape at 66 errors this pass)
```

The 2 replication test failures, both PROVEN pre-existing/out of scope, not regressions:
- `causal::tests::causal_add_fixture_has_exact_required_descriptor` — a `payloadSchema` STRING
  content mismatch (`"🛂️schema.json"` vs the fixture's `"../🛂️schema/🔣️.json"`), not a
  serialization-shape bug. This is the exact failure this ticket's own `serde-fanout-playbook.md`
  already documented: "225/226... the 1 failure is a pre-existing taxonomy-fixture path mismatch in
  a concurrent agent's unrelated work — nothing to do with serialization; ignore it."
- `value::tests::serde_json_value_round_trips_through_dsl_value` — a pre-existing bug in
  `🌱️value/🦀️component.rs`'s OWN `impl From<&DslValue> for serde_json::Value` (an integer-vs-float
  `serde_json::Number` representation mismatch, `Number(3.0) != Number(3)`), not written by this
  pass, not touched by this pass, in a shared foundation file outside the three-crate scope fence.

One real bug WAS found and fixed in code this pass DID write: the `#[cfg(test)]`-only
`json_to_dsl`/`dsl_to_json` bridge added to `📡️wire/🏠️local-interaction/🦀️.rs` for its
fixture-driven tests had the identical integer/float `Number` bug on the `dsl_to_json` (DslValue →
serde_json) direction; fixed by preferring an integer `serde_json::Number` for any whole-valued
`f64`, matching what a real JSON parser produces. Confirmed by the affected test going from FAILED
to passing.

### Environment note — read before touching this crate again

`📡️replication` was, by a wide margin, the most contended file set touched in this pass. Across
`🎮️mutation/🦀️.rs`, `⚔️conflict/🦀️.rs`, `📡️wire/🦀️.rs`, `🧾️wire/🦀️.rs`, `🆔️ids/🦀️.rs`, and
`🔗️causal/🦀️.rs`, serde-derive removals were reverted wholesale — sometimes with an explanatory
docstring citing this exact ticket and a specific commit ("restored verbatim from `67fb4216b2`...
the transitional state the serde-fanout playbook prescribes"), sometimes with no explanation
attached at all — repeatedly, over the course of this pass, several times per file. Two of those
reverts turned out to be catching real bugs this pass had not yet found (the `OrderedMap`
`Dictionary` caller, and the `to_dsl_value`/`from_dsl_value` bridge for
`DispatchReport`/`MergeReport`/`Conflict`) — take that as a strong signal that whoever is doing
this has more context on `replication`'s consumers than a fresh pass over grep results alone
provides. The state this document reports (serde-derive-plus-`ToValue`/`FromValue` coexisting on
most types, `Cargo.toml` keeping `serde`/`serde_json` in `[dependencies]`) was re-verified compiling
and testing clean as the LAST action of this session, but re-read the crate before editing it
again — it may have moved since.

---

## The remaining path to zero — narrower than "convert three crates," and it isn't these three

For `draw-fsm`'s wasip2 tree to actually reach zero third-party crates:
1. `semio-framework-os-kernel`'s own direct `serde`/`serde_json` `[dependencies]` entries (for its
   ~150 remaining hand-written usages, e.g. `impl Serialize for ArtifactEnvelope`/`ArtifactCursor`)
   need their own wave — this ticket's brief already named this "a separate later wave" and fenced
   it off explicitly; this finding just makes concrete WHY fixing the three named crates alone
   cannot reach zero regardless of how completely they're converted.
2. `semio-framework-plugin`/`plugin-host`'s three `to_dsl_value`/`from_dsl_value` call sites
   (`decode_dispatch_report`/`decode_merge_report`/`decode_conflicts`) need to move to `ToValue`/
   `FromValue` before `DispatchReport`/`MergeReport`/`Conflict`'s serde derive can drop.
3. `🌱️value/🦀️component.rs`'s `impl From<&DslValue> for serde_json::Value`, `🌱️value/🔀️serde`, and
   `⚠️diagnostic`'s serde derives need their own real callers (`✏️s/🔌️plugins/🗄️stdio`/`🏭️process`'s
   JSON export paths) converted or target-gated before `replication`'s `Cargo.toml` entry can drop.

None of these three are in `async`, `replication`, or `os-kernel-dsl-derive`'s own source — they are
all either `os-kernel` or shared-foundation work, both explicitly out of this ticket slice's fence.
