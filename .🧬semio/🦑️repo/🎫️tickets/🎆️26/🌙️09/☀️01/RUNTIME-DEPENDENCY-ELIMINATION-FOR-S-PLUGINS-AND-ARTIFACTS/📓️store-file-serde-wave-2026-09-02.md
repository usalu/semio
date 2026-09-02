# 🏪️ `🏪️store/🦀️.rs` — this wave's three assigned items, re-verified against the live file

Assignment (`📓️store-production-serde-surface.md`): three items, in order, compiling between each.
Re-checked every item against the file as it actually stands right now (not the briefing's assumed
state) before editing — two of the three had already moved since the briefing was written, and one
has a real, in-workspace blast radius the briefing didn't account for.

## Item 1 — `ArtifactCursor` hand-written serde impls (~2217-2231)

**Already done before this wave started.** `ArtifactCursor` already carries hand `impl ToValue`/
`impl FromValue` (lines 2196-2206), and the old `impl serde::Serialize`/`impl<'de> Deserialize<'de>`
are already `#[cfg(test)]`-gated (2216-2229), with a docstring crediting the conversion to this same
ticket (`📓️os-kernel-serde-final.md`). No change needed. No-op, confirmed by reading the file, not
assumed.

## Item 2 — three production derives (2063/2065, 3419/3421, 17256/17258)

Checked each individually for a load-bearing external consumer before touching it (this ticket's own
standing rule: gate/inspect before converting, never convert blind).

- **`ArtifactBackboneRef`** (was 2063/2065) — **converted**. Repo-wide grep found its only
  consumers are `🖥️host/🦀️.rs`'s `BackboneDocument` (already `ToValue`/`FromValue`-only, its own
  docstring documents dropping `Serialize` entirely for this exact reason) and `🔌️plugin/🦀️.rs`'s
  `backbone_ref()` accessor (returns `&ArtifactBackboneRef`, no serialize bound). No test in this
  crate uses `serde_json` as an oracle on it. Dropped `serde::Serialize, serde::Deserialize` and
  `#[serde(rename_all = "camelCase")]` outright (not even `cfg_attr(test, …)` — nothing needs it),
  matching the sibling pattern already used for `HistoryLane`/`MigrationProvenance`/`OwnerRef`.

- **`LaneItemReceipt`** (3419/3421) — **left unconditional, not converted.** Its own docstring
  states the reason and I independently verified it: `🔌️plugin/🦀️.rs:13089`,
  `fn try_serialize<T: serde::Serialize>(...) -> Result<Self, Fault>` (production code, not
  `cfg(test)`) calls `serde_json::to_writer(&mut writer, value)`, and `LaneItemReceipt` is passed to
  it at `🔌️plugin/🦀️.rs:22231` (`TypedOperationResultPage::try_serialize(token, pending_lane,
  &receipt)`), a live production path, not a test. Gating this to `cfg_attr(test, …)` would break
  `semio-framework-os` production compilation. Not touched.

- **`BlobRef`** (17256/17258) — **left unconditional, not converted.** Its own docstring already
  states the reason: `🪐️space/🦀️.rs` serializes a `BlobRef` through `workflow_kernel` at runtime, so
  gating it breaks the `s` plugin's wasip2 build. Not independently re-verified beyond reading the
  docstring (out of this wave's fenced file), but no reason to doubt it — it matches the same
  pattern as `LaneItemReceipt` above, and the prior wave's own research (`📓️store-serde-final.md`,
  "Deferral 2") already reached the same conclusion for this exact cluster.

Net: 1 of 3 converted, 2 correctly left alone with a compiler-grounded reason (not the "test oracle"
exception the briefing anticipated, but a stronger one — a genuine unconditional production
consumer in another crate).

## Item 3 — the `serde_json::Value` pack API (`pack_rt`, ~4817-4894, `:9198`, `:19820-19825`) — **not converted, deliberately**

The briefing's own text calls this "the architectural one" and says to re-type
`encode_json_value`/`decode_json_value`/`renormalize_json_wire_value`/`json_value_to_dsl`/
`dsl_value_to_json`/`json_values_equal`/`impl ArtifactPack for serde_json::Value` and the generic
`InteractionState` bridge onto `DslValue`. Before doing that I checked every one of these names for
external callers repo-wide (not just inside `🏪️store`), because CLAUDE.md requires validating
assumptions rather than trusting a briefing's line-number list at face value. Result: **this is a
real, wide, in-workspace production fan-out that the briefing did not account for**, not a
self-contained change confined to this file.

Concrete evidence (`grep -rn`, whole repo, excluding this ticket's own scratch files):

- `store::pack_rt::dsl_value_to_json` — called from production code in:
  `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs:446`,
  `🧰️framework/🔨️modules/✍️editor/🦀️.rs:652`,
  `✏️s/🔌️plugins/🌊️flow/…/📋️duplicate-widget/🦀️.rs:305`,
  `✏️s/🔌️plugins/💡️reasoning/…/🖱️canvas-pointer-down/🦀️.rs:71`,
  `✏️s/🔌️plugins/📋️forms/…/🧪️set-try-values/🦀️.rs:419` and two more `set-try-value` sites,
  `✏️s/🔌️plugins/📏️layout/…/🖱️canvas-pointer-down/🦀️.rs` (three call sites).
- `store::pack_rt::json_values_equal` — called from
  `✏️s/🔌️plugins/🧩️puzzle/…/🖐️5d/…/🧬️mutations/🦀️.rs:399` and
  `✏️s/🔌️plugins/🧩️puzzle/…/◻2d/…/🧬️mutations/🦀️.rs:364` (both as the production `PartialEq` for a
  mutation payload wrapping a `serde_json::Value`).

That is at minimum 7 other crates (`framework-surface`, `framework-editor`, `s-plugin-flow`,
`s-plugin-reasoning`, `s-plugin-forms`, `s-plugin-layout`, `s-plugin-puzzle`) whose production code
would stop compiling if `dsl_value_to_json`/`json_values_equal` were re-typed onto `DslValue` — none
of them are `🏪️store`, none are mine to touch this wave (my assignment is this one file, alone),
and I have no way to fix them within this wave's fence. Converting anyway would be handing back a
regression across seven crates in exchange for a green `cargo check -p semio-framework-os-kernel`,
which is not a net improvement.

`impl ArtifactPack for protocol::InteractionState` (the generic bridge, `~19820-19825` in the
briefing, `19819-19827` as the file stands now) is independently blocked for a different,
already-documented reason: `InteractionState` itself (defined in `📡️replication`, a different
crate) has no `ToValue`/`FromValue` yet, so its bridge round-trips through
`serde_json::to_value`/`from_value` → `<serde_json::Value as ArtifactPack>`. This is the exact
"tenth-seam session's Blocker 3" already recorded in `📓️store-serde-final.md`. Since this bridge is
the one production caller of `impl ArtifactPack for serde_json::Value`
(`encode_json_value`/`decode_json_value`'s only consumer), that impl — and therefore
`encode_json_value`/`decode_json_value` themselves — must also stay as-is until `InteractionState`
gains `ToValue`/`FromValue` in `📡️replication` (out of scope here).

`json_value_to_dsl` is already private (`fn`, not `pub fn`) and only used inside `pack_rt` itself —
not an export in the CLAUDE.md sense, no change needed either way.

`renormalize_json_wire_value` is the one genuinely dead/unreachable target in this cluster —
repo-wide grep (all file types, not just `.rs`) found zero callers anywhere, including in this
ticket's own prior research docs that merely mention it. Left untouched anyway this wave: converting
a single dead function's signature doesn't move the needle on the Cargo.toml line (the other six
names in this same cluster are what actually block it), and deleting dead code wasn't part of the
assignment — flagged below instead of acted on.

**Conclusion, matching (and adding concrete grep evidence to) the prior wave's own finding in
`🔍️research/📓️store-serde-final.md`** ("pack_rt compose bridge — unchanged, permanent"): item 3 is
correctly deferred, not converted. It needs a coordinated multi-crate wave (7+ plugin/framework
files get their own PR-sized conversion first, matching this ticket's existing `serde-fanout-flow`/
`serde-fanout-imperative`/etc. research docs' pattern), plus `InteractionState` gaining
`ToValue`/`FromValue` in `📡️replication`. Neither fits "one file, alone, this wave."

## Verification

`cargo check -p semio-framework-os-kernel --message-format=short`, foreground, after the item 2
edit (`ArtifactBackboneRef` only — the only production code change made this wave). See this
ticket's own `📌️important` / final chat report for the verbatim tail and exit status — the repo had
many other agents' `cargo check` processes running concurrently at the time, so this run queued
behind them.

## Files touched this wave

`🏪️store/🦀️.rs` only — the `ArtifactBackboneRef` derive line and its docstring (lines ~2062-2068).
Nothing else in this file or any other file was edited.

## What remains for whoever picks this up next

1. **Item 3's real work is a 7+ crate fan-out**, not a `🏪️store`-only change: every
   `dsl_value_to_json`/`json_values_equal` call site listed above needs to move to a `DslValue`-
   native comparison/construction first (or `pack_rt::json_value_to_dsl`-equivalent at the call
   site), *then* the `🏪️store` signatures can drop `serde_json::Value`, *then*
   `encode_json_value`/`decode_json_value`/`impl ArtifactPack for serde_json::Value` can go away —
   but only once `InteractionState` (in `📡️replication`) also gains `ToValue`/`FromValue`, since
   that's the last production caller of the `ArtifactPack for serde_json::Value` bridge.
2. `renormalize_json_wire_value` (`🏪️store/🦀️.rs:4867`) is genuinely dead code (zero callers,
   repo-wide, any file type) — a candidate for outright deletion in a future wave, not touched here
   since it wasn't the assignment and deleting it doesn't unblock the Cargo.toml line on its own.
