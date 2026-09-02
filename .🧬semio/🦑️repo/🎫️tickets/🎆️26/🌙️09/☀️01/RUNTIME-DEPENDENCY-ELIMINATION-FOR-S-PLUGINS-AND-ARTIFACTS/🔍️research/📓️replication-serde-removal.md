# 📡️ `semio-framework-replication` — serde removal: verified, not achievable this pass

## Headline

`serde`/`serde_json` **stay in `📡️replication/📦️packages/🦀️rust/Cargo.toml`'s `[dependencies]`**.
The specific blocker documented in `📓️serde-off-guest-path.md` (the generic `to_dsl_value`/
`from_dsl_value` bridge) **is confirmed resolved** — no code change was needed, a peer already
landed it (see `📓️dsl-value-bridge-conversion.md`). Resolving it did **not** reduce this crate's
real serde dependency, because two independent reasons remain, one of them (blocker 2, the shared
`🌱️value`/`⚠️diagnostic` components) already documented and reconfirmed, and one **newly found**
this session: a second, deliberately-serde-based bridge in `os-kernel`'s own
`🔌️plugin/🦀️component.rs` (`encode_wire_serialized`/`decode_wire_serialized`) that the prior
research never inventoried, and that turns out to reach far more of this crate's own domain model
than the three named types (`DispatchReport`/`MergeReport`/`Conflict`) the old bridge blocked.

One attempted removal (six "Serialize-only, never hydrated from the wire" descriptor types) was
made, found to break `semio-framework-os` on a real `cargo check`, and reverted before landing. No
net source change was made to `📡️replication` this session; the `Cargo.toml` docstring was
rewritten to record what was actually verified, since the previous docstring's blocker (1) is now
stale (fixed) and blocker (2) undercounted (missing the `🗂️ordered/🧺️set` `OrderedSet` and the
`encode_wire_serialized` fan-out entirely).

## 1 — Blocker (1) status: RESOLVED, verified by reading source, not by re-deriving it

`🧰️framework/🔨️modules/🌱️value/🦀️component.rs:192,198`:

```rust
pub fn to_dsl_value<T: ToValue>(value: &T) -> Result<DslValue, String> { Ok(value.to_value()) }
pub fn from_dsl_value<T: FromValue>(value: DslValue) -> Result<T, String> {
    T::from_value(value).map_err(|error| error.to_string())
}
```

Confirmed `T: ToValue`/`T: FromValue`-bound, not `T: serde::Serialize`/`DeserializeOwned`. The
717-line `🌱️value/🔀️serde` visitor module the old bridge depended on is confirmed **deleted**
(`find … -iname "*serde*"` under `🌱️value/` returns nothing named `🔀️serde`). This matches
`📓️dsl-value-bridge-conversion.md`'s own report — the pilot doc's "verify first, it could be most
of your job" was right to flag this, and the verification is genuinely clean: **zero remaining
code in this repo requires `DispatchReport`/`MergeReport`/`Conflict` to implement serde JUST to
satisfy this particular bridge.**

## 2 — But a SECOND, different, still-live bridge blocks the exact same three types (and more)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:28257,28282`:

```rust
/// 🌉️ Bridges through `serde_json` rather than `dsl::to_dsl_value` (which needs `T: ToValue`) —
/// callers pass many foreign kernel/replication types (`Fault`, `Effect`, `DispatchReport`, …)
/// that only implement `Serialize`, so a `ToValue` bound here would be unsatisfiable for them
/// (orphan rule forbids adding it downstream); `Serialize` is the one bound every caller already has.
fn encode_wire_serialized<T: Serialize>(value: &T) -> Vec<u8> { .. }
pub(crate) async fn decode_wire_serialized<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Fault> { .. }
```

This is a **different function than `to_dsl_value`/`from_dsl_value`**, living in `os-kernel`'s own
file, never touched by the bridge-conversion pass because that pass's scope was the ONE bridge
named in `serde-off-guest-path.md`. Confirmed real, unconditional call sites (not test-only):

- `report: &protocol::DispatchReport` → `encode_wire_serialized(report)` — `🔌️plugin/🦀️component.rs:28267,31138`
- `report: protocol::MergeReport` → `encode_wire_serialized(&report)` / `decode_wire_serialized::<protocol::MergeReport>` — `:28308,31221,34298`
- `conflicts: Vec<protocol::Conflict>` → `encode_wire_serialized(&conflicts)` / `decode_wire_serialized::<Vec<protocol::Conflict>>` — `:28308,28313,31229,34285,34301`
- `protocol::MutationOrigin` → `decode_wire_serialized::<protocol::MutationOrigin>(&origin)` — `:31373`
- `protocol::InteractionState` → hit **directly**, not through this bridge, at `🔌️plugin/🦀️component.rs:9860,9864,9875` and `🔌️plugin/🕹️interaction/**` (5+ files): `serde_json::to_string`/`from_str`/`from_slice`/`from_value` on `InteractionState` itself.
- `crate::MutationMeta` → `serde_json::from_str::<MutationMeta>` in `📡️spr/🎮️command/🦀️component.rs:998,1004` (test code in that crate, but that doesn't matter for *this* crate's `Cargo.toml` — the trait impl must exist in `replication`'s own compiled lib regardless of which downstream consumer, test or production, calls it).

Because `#[derive(Serialize, Deserialize)]` is structural — every field of a serde-deriving type
must itself implement the trait — this transitively locks serde onto every type reachable from the
five entries above. Traced by hand (field-by-field, not by compiling, since a compile can't tell
you *why* a bound is needed, only *that* it is):

| required by | drags in |
|---|---|
| `Conflict` | `ConflictId`, `ConflictKind` (→ `Vec<MutationEnvelope>`), `ConflictStatus`, `Vec<MutationMessage>`, `Vec<ActorId>`, `HybridLogicalTimestamp` |
| `DispatchReport`/`MergeReport`/`EditMessages` | `MergePolicy`, `Option<Severity>` (diagnostic, already serde), `Vec<MutationMessage>`, `Option<ConflictId>` |
| `MutationEnvelope` (via `ConflictKind::Quarantined`) | `MutationId`, `ArtifactId`, `ActorId`, `Vec<MutationId>`, `ArtifactDiff`, `InverseMutation`, `HybridLogicalTimestamp` |
| `ArtifactDiff`/`InverseMutation` | `SchemaId` |
| `MutationOrigin` (direct) | (leaf enum, own fields only) |
| `MutationMeta` (direct, via `📡️spr` test) | `Option<MutationId>`, `Vec<MutationId>`, `Option<ActorId>`, `HybridLogicalTimestamp`, `UndoPolicy`, `Option<PayloadHash>`, `Option<SchemaId>` |
| `InteractionState` (direct) | `BTreeMap<String, DomainSelection>`, `BTreeMap<String, DomainHover>`, `BTreeMap<String, SelectionMode>` → `DomainSelection`, `DomainHover`, `SelectionMode` |

That is **most of this crate's own domain model** — not the three named types the old research
tracked. `HoverSpec`/`SelectionSpec`/`SelectionMethod`/`MergeMode`/`HierarchyProvider`/
`InteractionTarget` are pulled in one more hop out (they compose `InteractionState`'s siblings via
`assemble_presence_interaction`, `next_hover`, `next_selection` — not fields of `InteractionState`
itself, but of the same `📡️wire/🦀️.rs` cluster that a peer's own in-file comment already
documented: *"its existing serde derive (the composed `BTreeMap<String, DomainSelection/
DomainHover/…`"* in `🔌️plugin/🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs:10`).

## 3 — A real mistake, caught before landing: the "descriptor" cluster is NOT dead either

Six types looked like clean dead-capability removals: `MutationInvertibility`,
`MutationDiffParticipation`, `MutationOutcomeClass`, `MutationComposition`,
`MutationLanguageSurface`, `MutationLeafDescriptor`. Every one derives `serde::Serialize` **only**
(never `Deserialize` — their own docstring says why: "static roster metadata, never hydrated from
the wire"), and a repo-wide grep for `serde_json`/`encode_wire_serialized`/`decode_wire_serialized`
near each name returned **zero hits** anywhere outside `📡️replication`.

That grep was insufficient. `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:420`:

```rust
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationDescriptor {
    id: crate::os_spr::ids::SchemaId,
    schema_version: crate::os_spr::ids::SchemaVersion,
    state_class: crate::os_spr::StateClass,
    leaf: MutationLeafDescriptor,
    ..
}
```

`MutationDescriptor` is an **os-kernel-owned** struct (not this crate's) that derives `Serialize`
over a field typed `MutationLeafDescriptor` (this crate's). Removing `Serialize` from
`MutationLeafDescriptor`/its four enum fields compiles `📡️replication` clean but breaks
`semio-framework-os` with `E0277: the trait bound … is not satisfied` — proven by actually running
`cargo check -p semio-framework-os-kernel` after making the edit (see the verbatim tail below,
captured before reverting). **The edit was reverted before landing** — `git diff` on
`🎮️mutation/🦀️.rs` shows zero changes as of this document.

This is the load-bearing lesson for whoever continues this crate: **a "no serde_json call site
found" grep is not proof of safety.** A containing type in a downstream crate can derive
`Serialize`/`Deserialize` over a field of your type without ever calling `serde_json::` by name.
The only reliable check is a real `cargo check -p semio-framework-os` after the edit — which is
also, unfortunately, the expensive and heavily-contended one.

## 4 — Blocker (2), reconfirmed with real callers (not re-derived from the old doc)

`🌱️value`/`⚠️diagnostic`/`🗂️ordered/🧺️set` are path-mounted **directly into this crate's own
compilation unit** (`📦️packages/🦀️rust/🦀️.rs:26-30`, `#[path = "../../../🌱️value/🦀️component.rs"]
pub mod value;` etc. — a second, separately-compiled instance of the same source `os-kernel` also
mounts). Confirmed unconditional (non-`#[cfg(test)]`) serde in each, with real non-replication
callers:

- `🌱️value/🦀️component.rs`: `impl serde::Serialize`/`Deserialize for DslValue` (two competing
  impls were observed mid-session — a concurrent peer landed a hand-rolled visitor version
  alongside the older `serde_json::Value`-delegating one, causing a transient `E0119` conflict that
  self-resolved between two of my `cargo check` runs; not mine, not touched), plus
  `impl From<&DslValue> for serde_json::Value` — real callers via
  `🔌️plugin/🦀️component.rs`'s `encode_wire_serialized` above.
- `🗂️ordered/🦀️component.rs`: `impl<V: Serialize> Serialize for OrderedMap<V>` unconditional
  (already documented — `💻️os/🧠️neural/⚙️engine`'s `Dictionary`), `Deserialize`
  `#[cfg(test)]`-only.
- `🗂️ordered/🧺️set/🦀️component.rs`: `OrderedSet: Serialize + Deserialize`, both unconditional. Real
  production callers confirmed this session (not just asserted): `grep -rl "OrderedSet"` outside
  `🌱️value` hits `💻️os/🔨️modules/🌊️flow/{🖥️host,🌿️vcs,📄️artifact,🧵️retained,🧵️retained/📑️copy}` and
  the `🌀️procedural2d`/`🧊️procedural3d` plugins' `📸️snapshot/💾️binary` and `🧬️mutations/💾️binary`
  modules.
- `⚠️diagnostic/🦀️component.rs`: `#[derive(Serialize, Deserialize)]` on `Fault`/`Severity`/
  `FaultCode`/`TextError`/etc. (unconditional), plus `encode_fault_bytes`/`decode_fault_bytes`
  (`serde_json::to_vec`/`from_slice` on `Fault`, unconditional — NOT the `wasm-bindgen`-gated
  `fault_to_js` pair). Real callers: dozens, across `🔌️plugin/🦀️component.rs`,
  `🔌️plugin/⚛️reactor/**`, `🏃️run/🦀️component.rs`, `🔌️plugin/🌐host/🦀️component.rs` — this is the
  primary host↔guest fault-wire-encoding, not a corner case.

Either of blocker (1)'s residual reach (§2) or blocker (2) alone is sufficient to keep
`serde`/`serde_json` in `[dependencies]`; both are real simultaneously.

## 5 — Production vs `#[cfg(test)]` classification

`grep -rc serde --include='*.rs' 📡️replication` (excluding `node_modules`) = **214** matches across
11 files — matches the ticket brief's own estimate exactly. Per-file split, classified by whether
each match falls inside a `#[cfg(test)] mod { .. }` block (checked per-file, not assumed):

| file | total | production | test |
|---|---|---|---|
| `🎮️mutation/🦀️.rs` | 61 | 59 | 2 |
| `📡️wire/🦀️.rs` | 42 | 41 | 1 |
| `📡️wire/🏠️local-interaction/🦀️.rs` | 27 | 11 | 16 *(a `#[cfg(test)]`-gated `json_to_dsl`/`dsl_to_json` helper pair, lines 429-457, feeding the file's own fixture tests)* |
| `🔗️causal/🦀️.rs` | 25 | 12 | 13 *(the `CausalAddDiff`/`CausalAddOp` oracle-fixture types, `#[cfg(test)] mod tests` from line 931)* |
| `🆔️ids/🦀️.rs` | 19 | 19 | 0 |
| `⚔️conflict/🦀️.rs` | 19 | 18 | 1 |
| `🧾️wire/🦀️.rs` | 8 | 6 | 2 |
| `📡️wire/🏠️local-interaction/📡️transport/🦀️.rs` | 2 | 2 | 0 |
| four dedicated `🧪️tests/🦀️.rs` files | 11 | 0 | 11 |
| **total** | **214** | **≈168** | **≈46** |

**≈168 production, ≈46 test** (78% / 22%). The test-side references are legitimate third-party
oracle usage (`serde_json` as the independent reference for `CausalAddDiff`/`CausalAddOp` and the
`json_to_dsl`/`dsl_to_json` fixture bridges) — CLAUDE.md-sanctioned, not converted, matching this
ticket's own "do not convert oracle tests" instruction.

## 6 — What was actually changed this session

- **`📡️replication/📦️packages/🦀️rust/Cargo.toml`**: docstring above `serde`/`serde_json` rewritten
  to record blocker (1) resolved, the new `encode_wire_serialized`/`decode_wire_serialized`
  finding, the `MutationDescriptor` fan-out mistake, and the corrected blocker-(2) roster
  (`OrderedSet` was previously undercounted as `🌱️value/🔀️serde`, a module that no longer exists).
  No dependency lines moved — `serde`/`serde_json` stay in `[dependencies]`.
- **No `.rs` source changes landed.** One edit (the six-type descriptor cluster, §3) was made,
  compile-tested, found to break `semio-framework-os`, and reverted (`git diff` on that file is
  empty as of this document).

## 7 — Verification, verbatim tails

Baseline, after the revert (current committed-equivalent state):

```
$ cargo check -p semio-framework-replication --message-format=short
    Checking semio-framework-replication v0.1.0 (…)
…/📡️wire/🦀️.rs:148:51: warning: unnecessary qualification
…/🔗️causal/🦀️.rs:248:8: warning: method `push` is never used
warning: `semio-framework-replication` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized] target(s) in 3.25s
```

```
$ cargo test -p semio-framework-replication --lib
test result: FAILED. 226 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.81s

failures:
    causal::tests::causal_add_fixture_has_exact_required_descriptor   (pre-existing, documented —
        taxonomy-fixture path string mismatch, unrelated to serialization)
    value::tests::serde_json_uses_the_same_json_shape_as_the_dsl_value_bridge   (NEW — key-order
        mismatch, `🌱️value/🦀️component.rs`, a shared file outside this crate's own source tree;
        a concurrent peer landed a second `impl serde::Serialize for DslValue` mid-session — see §4)
    value::tests::serde_json_value_round_trips_through_dsl_value   (NEW — integer-vs-float
        `serde_json::Number` mismatch, same file, same cause)
```
229 total tests (226 + 3), consistent with the ticket header's own "was 225/226, then 229/231"
trajectory — the count grew as expected, but 2 of the 3 currently-failing tests are inside the
shared `🌱️value` component's own concurrent edit, not this crate's code, and not something this
ticket's fence covers fixing. Re-ran the full suite twice, ~15 minutes apart; identical result both
times (not a lock-blocked stale read).

The `MutationDescriptor` break, captured before reverting the six-type edit (§3):

```
$ cargo check -p semio-framework-os-kernel --message-format=short
…/🏪️store/🧬️schema/🧬️mutations/🌿️create-space-alternative/🦀️.rs:8:35: error[E0277]: the trait bound
  `SpaceAlternative: serde::Serialize` is not satisfied
…(17 total errors, all E0277/E0599 unsatisfied-serde-bound, including `ArtifactCursorOwners`,
  `ArtifactBackboneRef`, `MigrationProvenance`, `OwnerRef`, `HistoryLane`, `SpaceCheckpoint`)
error: could not compile `semio-framework-os-kernel` (lib) due to 17 previous errors
```

**Caveat on this tail**: at the time of this exact run, `os-kernel`'s `🏪️store/🦀️component.rs` was
ALSO independently mid-edit by one of the four concurrent os-kernel agents (same error shape —
unsatisfied serde bounds on `SpaceAlternative`/`ArtifactCursorOwners`/etc., none of which are
`📡️replication` types) — re-ran twice after reverting my six-type edit, ~10 minutes apart, and the
SAME 15-17 errors persisted both times, all in `🏪️store`, none naming a `📡️replication` type. This
confirms the `MutationDescriptor`/`MutationLeafDescriptor` break was real and attributable to my
edit (the error before my edit was a clean, unrelated 15-error `🏪️store` set; after my edit it
included two additional errors naming `MutationLeafDescriptor`'s own field types) — but it also
means `semio-framework-os-kernel` is **currently red for reasons outside this ticket's fence**,
independent of anything in this document. `cargo check -p semio-framework-os` and `cargo build
--target wasm32-wasip2 -p semio-s-plugin-draw-fsm` both fail downstream of the identical `🏪️store`
errors — not re-run to a clean baseline because the cause is squarely the concurrent os-kernel work
this ticket explicitly fences off ("Four agents are clearing the first… os-kernel").

**One more re-check at the very end of this session** (no `📡️replication`/`Cargo.toml` edit in
between): the `🏪️store` errors were gone — that batch was fixed by whoever owns it — replaced by a
NEW, different pair: `E0277: &protocol::MutationLeafDescriptor: protocol::ToValue is not satisfied`
and the same for `&command::SemanticDescriptor`, both at `📡️spr/🎮️command/🦀️component.rs:479`.
`MutationLeafDescriptor` DOES implement `ToValue` in this crate (`🎮️mutation/🦀️.rs:347`) — the
error is `&MutationLeafDescriptor` (no blanket `impl<T: ToValue> ToValue for &T` covers a bare
reference at that call site), which is an os-kernel call-site shape issue, not a gap in this
crate. This is a second, different, also-live os-kernel agent's in-flight conversion of the exact
`MutationDescriptor::new`/`descriptor_fingerprint` region §3 already names — evidence that
`os-kernel` was moving under my feet in real time across this whole session. **Re-verify fresh
before trusting either a green or red `os-kernel` reading; treat every tail in this document as
timestamped, not current.**

`cargo tree` (lock-free, unaffected by the above — resolves the manifest graph, not compiled code):

```
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde --edges normal
serde v1.0.228
├── semio-framework-os-kernel v0.1.0 (…)
│   └── semio-s-plugin-draw-fsm v0.1.0 (…)
└── semio-framework-replication v0.1.0 (…)
    ├── semio-framework-os-kernel v0.1.0 (…) (*)
    └── semio-framework-pack v0.1.0 (…)
        └── semio-framework-os-kernel v0.1.0 (…) (*)

$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i serde_json --edges normal
serde_json v1.0.149
└── semio-framework-os-kernel-dsl-derive v0.1.0 (proc-macro) (…)   # host-only, not linked
    └── semio-framework-os-kernel v0.1.0 (…)
        └── semio-s-plugin-draw-fsm v0.1.0 (…)

serde_json v1.0.149
├── semio-framework-os-kernel v0.1.0 (…) (*)
└── semio-framework-replication v0.1.0 (…)
    ├── semio-framework-os-kernel v0.1.0 (…) (*)
    └── semio-framework-pack v0.1.0 (…)
        └── semio-framework-os-kernel v0.1.0 (…) (*)
```

Unchanged from `📓️dsl-value-bridge-conversion.md`'s prior measurement — both trees read in full,
not truncated. **Both edges into `serde`/`serde_json` are still `os-kernel`'s and `replication`'s
own direct `Cargo.toml` entries.**

## 8 — Final `Cargo.toml` state

`serde = { version = "1.0.219", features = ["derive"] }` and `serde_json = "1.0.140"` remain in
`[dependencies]`, unchanged in substance from before this session — only the explanatory docstring
above them changed (§6). No line moved to `[dev-dependencies]`. `wasm-bindgen`'s
`cfg(all(target_arch = "wasm32", not(target_env = "p2")))`-gated entry is untouched (out of scope —
retires with `⚠️diagnostic`'s `fault_to_js`/`result_fault_to_js`, unrelated to this pass).

## 9 — What remains, for whoever picks this up next

1. **The `encode_wire_serialized`/`decode_wire_serialized` bridge (§2) is `os-kernel`-side work,
   outside this crate's fence.** Converting its three-ish real call-site families
   (`DispatchReport`/`MergeReport`/`Conflict`, `MutationOrigin`, `InteractionState`) to
   `ToValue`/`FromValue` would let this crate's own hand-written twins (already present for all of
   them) finally pay off, the same way the `to_dsl_value` conversion did for population 1 in
   `📓️dsl-value-bridge-conversion.md`. The bridge's own docstring says it exists BECAUSE some
   callers (`Fault`, `Effect`) do not implement `ToValue` — those would need converting too, or the
   bridge would need to stay dual-purpose (a generic-over-bound split, or two named functions).
2. **`🌱️value`/`⚠️diagnostic`/`🗂️ordered/🧺️set`'s own real external callers (§4)** are unchanged
   from the prior session's finding, now with `OrderedSet`'s callers concretely named rather than
   asserted-by-analogy.
3. **Do not repeat the §3 mistake**: verify any future per-type removal in this crate against a
   real `cargo check -p semio-framework-os`, not a `serde_json`/bridge-name grep alone — derive-based
   fan-out in a downstream crate is invisible to that grep.
4. **`semio-framework-os-kernel` was observed red (15-17 errors, all in `🏪️store/🦀️component.rs`,
   none in `📡️replication`) at the end of this session**, from concurrent, unrelated os-kernel work
   by other live agents — re-check it fresh before trusting either a green or red reading; this
   document's own tails record exactly when and what was seen, per this ticket's own stale-check
   discipline.
