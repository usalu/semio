# 🧱️ PDF descriptor size and instance retirement (2026-11-25)

Follow-up to `🌐️pdf-browser-journey.md` §"End-to-end verification, round 3", remaining failures 3 and 4.

## A. Descriptor: edit verbs declared once per app

### Cause

`inspector_window_definition` (`pdf.object`) copied the page window's whole action list, so each of the seven PDF 1.7 editors carried the ~80 edit verbs twice (page window and Object pane). The thumbnail strip had the opposite defect: it binds `move-page` (drop and the "move earlier/later" rows), but `move-page` was claimed by `pdf.page`. The framework resolves a verb for a window as *the window's own roster, then every app-roster verb that no window claims* (`window_kind_actions`, TS twin `resolveWindowActions`), so the strip's `move-page` was refused (`window kind pdf.thumbnails does not own action move-page`).

### Fix (the schema's own sharing mechanism)

The descriptor schema already allows sharing inside one app: `AppDefinition.actions`. It is exactly for verbs no window owns (DS1 in its docstring).

- `🖼️page/🪟️windows/🦀️.rs`: `window_definition()` (`pdf.page`) now declares only what the canvas alone dispatches: three pointer gestures, the ten view tools and the presence lane. `edit_action_definitions()` is the document edit catalog. `edit_actions_with(builder)` declares it **once** on the app roster. `inspector_window_definition()` declares no actions of its own.
- Builders: `create_pdf17_editor` (base; drops the generic `insert-image` in favour of the PNG verbs as before, now on the roster), the `pdf17_editor!` kit (six subset editors), and the `pdf14_editor!` kit through `canvas::edit_actions_with` (1.4 page domain: `set-text`, `insert-page`, `remove-page`, `move-page`, `set-page-size`; `canvas::window_definition()` is now the shared page window).
- New law `every_binding_a_pdf17_pane_renders_resolves_through_its_own_window` (page tests). For the base editor and a kit editor it checks:
  - every binding the Object pane renders resolves through `pdf.object` (every sample object selected, every page, no selection; en and de);
  - every binding of the thumbnail strip resolves through `pdf.thumbnails`;
  - no window claims an edit verb;
  - `set-info-field` is declared exactly once.
- Updated tests: the seven subset `definition_declares_the_page_canvas` tests, `authoring_actions_are_declared_with_english_and_german_labels`, `every_typed_snapshot_field_has_a_declared_control_action`, and 1.4 `canvas_window_offers_exactly_the_fixture_actions` (the window's own actions plus the roster edits equal the fixture, which is unchanged).

### Sizes (pretty JSON form, the one the 4 MiB catalog bound refuses)

| Section | committed (stale) | before fix (inspector copies) | after fix |
| --- | ---: | ---: | ---: |
| whole descriptor | 3,656,980 | ≈ 6,841,000 | **4,495,164** |
| `manifest.apps` | 3,207,355 | ≈ 6,003,000 | 3,918,556 |
| one 1.7 editor (`@1.7/a#editor`) | 340,897 | ≈ 671,900 | 392,854 |
| … its `actions` (app roster) | 63,605 | ≈ 65,700 | 234,308 |
| … its `windowKinds` | 245,477 | ≈ 540,600 | 111,597 |
| … `pdf.page` | 197,168 | ≈ 221,600 | 36,332 |
| … `pdf.object` | 7,365 | ≈ 221,600 | 13,258 |
| pack form | 757,233 | – | 967,916 |

The "before" column is reconstructed from the emitted pair by moving the edits back into `pdf.page` and copying them into `pdf.object`. The emitter run of round 3 refused the real pair before writing sizes. The committed pair predates the view and presence actions, the thumbnail and text-layer windows, and fault notices.

### Still over the bound: duplication the schema forces across apps

After the fix the pair is 4,495,164 bytes. That is 300,860 bytes over `CATALOG_DESCRIPTOR_MAX_BYTES` (4,194,304), so the committed pair could not be regenerated. The `🛂️committed-descriptor` test now says so first, with both sizes and the path of the described pair (`CARGO_TARGET_TMPDIR/stdio-pdf-described`). It still fails, as it must.

The remainder is cross-app duplication, which the schema cannot express:

- Measurement: all action definitions in the descriptor total 2,698,934 bytes, but only 312,819 bytes are distinct.
- Sources:
  - Ten PDF editors (7 × 1.7, 3 × 1.4) each carry the same edit catalog and framework roster.
  - Every window kind carries the same six framework-reserved actions (`cancelTypedOperation`, `hostEvent`, `exportArtifactDocument`, `importArtifactDocument`, …). `try_build_definition` copies them into each window instead of keeping them on the app roster.
- There is no plugin-level place to declare an action once.

Required next step (framework schema, not done here):

1. Add a plugin-level action table, e.g. `PluginManifest.actionSets[{id, actions}]` plus `AppDefinition.actionSets[id]`, in the Rust manifest, its owned schema and the generated TS. Resolve it in `window_kind_actions` and `resolveWindowActions`.
2. Move the reserved window actions to the app roster.

Estimated result: about 2.1 MB for `stdio-pdf`. Every plugin that has an editor and a viewer gets smaller too. Both changes alter the emitted pair of every component, so every committed descriptor has to be regenerated with `repo run workspace:components -- describe --all`. That needs the component wasm builds this session is not allowed to run.

Regeneration path for this pair once it fits:

1. Run `committed_descriptor` (`run-committed-descriptor.sh`). It writes the described pair.
2. Run `bun settle-pdf-descriptor.ts <dir>` from the workspace root. This does the same verify and `write` settle as `components describe`.

## B. Instance retirement after surfaces were requested

### Cause (found with the close census)

`shard_close_census` is printed when an instance does not retire. It showed:

- the close ladder admitted and active;
- the close worker alive;
- the app cell unshared;
- the surface registry not empty: `displaced=16…39`, a retirement in flight, `slots=0`, no capture lease.

Every `Event::SurfaceVisible` refresh displaces the previous `ViewModel` projection. Two defects in `SurfaceContexts` (`⚛️reactor/🪟️surfaces/native.rs`) kept those projections alive:

1. **One unit per call.** `retire_displaced_step` (live, once per kernel turn) and `close_original_step` (close job, once per job step) each retired exactly one unit (about one copied item) per call. A `ViewModel` retires in hundreds of units. Refreshing panes therefore displaced views faster than they retired: 39 were pending after 20 surface requests. The close needed far more than 16,384 host turns, and in the browser the backlog grows without bound while panes refresh.
2. **Latched body.** `closing_body` latched the copy budget of the first retirement call's grant (509 MB under a wide loan). It then priced every later step with it, so a narrower later grant could never admit the step.

### Fix

- `retire_displaced_step` and `close_original_step` loop over single units (`retire_displaced_unit`, `close_original_unit`). They keep going for as long as the caller's grant funds them and nothing yields, bounded by `SURFACE_RETIREMENT_UNITS_PER_STEP` (4,096). Every unit is still paid exactly as before.
- `closing_body` is removed. Each unit prices its retirement from the current grant's copy budget (a ticket's `next_demand(copy)` only bounds the unit size, so a smaller budget is valid).
- `SurfaceContexts::close_census()` reports the capture lease, source, slots, view and displaced projections for stall reports.
- Test harness: `ShardShell` releases its retained surface states before the UI fixture arena closes. The surface laws now reach that point, which no run had reached before.

### Result

`every_declared_pdf17_window_publishes_its_surface_before_and_after_the_document_mounts` and `…pdf14…` pass: 20 of 20 surfaces published per version, then hidden, closed, `Retired` and released, in 88 s for both together. The generic part stays in the PDF test module. Lifting it into `artifact_app_laws` would rebuild the whole framework test chain under the memory gate, so that is left as a follow-up.

## Verification

Native, through `native-cargo.sh` + `watchdog-cargo.py`, filtered, with the temporary traces removed:

- Page tests: 4/4 (`every_binding_a_pdf17_pane_renders_resolves_through_its_own_window`, `page_window_declares_object_selection`, `selected_object_fields_commit_through_the_value_argument`, `thumbnails_scale_each_page_and_reorder_through_move_page`).
- Authoring tests: 2/2.
- Surface laws (`every_declared_pdf17_window_…`, `every_declared_pdf14_window_…`): 2/2.
- Editor and viewer assembly of all 13 editors and viewers: 20/20.
- 1.4 `canvas_window_offers_exactly_the_fixture_actions`: 1/1.
- `committed_descriptor`: fails as intended with the size verdict above (4,495,164 > 4,194,304).
- `media_in_of_a_small_pdf_reaches_its_mount_through_the_real_runtime_under_the_shard_budget`: fails with "the shell received no typed-operation result page" (MediaIn completes in 64 turns without a result page). This law requests no surface and touches neither action ownership nor surface retirement. It fails while other sessions are editing the typed-operation and media-in path (`[DEBUG] lifetime work` traces in the same binary are theirs), so it has not been attributed to this change and is left to that owner.
- Not run: the seven subset `definition_declares_the_page_canvas` tests did not match a filter in these crates.
- Several builds were SIGKILLed by memory guards, or blocked by a temporarily uncompilable lift module from another session, and were rerun.

## Changed files

- `✏️s/…/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🪟️windows/🦀️.rs`
- `…/🧱️base/✏️editor/🦀️.rs`, `…/✏️editor/🧰️kit/🧾️pdf17/🦀️.rs`, `…/✏️editor/🧰️kit/🧾️pdf14/🦀️.rs`
- `…/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🖼️page/🔭️canvas/🦀️.rs` and its `🧪️tests/🔬️unit/🦀️.rs`
- `…/🧱️base/✏️editor/🖼️page/🧪️tests/🔬️unit/🦀️.rs` (binding law), `…/🧪️tests/🔬️unit/✍️authoring/🦀️.rs`
- seven `…/🪆️subsets/*/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs`
- `…/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (close census, shell release)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/native.rs`, `…/🪟️surfaces/🦀️.rs`, `…/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs` (docstring)
- `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧪️tests/🛂️committed-descriptor/🦀️.rs` (bound check, described pair written for settling)
- Ticket: `run-descriptor-retirement.sh`, `run-committed-descriptor.sh`, `settle-pdf-descriptor.ts`, this file. Temporary `[DEBUG]` traces in `🔁️cleanup-turns` and `🧯️cleanup-pumps` were added and removed (both files are back to their committed content).
