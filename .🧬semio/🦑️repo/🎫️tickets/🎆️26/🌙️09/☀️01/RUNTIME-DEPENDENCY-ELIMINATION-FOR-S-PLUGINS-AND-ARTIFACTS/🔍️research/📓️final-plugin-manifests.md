# Final Plugin Manifests — the last six serde-carrying manifests

Batch: the six manifests `📓️verified-outcomes.md` names as the ticket's remaining real violations —
`➗️mathematical`, `🔋️energy`, `📖️playbook/🧩️extensions/🌀️procedural`, `🌀️procedural`, `🏗️fem`, `🗄️stdio`.
Companion reading: `📓️serde-fanout-playbook.md` (mechanical recipe), `📓️serde-fanout-cad-math-energy.md`
and `📓️serde-fanout-fem-process.md` (predecessor state for these exact crates).

## Headline

**0 of 6 manifests are fully third-party-free.** Two (`📖️playbook/🧩️extensions/🌀️procedural`, `🏗️fem`)
now carry only a single, narrow, *permanently necessary* remainder each, down from a much larger
surface. `➗️mathematical` went from "additive, ~53 types still deriving serde" to two precisely
enumerated, structural blockers. `🔋️energy` gained one real fix plus a major new finding: its
manifest cannot ever reach zero under the current architecture (see below) — this corrects the
prior session's optimistic "serde-free in production paths" claim. `🌀️procedural` and `🗄️stdio` were
re-measured, not touched — both are genuinely too large for this session (187 files / 1229 sites,
and 573 files / 7084 sites respectively).

**A fleet-wide discovery, not scoped to this batch**: `semio_framework_plugin::app_commands!`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:10553`) unconditionally derives
`::serde::Serialize, ::serde::Deserialize` on every `Command` enum it generates — hardcoded in the
`macro_rules!` body, not opt-out. **156 files repo-wide invoke this macro.** Any plugin whose
manifest claims zero-serde while using `app_commands!` either (a) never actually compiled after its
serde strip, or (b) is quietly wrong. Two real instances of (a) were found and fixed this session
(`➗️mathematical`, `🏗️fem` — see below); the other 154 files were not audited, out of this batch's
scope, but the pattern is worth a fleet-wide grep before anyone else's manifest is declared clean:

```bash
grep -rl "app_commands!" ✏️s | wc -l   # 156
```

---

## `📖️playbook/🧩️extensions/🌀️procedural` — serde REMOVED, serde_json retained (genuine blocker)

**Converted this session** (the crate was previously fully additive, `serde`+`serde_json` in
`[dependencies]`, ~19 real `serde_json::` call sites):
- `ModuleRenderPayload`, `ModulePayloadMutation`, `ModulePayloadDiff`: `Serialize`/`Deserialize`
  derives and matching `#[serde(...)]` attrs removed; `ToValue`/`FromValue` already present.
- `Command` enum: `Serialize`/`Deserialize` removed (nothing required them).
- All JSON manipulation (`apply_flow_params`, `evaluated_preview_payload`,
  `evaluated_preview_geometry_handles`, `handle_export_solid`/`handle_import_solid`, the params
  UI, `module_action`, `default_payload`) moved from `serde_json::{Value, Map, json!}` to
  `pack::{JsonValue, JsonObject, json!}` (`semio_framework_pack`'s first-party JSON tree, which
  mirrors `serde_json`'s ergonomics closely enough that this was a near-mechanical swap — see
  `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs`).
- The one `serde_json`-comparison test (`module_payload_value_codec_matches_serde_json`) was
  rewritten as a self-consistency round-trip (`module_payload_value_codec_round_trips`) — no oracle
  needed once the type no longer derives `Serialize`.
- Three `app.handle_action(..., Some(&json!(...)), ...)` call sites in tests were **latently broken
  before this session** (passing a `serde_json::Value` where `PluginApp::handle_action` requires
  `Option<&DslValue>` — confirmed by reading the trait signature directly,
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:11581`); fixed to
  `json_to_dsl_value(&pack::json!(...))`.
- Two `serde_json::to_string(&UiNode)` label-content assertions converted to `format!("{node:?}")`
  (Debug), matching the precedent in `📓️serde-fanout-cad-math-energy.md` for framework UI types
  that haven't gained `ToValue` themselves.

**Framework addition landed as part of this work** (benefits every future batch, not just this one):
`pack::json::Value::pointer(&self, pointer: &str) -> Option<&Value>` — an RFC 6901 JSON Pointer
lookup mirroring `serde_json::Value::pointer`, previously missing from `pack::json`. Added at
`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` (used later this session by several of
`➗️mathematical`'s fixture-test conversions, see below).

**Genuine remaining blocker — `serde_json` stays, `serde` does not**: `flow::playbook::visible_blocks`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs:277`) is hard-typed to
`&serde_json::Map<String, serde_json::Value>`, and `FlowFixture` (the fixture format this module
parses) only implements `serde::{Serialize, Deserialize}`, not `ToValue`/`FromValue` — both are
`🌊️flow` framework module surface, one of the three areas this ticket's brief names as another
agent's concurrent in-flight wave. Bridged with a throwaway `serde_json::Map` built via
`serde_json::from_str(&json_to_string(...))` at the one call site that needs it
(`render_params_body`). `serde` (base) is genuinely unnecessary now — only `serde_json` remains,
and only for this one crossing.

**Cargo.toml**: `serde` line removed; `serde_json` retained with an inline comment naming the exact
blocker; `pack` added as a new dependency (path depth 7×`../`, resolve-checked with `ls -d`).

**Verification**: WRITTEN BUT UNVERIFIED. `cargo check -p semio-s-plugin-playbook-procedural` never
reaches this crate — it fails first inside `semio-framework-ui` (baseline, pre-existing this
session) and, after that cleared, inside `semio-framework-plugin` itself (28 errors, all
`PackageDescriptor`/`MediaType`/`ViewModel`/`CapabilityRequirement`/etc. missing `ToValue`/
`FromValue` — a live, unrelated, in-progress framework wave; grep-confirmed zero of these errors
name anything in this batch). Every API used (`pack::json::*`, `pack::json_to_dsl_value`,
`pack::json_from_dsl_value`, `.pointer()`) was traced against the actual framework source, not
assumed.

---

## `🏗️fem` — serde reduced to ONE genuine, narrow, permanent surface — and a real latent bug found and fixed

**Starting state (measured fresh, not from the predecessor doc)**: outside `🧪️oracle`/`🔬️probes`/
`🏭️generator`/`🧫️fixtures`, **zero** files referenced `serde`/`serde_json` at all — the predecessor
wave's 905-call-site conversion had, in fact, fully landed. The manifest already carried no
`serde`/`serde_json` line. On paper, done.

**Real bug found**: `semio_framework_plugin::app_commands!` unconditionally derives
`::serde::Serialize, ::serde::Deserialize` on the `Fem2dCommand`/`Fem3dCommand` enums it generates
(`✏️editor/🦀️component.rs`, both artifacts). Every one of the **37** command payload structs under
`✏️editor/🎮️commands/*` (18 fem3d + 19 fem2d) had had `Serialize`/`Deserialize` stripped by the
predecessor's conversion pass along with everything else — which means `Fem2dCommand`/
`Fem3dCommand`'s own derive could never have compiled. `cargo check -p semio-s-plugin-fem` was
never run last session (explicitly logged as skipped, "no reason to spend a lock-contended check"),
so this was never caught. **This crate's "clean" manifest was not proven by a passing build.**

**Fixed this session**: restored `Serialize, Deserialize` (alongside the existing `ToValue,
FromValue`) on all 37 command payload structs, plus the two custom types they compose as fields
(`FemDof` — a hand-written-codec bare-string enum, `🗿️artifacts/◻2d/🦀️component.rs` — and
`FemCombinationTerm`, a plain record in the same file). Traced the field graph of every one of the
37 structs by hand (`grep`-extracted every `pub <field>: <Type>`, resolved every non-primitive type)
to confirm this is the *exact and complete* set — nothing over- or under-restored. 12 of the 37
needed a matching `#[serde(rename_all = "camelCase")]` beside their existing `#[value(rename_all =
"camelCase")]`; the rest have no container/field attributes at all.

**Cargo.toml**: `serde = { workspace = true }` added back (no `serde_json` — no JSON-text call site
survives anywhere in the crate; verified by grep for `serde_json::` returning zero hits), with an
inline comment stating the exact reason and file set so nobody "cleans it up" again without reading
why.

**Verification**: WRITTEN BUT UNVERIFIED — same shared `semio-framework-plugin` blocker as above (28
errors, none naming `fem`/`Fem2d`/`Fem3d`). The fix itself is high-confidence: every restored type
was traced against real field declarations, not guessed, and the same `app_commands!` macro body
was read directly from framework source to confirm the exact derive it emits.

**Flag for whoever audits the other 154 `app_commands!` files**: this exact bug class (a plugin's
own conversion pass strips `Serialize`/`Deserialize` from `Command` payload types without noticing
the macro that wraps them still demands it) is plausible anywhere serde was removed without a
completed `cargo check`. `➗️mathematical` (below) had the identical bug, independently confirmed.

---

## `➗️mathematical` — from "additive, ~53 types" to two enumerated, structural blockers

**Starting state**: `serde`/`serde_json` in `[dependencies]`; every production type still derived
both `Serialize`/`Deserialize` alongside `ToValue`/`FromValue` (the pilot's "additive, not yet
stripped" interim shape, per `📓️serde-fanout-cad-math-energy.md`). 15 per-mutation regression-test
fixture files (`🧬️mutations/*/🧪️tests/<case>/component.rs`) were **not** converted at all —
undiscovered by the prior session's own accounting (which enumerated ~20 different files: dispatch
enum, diff, io/editor — never these).

**Converted this session**:
1. **The stdio `JsonSnapshot` import blocker eliminated, not just documented.** `🚪️io/📥️import/…/
   json/…/🦀️component.rs`'s read direction decoded straight into `MathematicalSnapshot` via
   `serde_json::from_value::<MathematicalSnapshot>(json.to_serde_value())`. The framework's reverse
   bridge `impl From<&serde_json::Value> for DslValue`
   (`🧰️framework/🔨️modules/🌱️value/🦀️component.rs:134`, landed by an earlier `🏭️process`/`🏗️fem`
   wave) makes this avoidable: rewritten to `DslValue::from(&json.to_serde_value())` then
   `MathematicalSnapshot::from_value(...)`. `serde_json` is still needed for the `JsonSnapshot`
   half (foreign stdio API), but `MathematicalSnapshot: Deserialize` is no longer required by
   *production* code anywhere — a strictly smaller blocker than previously believed.
2. **The `"wireOmission"` differential-oracle test converted to a first-party self-consistency
   check** (top-level `🦀️component.rs`) — `left.to_value()`/`right.to_value()`/
   `MathematicalSnapshot::from_value(...)` replace the `serde_json::to_value`/`from_value` triple.
   This was the *last* thing requiring `Serialize`/`Deserialize` on `MathematicalSnapshot` itself.
3. **All 15 mutation-fixture test files converted** to `pack::{from_json_str, json_from_dsl_value,
   parse_json, json!}`, mirroring the pattern above. Canonical-JSON equality assertions
   (`assert_eq!(reencoded, original, …)`) were upgraded to
   `pack::json::value_eq_ignoring_object_order` (an order-insensitive structural comparison already
   built by the `🏭️process` predecessor wave) rather than left as literal `assert_eq!`, since
   `pack::json::Object` is insertion-order (Vec-backed) while `serde_json::Map` without
   `preserve_order` is key-sorted — a naive swap could have introduced spurious key-order test
   failures on committed fixtures never regenerated against the new codec.
4. **`Serialize`/`Deserialize` mechanically stripped from every type whose only remaining reason for
   them was the additive-interim state**: `MathematicalMutation` (dispatch enum), `MathematicalDiff`,
   `MathematicalSnapshot`, `MathematicalArtifact`, `EquationNodeLabel`, all 15 mutation-leaf structs,
   `MathematicalInference`/`MathematicalRoot`/`MathematicalTopology`, the presence/config schema
   pairs — 32 files touched by a derive-line script (remove `Serialize`/`Deserialize` tokens only
   from derive lines that also carry `ToValueDerive`/`FromValueDerive`, delete the paired
   `#[serde(...)]` line when a `#[value(...)]` line immediately follows it, drop the `use
   serde::{...}` import once nothing in the file needs it any more) plus 4 hand-written-impl types
   fixed by hand (`MathematicalSnapshot`, `MathematicalArtifact`, `MathematicalDiff`,
   `EquationNodeLabel` — these have hand-written `ToValue`/`FromValue`, so the script's "only touch
   derive lines that already carry `ToValueDerive`" safety check correctly skipped them).
5. **A real bug in step 4's first pass, found and fixed before it could break anything**: the
   mechanical strip touched `MathematicalCamera`/`MathematicalPoint`/`MathematicalGeometry` and all
   7 `✏️editor/🎮️commands/*` payload structs (`SetArtifact`/`SetAlgorithm`/`SetDirected`/
   `NodeGraphEdit`/`NodeGraphViewport`/`SetPoints`/`SetLocale`) — the SAME `app_commands!` bug class
   documented above for `🏗️fem`, caught here by reading `✏️editor/🦀️component.rs`'s own
   `app_commands!` invocation and its doc comment ("`app_commands!` unconditionally derives
   `Serialize`/`Deserialize`… even though its actual wire codec is `dsl::DslOps`") *before* trusting
   the strip. All 10 types restored, with an inline comment explaining why, at both the type
   definitions and the manifest.

**Genuinely remaining, permanent, two independent blockers — precisely enumerated, not a "some
files aren't done yet" hand-wave**:
1. `serde` — the `app_commands!` macro constraint above: `MathematicalCamera`/`MathematicalPoint`/
   `MathematicalGeometry` + all 7 command payloads. Every OTHER production type in the crate is
   `ToValue`/`FromValue`-only now.
2. `serde_json` — `🚪️io/📥️import/…/json/…/🦀️component.rs`'s `JsonSnapshot::from_value`/
   `.to_serde_value()` calls, hard-typed to `serde_json::Value`, a foreign `🗄️stdio` API
   (`🗄️stdio`'s own ~563-file deferred wave). The sibling export/json leaf needs neither serde crate
   at all (confirmed clean by grep).

**Cargo.toml**: both lines retained, each with an inline comment naming the exact blocker instead of
a generic "not yet converted" note.

**Verification**: WRITTEN BUT UNVERIFIED — blocked by the same shared `semio-framework-plugin`
in-flight breakage (28 errors, grep-confirmed zero name `mathematical`/`Equation`/anything in this
crate). Every API and trait bound cited above (`app_commands!`'s literal derive line, `MutationLeaf`/
`MutationKind`/`Mutation`/`MutationDiff`'s current (already-migrated) trait bounds, the `DslValue`
reverse bridge) was read directly from framework source, not assumed from memory or from the
predecessor docs.

---

## `🔋️energy` — one real fix; a major, previously-unrecognized structural blocker confirmed

**Converted this session**: the one remaining unconverted test-fixture file
(`🧬️schema/🧬️mutations/♻️replace-model/🧪️tests/degrades-an-empty-model-payload-to-a-no-op/
🦀️component.rs`) — the sole file in the crate lacking `ToValue`/`FromValue` alongside its `serde`
usage. Converted to `pack::{from_json_str, json_from_dsl_value, parse_json}` plus
`value_eq_ignoring_object_order` for the canonical-JSON checks, matching the `➗️mathematical`
pattern exactly (same predecessor-established shape).

**Corrects the prior session's own summary** (`📓️serde-fanout-cad-math-energy.md`: "serde-free in
production paths"): **`🔋️energy` cannot reach zero-serde under the current architecture, full stop,
not just "not yet converted."** `🗿️artifacts/🔋️model/🦀️component.rs`'s
`energy_structure_from_model`/`energy_model_from_structure` — the *deliberately-kept* `Model` ↔
`SemioValue` bridge, documented by the predecessor as an intentional int/float-fidelity exception —
calls `serde_json::to_value(model)`/`serde_json::from_value(...)` **directly on `&crate::model::
Model`**. `Model`'s own struct derives `Serialize, Deserialize` and has **30+ direct fields**
(`Site`, `Zone`, `Space`, `Surface`, `Fenestration`, `Material`, `Construction`, `PeopleGain`,
`LightingGain`, `EquipmentGain`, `Thermostat`, …) spanning nearly the entire ~44-file BEM engine
tree. Every one of those field types therefore *also* needs `Serialize`/`Deserialize`, transitively,
for `Model`'s own derive to compile. This is not "2 documented exceptions" (the earlier session's
framing) — it is the crate's entire engine domain model, pinned to `serde` by one production
function this session did not touch (the fidelity argument for keeping it is sound and was not
revisited).

**Consequence**: `🔋️energy`'s manifest keeping `serde`/`serde_json` is not an interim state to chip
away at file-by-file — it is structural, permanent, under the current architecture, exactly the
same class of blocker `➗️mathematical` has via `app_commands!`, just with a much larger blast
radius (the whole engine tree vs. 10 types). Resolving it for real would mean either (a) rewriting
the `SemioValue` bridge to preserve int/float fidelity through `DslValue` — which `DslValue::Number`
cannot currently do (`f64`-only, no signed/unsigned/float distinction), a real framework change, or
(b) accepting the fidelity loss, which the predecessor explicitly ruled out as "a silent
behavior/fidelity regression." Neither is in scope for a manifest-cleanup batch.

**Cargo.toml**: unchanged (`serde`, `serde_json` both `{ workspace = true }`) — correctly, now for a
precisely stated reason rather than an open-ended "not yet converted."

**Verification**: the one converted file was traced against `EnergyModelSnapshot`/
`EnergyModelMutation`/`EnergyModelDiff`/`crate::model::Model`'s already-confirmed `ToValue`/
`FromValue` impls (all four already had them, unaffected by this session's `Model`-tree finding —
`FromValue`/`ToValue` were never the issue, only the co-existing `Serialize`/`Deserialize`
requirement is permanent). WRITTEN BUT UNVERIFIED by a passing `cargo check` — same shared
framework blocker.

---

## `🌀️procedural` — re-measured, not touched (too large for this session)

384 `.rs` files outside fenced dirs; **187 files reference `serde`, 1229 `serde_json::` call
sites** — matches the predecessor's ~1277 estimate closely; this session's count is exact (fresh
`grep`, not carried forward). Also confirmed to use `app_commands!` (multiple files under
`✏️editor/…`), so — per the `➗️mathematical`/`🏗️fem` finding above — even a hypothetical full
conversion would still need `serde` for its own `Command` enum's payload types. Left untouched:
`Cargo.toml` unchanged (`serde.workspace = true`, `serde_json` with `float_roundtrip`). No source
files touched. This is its own multi-session wave, not a slot in this batch, exactly as the tail
predecessor doc already concluded before this session started — now confirmed with a fresh count
and the additional `app_commands!` finding.

## `🗄️stdio` — re-measured, not touched (too large for this session)

3912 `.rs` files outside fenced dirs; **583 files reference `serde`, 7084 `serde_json::` call
sites**. Consistent with `verified-outcomes.md`'s own "~563 real call-site files" estimate (this
session's file count differs slightly — 583 vs ~563 — likely fixture/test files the earlier
estimate excluded differently; not reconciled, not material to the conclusion). Not started this
session — an order of magnitude beyond what remained of the time budget after `➗️mathematical`/
`🔋️energy`/`🏗️fem`/`📖️playbook`'s procedural extension. `Cargo.toml` unchanged
(`serde`/`serde_json` both `{ workspace = true }`).

---

## Final Cargo.toml state — verbatim dependency lines

```toml
# ➗️mathematical/📦️packages/🦀️rust/Cargo.toml
pack = { path = "...", package = "semio-framework-pack" }
serde = { workspace = true }        # app_commands! (10 types) — see inline comment
serde_json = { workspace = true }   # stdio JsonSnapshot import blocker — see inline comment

# 🔋️energy/📦️packages/🦀️rust/Cargo.toml
pack = { path = "...", package = "semio-framework-pack" }
serde = { workspace = true }        # Model<->SemioValue bridge, whole engine tree — see inline comment
serde_json = { workspace = true }   # same bridge

# 📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml
pack = { path = "...", package = "semio-framework-pack" }   # NEW this session
# serde REMOVED this session
serde_json = "1.0.140"              # flow::playbook::visible_blocks / FlowFixture — see inline comment

# 🌀️procedural/📦️packages/🦀️rust/Cargo.toml — UNCHANGED
serde.workspace = true
serde_json = { workspace = true, features = ["float_roundtrip"] }

# 🏗️fem/📦️packages/🦀️rust/Cargo.toml
serde = { workspace = true }        # app_commands! (37 command payloads + 2 nested types) — NEW this session
# serde_json NOT added — no JSON-text call site anywhere in the crate

# 🗄️stdio/📦️packages/🦀️rust/Cargo.toml — UNCHANGED
serde = { workspace = true }
serde_json = { workspace = true }
```

**Net manifest delta this session**: `📖️playbook/🧩️extensions/🌀️procedural` lost `serde` (kept
`serde_json`, gained `pack`). `🏗️fem` gained `serde` back (a correction, not a regression — it was
never validly absent). `➗️mathematical`/`🔋️energy`/`🌀️procedural`/`🗄️stdio` unchanged at the
manifest level, though `➗️mathematical`'s and `🔋️energy`'s *reasons* for keeping them are now exact
rather than open-ended.

---

## Verbatim tail of every verification attempt this session

All `cargo check` commands were run in the foreground, one at a time, no `CARGO_TARGET_DIR`
override, per the ticket's own hard constraints. In chronological order:

1. **Baseline, before any edit this session** — `cargo check -p semio-s-plugin-playbook-procedural`:
   ```
   error[E0277]: the trait bound `DslValue: serde::Serialize` is not satisfied
     --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:14:39
   ... (14 errors total, all in semio-framework-ui, zero naming this batch)
   error: could not compile `semio-framework-ui` (lib) due to 14 previous errors; 50 warnings emitted
   ```
   Confirmed pre-existing (baseline, before this session's edits), confirmed unrelated (`DslValue`
   itself has never derived `Serialize`; `ActionDescriptor`/`StyleSpec` in that wgpu module do — the
   "generic `to_dsl_value` bridge" area this ticket's brief names as another agent's).

2. **`cargo check -p semio-s-plugin-fem`, first attempt** (before the `app_commands!` fix) — same 14
   `semio-framework-ui` errors, unrelated.

3. **`cargo check -p semio-s-plugin-fem`, second attempt** (after `semio-framework-ui`'s blocker
   cleared on its own between attempts, confirming it was a live peer edit, not a permanent break):
   ```
   error[E0277]: the trait bound `PackageDescriptor: ToValue` is not satisfied
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/🦀️component.rs:144:58
   ... (42 errors total, all in semio-framework-plugin, zero naming fem/mathematical/energy)
   error: could not compile `semio-framework-plugin` (lib) due to 42 previous errors; 112 warnings emitted
   ```

4. **`cargo check -p semio-s-plugin-mathematical`** (after the `app_commands!` fix in both crates)
   — same failure point, error count already dropping (42 → still in `semio-framework-plugin`,
   confirming this is itself a live, actively-being-fixed peer wave).

5. **`cargo check -p semio-framework-plugin` directly, twice more, spaced by the rest of this
   session's work** — 28 errors both times (stable), all `ToValue`/`FromValue` gaps on
   `PackageDescriptor`/`MediaType`/`MediaWireFormat`/`ViewModel`/`Effect`/`ExecutionMode`/
   `CapabilityRequest`/`ArtifactContributionDescriptor`/`PluginDependency`/`CapabilityRequirement`/
   `dsl::Fault`/`&str`/`serde_json::Value` — every one a `semio-framework-plugin`-internal type,
   grep-confirmed zero mentions of `mathematical`/`energy`/`fem`/`playbook`/`procedural` anywhere in
   the error list across all attempts.

**No crate in this batch reached its own compilation this session.** `semio-framework-plugin` is a
direct dependency of every plugin in the repo (it hosts `ArtifactApp`/`PluginApp`/`app_commands!`
itself), so its own in-flight breakage blocks every downstream `cargo check` regardless of what the
downstream crate's own edits are. This is the same "stale/lock-blocked check" risk class
`📓️verified-outcomes.md` already documents at length — the difference this session is that the
blocker never cleared in time to get a real signal, not that a check ran and passed unnoticed.

## Honest status per manifest

| manifest | serde? | serde_json? | reason if present | status |
|---|---|---|---|---|
| `➗️mathematical` | yes | yes | `app_commands!` (10 types) / stdio `JsonSnapshot` | WRITTEN, UNVERIFIED |
| `🔋️energy` | yes | yes | `Model`↔`SemioValue` bridge (whole engine tree, structural) | WRITTEN, UNVERIFIED |
| `📖️playbook/🧩️extensions/🌀️procedural` | **no** | yes | `flow::playbook::visible_blocks` / `FlowFixture` | WRITTEN, UNVERIFIED |
| `🌀️procedural` | yes | yes | not attempted — 187 files / 1229 sites, own wave | measured only |
| `🏗️fem` | yes (restored) | **no** | `app_commands!` (37 payloads + 2 types) | WRITTEN, UNVERIFIED |
| `🗄️stdio` | yes | yes | not attempted — 583 files / 7084 sites, own wave | measured only |

Every "WRITTEN, UNVERIFIED" row is blocked by the identical, confirmed, out-of-scope
`semio-framework-plugin` breakage — not by any defect this session found in its own edits. Whoever
picks this up next should re-run the six `cargo check -p <crate>` commands once that crate compiles
clean; if `➗️mathematical`/`🔋️energy`/`📖️playbook`'s extension/`🏗️fem` still don't, the error list
will now be small and load-bearing, not fleet-wide noise.
