# W2-C — read-only review of today's energy plugin changes (lanes A–D)

Scope: `git diff $(cat 🗑️generated/start-commit.txt) -- "✏️s/🔌️plugins/🔋️energy"` plus the live working
tree, excluding `🧫️fixtures/🧬️mutations`, `🖼️assets`, `🔣️.json` descriptors and the `🧫️fixtures/🏛️bestest-*`
model dumps. Cross-read against `📓️w1-a-schema.md`, `📓️w1-b-3d-window.md`, `📓️w1-c-panels-inspector.md`,
`📓️w1-d-results.md`. All findings below were verified by reading the actual source (not the lane
reports) against the live files at review time; the working tree was still being edited by an active
lane while this review ran (a zone-volume convex-hull feature and a `set-construction-property` verb
landed mid-review), so line numbers were re-checked against the live files, not a single diff snapshot.

## Findings, ranked

### 1 — BLOCKER: editing Site or Thermostat fields from the inspector silently writes the wrong value

**Where:** the bridge parsing is pre-existing and unchanged —
`✏️editor/🦀️.rs:309` (`SET_THERMOSTAT_SETPOINTS_ACTION_ID`) and `✏️editor/🦀️.rs:316` (`SET_SITE_ACTION_ID`) —
but this ticket newly wires per-field inspector controls onto them in
`✏️editor/📌️panels/🔍️inspection/🦀️.rs:424` (`thermostat_args`), `:440`/`:449`/`:455`
(`thermostat_select`/`thermostat_number`/`bind_full`), `:476` (`site_args`) and `:491` (`site_number`).

**What breaks.** The framework convention (documented identically in fem's editors and in this
file's own module doc, line 13: *"the host merges the control's own value under `value`"*) is: a
`Trigger::Change` control authors its identifying args, and the host overwrites/creates the literal
key `"value"` with whatever the user typed, right before dispatch. `thermostat_args`/`site_args`
follow that convention on the AUTHORING side — they deliberately drop the edited field's own key
(`entries.into_iter().filter(|(key, _)| *key != edited)`) and let the host fill `"value"` in.

But `command_from_action`'s arms for these two actions (lines 309–322) never read `"value"` — they
read each field by its OWN name: `heating_schedule: u32_or("heatingSchedule", 0)`,
`heating_throttle_range_k: f64_or("heatingThrottleRangeK", 2.0)`, `north_axis_deg:
f64_or("northAxisDeg", 0.0)`, etc. Since the edited field's key was deliberately removed from the
args map, the bridge never finds it and falls back to its hard-coded default — discarding the user's
typed value and, for `northAxisDeg`/`latitudeDeg`/`longitudeDeg`/`elevationM`/`timeZoneHours`,
silently overwriting the site with **0.0** instead. For the throttle ranges it silently overwrites
with the hard-coded **2.0**. For a schedule select it writes id **0**, which will not exist and gets
refused loudly by `schedule_exists` — the only one of the six fields that fails loudly instead of
silently.

**Concrete repro:** select the site (empty selection → summary), site north axis shows `15.0`°, type
`30.0` into "North axis (°)" → dispatch merges `value:"30.0"` but drops `northAxisDeg` from the args
→ `command_from_action` reads `f64_or("northAxisDeg", 0.0)` → **the site's north axis silently
becomes `0.0`**, not `30.0` and not the prior `15.0`, and the app reports a normal "Set site" history
entry with no error. Same for any thermostat throttle-range edit (always reverts to `2.0`).

**Fix:** give `SET_SITE_ACTION_ID`/`SET_THERMOSTAT_SETPOINTS_ACTION_ID` the same `{field, value}`
indirection already proven correct for the five new inspector verbs — i.e. have `command_from_action`
read a `field` marker and copy the merged `"value"` into whichever of the five named slots it
designates, instead of reading five fixed key names directly.

### 2 — MAJOR: the inspector's "Boundary → interzone" option can never actually be applied

**Where:** `✏️editor/📌️panels/🔍️inspection/🦀️.rs:255–262` (`surface_rows`'s `boundary_options()`
select, which offers `"interzone"` from `OUTSIDE_BOUNDARY_KIND_IDS`) vs. `✏️editor/🦀️.rs:1330`
(`set_surface_property`), which requires a nonzero `partner_surface`.

**What breaks.** `args_bridge` reads `partner_surface: u32_or("partnerSurface", 0)`, and nothing in
the inspection panel ever authors `partnerSurface` (confirmed: no occurrence of `"partner"` anywhere
in that file outside the READ-ONLY row at line 260 that only *displays* an existing partner). So
choosing `"interzone"` from the boundary select always dispatches with `partner_surface = 0`, and
`OutsideBoundary::from_parts(Interzone, None)` refuses (per lane C's own comment on this exact line).
Every attempt to turn a surface into an interzone boundary from the inspector fails with
`'interzone' is outside the admissible range of property 'boundary'`, with no UI clue why — the
control just doesn't work for the one option that needs a second piece of data.

**Fix:** add a partner-surface picker row (visible for `boundary == "interzone"`) that supplies
`partnerSurface` alongside the boundary change, or remove `"interzone"` from the offered options
until that control exists.

### 3 — MAJOR: a marked/selected surface can be dropped from the artifact tree entirely under row pressure

**Where:** `✏️editor/📌️panels/🗿️artifact/🦀️.rs:221` (`surfaces_with_windows`), breadth-allocation loop
(`for slot in allowance.iter_mut() { ... }`).

**What breaks.** The function's own doc claims *"the surface the domain currently marks... claims its
whole demand FIRST"*, but that priority is only applied in the DEPTH pass (window sub-rows). The
BREADTH pass that decides which surfaces get a ROW AT ALL walks `surfaces` in plain document order,
unconditional of marking — `order.sort_by_key(|index| usize::from(!marked(...)))` is computed and
used only afterwards, for the leftover window budget. If a zone's own nested row budget
(`budget.remaining()`, shrunk by `section_quotas`/arena pressure per lane C's §6.2 note) is smaller
than its surface count, only the first N surfaces in document order get `allowance[i] = 1`; any
surface after that — including the one currently selected in the 3d viewport — gets `allowance[i] =
0` and is skipped (`if share == 0 ... continue`), with no row and no `+N` marker naming it
specifically. This defeats "tree pick ⇄ 3d pick": picking surface #10 of 12 in the 3d window while
the page can show only 8 surface rows makes the tree simply not show that surface, selected or not.
Not exercised by BESTEST (≤8 surfaces/zone) or by the lane's own test (2 surfaces, page for exactly
one), so it passed review-time gates.

**Fix:** reuse the already-computed marked-first `order` for the breadth pass too (assign row slots
in marked-first order, then render in document order), instead of two independently-ordered passes.

### 4 — MINOR/test-quality: the test that should catch #1 asserts the broken shape as correct

**Where:** `✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs:160`
(`a_selected_thermostat_carries_the_whole_setpoint_payload_per_control`).

**What it does:** asserts `!bindings.contains("heatingThrottleRangeK")` with the message *"the edited
key is the one the host merges"* — i.e. it certifies the exact shape that finding #1 shows is
incompatible with this action's own `command_from_action` arm. It never round-trips the built
binding through `args_bridge::command_from_action` (with a simulated host merge inserting `"value"`)
to check the resulting `SetThermostatSetpoints`/`SetSite` command actually carries the new number.
The companion editor-tests (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:184`, `:222`) construct
`EnergyModelEditorCommand::SetSite{...}`/`SetThermostatSetpoints{...}` directly, bypassing the bridge
entirely, so no test anywhere exercises the wire path an inspector click actually takes for these two
entities.

**Fix:** add an end-to-end test per §1 that builds the row, simulates the host's `"value"` merge, runs
it through `command_from_action`, and asserts the decoded command's edited field equals the new value
(the five new `set-*-property` verbs already have this kind of coverage per lane C's §"InspectorVerbs"
tests — the same pattern should cover site/thermostat).

## Verified correct (spot-checked, no defect found)

- Sign convention and settled-temperature timing of the new per-surface conduction accumulator
  (`unit_commit`/`unit_settle`, kernel.rs) — `work.free_temp_c` and `inside_c` really are the settled
  values by the time `accumulate_conduction` reads them; matches the lane's own two-ULP measurement.
- `close_step`'s new `per_surface.pop()` retirement step is correctly inserted before `self.state =
  None`, after `windows.pop()`.
- `🚪️delete-fenestration/↩️inverse` and `🪚️delete-surface/↩️inverse`: the reversed-order replay
  reasoning holds up under manual trace (double reversal in the surface case really does cancel out
  to build order = execution order; the fenestration case's single reversal really does put
  `create-fenestration` before `replace-fenestration-vertices` at execution time).
- Taxonomy: independently confirmed `🎬️scene` is a registered member of `members-of-engine` and
  `🧊️model` a registered member of `members-of-windows` in
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.
- Viewer purity: `👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs` has no `use crate::editor` /
  `editor::` reference.
- The `energy_surface_payload_bytes`/`encode_surface_energy_payload` wire format and its 8 192-row
  truncation are internally consistent (already self-flagged by lane D as a known silent-truncation
  limit, not a new defect).

## Top 5 cleanups

1. **Leftover dead branch** — `if false { return Vec::new(); }` in
   `🧬️mutations/🚪️delete-fenestration/↩️inverse/🦀️.rs:13`, immediately after an equivalent early
   return two lines above. Harmless but confusing; delete it.
2. **`eprintln!("[DEBUG] ...")` scaffolding still in production fault paths** — `⚙️engine/🧪️sim/🦀️.rs`
   has ~24 remaining `eprintln!` calls in `begin_fault`/reservation paths (e.g. lines 2425, 2953,
   2960, 3120, 3280...). This lane already cleaned four sibling call sites (kernel admission of the
   new `per_surface` table) but left the rest; worth one pass to route through the existing
   `Fault`/diagnostics machinery instead of stdout prints that fire in wasm builds.
3. **`model_edit`'s probe still clones instead of projects for materials/thermostats/zones/surfaces**
   (`✏️editor/🦀️.rs`, `probe.materials = model.materials.clone()` etc.) — self-acknowledged by lane C.
   Concretely live for `Material::roughness`, which has no mutation kind at all: nothing today writes
   it, but the moment something does, it will vanish silently instead of loudly faulting, which is
   exactly the bug class this ticket exists to close everywhere else.
4. **Zone-volume convex hull has no memoization** (`⚙️engine/🎬️scene/🦀️.rs`,
   `convex_hull_triangles`/`zone_hull_points`, added mid-review). It recomputes an O(n²) point dedupe
   plus an incremental hull (up to the 4 096-point refusal ceiling) on every 3d-window render with no
   caching keyed on the zone's geometry. Bounded, but for a model near the ceiling combined with a
   running simulation that republishes tick payloads (forcing a re-render to recolour), this is worth
   profiling against the reactor's per-turn budget rather than assuming "bounded" implies "cheap".
5. **Inconsistent inspector-verb shape**: `set-material-property` still takes `value: f64` while the
   five newer verbs take `value: String`, so material name/roughness stay unreachable from the
   inspector (already noted by lane C, listed here only because it compounds finding #3 — the one
   field with the masking gap is also the one field with no reachable control, so the gap is currently
   silent both ways).

## Not independently re-verified (time-boxed out)

Deep correctness of the new convex-hull geometry algorithm (horizon/visible-face bookkeeping) and the
non-rectangular epJSON aperture area/height derivation were read but not hand-traced against a second
example; both look structurally sound and are covered by the lane's own unit tests. The 10 new
mutation-kind boilerplate files (glazing/gas property changes, `replace-fenestration-vertices`) were
spot-checked via the inverse-ordering trace above; the remaining 8 were not individually re-derived
beyond confirming they follow the same copy-paste shape lane A describes.
