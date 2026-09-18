# 🖼️ B1 — the bitmap playground, made to work in the browser

Slice B1 of `EXTRACT-WFC-PLUGIN`. Served by the coordinator at `http://127.0.0.1:6041/?plugin=wfc`
(react renderer, variant `bitmap`). Every command below ran in the foreground, `-j 4`, one cargo at a
time; logs under `🗑️generated/playground-bitmap/`.

---

## 1. Verdict

| gate | result |
|---|---|
| `cargo test -p semio-s-artifact-wfc-bitmap --features component-app-assembly --lib -j 4` | ✅️ **185 passed, 0 failed, 1 ignored**, zero warnings from this crate (`test-final.txt`) |
| `cargo check -p semio-s-plugin-wfc --lib -j 4` | ✅️ green (`plugin-check-final.txt`) |
| `cargo test -p semio-s-plugin-wfc --test close_ladder -j 4 -- --test-threads=1` | ✅️ **11 passed** — L's hooks survive this slice's editor rewrite (`close-ladder.txt`) |
| `activate-bitmap-react-dev` (wasm component) | ✅️ green ×4 restages (`activate-{4,5,6,7}.txt`) |
| `🐍️bitmap-console-dump-probe.mjs` (boot) | ✅️ **0 fault lines**, both hosts render a canvas |
| `🐍️bitmap-interact-probe.mjs` (7 steps) | ✅️ 0 fault lines on 6 of 7 steps; the 7th carries only the framework's own `shell.windowActivate` notice (§5.1) |
| `🐍️bitmap-viewer-probe.mjs` | ✅️ both viewer windows render a canvas; one framework-side notice (§5.2) |

The bitmap editor now boots without a refusal, loads either example from the navbar, paints with the
armed palette colour as ONE edit per drag, undoes that edit, pins a pixel, and **collapses the
overlapping model into a visible output bitmap** that is locally similar to its input.

---

## 2. What was broken (six real defects, in the order they surfaced)

The slice opened with the editor's whole ROSTER declared, classified `Migrated` and unit-tested — and
every single verb dispatch-dead in the live shell. Each fault below hid the next one.

### 2.1 No `command_from_action` → the whole action roster was dead
`VcsArtifactApp::dispatch_action` (`🔌️plugin/🦀️.rs:26367`) resolves EVERY host action through
`A::command_from_action`, whose `ArtifactEditor` default answers a hard
`app.command.unsupported`. The editor overrode neither it nor `command_id`, so `set-input-pixels`,
`solve`, the three stroke verbs and everything else were unreachable from the Actions pane — with a
manifest and a classification audit that both looked perfect.
**Fix**: `args_bridge::command_from_action` (kebab ids for the document/gesture verbs, camelCase
`setActiveExample`) + `command_id` off the new `bitmap_command_id`.

### 2.2 No app-owned retained factory → every verb refused before the handler
`qualified_tool_proof` (`🔌️plugin/🦀️.rs:21752`) refuses a verb that has only a generic bounded proof
(`interactive-job.missing-owned-reducer`), and `require_complete_tool_operation_pipeline` refuses any
proof that is not `AppOwned`.
**Fix**: `BitmapCommandJobFactory` (one `ArtifactOwnedToolJobFactory` over all sixteen ids),
`register_tool_job_factories`, `build_tool_job`, `bounded_first_step_tool_proofs!` with
`factory_type:`, `BITMAP_PUBLICATION_CONTRACTS` (Artifact / WindowConfig / Transient per verb) and the
document-lane publication authority.
**Trap inside the trap**: `validate_tool_job_rows` joins the proofs against
`<A::Command as OpBinary>::TOOL_JOB_IDS`, whose trait DEFAULT is `["typed-command"]`. Without
overriding that const the expected set is EMPTY and every proof is rejected with
`interactive-job.catalog-authority`. `BitmapEditorCommand` now declares `TOOL_JOB_IDS`.

### 2.3 `setActiveExample` undeclared → dead picker AND a boot refusal
The shell's navbar picker and its automatic boot announcement both dispatch the exact id
`setActiveExample`; the app-wide undeclared-action gate dropped it
(`semio: app "s.wfc.bitmap@1/*#editor" dropped action "setActiveExample"…`).
**Fix**: an app-level `.action_with(...)` + `.action_args(exampleId: select)` +
`.action_interactive_job(Migrated)` (`try_build_definition` copies an unowned action onto every window
kind, which is what the app-wide gate wants), a `SetActiveExample { example_id }` command variant, and
`✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs` — a **declared-state diff**, not a document replace.
Re-selecting the document's own example answers an EMPTY mutation set (`canUndo` stays false), which is
pitfall #3 of the ledger fixed by construction.
Ordering is the whole difficulty and is asserted by a unit test: release the pins that do not survive →
GROW the palette → recolour the shared prefix → resize the sample → rewrite the buffer → SHRINK the
palette (now provably unused) → output → model → seed → the example's own pins. `remove-palette-color`
is fatal while its colour is painted or pinned, and `set-input-pixels` is fatal on an index the palette
does not hold, so any other order refuses.

### 2.4 The transient's envelope id PANICKED the guest on the first solve
`BitmapTransient` declared `#[dsl(extension = "wfcbitmaptransient")]` and no `id`. `DslArtifact` falls
back to the extension for `__DSL_ENVELOPE_ID`, and `SemioEnvelope::from_envelope_id` refuses an id
without a `.` — so `ArtifactDsl::print_dsl`'s own `.expect(…)` aborted the guest the moment the
framework published the solve on the transient lane, and every later dispatch in the session answered
`unreachable`.
**Fix**: `#[dsl(id = "wfc.bitmaptransient")]`. The other nine wfc transients/configs already carry a
dotted id; this was the only one. A regression test prints and packs the transient and asserts the
preamble.

### 2.5 The document lane's one-item authority under-declared two mutations, and masked its own errors
Two of this artifact's ten verbs are NOT point-invertible — `resize-input` answers `resize back` PLUS a
full-buffer restore, `resize-output` answers its prior spec PLUS every pin the shrink cascaded — so the
SDK's `bounded_config_store_one_item_preparation_factory` (a flat `for_one_invertible_item`, two rows)
under-declares them for `ArtifactStore::fold_batch_item`. Worse, that factory CONSUMES its mutation
owner before its fallible steps, so the store's second admission attempt reports
`…-mutation-owner-missing` and the real first failure is printed nowhere.
**Fix**: `BitmapOneItemPreparationFactory` in `✏️editor/🦀️.rs` — `for_one_item(bitmap_inverse_rows(m), …)`
per kind, and a sticky `failure` so a retry reports the real cause.

### 2.6 The solve itself was quarantined in the guest — while the native tests stayed green
With everything above fixed the Solve verb dispatched, published, refreshed… and the output window
stayed black. Made honest (§3), the guest answered `wfc-bitmap-solve-failed:job-session.terminal-fault`.
`HEADLESS_STEP_BUDGET_US` was **50 000 µs** against
`semio_framework_trace::INTERACTIVE_STEP_CEILING_US = 8 000`: every session step reported a contract
violation, and `SUSTAINED_OVERRUN_QUARANTINE_STEPS = 4` consecutive violations quarantine the whole
session with the pre-admitted `job-session.terminal-fault` page. A test binary installs no monotonic
clock, so `StepOverrunLedger` records no violation at all and all 185 native tests passed over the same
constant.
**Fix**: `HEADLESS_STEP_BUDGET_US = 4_000` (`🧬️schema/💡️inferences/🦀️.rs`). The budget bounds one step,
never the collapse — the driver resumes until the job is terminal.

---

## 3. What else changed, and why

- **`Solve` emits `ui_scope: UiDirtyScope::Full`.** A transient publication carries no document
  revision, so with the default scope the collapse landed in the store and the output window kept
  rendering the buffer it already had.
- **`solve_transient` returns `Result`.** An UNSATISFIABLE problem is a real transient with no pixels
  (the document's own answer); an inference that could not RUN is a FAULT. Folding the two together is
  precisely what made a quarantined solve look like a black square with no diagnostic anywhere.
- **`ArtifactEditor::ephemeral` is documented as dead.** Its doc says "called on every dispatched
  command", but nothing in the framework invokes it (`grep` over the whole tree finds only the two
  delegating impls). The ephemeral lane is reached exclusively through a retained work step's
  `ArtifactCommandWorkStep::CompleteWithEphemeral`, which is where the solve is published now.
- **Staged argument forms** on every window action that takes one (`x`/`y`, palette channels,
  `patternSize`/`symmetry`, …), so the Actions pane can drive the whole vocabulary.
- **Five new unit tests** (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`, region `🚚️LiveDispatch`) + four for the new
  command: transient envelope round trip, a real collapse on the transient lane, every declared action
  bridges to the command it names, the four-way roster agreement (`TOOL_JOB_IDS` = factory `TOOL_IDS` =
  publication contracts = bounded-first-step proofs = window declarations), and the example picker's
  declaration. 176 → **185**.

---

## 4. Probes (all cloned into `$T`, remodel-derived)

Run from the ticket folder, headless chromium with `--use-angle=metal`.

| probe | command | result |
|---|---|---|
| `🐍️bitmap-console-dump-probe.mjs` | `SEMIO_PROBE_OUT=playground-bitmap/boot bun 🐍️bitmap-console-dump-probe.mjs` | shell ready `bitmap`, both hosts 1 canvas, `combobox: "Rooms 16"`, **faults: 0** → `🗑️generated/playground-bitmap/boot/final.png` |
| `🐍️bitmap-interact-probe.mjs` | `SEMIO_PROBE_OUT=playground-bitmap/interact bun 🐍️bitmap-interact-probe.mjs` | seven steps, see below → `🗑️generated/playground-bitmap/interact/` |
| `🐍️bitmap-viewer-probe.mjs` | `SEMIO_PROBE_OUT=playground-bitmap/viewer bun 🐍️bitmap-viewer-probe.mjs` | role → Viewer, both windows 1 canvas → `🗑️generated/playground-bitmap/viewer/viewer.png` |

The interact probe measures a **distinct-colour census** of each pane's own screen region (a screenshot
decoded through a 2D canvas inside the page — a WebGL drawing buffer is not readable after
compositing). A blank output is one flat dark rectangle; a real collapse carries the input's palette.

| step | evidence | screenshot |
|---|---|---|
| boot | output census top colours `#f0ecdd 0.85 / #e3e1d3 0.07 / #171a1f 0.052` — `#171a1f` IS the empty 24 × 24 extent | `interact/1-boot.png` |
| `change-seed` (staged form, seed 4242) | executed, 0 faults — the artifact lane publishes | `interact/2-change-seed.png` |
| **`solve`** | `#171a1f` GONE; `#e2ded2 0.0299` (floor) + `#26262e 0.0118` (wall) + doors — the rooms palette, 260 distinct | **`interact/3-solve.png`** |
| **paint** (`set-active-color 2` → `stroke-begin 6,6` → `stroke-extend 9,9` → `stroke-commit`) | one orange 4 × 4 block at (6,6)–(9,9) in the input, from ONE `set-input-pixels` | **`interact/region-after-paint-input.png`** |
| **undo** (`mod+z`) | the block is gone on the FIRST press | **`interact/region-after-undo-1-input.png`** |
| `pin-pixel 3,3 colour 2` | the pin marker appears over the cached collapse | `interact/region-after-pin-output.png` |
| example switch → **Flowers 24** | input becomes sky/ground/stem/petal (`#94c6e8 0.097`, `#607840`), combobox follows, 0 faults | `interact/7-example-switch.png` |
| **`solve` on flowers-24** | stems with petal crowns on the ground band under sky — the flowers palette (`#94c6e8 0.052`, `#607840`) | **`interact/8-solve-second-example.png`** |

`faultLines()` (the remodel regex over `trapped|panicked|action failed|shell fault|faults=|pageerror|
unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey`) is EMPTY for
every step except `undo` (§5.1).

---

## 5. Remaining gaps

1. **`shell.windowActivate` is refused by the app-wide undeclared-action gate.** The SHELL dispatches
   it itself (`🏛️ShellHost/🟦️.tsx:10395`, through `noteShellCommand`) whenever a pane is focused, and the
   gate then drops it because no window kind of the app declares it. It is a framework chrome command,
   not an app verb; nothing in the app is affected (the pane activates anyway). Framework-side, and it
   will fire identically on every other plugin — recorded, not worked around here.
2. **The VIEWER still takes the boot `setActiveExample` refusal** (pitfall ledger #5). The viewer's own
   `initial_snapshot` already parses the committed default example, so the render is correct; the
   refusal is the shell announcing an example to a read-only surface whose only command is `Noop`.
   Declaring the verb on the viewer would mean giving a read-only surface a retained document route,
   which is worse than the notice. The ledger's own answer is the ShellHost-side skip.
3. **No automatic solve.** The output window is blank until `Solve` runs, which is what the artifact's
   own design states (`📓️bitmap.md` §5: "a `Solve` command recomputes it; every other command leaves it
   alone"). An auto-solve on document change would be one line in `BitmapCommandWork::step`, but every
   paint stroke would then pay a collapse; left as an explicit verb on purpose.
4. **`resize-output`'s declared pin cascade is a structural bound** (`BITMAP_PIN_CASCADE_ROWS = 1_024`).
   `preflight` never sees the base, so a document pinning more than that cannot shrink its output in one
   gesture. Honest refusal rather than a half-applied cascade, but a real ceiling.
5. **`describe` was not re-run for the new actions.** `cargo test -p semio-s-plugin-wfc --lib`'s
   `descriptor_is_fresh` is red for the whole B wave (L recorded the same); the committed descriptor
   needs one `bun nx run @semio-tech/wfc-plugin:describe` once the B slices settle. The SERVED dev
   descriptor is regenerated by `activate-bitmap-react-dev` on every restage and does carry
   `setActiveExample` (verified by `curl`ing the served `🔣️.json` and counting it).

6. **`verify taxonomy enforce` is red for a repo-wide reason**, not for this artifact: `Current
   compiler input manifest compiler input bytes differ: bun.lock` — it never reaches the scope. Same
   class as `📓️bitmap.md` §8 item 1, reproduced identically when scoped to untouched folders. The new
   `✏️editor/🎮️commands/🎬️set-active-example` directory name is the one remodel already uses at exactly
   the same path, so no taxonomy row was added by this slice (its `📌️.empty.md` marker is gone, the
   folder is no longer empty).

---

## 6. Files this slice owns

- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — action bridge, retained factory + work, publication contracts, proofs, one-item preparation, manifest.
- `…/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs` (+ `🧪️tests/🔬️unit/🦀️.rs`) — the declared-state diff.
- `…/✏️editor/🫧️transient/🦀️.rs` — dotted envelope id.
- `…/✏️editor/🎭️modes/✏️edit/🪟️windows/{🖼️input,🧩️output}/🦀️.rs` — staged argument forms.
- `…/🧬️schema/💡️inferences/🦀️.rs` — `HEADLESS_STEP_BUDGET_US`.
- `🗿️artifacts/🖼️bitmap/🦀️.rs` — the `editor::bitmap::commands` mount.
- `$T/🐍️bitmap-{console-dump,interact,viewer}-probe.mjs`, `$T/🗑️generated/playground-bitmap/**`.
