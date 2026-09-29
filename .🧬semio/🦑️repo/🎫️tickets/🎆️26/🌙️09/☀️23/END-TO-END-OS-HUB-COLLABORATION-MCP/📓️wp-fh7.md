# WP-FH7 — Fault Class Review, Family CEFG

Slice: FH7 (session 15, helper of S20; spec `📓️fault-localization-api.md` §7, consistency rules of `📓️wp-fh6.md` as relayed by
the coordinator). Work list `.🧬semio/🌐hub/s14-s20-sets/class/family-CEFG.json` (1 227 codes, 20 plugins, 271 `rule: default`).
Only the `class` value was edited, one entry per line. A check confirmed that owner, code, rule and en are identical to the
baseline and that 582 lines differ. No source file or overlay was touched. Tools in `wp-fh7/`:
- `fh7-show.py` prints each entry with its census raise sites (enclosing fn and context from the overlay the census indexes).
- `fh7-decisions.tsv` holds every reviewed decision: owner, code, class and a one-line reason.
- `fh7-apply.py` writes the decisions byte-stably and prints the counts.
- `fh7-baseline.json` is the pre-review list.

## Session 15

### Counts

| class | before | after |
|---|---|---|
| internal | 239 | 531 |
| input-invalid | 372 | 342 |
| precondition-failed | 526 | 301 |
| unavailable | 55 | 29 |
| conflict | 32 | 22 |
| permission-denied | 0 | 2 |
| cancelled | 3 | 0 |

582 changed. Transitions: precondition-failed→internal 201, precondition-failed→input-invalid 112, input-invalid→internal 81,
input-invalid→precondition-failed 73, unavailable→internal 31, conflict→precondition-failed 21, precondition-failed→unavailable 10,
internal→input-invalid 10, precondition-failed→conflict 9, internal→precondition-failed 8, unavailable→precondition-failed 7,
internal→unavailable 6, unavailable→input-invalid 4, cancelled→internal 3, input-invalid→conflict 2,
precondition-failed→permission-denied 2, conflict→internal 1, internal→conflict 1. I judged all 271 `default` entries against
their raise sites and changed 231 of them.

Changes per plugin: puzzle 168, sequence 49, raster 47, flow 45, space 40, procedural 37, mathematical 27, lowpoly 21,
forms 21, wfc 20, trinity 17, reasoning 16, note 13, process 11, cad 10, gis 9, vcs 9, architect 8, norm 8, layout 6.

### Rules applied (by what the raise site checks)

- **Retained-work protocol → internal.** This covers:
  - own checkpoint bytes (`*-checkpoint-{invalid,cursor,identity,extent,digest,…}`), because the framework already checks
    the context digest and workspace identity before `restore`
  - checkpoint target buffer (`*-checkpoint-capacity`)
  - step after Complete (`*-complete-repolled`, `*-work-terminal`, `*-is-terminal`, `*.finished`, `*-repeated`)
  - step after `begin_close` (`*-closing` at work level, `lowpoly-retained-work-closing`)
  - cursor/owner/phase invariants
  - route fallthroughs (`*-route-rejected`, `*-route`, `*-unmapped`, `*-command-mismatch`)
  - batch handlers that always refuse, because the command always routes retained (`*requires-retained-job`,
    `*-job-only`, `*-background-only`, `*-requires-retained-work`, `*-retained-context-required`,
    `*-requires-retained-window-owner`). I checked the retained routing: it keys on the tool id alone (raster, forms).
- **Bundled example/template fails to parse → internal.** Choosing another example does not repair a broken bundled asset.
- **Our own document data unreadable → internal.** This covers typed decode of rows of the app's own store (puzzle
  `*-malformed`, `*-not-object`, `*-nodes-missing`). Caller payloads that do not decode stay input-invalid (`*-json`, file
  imports, `eventsJson`, mesh pages, base64).
- **Host-built requests that do not parse → internal**, not cancelled. This covers `extension.*-cancel.bad-request`,
  `gis.gismap.inference.cancellation`, `*.inference.snapshot-decode` and contribution paging.
- **Size limits:**
  - the request's own payload or arguments → input-invalid
  - the stored document (whole-document scans, catalogs, per-object caps, export budget) → precondition-failed
  - interaction selection → precondition-failed, by analogy with "nothing selected"
  - selection ids taken from the command → input-invalid
  - `try_reserve` failures sized by the change, where the text says "make a smaller change" → input-invalid
  - mixed cases follow the text's remedy
- **Window no longer open (`*window-stale`, `*route-stale`) → precondition-failed** (FH6 rule). Window transient or
  config not created yet ("wait until it has loaded") → unavailable. Content child not loaded yet → unavailable.
- **Document element lookup failing (unknown tile/rule/slot/edge, studio, layer) → precondition-failed** (element gone).
  An unknown name from static vocabulary (action, example, kind, format, strategy) → input-invalid.
- **Missing or empty argument → input-invalid** (FH6 rule). This includes `*media-required` / `*input-required`, where the
  reserved input carries the wrong variant.
- **Expected-revision checks → conflict.** This covers `expected_image_key` mismatch, generation or freshness drift
  (lowpoly), replay digest drift (equation, lowpoly paint), superseded run (`jack.query.owner-changed`), directory feed
  frontier race and forms continuation base revision.
- **No signed-in session identity → permission-denied** (`s.home/s.space.session-identity-required`).

### Judgement calls (one line each)

- `*-closing` (31 puzzle codes): `begin_close` sets stage Closing, and the scheduler never steps a closing job → internal.
  The texts say "wait a moment", so S20 may retext them.
- `*eval-session-closing` / `*preview-eval-session-closing` (flow, procedural): the instance owner is closing (document
  teardown or restart), not a job → unavailable, kept per text.
- `puzzle5d-planner-tick-admission` → unavailable: the preview payload admission was rejected (pool full), so a retry can
  succeed.
- `puzzle5d-brush-run-search-missing` and `fill-run-planner-missing` → internal: the inner 3d builder returns None only on a
  tool mismatch.
- `flow.widget-id-unavailable` → input-invalid: the new name is empty or taken. It is not transient.
- `wfc2d.id.taken`, `forms.question.duplicate-question`, `forms.choice.duplicate-option` → input-invalid: choose another
  id or value.
- `raster.pixel-selection.dimensions-changed` → input-invalid: it runs after the image-key check passed, so the selection
  arguments are wrong (FH6 matched-revision rule).
- `cad.apply-transformation-unavailable` → precondition-failed and `cad.import-object-unavailable` → input-invalid: both are
  permanent feature limits, not transient.
- `s.home.create-studio.filesystem-unavailable` → precondition-failed: the device has no filesystem, which is permanent.
- `architect.register-not-addable` / `not-removable` → input-invalid: the register kind (static schema) does not support the
  operation.
- `equation-work-replay-drift`, `lowpoly-retained-paint-replay-digest` → conflict: the replayed state moved underneath.
- `jack.query.results-window-required` → internal: it is unreachable after the tool check.
- `*-mismatch-or-capacity` (trinity, architect) → input-invalid: the payload cap is the only cause a caller can reach.

### Ambiguous → S20

- `lowpoly-retained-checkpoint-identity-mismatch`, `process3d-retained-checkpoint-extent-mismatch` (set internal): these are
  conflict if a base-revision change can reach `restore` past the framework's context-digest check.
- `generation3d(-view)-flow-eval-window-owner-mismatch` (kept internal): the payload window differs from the context
  transient. That is input-invalid if agents send FlowEvalTick themselves.
- `flow-duplicate-source-stale` (kept conflict): retained inputs are immutable, so this may be internal.
- `cad.preview.invalid` (set conflict): the engagement session in config moved away from the runtime derived from it. It
  may be internal.
- `puzzle5d-planner-suggestion-kind/-grip` (kept precondition-failed): the preview names a kind or grip that is gone. This
  is conflict if the catalog changed concurrently.
- `puzzle5d-planner-host-grip` (set internal): the text suggests a legitimate "choose another part".
- IO faults kept precondition-failed: `s.home.*.io-failed`, `*.catalog-refused`. They could be unavailable when the fault is
  transient.
- Pipeline faults with a `{reason}` kept precondition-failed: `raster.bake.failed`, `composite.failed`, `export.failed`,
  `pixel-edit.failed`, `generation3d.mesh-edit.failed`, `transform.failed`, `flow.edit-refused`, `wfc2d-solve`,
  `wfc-bitmap-solve-failed`.
- `gis.gismap.inference.budget` (kept unavailable, per the §7 "budget" wording): this is precondition-failed if the host
  budget is fixed.
- Window-context arguments that are empty (`forms-next/previous-step-window-required`, `flow-eval-*-window-required`, kept
  precondition-failed): FH6's empty-argument rule would make them input-invalid.

### Same code, different reasons (never split)

- `flow-retained-child-group-payload-capacity`, `flow-retained-graph-payload-capacity`,
  `sequence-retained-artifact-envelope`, `sequence.retained.config-command` / `example-command`: payload admission vs
  tool mismatch / completed.
- `sequence-persistent-progress-capacity`: completed / tool mismatch / step cap.
- `wires-window-drag-work-capacity`: consumed vs document scan cap.
- `lowpoly-retained-paint-buffer-capacity`: before/after length mismatch vs texture cap.
- `flow-retained-extension-capacity`: stored map size vs id length.
- `raster.pixel-edit.failed`: job construction vs encoder.
- `vcs-edit-input-capacity`: text length vs stored tag count.
- `wfc-bitmap-unknown-palette-color`: pin payload vs stroke color.

### Texts contradicting their trigger (retext candidates for S20)

- `flow-retained-extension-item-capacity` ("name too long"): the trigger is a full extension map.
- `flow-retained-preview-off-item-capacity` ("name too long"): the trigger is a full preview list.
- `equation-command-json-decode` ("could not be read"): the trigger is a command-variant mismatch.
- `equation-work-extent-overflow`, `puzzle3d/5d-kind-weight-changed-owner`: these are owner or invariant faults worded as
  user-facing.
- All puzzle `*-closing`: the texts say "wait a moment".
