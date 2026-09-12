# Wave B30 — the example id at boot, and the import lane

Implementation pass, 2026-09-12 early CEST, off battery **#50**
(`🗑️generated/battery-2026-09-12-50-6013.txt`, `🗑️generated/probe-2026-09-12T00-44-50.md`).

Two items:

1. `example-switch FAIL example=`, `undo-unwind FAIL example=`, `undo-redo FAIL example=`,
   `export-names-the-example FAIL download=puzzle-3d.json expected=.json` — the navbar example label read
   **EMPTY** on every one of them, including before any switch had happened.
2. `import-distinct FAIL before=1 after=1` while the guest taps said
   `puzzle3d.import.parsed objects=2 before=1` → `puzzle3d.import.apply ops=1 after_objects=2`.

**Headline** — item 1 is ONE product defect, host-side and vite-live: the picker never rendered the id it
was handed, so `#playground.navbar.fixture` resolved to nothing at all and every reader silently fell
back to the first `[role="combobox"]` in the document. Item 2 is **not** a publication-contract or
dirty-scope defect: the completion's own `UiDirtyScope` was measured live and already names the world
body — `import-distinct` is **PASS** on the live `:6013` shell in this pass, on the same wasm #50, at
~17 s end-to-end (the #50 verdict's poll budget was 10 s). Separately, the guest's config lane now names
the boot example from the FIRST render instead of claiming the boot document came from no example at all
(rides **#51**).

---

## 1. The example label — `#playground.navbar.fixture` named nothing

### 1a Root cause (host, vite-live)

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧪️NavbarExampleSelect/🟦️.tsx:61` spent the component's own `id`
entirely on derived children and never on the composite itself:

```tsx
<Label id={`${id}.label`} …/>
<Select id={`${id}.select`} …>
  <SelectTrigger … id={`${id}.trigger`} …>
```

`ShellHost` mounts it as `id="playground.navbar.fixture"` (`🏛️ShellHost/🟦️.tsx`, `exampleSelectElement`),
so the live DOM carried `playground.navbar.fixture.label`, `.select` and `.trigger` — and **no element
with the bare id at all**. Every reader of the picker is written as
`document.getElementById("playground.navbar.fixture") ?? document.querySelector('[role="combobox"]')`,
so all of them fell through to the FIRST combobox in document order, which on this shell is not the
example picker and whose `innerText` is empty. That is the whole of `example=` in #50: the `readExample`/
`chromeState`/`historyState` readings were taking an unrelated control, which is why #48 (a bare
`select`/`combobox` `.first()`) and #50 (id-addressed) disagreed about a shell that had not changed.

Note what this ALSO means for the three undo lanes: they were never measuring a relabel failure. B26's
real host fix underneath them (`rememberedExampleIdFromDispatchV1`, so a REDONE `Set Active Example` row
can relabel the picker) was already live in #50 and could not be scored.

### 1b Fix (host, vite-live)

`🧪️NavbarExampleSelect/🟦️.tsx` — the component's `id` now names the **trigger**:

```tsx
<SelectTrigger className="h-medium w-full min-w-[12rem] max-w-md" id={id} size="sm">
```

The trigger is the right owner on three independent grounds: it is the control a press addresses (the
wgpu shell handles a press on the bare control id `playground.navbar.fixture` —
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6214` — so this is cross-target parity, not a probe convenience); it is
the element with `role="combobox"`, i.e. what the fallback was reaching for; and its text is exactly the
active example's label. The visually-hidden `${id}.label` caption stays OUTSIDE it deliberately — a
`sr-only` element is still "being rendered", so an `innerText` read of a wrapper would have returned
`"Example Concrete Forest"` and any id slugged back out of it would have been wrong.
`${id}.label`/`${id}.select` are unchanged; `${id}.trigger` is retired, and its one call site is updated
(below). `context.triggerId` is only ever a fallback id inside `Select` and `SelectContent` reads
`triggerRef.current?.id` live, so no aria wiring depends on the old spelling.

**Law** `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` (the file that
already owns this element's laws) — the existing select/example law now asserts the identity contract
before it drives the picker:

```tsx
const exampleTrigger = document.getElementById("playground.navbar.fixture")!;
expect(exampleTrigger.getAttribute("role")).toBe("combobox");
expect(exampleTrigger.textContent).toContain("Nakagin Capsule Tower with an intentionally long label");
expect(exampleTrigger.textContent).not.toContain("Example");
await user.click(exampleTrigger);
```

```
bun x vitest run --config 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts \
  -t "keeps select chevrons and mounts playground example options only while open"
 Test Files  1 failed | 1 passed | 20 skipped (22)
      Tests  1 passed | 715 skipped (716)
```

The one failed FILE is a pre-existing collection error in `.storybook/🧪️tests/🧭️scope-resolution`
(`Cannot bundle built-in module "bun:sqlite"`), unrelated to this element and to this wave.

### 1c The guest half — the config lane knows the example from the first render (rides #51)

`ArtifactApp::initial_snapshot` (`✏️editor/🦀️.rs:7874`) seeds a fresh document from `default_fixture()`,
i.e. the **Concrete Forest** example, while `Puzzle3dConfig::active_example_id` (B26's new field)
defaulted to `""` — "this document came from no example at all". So the one field every downstream answer
reads disagreed with the document from the first frame: `puzzle3d_export_filename` names the download
after it, which is why #50's export of a boot Concrete Forest document landed as the app-generic
`puzzle-3d.json` (`export download=puzzle-3d.json`), and there is nothing else for the shell's picker to
agree with.

**Fix (product, guest, rides #51)** `✏️editor/🎚️config/🦀️.rs` — new
`default_active_example_id() -> PUZZLE3D_EXAMPLE_CONCRETE_FOREST`, wired into all four places a config
comes into existence: `#[value(default = "default_active_example_id")]` on
`Puzzle3dConfig::active_example_id` and on `Puzzle3dRuntime::active_example_id`, and both `Default` impls.
`set_active_example("")` still writes the empty id EXPLICITLY, so a deliberately blanked document keeps
the generic name; only an ABSENT field now decodes to the seeded example.

This also removes a no-op undo row: `dispatch_step` publishes a config mutation only when
`shared_after != shared_before`, so the shell's boot `setActiveExample concrete-forest` no longer moves
the field it was already on.

**Law** `✏️editor/🧪️tests/🔬️example-switch/🦀️.rs` →
`a_fresh_session_config_names_the_example_its_document_was_seeded_from`: the Rust `Default`, a decode of a
record that omits every field (`parse_dsl("{}")`) and a live instance must all name the same example, the
boot document's object ids must BE `default_fixture()`'s, and the switch it is measured against must
really replace the document.

Three existing filename laws carried the old truth and are rewritten to the new one rather than left
asserting a default that no longer exists:

- `leftover_export_fixture_downloads_puzzle_3d_json` → `leftover_export_fixture_downloads_the_boot_example_json`
  (`concrete-forest.json`);
- `export_fixture_downloads_round_trippable_json` (`🧪️tests/🔬️unit/🦀️.rs:5189`) — same;
- `export_fixture_names_the_download_after_the_active_example` now reaches the generic name the only way a
  user can, by CLEARING the picker first, and keeps the blank/concrete/nakagin-alias/blank sequence;
- `set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin`'s "leaves shared app
  preferences untouched" comparison no longer hard-codes `String::new()` for the field under test.

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib example -- --test-threads=1
test editor::puzzle3d::component::example_switch::a_fresh_session_config_names_the_example_its_document_was_seeded_from ... ok
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 695 filtered out; finished in 4.47s
```

```
RUST_MIN_STACK=134217728 cargo test … --lib import -- --test-threads=1
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 711 filtered out; finished in 29.23s
```

```
RUST_MIN_STACK=134217728 cargo test … --lib config -- --test-threads=1
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 714 filtered out; finished in 0.52s
```

### 1d What this wave deliberately did NOT build, and why

The brief asked for the navbar label to read the guest's `active_example_id` **lane**, with B26's host
memory kept only as a fallback. **There is no host-side read of a guest config record at all**, and this
wave did not invent one:

- `refreshUi`'s batched response carries windows / panels / engagements / measures / tools / labels /
  catalogue only (`PluginUiRefreshResponse`, `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`) — no config section;
- the live puzzle3d session dispatches through the direct browser-actor route, whose handoff result is an
  exact, bounded `{outcome, mutationCount, hostEffects}`
  (`🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts`, `parseBrowserActorActionResultV1` rejects
  any other field) — mutation bodies deliberately stay on the Commands lane;
- `HistoryEntry.opLines` is built from the DOCUMENT edit's forward ops only
  (`🔌️plugin/🦀️.rs:21234`), never from the row's `config_edit_id`, so the history patch cannot carry the id
  either;
- `readAppDocumentPack` reads the document, not the config.

The idiomatic channel already in the tree is an EFFECT — `Effect::SetActiveUtility` / `SetActiveTool` are
exactly "the guest teaches the host chrome" (`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:483-492`). Adding
`Effect::SetActiveExample { example_id }` is the clean long-term answer and would cover the palette /
context-menu / replayed-command routes without any host memory, but it lands in the kernel enum, its TS
mirror, the WIT effect record and its three lift sites (`🌐host`, `🖥️host`, `⚛️reactor` + the
`🔄️turn` name map), `wireEffectToFriendly`, `applyHostEffects` and the wgpu shell — a WIT change, a
binding regeneration and a wasm rebuild, in files peers are live in. Undo/redo would still need B26's
fallback (they replay mutations; no guest arm runs). It is **not** required by any verdict once 1b and 1c
land, so it is named here as the next step rather than half-built.

---

## 2. The import lane — the contract was already right; the verdict budget was not

### What was measured, live, on `:6013` with wasm #50

Three temporary `[DEBUG]` taps were added to `🏛️ShellHost/🟦️.tsx` for this pass (the existing
`completion apply` line un-gated from `runtimeDiagnosticsEnabled()`, plus one at the browser-actor
dispatch and one on the `refreshUi` response) and **removed again** before finishing. `--only=export-import`
on a free serve, `🗑️generated/probe-2026-09-12T01-19-48.md`:

```
[26.1s] distinct setFiles
[42.9s] distinct ingress hops=[… "puzzle3d.import.parsed objects=2 before=1", "puzzle3d.import.apply ops=1 after_objects=2"]
[43.0s] distinct instances before={"count":1,"ids":["seed-left-001"]}
                          after={"count":2,"ids":["seed-left-001","probe-distinct-2026-09-12T01-19-48"]}
[43.0s] verdict import-distinct PASS
```

and the import's own completion, from the tap:

```
[DEBUG] completion apply {"operation":384,"scope":{"kind":"partial","measures":true,
  "panelBodies":["puzzle.3d.play.inspector","puzzle.3d.play.document","framework.body.history","puzzle.3d.play.kinds"],
  "windowBodies":["puzzle3d.play.composite"], …}
[DEBUG] b30 refreshUi {"scope":{… same …}}
```

So every hop the brief asked to trace is already correct and needs no change:

- `puzzle3d_command_scope_class("importFixture")` is `Puzzle3dScopeClass::Document` (`✏️editor/🦀️.rs:2341`),
  and `puzzle3d_scope(Document)` names `window_bodies: [main::BODY_KEY]` **plus**
  `puzzle3d_document_panel_bodies()` — the world body is in scope, not panel-only and not `none`;
- `importFixture` is a typed operation (`action_interactive_job(…, Migrated)`, `✏️editor/🦀️.rs:8573`;
  `Puzzle3dWindowCommandWork`, `:7842`), so its admitting reply reports `mutationCount: 0` and the scope
  rides the completion — which is exactly what B27's `typedOperationCompletionRefreshV1` consumes, proven
  above by the `completion apply` → `b30 refreshUi` pair carrying the identical partial scope;
- its publication contract is `ArtifactToolPublicationContract { tool_id: "importFixture", lanes: &[Artifact] }`
  (`✏️editor/🦀️.rs:7124`) and the emit's `artifact_mutations` are the fixture delta — `ops=1` for the
  distinct payload, `ops=0` for the identity re-import.

**Root cause of the #50 red: the verdict's own budget.** The end-to-end latency from `setFiles` to a
republished `data-instances-json` is **~17 s** on an idle serve (26.1 s → 43.0 s above): host file read →
re-dispatch as `importFixture` → admission → retained-work turns → completion → hash-conditional
`refreshUi` re-serializing the world body for two window instances. #50's step polled
`10 × 1000 ms` after `setFiles` and gave up at 10 s, under the full battery's load. B29's extension of
the verdict budgets to 30 s is the right and sufficient response; nothing in the guest or the host scope
needed to move, and this wave changed neither for this lane.

### Two findings handed on

- **`import-distinct-records-history` is measuring a closed panel, not a missing row.** Its reading is
  `before=3 after=0 entries=[] sections=[]` — `sections` empty means the history panel BODY is not in the
  DOM at all, while 19 s earlier (same step, first import) it listed every row. `openHistory()`
  (`🔍️browser-probe.ts`) clicks `#framework.panel.history`; called a second time on the already-active
  tab that press toggles the panel shut, and it then also presses `#framework.history.undo` and
  re-collapses up to six expanded tree items. The verdict cannot see a row that a closed panel is not
  rendering. **B29's** — the probe is B29's lease and this wave did not touch it.
- The completion stream is noisy for an idle shell in a way B27 §2 predicted: the same `--only`
  run recorded `{"kind":"full"}` completions (operations 320, 448) interleaved with the import's own
  partial ones. Worth a look by whoever owns the retained-operation drain; not this lane.

---

## Files

Product (host, **vite-live now**):

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧪️NavbarExampleSelect/🟦️.tsx`

Product (guest, **rides wasm #51**):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`

Laws:

- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`

`🏛️ShellHost/🟦️.tsx` was instrumented and restored — no net change from this wave (`b30` occurrences: 0,
`completion apply` back behind `runtimeDiagnosticsEnabled()`).

## Verification

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
    Finished `dev` profile [unoptimized] target(s) in 2.47s                      (0 errors)
cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
    Finished `dev` profile [unoptimized] target(s) in 1m 14s                     (0 errors)
RUST_MIN_STACK=134217728 cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests
    Finished `dev` profile [unoptimized] target(s) in 40.14s                     (0 errors, 110 pre-existing warnings)
```

Full guest lib suite:

```
RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1
test result: FAILED. 708 passed; 13 failed; 0 ignored; 0 measured; 0 filtered out; finished in 124.52s
```

The 13 are the ticket's standing order-dependence family
(`📓️2026-09-10-order-dependent-tests-audit.md` — process-global surface-output pool + resident ledger),
plus three that fail in ISOLATION too and are none of this wave's business. Both halves are proven, not
assumed:

- 10 of the 13 pass alone, e.g. all four `panels::catalogue::tests` rows:
  `test result: ok. 5 passed; 0 failed; … finished in 0.16s`;
- the three that fail alone (`gumball_active_only_for_transform_utilities_with_object_selection`,
  `every_advertised_engagement_verb_is_implemented`,
  `open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin`) were **re-run with
  `default_active_example_id()` temporarily reverted to `String::new()`** and failed identically —
  a control run, so they are not this wave's:
  `gumball … assertion failed: an unattached gumball must never render / left: Some(true) / right: Some(false)`.

One real regression this change DID cause was found this way and fixed:
`export_fixture_downloads_round_trippable_json` still asserted the old constant filename
(`left: "concrete-forest.json" right: "puzzle-3d.json"`) — the pass count moved 707 → 708.

Renderer-react vitest lane, full (`SEMIO_TEST_LEVEL=full`):

```
 Test Files  2 failed | 20 passed (22)
      Tests  7 failed | 727 passed (734)
```

**No new failures.** Six are the `🧪️tests/🧩️package-integration` wgpu-worker rows failing on
`ReferenceError: Bun is not defined`, and the seventh is `buildNoteShellCommandAction`, which B26 already
recorded as a live peer's in-flight `inverseCommandId`/`inverseArgs` change. None touch
`NavbarExampleSelect` — that element's lane is the ui-react one quoted in §1b.

`bun x tsc --noEmit -p 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/tsconfig.json`
reports **zero** errors in `🧪️NavbarExampleSelect/🟦️.tsx` and zero at the edited lines of the ui test
file. The invocation as a whole is pre-existing noise across the repo (152 errors in that test file
before this wave, `Cannot find namespace 'THREE'`, `Type 'Command' is not generic`, …).

### Probe lanes

**One lane was run, before the serve died, and it is the one that matters for item 2:**

```
bun 🔍️browser-probe.ts --only=export-import --port=6013        (03:19–03:20 local)
[43.0s] verdict import-distinct PASS
[43.0s] verdict import-distinct-records-history FAIL before=3 after=0 entries=[]
[14.1s] verdict export-only PASS
[14.1s] verdict export-names-the-example FAIL download=puzzle-3d.json example=UNRESOLVED
[47.6s] battery PASS=7 FAIL=2 FAULTS=0 first-hard-fault-at=none guest-death-faults=0
→ 🗑️generated/probe-2026-09-12T01-19-48.md
```

`import-distinct` **PASS** (was FAIL in #50) — §2. `export-names-the-example` reads `example=UNRESOLVED`
because that run predates §1b landing in the page (and its guest half rides #51 regardless).

**`--only=example-switch,history-open,undo-unwind,undo-redo,export-import --port=6013` could NOT be run:
the `:6013` serve is WEDGED, and per this wave's standing instruction that is reported, not restarted.**
Evidence, all of it read-only:

- `curl --max-time 10 http://127.0.0.1:6013/?plugin=puzzle3d` → **000**, continuously, on every check
  between 04:07 and 04:37 local (22 polls; a separate 40-minute watch ending 04:37 reported
  `still busy after 40 min` with the serve never answering once);
- the vite process still HOLDS the port (`lsof -nP -iTCP:6013 -sTCP:LISTEN` → `bun 91282 … (LISTEN)`) and
  simply never answers — it is hung, not gone; its own log stops at 02:44;
- **B29's** own lane hit it first and is still hanging on it:
  `🗑️generated/b29-lane-gumball-drag.txt` reads, in full,
  `[0.4s] navigating` / `[60.4s] boot goto: TimeoutError: goto: Timeout 60000ms exceeded … navigating to
  "http://127.0.0.1:6013/?plugin=puzzle3d"`, and `bun …🔍️browser-probe.ts --only=gumball-drag --port=6013`
  (pid 19413) has been live since 03:26 with a 0-byte `probe-2026-09-12T01-30-41.ndjson`;
- this is the already-root-caused chokidar/FSEvents wedge, being worked in another lane right now —
  `🎆26/🌙09/☀️09/PROCEDURAL-3D-END-TO-END/📓️vite-serve-wedge-2026-09-12.md` (written 04:15), and a live
  `sample 91282` is writing `🗑️generated/vite-wedge-sample-2026-09-12.txt` in THIS ticket's folder.

**What the four lanes must read once `:6013` answers again** (the first three need only §1b, which is
vite-live; the fourth also needs wasm **#51**):

| verdict | on the current wasm #50 + §1b | note |
|---|---|---|
| `example-switch` | PASS | the picker id now resolves, so `readExample()` reads the trigger's own text |
| `undo-unwind` | PASS | B26's popped-row branch already answers `bootExampleId` = Concrete Forest |
| `undo-redo` | PASS | B26's `rememberedExampleIdFromDispatchV1` was live in #50 and could not be scored |
| `export-names-the-example` | needs **#51** | the label will resolve, but a BOOT document's guest-side `active_example_id` is `""` on #50 (§1c is what makes it `concrete-forest`); within the battery's `replace` group it follows `undo-redo`, so it may also pass on #50 if the document is Nakagin at that point |

`import-distinct` is green as quoted above; `import-distinct-records-history` needs the probe-side repair
named in §2, which is B29's.
