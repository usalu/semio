# Phase 2 (serde removal) blocker inventory — top of the DAG

Crates: `semio-framework`, `semio-framework-os-kernel`, `semio-framework-plugin`.
Method: `grep -rn serde` over every file the crate actually mounts (via `#[path]`), classified
by hand into (a) type derives, (b) `#[serde(…)]` attrs, (c) `serde_json::` call sites, (d) `use
serde…`, (e) trait bounds, (f) test-only; each non-test hit traced to why it's still there.

## TL;DR ranking (do this order)

1. **`semio-framework-os-kernel`** — closest to done. Production code is serde-free except two
   real architectural edges (§3.1, §3.2) plus one non-trivial VCS-codec edge (§3.3). 6 of its 11
   mounted modules (`vcs`, `identity`, `engine`, `inference`, `semio`, `extension`) already have
   **zero** serde tokens anywhere, test or prod.
2. **`semio-framework-plugin`** — blocked almost entirely by re-exporting `semio-framework`'s own
   blockers (`PackageDescriptor`/`PluginManifest`/`AppDefinition`) plus one external crate
   (`ui_contract`) not owned by this pass. Fix manifest's 3 external types first (§2.1) and most
   of this crate's production serde drops out for free.
3. **`semio-framework`** — the manifest module (630 of 682 refs) is the long pole. Real blocker
   set is only **3 external types**, all outside this pass's ownership (§2.1). Everything else in
   manifest is already dual-derived (`Serialize/Deserialize` + `ToValue/FromValue`) or test-only.

## Part 2 result: nothing stripped this pass

No crate's serde use is entirely test-only or entirely convertible in-crate — every one of the
three still has at least one non-test, non-dual production edge (manifest's 3 blocked types;
os-kernel's `serde_json::Value` `ArtifactPack` bridge + VCS text codec; plugin's
`PackageDescriptor`/`ui_contract` bridges). Manifest lines is a genuine strip candidate.
Per the standing rule ("never clear a Cargo.toml line you have not compiled"), manifests were
**left untouched**. See §5 for the one item that could be attempted next as a scoped in-crate fix
(`TutorialDefinition::from_json`) — not attempted this pass, flagged only.

---

## 1. `semio-framework` (crate root + 9 mounted modules)

Root `🦀️.rs` (2181 lines): **0** real serde tokens — its 5 grep hits are all doc-comment prose.

| module | file | serde refs | verdict |
|---|---|---|---|
| `🎯️action-bus` | `🦀️.rs` | 1 | (c) — see §1.2, real cross-crate blocker |
| `🚪️io` | `🦀️.rs` | 3 | comments only, 0 real |
| `🌉️abi` | `🦀️.rs` | 0 | clean |
| `🖥️platform` | `🦀️.rs` | 0 | clean |
| `🕹️interaction` (+ `🧬️schema`) | `🦀️.rs` | 10 | (a)+(f) — 3 dual derives, 1 non-dual (blocked, see below), 2 test round-trips |
| `🔁️workflow` | `🦀️.rs` (os product) | 33 | fully dual/additive already — see §1.3, **candidate for full strip once tests migrate** |
| `🛂️manifest` | `🦀️.rs` | 630 | the long pole — see §1.1 |

### 1.1 `🛂️manifest/🦀️.rs` (6957 lines, 630 serde refs)

Classification:

- (a) derive lines with `Serialize`/`Deserialize`: **114**, of which **99 already dual**
  (`ToValue`/`FromValue` alongside) or paired with hand-written `ToValue`/`FromValue` impls
  (`ResourceSelector` — see below). Only **8 struct-level derives are genuinely serde-only**.
- (b) `#[serde(…)]` attribute lines: 317 (mirrors the derive count — `rename_all`, `default`,
  `skip_serializing_if`, `transparent`, `deny_unknown_fields`; every one has a `#[value(...)]`
  sibling on the dual types).
- (c) `serde_json::` call sites: 152, of which **147 are inside `#[cfg(test)]` mods** (oracle
  round-trip / fixture-driven tests — required by this repo's "same output as a third-party
  library" test mandate, keep). Only **5 are outside tests**, and 3 of those are doc-comment
  prose. The 2 real ones are both `TutorialDefinition::from_json` (line 2187-2188) — see §5.
- (d) `use serde…` imports: 1 (top-level, crate-wide).
- (e) trait bounds requiring `Serialize`/`Deserialize`: 17 hits, all inside hand-written
  `Serialize`/`Deserialize` impls (`TopicContribution`, line ~3705-3725) or test-local `Raw`/`Case`
  deserialize-only helper structs (lines 5530, 6928) — no generic `T: Serialize` bound anywhere
  in production code.
- (f) test-only: the two `#[derive(serde::Deserialize)]` local structs at lines 5530/6928, plus
  the 147 test-only `serde_json::` sites above. **Not real blockers** — but can't move to
  `[dev-dependencies]` in isolation because the crate still needs `serde` unconditionally for the
  8 real blockers below.

**The 8 real (non-dual) production blockers**, each with an explicit `🚧️ BLOCKED
(26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS)` docstring already in the
file naming its cause:

| type (line) | blocked on | owner |
|---|---|---|
| `ResourceSelector` (696) | **nothing** — mis-flagged by grep: has hand-written `ToValue`/`FromValue` impls just below it (derive would conflict, E0119). Already effectively dual. |  |
| `UtilityDefinition` (1343) | `ui_wgpu::wgpu::UtilityCategory` — not one of the 7 keystone ui_wgpu types converted this ticket | 🖱️ui, out of scope this pass |
| `WindowKindDefinition` (3186) | `kernel::CapabilityRequirement` — no `ToValue`/`FromValue` yet | 🎠️kernel, other agent this pass |
| `AppDefinition` (3410) | transitively `UtilityDefinition` + `WindowKindDefinition` (both above) | same two |
| `PluginManifest` (4057) | `kernel::CapabilityRequirement` (same as `WindowKindDefinition`) | 🎠️kernel |
| `ViewModel` (4573) | `ui_wgpu::wgpu::{Locale, Terminology}` — not among the 7 keystone types | 🖱️ui |
| `ExtensionPointDeclaration` (4679) | `kernel::ActivationEvent` — no `ToValue`/`FromValue` yet | 🎠️kernel |
| `PackageDescriptor` (4865) | transitively `ExtensionPointDeclaration`/`ContributionSet` → `kernel::ActivationEvent` | 🎠️kernel |

**So the entire manifest blocker set reduces to 3 external types**, none owned by this pass:

1. `ui_wgpu::wgpu::UtilityCategory`
2. `ui_wgpu::wgpu::Locale` / `ui_wgpu::wgpu::Terminology`
3. `kernel::CapabilityRequirement` and `kernel::ActivationEvent` (🎠️kernel-owned)

Convert these 3 (really 4 types across 2 owners) and `AppDefinition`, `PluginManifest`,
`PackageDescriptor`, `WindowKindDefinition`, `UtilityDefinition`, `ExtensionPointDeclaration`,
`ViewModel` all drop straight to dual-only-by-default, and manifest's serde-only derive count
goes to **0**. That in turn unblocks `🕹️interaction::InteractionRef` (see §1.2) and most of
`semio-framework-plugin`'s remaining production serde (§4.1).

### 1.2 `🕹️interaction/🦀️.rs` (10 refs)

- `InteractionDefinition`, `GranularityDefinition`: already dual (kept additive per their own
  comments — consumed by `🛍️products/💻️os` + `✏️s/🔌️plugins/**` while those still serde-derive).
- `InteractionRef` (line 90, `#[serde(transparent)]`): **non-dual**, blocked because
  `WindowKindDefinition.interactions: Vec<InteractionRef>` is itself still serde-only. This is
  the exact §1.1 `WindowKindDefinition` blocker (→ `kernel::CapabilityRequirement`) — same fix
  unblocks both.
- 2 refs are the test's own `serde_json::to_string`/`from_str` round-trip (test-only, fine).

### 1.3 `🔁️workflow/🦀️.rs` (2730 lines, 33 refs) — already fully additive, real blocker is a codec, not a type

Every struct/enum with `Serialize`/`Deserialize` in this file **also** derives (or hand-writes)
`ToValue`/`FromValue` — confirmed for all ~20 derive sites including the tuple-struct-blocked
`MediaContract` (hand-written `dsl::DslField`/`ToValue`/`FromValue` at lines 257-333, alongside
its `#[derive(..., Serialize, Deserialize)]` at line 39). `RunArtifact`/`WorkflowSnapshot` already
use handcrafted `ArtifactDsl`/`ArtifactPack` impls (`🔖️HandcraftedWorkflowSnapshotCodecs`,
`🔖️HandcraftedRunArtifactCodecs`) that never touch serde.

Checked every external consumer of `WorkflowSnapshot`/`RunArtifact`/`WorkflowNode`/
`WorkflowEdge`/`MediaContract` outside this module (`🛂️manifest`, `🏃️run`, `🏃️run/📦️bin.rs`,
`🪐️space`, renderer `Shell` wgpu target, and all 52 files across `✏️s/🔌️plugins/**` that
reference these type names): **zero** of them call `serde_json::`/require `Serialize`/
`Deserialize` on these types. The only remaining serde use in this module is:

- 5 `#[derive(...Serialize, Deserialize)]` lines that are **already dual** — dead weight now,
  not a blocker, just unremoved.
- 33 total refs, of which the ~6 real `serde_json::` call sites (lines 2581-2586) are inside
  `#[cfg(test)]` (`run_payload_serde_uses_exact_camel_case_and_rejects_unknown_fields`) — testing
  wire-shape (camelCase, `deny_unknown_fields`) that would need re-proving via `ToValue`/dsl
  before the derive can be dropped.

**This module is the single best strip candidate found this pass** — plausibly fully
serde-free with (a) dropping `Serialize, Deserialize` from all ~20 derives + their `#[serde(…)]`
attrs, (b) rewriting the 3 camelCase/deny-unknown-fields test assertions to go through
`ToValue`/dsl instead of `serde_json`. **Not attempted this session** — it sits inside the same
`semio-framework` crate as manifest (mounted via `#[path]`), so `serde` stays a hard dependency of
the crate regardless until manifest's 3 external types (§1.1) also convert; stripping workflow
alone would not let `serde` leave `[dependencies]`, only reduce its blocker count to manifest's.

---

## 2. `semio-framework-plugin`

Root `🦀️.rs` (21 lines) mounts one file, `🔌️plugin/🦀️.rs` (37878 lines) — plus its own
`🏗️builder`, `⚛️reactor`, `🌐host`, `🛂️describe`, `🧵️retained-command`, `🕹️interaction`
(19 lines, 0 serde) sub-mounts, plus 4 `🧪️tests/**` mounts (out of scope to edit, counted for
completeness).

| file | serde refs | of which non-test (prod) |
|---|---|---|
| `🔌️plugin/🦀️.rs` | 194 | **15** (2 are doc comments → 13 real) |
| `🏗️builder/🦀️.rs` | 6 | test helpers only (mesh-importer test doubles) |
| `⚛️reactor/🦀️.rs` | 11 | **6** real (see §2.2) |
| `🌐host/🦀️.rs` | 5 | **4** real (see §2.3) |
| `🛂️describe/🦀️.rs` | 11 | **all 11** real — the `PackageDescriptor` bridge (§2.1) |
| `🧵️retained-command/🦀️.rs` | 9 | test-only (local Deserialize fixtures + fixture loads) |
| `🧪️tests/**` (3 files, not owned) | 10 | out of scope |

### 2.1 `PackageDescriptor` bridge — direct consequence of §1.1

`🛂️describe/🦀️.rs:146-241` and `🔌️plugin/🦀️.rs:28504-28509` both bridge
`semio_framework::PackageDescriptor`/related types through `serde_json::to_value`/`from_value`
into `dsl::DslValue`, with an explicit in-repo comment calling this "the sanctioned transitional
bridge for a type that still derives serde", citing `PackageDescriptor` deriving only serde
"transitively through `PluginManifest`/`AppDefinition`'s own huge [blocker]". **This is the exact
§1.1 finding** — once manifest's `kernel::CapabilityRequirement`/`kernel::ActivationEvent`/
`ui_wgpu::UtilityCategory`/`{Locale,Terminology}` convert, this bridge becomes deletable and
`🛂️describe` drops to near-zero serde. Also affects `wire_list_artifact_inference_services`
(`🏗️builder/🦀️.rs:1014`, `WireArtifactInferenceMetadata` round-trip).

### 2.2 `⚛️reactor/🦀️.rs` — blocked on `ui_contract` crate (not owned by this pass)

- Lines 1425, 2042: `serde_json::from_value::<ui_contract::UiIntent>` /
  `<ui_contract::UiPatchOps>` — both foreign types from the `ui-contract` crate dependency, no
  `ToValue`/`FromValue` there yet.
- Lines 2448-2464: `pack_patch_field<T: serde::Serialize>` helper + a local
  `#[derive(serde::Serialize)]` struct — comment explicitly says "`ui_contract::Activity` has no
  `ToValue` of its own", so bridges through `serde_json`.
- These 6 refs are one external-crate blocker (`ui_contract`), not owned by
  `semio-framework`/`os-kernel`/`plugin`. **List for a separate ticket against `ui-contract`.**

### 2.3 `🌐host/🦀️.rs` — same shape, partly already dual

- Line 166: `#[derive(serde::Serialize, ToValue, serde::Deserialize, FromValue)]` — already dual,
  fine.
- Lines 107-108: `pack<T: serde::Serialize>` generic helper — used for payloads without
  `ToValue` (same pattern as reactor's bridge); not yet inventoried which concrete `T`s hit this
  path, follow-up if this crate's strip is picked up again.
- Lines 420, 431: two more local `#[derive(serde::Serialize, ToValue)]` — already dual, not
  blockers.

### 2.4 `🔌️plugin/🦀️.rs` root file, 13 real production refs

- Lines 116-191 (`PollInput` region): `serde_json::to_value`/`from_value` bridge, comment at 130
  says "reusing the existing `DslValue <-> serde_json::Value` conversion" — deliberate bridge,
  not obviously convertible without touching the `DslValue`/`serde_json::Value` `From` impl
  itself (owned by `dsl`/`os-kernel`, see §3.1 — same escape hatch).
- Lines 9788-9872 (`InteractionState` mutation wire): serde-only local derive +
  `serde_json::to_string`/`from_str`/`to_vec`/`from_slice` — real runtime wire codec for
  interaction-config mutations, not yet on `ToValue`/dsl.
- Lines 11469-11491 (`MediaType`/`MediaWireFormat`): explicit comment "through `serde_json` via
  `#[value(serialize_with=…, deserialize_with=…)]` instead of deriving" — deliberate value-derive
  escape hatch, not a blocker so much as a design choice already compatible with serde removal
  (it's `#[value(...)]`-driven, `Serialize`/`Deserialize` aren't required by it — needs a closer
  read to confirm the serde half can just be deleted here without also deleting the bridge; flagged,
  not fixed this pass).
- Lines 27886-28509 region (`ArtifactPack` handcrafted codecs, `ViewModel` decode at 28341,
  `PackageDescriptor` at 28504-28509): mix of already-serde-free handcrafted codecs and the
  §2.1 `PackageDescriptor` blocker recurring.

---

## 3. `semio-framework-os-kernel`

Root `🦀️.rs` (370 lines): **0** real serde (2 hits are doc-comment prose). Mounts 11 module
trees; per-module **production+test** serde-token totals:

| module | total serde refs | verdict |
|---|---|---|
| `🌿️vcs`, `🪪️identity`, `⚙️engine`, `💡️inference`, `🧬️semio`, `🧩️extension` | **0** | fully clean, nothing to do |
| `🎒️pack` | 6 | all in `🔎️scalar-witness/🧪️tests/` (out of scope, test-only) |
| `📇️directory` | 21 | **all test-only** — production code already routes through `pack::json`/`ToValue`/`FromValue` per its own docstrings (`🔌️client/🦀️.rs:266`); a prior pass (`📓️directory-spr-serde-removal.md`, referenced in-file) already did this conversion. 21 refs are `FakeTransport::json_response` test helpers + fixture loads. **Candidate: move `serde`/`serde_json` test usage in this module to dev-deps once verified no other module needs it as a regular dep** (moot while store/spr still need it — see below). |
| `🗣️dsl` | 67 | production clean (`✨️derive/🦀️.rs`'s 32 + root's 5 non-comment hits all inside `#[cfg(test)]` mods, verified by brace-scan); remaining ~30 are macro-test fixtures |
| `📡️spr` | 125 | production clean — only non-dual derive found (`🎮️command/🦀️.rs:1477`) is a local struct inside a `#[test]`-region; `🧵️channel`/`📜️history` have zero non-dual derives |
| `🏪️store` | 338 | **the real blocker set** — see §3.1-3.3 |

### 3.1 `impl ArtifactPack for serde_json::Value` (store `🦀️.rs:9206`)

Explicit "Compose-only pack bridge (external technology)" doc comment: a deliberate, permanent
escape hatch letting apps whose document schema is genuinely foreign/opaque JSON (not owned by
this codebase) round-trip through the pack format via `pack_rt::encode_json_value`/
`decode_json_value`. **This is architectural, not a "not yet converted" blocker** — removing it
means either (a) accepting compose-only apps can no longer use raw JSON documents, a product
decision, not a serde-removal mechanics problem, or (b) it's inherently fine to keep since it's
gated behind an explicit `serde_json::Value` foreign-type impl and doesn't force the *rest* of
the crate to keep serde as a non-optional dependency once every other edge converts (the impl
itself still needs `serde_json` in `[dependencies]`, just scoped to this one bridge).

### 3.2 `PollInput`/DslValue bridges reusing `serde_json::Value` (see §2.4, `os_dsl`/`os_pack` side)

Root cause lives in whichever module defines `From<&serde_json::Value> for DslValue` (referenced
by multiple call sites across manifest/plugin/store docstrings as "the sanctioned transitional
bridge") — did not get to tracing its exact file this pass; flag for next recon: grep
`impl From<&serde_json::Value> for` / `impl From<DslValue> for serde_json::Value` to find the
one true bridge implementation all these call sites funnel through.

### 3.3 VCS text-codec `serde_json::to_string` on `mutation_meta`/messages/conflicts (store `🦀️.rs:10172,10212,10218`)

Real production code in the VCS ops-log text printer (`print_document_spr`-adjacent path):
`serde_json::to_string(meta)`/`(message)`/`(conflict)` serialize `MutationMeta` and edit-message/
conflict payloads into the persisted `.ops` text header lines. This is a genuine **fixable-in-crate**
item (the types are presumably already `ToValue` given `📡️spr` above is production-clean) —
swapping to a `ToValue`-based printer would need the exact wire text format re-proven byte-for-byte
(the file's own comments show this codebase is very strict about exact wire-format stability).
**Not attempted this pass** — flagged as the clearest next fixable-in-crate os-kernel item, cheaper
than store's other 335 refs (which are `#[cfg_attr(test, derive(...))]`-gated, i.e. already inert
in non-test builds — see below).

### 3.4 `store/🦀️.rs`'s remaining 141 refs are mostly already test-gated

All 8 non-dual `derive(...Serialize, Deserialize...)` lines found in `store/🦀️.rs` are
`#[cfg_attr(test, derive(Serialize, Deserialize))]` — i.e. **the derive itself only exists in test
builds**, already effectively equivalent to a dev-dependency-only derive. Same pattern confirmed
in `🧵️canonical-edit/🦀️.rs:604` and (unconditionally-derived but proven test-local) `👥️presence/
♻️retirement/🦀️.rs:270` (`Value` struct declared and used only inside a `#[cfg(test)]`-nested
`mod`). `🔄️sync/🦀️.rs`'s 3 derives are already fully dual. **None of these block a strip** — the
only true blockers in `store` are §3.1-§3.3.

---

## 4. Cross-crate coupling summary (what's NOT owned by these 3 crates)

| blocker | owner | blocks |
|---|---|---|
| `kernel::CapabilityRequirement` | 🎠️kernel (other agent this pass) | `WindowKindDefinition`, `PluginManifest`, transitively `AppDefinition`, `🕹️interaction::InteractionRef` |
| `kernel::ActivationEvent` | 🎠️kernel | `ExtensionPointDeclaration`, `PackageDescriptor`, transitively all of `🛂️describe`'s bridge (§2.1) |
| `ui_wgpu::wgpu::UtilityCategory` | 🖱️ui (out of scope this pass) | `UtilityDefinition`, transitively `AppDefinition` |
| `ui_wgpu::wgpu::{Locale, Terminology}` | 🖱️ui | `ViewModel` |
| `ui_contract::{UiIntent, UiPatchOps, Activity}` | ui-contract crate (separate, not inventoried this pass) | `⚛️reactor/🦀️.rs`'s 6 production refs |
| `🎯️action-bus::optional_json_to_dsl(args: Option<serde_json::Value>)` | this pass, but ~26 callers across `✏️s/🔌️plugins/**` (listed in the ticket's own known-facts, not re-derived here) | atomic cross-crate signature change, do LAST |

Convert `CapabilityRequirement` + `ActivationEvent` (both 🎠️kernel) first — they unlock the most
(manifest's biggest structs + the plugin-crate `PackageDescriptor` bridge). `UtilityCategory`/
`Locale`/`Terminology` (🖱️ui) are smaller, single-struct unlocks. `ui_contract` is independent and
can happen in parallel.

## 5. Not attempted, flagged for next pass

- `TutorialDefinition::from_json` (manifest, line 2187): calls `serde_json::from_str::<Self>`
  directly even though `TutorialDefinition` is already dual (`ToValue`/`FromValue` present) — its
  own docstring's "BLOCKED" comment is **stale**, the type unblocked already. Trivial: swap to a
  dsl/`FromValue`-based JSON parse. Left alone this pass (out of the 3-crate recon's critical path,
  and small enough to be a 5-minute follow-up once someone is already in this file).
- `🔁️workflow/🦀️.rs` full strip (§1.3) — best single strip candidate found, blocked only by
  sharing a crate (and thus a `Cargo.toml`) with manifest's still-open 3 external types.
- Locating the canonical `serde_json::Value ↔ DslValue` bridge impl (§3.2) to confirm it's the
  single choke point every other "sanctioned transitional bridge" comment in the repo refers to.

## VERIFY (this recon made no code changes — manifests untouched)

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/iso3
export RUSTC_WRAPPER=""
cargo check -p semio-framework --message-format short   # unchanged from session start
cargo metadata --no-deps --format-version 1 >/dev/null; echo $?
```
