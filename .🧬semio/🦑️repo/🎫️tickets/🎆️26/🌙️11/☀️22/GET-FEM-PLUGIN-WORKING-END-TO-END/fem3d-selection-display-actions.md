# 3D (and 2D) palette rows: selection, display, example and load actions

Own Chromium runs on 6087 (3D) and 6086 (2D) with the wasm of 19:52, outputs in `C:\t\sa` (deleted afterwards). Harness only: `matrix-v3/lib.mjs`, `matrix-v3/checks-model.mjs`. No product source changed.

## Verdict per row

| Row | Kind | Root cause | Fix |
|---|---|---|---|
| act.clearSelection | harness (timing) | the check waited a fixed 3.5 s; the selection round trip takes 0.4 to 10 s under load | poll until Inspection shows the Summary; precondition asserts a node is selected first |
| act.selectAll | harness (timing) | fixed 3.5 s wait read the Summary too early (0 selected) | poll for the selection; assert N SELECTED equals the Elements count of the Summary (the verb selects every target of the active granularity) |
| act.setSelectionMode | harness (wrong semantics) | `SelectionMode` is `Single` or `Multiple` and bounds how many targets one domain may hold (`next_selection`: Single keeps only the last target of a dispatch, Multiple allows Shift/Ctrl merges). A plain click never adds. The check asked for `Additive` and expected two plain clicks to give 2. The click timeout came from the 8 s click on a covered control | Single: pick, Shift+pick leaves exactly 1; Multiple: Shift+pick grows to 2 or more |
| act.setResultDisplay | harness (wrong window) | `setResultDisplay` is declared on the Results window kind (`window_kind_action_refs(FEM3D_WINDOW_RESULTS)`, 2D likewise), not on Model. Also `refId` clicked a tree row while the Results panel covered the tree (the 8 s timeout), and in 2D the fallback returned the previous selection (`ridge`) | `runAction(..., { win: "results" })` opens the Results window palette and discovers the window prefix from the DOM; asserts the Model palette does not list it; `refId` runs before the panel opens and waits for the LOAD CASE heading |
| act.setActiveExample | harness (wrong options) | declared options are Concrete Forest, Default, House (3D) and Demo (2D); there is no No example. An unknown id falls back to an empty snapshot. Passing a RegExp through `runAction` also became the string `/House/` | the check loads House, asserts the counts changed, then loads Default and asserts the counts are restored. Marked slow (see open point) |
| act.addNodalLoad, addMemberUdl, addAreaLoad | harness (state dependence) | the verify counted lines ending ` Load` in the tree text; new loads sit under collapsed or scrolled-off cases after earlier checks added cases | `loadKeys` expands every case, scrolls the whole tree and diffs the set of `previous line` + `row` keys; values are unique per run |
| act.setSelfWeight | harness (state dependence) | the wanted value was derived from the tree text of the case, and the tag lookup used the case id although rows start with the name. Result `enabled=true (was true)` with a tag that never changed in the check's view; the product does flip the tag (probe: `Live Load · 2 Loads` becomes `· Self Weight`) | read the tag of the second case row, ask for the opposite, assert it flips |
| act.unknown-reference-refusals | harness (state dependence) | all notices were correct and the counts unchanged; the first row (`addSupport`) matched a stale alert left by the earlier unknown-material check, so its text no longer contained `ghost` and the assertion failed | wait for an empty alert list before each action and only accept alerts that quote `ghost` or `different nodes` |

Product: nothing in these nine rows is a product defect. The coordinator's expectation that the notices exist is met (3D and 2D passes).

## Live results (after the fixes)

3D (6087): clearSelection, selectAll, setSelectionMode, setResultDisplay, addNodalLoad, addMemberUdl, addAreaLoad, setSelfWeight, unknown-reference-refusals PASS. 2D (6086): the same rows PASS when run in isolation (the first 2D run failed on 8 s click timeouts while another agent's 2D run loaded the machine, and `refId` returned a stale `ridge`; fixed in `refId`). setActiveExample: see below.

## Open point (product, not fixed here): House example blocks the next example switch

Live result: `3d.act.setActiveExample` still FAILS (House loads, counts 16 to 63 nodes after 98 s; Default restore not applied within 1000 s; in a driver probe it applied after more than 10 min). The console shows `guest linear memory at 355074048 B, 66% of the 536870912 B budget, past the 60% install-peak ceiling`. 2D (only Demo) passes.


After `setActiveExample House` the next `setActiveExample Default` is accepted (History shows it) but is only applied after the House analysis finishes, observed above 10 minutes under load (caption empty, counts frozen at 63 nodes, then 16 nodes). The example switch is an expensive job with no cancellation, which contradicts the "progress and cancellation for expensive operations" rule. It also explains the existing `3d.example.no-example-then-demo` failure. Needs profiling and a cancellable analysis job in the Rust/wasm layer; after that the slow flag of `act.setActiveExample` can drop. Separate small inconsistency: the Artifact Example combobox lists `Demo` and `No example`, the palette action lists `Default` and has no empty option, for the same example ids.

## Harness changes

- `lib.mjs`: `windowPanel(win)`, `openPalette(win, probe)` (uses `tap`), `actionPfx(id)` (reads the window prefix from the DOM), `runAction(..., { win })` with a per-call prefix, `refId` waits for the entity heading.
- `checks-model.mjs`: rewritten rows listed above, helpers `selected`, `loadKeys`, `newLoad`, `uniq`.
- Helpers kept in the ticket: `sa-driver.mjs` (like `pa-driver.mjs` with the app key as argument).

`run.mjs --only` matches an exact id, an id prefix ending in `.`, an area, or an inventory id; comma-separated full ids work as used here.
