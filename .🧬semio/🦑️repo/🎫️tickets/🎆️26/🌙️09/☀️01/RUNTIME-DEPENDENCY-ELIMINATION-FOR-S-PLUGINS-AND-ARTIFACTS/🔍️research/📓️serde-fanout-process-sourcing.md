# Serde Fanout — `🏭️process` + `🪵️sourcing` Batch

Batch: 5 `🏭️process` manifests + 3 `🪵️sourcing` manifests (8 total). Companion docs:
`📓️serde-replacement-surface.md` (foundation survey), `📓️serde-fanout-playbook.md` (mechanical
recipe/pilot). This doc adds one foundation piece that was missing (item 5 of that doc's "What's
next") before doing the fanout.

## Foundation addition (shared by every future batch, not just this one)

`pack::json::Value` had no bridge to `DslValue`/`ToValue`/`FromValue` — needed anywhere a plugin
does raw JSON *text* I/O (not just deriving a mutation payload). Added to
`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` (`//#region 🔖️DslBridge`):

- `dsl_to_json(&DslValue) -> Value` / `json_to_dsl(&Value) -> DslValue` — structural, total walk
  (both trees are the same six shapes: Null/Bool/Number/String/Array/Object).
- `to_json_string<T: ToValue>(&T) -> String` / `from_json_str<T: FromValue>(&str) -> Result<T,
  ValueError>` — the `serde_json::to_string`/`from_str` analog.

Lives in `pack`, not `🌱️value`, because `pack` already depends on `protocol` (replication, where
`🌱️value` is mounted) — the reverse edge would cycle. Reachable from every plugin as
`semio_framework_os_kernel::json::{to_json_string, from_json_str, dsl_to_json, json_to_dsl}`: the
pre-existing `component.rs: pub use pack::*;` glob inside os-kernel's `os_pack` module already
carries `pack::json` up to the kernel crate root, so no separate re-export was needed (verified by
reading the mount chain, not assumed). 4 tests added next to the existing `pack::json` suite:
round-trip through both bridge directions, round-trip through `to_json_string`/`from_json_str`, a
malformed-text error case, and a differential test against `serde_json` (parsed-`Value` equality,
not raw bytes — `ToValue`'s object field order is declaration order, `serde_json::Map` without
`preserve_order` is key-sorted, so byte comparison would false-fail on multi-key objects; noted
inline in the test's own docstring).

## Method used on every catalog type below

1. Add `ToValue, FromValue` to the derive list (from `semio_framework_value_derive`), alongside
   the existing `Serialize, Deserialize` (kept — see "Not done" below for why).
2. Mirror every `#[serde(...)]` onto `#[value(...)]`: `rename_all` covers serde's
   `rename_all`+`rename_all_fields` pair in one shot (this derive applies one `rename_all` to both
   variant names and in-variant field names — checked against the derive's own codegen, not
   assumed). `tag = "…"` → `tag = "…"`. `default`/`skip_serializing_if = "path"` → same.
3. One case fell outside the derive's supported set: `StockQuantity` is a plain unit-only enum
   with NO `#[serde(...)]` attribute at all, so serde's default (bare-string, not internally
   tagged) representation applies — the derive only supports internally-tagged
   (`#[value(tag=…)]`) enums (see its own module docstring, "the ONLY enum representation this
   derive supports"). Hand-wrote `impl ToValue`/`impl FromValue` directly instead of extending the
   shared macro for a single 5-variant type — the derive's own docstring names this as the
   sanctioned escape hatch ("A crate needing one of these keeps it hand-written… rather than
   deriving"), so this is not a workaround.
4. Checked every type for existing `.to_value()`/`.from_value(` dot-call sites before adding the
   derive (both `dsl::DslRecord`/`DslEnum`/`DslScalar` and `ToValue`/`FromValue` define methods of
   the same name — the framework's own docstring on `🌱️value/🔁️codec` already documents this and
   requires UFCS disambiguation at any ambiguous call site). Zero hits in either plugin's own tree,
   so no caller-side fix was needed.

## Per-manifest

### `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml` — WRITTEN, PARTIAL, NOT VERIFIED

This is the crate the ticket prompt described as "differs (serde, not serde_json)" as if a minor
variant of the extension pattern. It is not: `grep` count for this one crate (excluding its
extensions) is 78 `#[derive(...Serialize, Deserialize...)]` sites, 65 `#[serde(...)]` attribute
sites, 41 `use serde::` lines, and 363 `serde_json::` call sites (raw JSON I/O in
`🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io` import/export codecs, base64+JSON snapshot encoding,
`json!`-built debug/report payloads in mutation tests, DSL/schema round-trip tests) spread across
~60 files. That is comparable in size to the animate/W7 render lift the plan already flags as
"largest single-plugin lift" — it needs its own wave, not a slice of an 8-manifest batch, if it is
to be done without rushing 300+ call sites into an unreviewed state.

What I actually did, scoped to unblock the 4 dependent extension crates cleanly rather than touch
the other ~55 files:

- Added `semio-framework-value-derive` as a direct path dependency (resolve-checked, `../../../../../`
  from this manifest — same depth as its existing `semio-framework-schema` entry).
- `StockQuantity`, `CapabilityRule`, `CapabilityParameter`, `MeasureRecipe`, `Capability`,
  `WorkshopMachine` (the whole `WorkshopMachine` value tree, in
  `🗿️artifacts/🧊️process3d/🦀️component.rs`) now derive `ToValue, FromValue` alongside their
  existing `Serialize, Deserialize` — additive, not a rewrite of the type.
- Added 2 tests (`workshop_machines_round_trip_through_the_first_party_json_bridge`,
  `workshop_machines_json_matches_serde_json_by_value`) proving the new derives round-trip through
  `semio_framework_os_kernel::json::{to_json_string, from_json_str}` and agree with `serde_json`
  by value.

**Not done**: `serde`/`serde_json` are still in this manifest's `[dependencies]` — removing them
requires migrating the other ~72 derive sites (mutation payloads, diff/snapshot/inference types)
and, separately and more riskily, the `🚪️io` import/export JSON codec and the snapshot
base64+JSON encoding, which have golden-fixture/round-trip tests downstream I did not want to
touch under time pressure without a dedicated pass. Recommend this becomes its own wave item
(same size class as W7), not folded silently into the next batch.

### 7 extension manifests — PROVEN BY REVIEW, NOT COMPILE-VERIFIED (see Verification)

```
✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust/Cargo.toml
✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust/Cargo.toml
✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust/Cargo.toml
✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust/Cargo.toml
✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust/Cargo.toml
✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust/Cargo.toml
✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust/Cargo.toml
```

All 7 had the identical shape: `serde_json::json!({...})` building a topic-contribution payload
(`appId`/`moduleId`/`label`/`iconId` + one or two `*Json` string fields from
`serde_json::to_string(&catalog_data)`), plus a round-trip test. `ExtensionBundle::contributes_topic`
and `TopicContribution` (both in `semio-framework-plugin`/`🛂️manifest`, framework-side, out of
scope) hard-type their payload as `serde_json::Value` — confirmed by reading `TopicContribution`'s
struct definition, not assumed. Change, per crate:

- `Cargo.toml`: removed `serde_json = { workspace = true }`, added
  `semio-framework-os-kernel` as a direct path dependency (resolve-checked, 7×`../` — same depth
  every sibling dependency in the same file already uses) — the only new surface these crates
  need (`semio_framework_os_kernel::{DslValue, json}`; both already reachable from the kernel
  crate root, confirmed above).
- Source: `serde_json::json!({...})` → `semio_framework_os_kernel::DslValue::object([...]).into()`
  (the pre-existing `impl From<DslValue> for serde_json::Value` in `🌱️value/🦀️component.rs`
  supplies the conversion `contributes_topic` needs — zero new `serde_json::` symbol appears
  anywhere in these 7 crates' own source). `serde_json::to_string`/`from_str` →
  `semio_framework_os_kernel::json::{to_json_string, from_json_str}`.
- All catalog data types these 7 crates serialize (`WorkshopMachine` tree for the 4 `🏭️process`
  extensions; `TypologyNode`/`GeometryRecipe`/`ObjectKind` for the 3 `🪵️sourcing` extensions) got
  the `ToValue, FromValue` derive treatment described above, in their OWNING crate
  (`semio-s-plugin-process` / `semio-s-plugin-sourcing` respectively) — added
  `semio-framework-value-derive` as a direct dependency to both owning crates' manifests too
  (resolve-checked) since that's where the derive now lives, even though those two manifests are
  not themselves in this batch's target list.

`grep -rn serde` (source + Cargo.toml) across all 7 directories: zero hits, confirmed after every
edit.

## Verification — honest

The machine was at ~15-20 concurrent `cargo check`/`cargo test`/`cargo rustc` processes the whole
session (confirmed via `ps aux`, matching the ticket's own saturation warning) — a
`cargo check -p semio-framework-pack --message-format=short`, run foreground, was still queued
(0.3s of actual CPU time accumulated after 15+ minutes) when I stopped waiting to finish the
batch. **WRITTEN BUT UNVERIFIED for all 8 manifests + the framework bridge** — no `cargo
check`/`test` completed this session. What backs the correctness claim instead:

- Every new path dependency resolve-checked with `ls -d` against the literal relative path (all
  green, shown above) — this is the #1 failure mode the ticket calls out and it's ruled out
  directly, not inferred.
- `TopicContribution.payload: serde_json::Value` confirmed by reading the struct definition
  (`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:3280`), not assumed from the call site.
- `pack::json` reachability from `semio_framework_os_kernel::json` traced through the actual
  `#[path]` mount chain (`os_pack::component: pub use pack::*;` → crate root `pub use
  crate::os_pack::*;`), not assumed from a sibling pattern.
- The derive macro's supported attribute set, enum-tag requirement, and helper-attribute
  registration (`attributes(value)` on both `#[proc_macro_derive(...)]`) were read from its actual
  source, not guessed from the ticket prompt.
- `.to_value()`/`.from_value(` dot-call ambiguity risk checked by grep against both plugins' own
  trees (zero hits) before relying on the framework's documented UFCS-disambiguation escape hatch.

None of this substitutes for a real `cargo check`. If this doc is read before a check has been run
centrally, treat every manifest above as WRITTEN BUT UNVERIFIED and run
`cargo check -p semio-s-plugin-process-metal -p semio-s-plugin-process-robotic -p
semio-s-plugin-process-concrete -p semio-s-plugin-process-wood -p semio-s-plugin-sourcing-slabs
-p semio-s-plugin-sourcing-windows -p semio-s-plugin-sourcing-beams -p semio-framework-pack
--message-format=short` (foreground, one shot, no `CARGO_TARGET_DIR` override) once contention
eases.

## Files touched

Framework: `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` (new DslBridge region + tests).

Owning-crate type derives (not batch manifests, but needed for the batch's extensions to compile):
`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs`,
`✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml` (dep added only, serde/serde_json NOT
removed — see above),
`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🦀️component.rs`,
`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`,
`✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/Cargo.toml` (dep added).

Batch manifests, fully cleaned (Cargo.toml + source, zero serde/serde_json):
`✏️s/🔌️plugins/🏭️process/🧩️extensions/{🔩️metal,🤖️robotic,🧱️concrete,🪵️wood}/📦️packages/🦀️rust/Cargo.toml`
+ their `🦀️component.rs`,
`✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/{🧱️slabs,🪟️windows,🪵️beams}/📦️packages/🦀️rust/Cargo.toml`
+ their `🦀️component.rs`.

Also edited (additive note only): `📓️serde-replacement-surface.md` — marked its own "What's next
item 5" (pack::json ↔ DslValue bridge) done, pointing here.

## Scoreboard

- 7 of 8 batch manifests: zero third-party `[dependencies]`, WRITTEN, resolve-checked, NOT
  compile-verified.
- 1 of 8 (`🏭️process`'s own crate): still has `serde`+`serde_json`; foundation laid
  (`ToValue`/`FromValue` on the shared catalog types + framework JSON bridge), full elimination is
  out of scope for one batch slot — recommend a dedicated wave, sized like W7.

## Update — `cargo check -p semio-framework-pack` completed, PROVEN, and one consolidation

The foreground check queued at the time this doc was first written came back: **exit code 0**,
`semio-framework-replication` + `semio-framework-pack` both compile clean (only 2 pre-existing
warnings in `semio-framework-replication` unrelated to this change, plus pre-existing
`unnecessary qualification`/`dead_code` warnings inside `🔤️json/🦀️component.rs` — see below).
37m17s wall time, almost entirely queueing (confirms the saturation reported above, not a slow
build).

**Found while reading the diagnostic output**: a concurrent session landed its own
`DslValue ↔ pack::json::Value` bridge in the SAME file, same session window —
`//#region 🔖️DslValueBridge` (`from_dsl_value`/`to_dsl_value`, lines 342-376), citing the exact
same gap (`📓️serde-replacement-surface.md` §"pack::json::Value ↔ DslValue conversion") this doc's
"Foundation addition" section above describes filling. Two structurally-equivalent bridges
compiled side by side without conflict (different names), but that's duplication CLAUDE.md's
greenfield rule forbids leaving in place. Consolidated: deleted my own `dsl_to_json`/`json_to_dsl`
(and their test), rewired `to_json_string`/`from_json_str` to call the peer's already-landed
`from_dsl_value`/`to_dsl_value` instead of my own copies, renamed my region
`🔖️DslBridge` → `🔖️ToFromValueBridge` (it's now ONLY the generic string convenience pair, not a
second structural walk), and added the round-trip test `from_dsl_value`/`to_dsl_value` itself
didn't have yet. Re-submitted `cargo check -p semio-framework-pack` after the consolidation edit;
queued again (background id `bjbfcfkic`) — not yet back at the time this update was written.

**Current shape of `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs`**:
- `from_dsl_value(&DslValue) -> Value` / `to_dsl_value(&Value) -> DslValue` — peer's, structural.
- `to_json_string<T: ToValue>(&T) -> String` / `from_json_str<T: FromValue>(&str) -> Result<T,
  ValueError>` — mine, layered on the above, the actual `serde_json::to_string`/`from_str` analog
  every plugin in this batch (and the extension crates below) calls.

Every call site in this batch already used `semio_framework_os_kernel::json::to_json_string`/
`from_json_str` (the convenience pair), never the structural functions directly, so no other file
in the batch needed touching after this consolidation.

## Update — consolidated bridge PROVEN

Re-ran `cargo check -p semio-framework-pack --message-format=short` after the consolidation
above: **exit code 0**, 3m42s (warm cache — the duplicate-qualification warnings from my deleted
`dsl_to_json`/`json_to_dsl` are gone too, only 1 pre-existing unrelated warning remains in
`semio-framework-pack`). The `//#region 🔖️ToFromValueBridge` shape (layered on the peer's
`from_dsl_value`/`to_dsl_value`) is now PROVEN BY A PASSING CHECK, not just written.

## Final status — stopped waiting on builds per coordinator instruction

A `cargo check -p semio-s-plugin-process-metal -p …-robotic -p …-concrete -p …-wood -p
…-sourcing-slabs -p …-sourcing-windows -p …-sourcing-beams --message-format=short` (all 7
extension manifests in one invocation) completed with errors, but every error it reported was
inside `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`'s generic
`ArtifactApp`/`ArtifactStore` machinery (`E0277`/`E0599`, `Mutation`/`Snapshot`/`Config`/`Draft`/
generic `M`/`P`/`C` not satisfying `serde::Serialize`/`Deserialize`) — zero errors named any file
under this batch's own crates. Per the ticket's own instruction ("errors originating in
semio-framework-os-kernel or semio-framework-replication are another agent's in-flight work…
don't fix, don't wait"), that's out of scope here regardless of root cause.

The coordinator (message received mid-check) reports the actual root cause was a one-line
`#[derive(ToValue)]` codegen bug (`match *self` binding fields by value instead of `match self`
binding by reference, `🌱️value/✨️derive/🦀️component.rs:363`) that was fanning out through
`semio-framework`'s `WorkflowDiff`/`RunDiff` and had been misreported by multiple agents (including
an earlier stale run in this same session, per the ticket's own documented risk: a check queued
behind the shared target-dir lock compiles the source as of when it STARTED, so a slow check can
describe a tree that no longer exists by the time it finishes) — already fixed, with a central
re-check in progress. I have not independently re-run or observed that fix; recording it here as
reported, not self-verified, per this ticket's own "never say a feature is working when you didn't
confirm" rule.

**Directly verified by me this session** (own completed checks, not reported):
- `cargo check -p semio-framework-pack` — exit 0, clean, twice (before and after the
  `from_dsl_value`/`to_dsl_value` consolidation). The new `to_json_string`/`from_json_str` bridge
  every manifest below calls is PROVEN BY A PASSING CHECK.

**Not directly verified by me** (stopped waiting per coordinator instruction — a central re-check
is in progress there instead of a duplicate one here):
- All 7 extension manifests (`process-metal`/`-robotic`/`-concrete`/`-wood`,
  `sourcing-slabs`/`-windows`/`-beams`): WRITTEN, path deps resolve-checked, source reviewed
  line-by-line against the actual framework APIs (struct defs, mount chains, derive macro source —
  not assumed), zero `serde`/`serde_json` left in Cargo.toml or source (grep-confirmed) — but no
  `cargo check` on these specific 7 crate names has yet completed clean. WRITTEN BUT UNVERIFIED.
- `semio-s-plugin-sourcing` / `semio-s-plugin-process` (owning crates, `ToValue`/`FromValue` added
  to their catalog types): same status — WRITTEN BUT UNVERIFIED pending the central re-check.

## Scoreboard (final)

- 7 of 8 batch manifests: zero third-party `[dependencies]`, WRITTEN, resolve-checked,
  WRITTEN BUT UNVERIFIED pending the coordinator's central re-check (a fleet-wide derive bug —
  not anything in this batch's own crates — was blocking every check in the fleet, per the
  coordinator; now reportedly fixed there, not re-confirmed here).
- 1 of 8 (`🏭️process`'s own crate): still has `serde`+`serde_json`, correctly left alone — 78
  derive sites / 363 `serde_json::` call sites across ~60 files, its own wave, not this batch slot.
- Framework foundation piece (`pack::json ↔ DslValue`, `to_json_string`/`from_json_str`):
  PROVEN BY A PASSING CHECK, directly, twice.
