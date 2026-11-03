STATUS: PATCH SET WRITTEN, NOT APPLIED, NOT COMPILED. GREEN only: all 37 patches pass `git apply --check` + apply in order on a copy outside the repo (`1102-tools/cadi18n/verify.sh`); the language-agnostic python test passes on the patched copy (60 specs, 615 texts, 560 en!=de, 0 defects); `bun test` of the new pure locale-request test passes (2 pass). UNVERIFIED: cargo (no run in the freeze), tsc, vitest, playwright, browser.

Tools: `C/1102-tools/cadi18n/` (`de.py` German tables, `gen-spec.py`, `gen-rust.py`, `gen-web.py`, `gen-test.py` regenerate the patches fail-closed from the live tree via `mk.py`; `verify.sh` applies them on a scratch copy). Patches: `C/1102-tools/cadi18n/patches/`. The tree was never modified.

## Part 1: cad interaction texts per language

### Design
- Reused type: the framework `LocalizedLabel` (`🧰️framework/🔨️modules/🖱️ui/🌐️locale/🏷️label`, wire schema `🌐️locale/🧬️schema/🏷️localized-label/🔣️.json`): complete `{native|reuse: {en, de}}` matrix, every cell required, unknown/missing locale refused, no fallback; Rust `ToValue/FromValue` exist. No second type was invented.
- Schema (the spec structs are the schema owner, `🎬️interaction-spec/🦀️.rs`): `InteractionSpec.label`, `TransitionSpec.label`, `SelectionSpec.prompt`, `DisplayItemSpec::Label.text` change from `String` to `LocalizedLabel`. New `InteractionSpec::localized_labels()` lists every viewer-facing text.
- Data: all 60 specs, 615 strings (367 label cells = 60 spec labels + 307 transition labels, 49 prompts, 199 display texts) rewritten with real German. The `reuse` row repeats the `native` row (specs have no building-owner wording yet). Only the three keys' lines change; layout and all other JSON stay byte-identical.
- Readers/selection path: the engine resolves at render with the viewer's `(terminology, locale)` from `ViewModel`, threaded explicitly (no global, no default): `state_prompt`, `preview_display_items`, catalog/keyed-transition labels (now `LocalizedLabel`), `build_world_scene_for_pane`, `cad_window_engagement`, the four window `render`/`engagement` fns, and the two call sites in the editor `🦀️.rs` (`view_state.terminology/locale`). The `last_response = "OK"` English literal is no longer written (the status row already falls back to the localized `labels.ok`).

### Patch list (apply in this order; uii18n `cad-step-labels-1/2` first, because `cad-mode-edit` is built on top of them: both edit `step_text` in the edit mode)
1. `cad-spec-rust-0` (Cargo.toml dep `semio-framework-ui-locale` on the spec crate), `cad-spec-rust-1` (types + `localized_labels`)
2. `cad-engine-0` (dep), `cad-engine-1` (engine, including the language-neutral names), `cad-engine-2` (engine tests incl. the Rust language test and German `state_prompt` assertions)
3. `cad-mode-edit`, `cad-mode-window-{shape,building,energy,structure}`, `cad-editor-callers`
4. `cad-tests-1..4` (existing call sites: pass `Native, En`)
5. object-name schema change (below): `name-geometry-1/2`, `name-terminology`, `name-kit`, `name-derive`, `name-derive-test`, `name-import`, `name-panels-artifact`, `name-panels-artifact-test`, `name-panels-inspection`, `name-panels-inspection-test`, `name-dwg-test`, `name-sample-scene`, `name-picking-test`, `name-history-test` (these edit `cad-engine-1`'s file once more through the same patch, `cad-editor-callers` adds `with_axes`)
6. `cad-spec-json-1` (the 60 JSON files), `cad-test-languages` (new `.../⚙️engine/🕹️interaction/🧪️tests/🌐️languages/🐍️.py`)
Production + schema + data must land in ONE window, compile-atomically (spec types, engine, mode, windows, callers, JSON together; the embedded JSON is parsed by the new types). Checks after applying: `--target wasm32-wasip2 --lib` for the spec crate, engine crate, editor crate; then native `--lib` tests of the engine crate and the editor crate; then `python3 -I <test path>`.

### Tests
- Language-agnostic (python stdlib `json`, independent of every cad crate = the third-party oracle): every text is a complete matrix, en and de non-blank in both rows, no plain string, 60 specs, >=600 texts, >=85% en!=de. Passes on the patched copy; output `60 specs, 615 texts, 560 translated, 0 defects`.
- Rust (`cad-engine-2`): `every_interaction_text_carries_both_languages_and_no_blank_cell` (via `localized_labels` + `has_blank_cell`), German `state_prompt` ("Stütze platzieren: Gegenüberliegenden Grundrisseckpunkt anklicken"). UNVERIFIED (not compiled).

### Known gaps / flags
- Persisted object names are language-neutral (coordinator decision 2): see the section below. No word of any language is written to the event log for a generated name.
- TS twin (`🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🎰️interaction-spec/🟦️.ts` types `label?: string` at 62/148 and `text: string` at 116; consumers `⚙️engine/🗿️artifact/📚️interaction-catalog:89`, `🏃️interaction-runtime:686`, `🎬️actions/🎰️interaction-state:563`; the TS runtime also still sets `lastResponse "OK"`) is NOT patched: it would read the new JSON as objects. Owner decision: delete the unused TS twin or mirror the generated `LocalizedLabel` type. Check usage before the window.
- `Cargo.lock` of the cad workspace needs the new dependency edge for the two crates (cargo adds it unless `--locked`).
- Wire verbosity: the matrix repeats text for `reuse` (4 lines per string). A terminology-invariant wire form would belong in the framework type; not invented here.
- Terminology tests/names: Rust test `texts >= 600` hard-codes the current count.

## Part 2: language switch remounts every pane

### Diagnosis
- `D/🟦️.tsx:205` `<LayeredOverview key={locale} ...>`: a locale change changes the React key, unmounting and remounting the whole overview and every `DemonstratorShell` (all eight panes stay live by `DEMONSTRATOR_LIFECYCLE`). Open edits and undo history are lost. The switcher is only visible on the overview (`renderChrome` hides it while a pane is opened), so the loss happens exactly for panes edited earlier and left via the overview button.
- Why the key existed: the shell consumes the locale only at mount. `FrameworkOsShell` (`🧰️framework/🛍️products/💻️os/.../🏛️ShellHost/🟦️.tsx:135-145`) freezes `initialLocale` in `useState` and `FrameworkOsShellMounted` passes it once to `createShellScope`; later prop changes were ignored, so the key forced a remount.
- Existing delivery path to running guests: the shell's locale is an event-sourced preference in the scope storage (`commitUiPreferencesConfigMutation(scope.storage, setLocale(l))` -> `subscribeUiPreferences` listener in `🧩️shell-core-state:410-426` -> `SET_UI_LOCALE` -> `uiPrefs.uiLocale` -> projection effect `scope.i18n.changeLanguage` + root `lang`, and `ViewModel.locale` for every plugin render). Demonstrator panes are `ephemeral: true`, so the scope storage is an in-memory port: the commit is ephemeral local-only, exactly the wanted class.

### Fix (patches)
- `web-shellhost-0/1` new dependency-free `🏛️ShellHost/🌐️locale-request/🟦️.ts` (`localeRequestToApply`) + `bun:test` table test.
- `web-shellhost-2`: `FrameworkOsShell` forwards the live `initialLocale` prop as `requestedLocale`; `FrameworkOsShellMounted` commits `setLocale(next)` into `scope.storage` when the request is new and the shell is not locked (`locks.locale`). (Not `resolveShellScopeStorage`: that returns a fresh memory port per call for ephemeral shells and would be the wrong store.)
- `web-demonstrator-1`: remove `key={locale}`; `DemonstratorShell` memoises `locks` on `pane.brand.locks` (stable across locales) instead of the per-locale `pane.brand` object.
- `web-acceptance-1`: playwright test `language switch keeps panes` (audit row 10 fixed: plain `TEST_TIMEOUT_MS`, no raised deadline, no edit round trip): opens koordinator, waits for the shell's ready signal, marks the shell element, returns to the overview, switches en->de->en and asserts the marked element survives and the shell root `lang` follows. A remount would drop the marker. UNVERIFIED (needs serve).

### Risks to watch at first run
- `LayeredOverview` may itself reset on a new `panes` array identity; `resolveShellDefaults(pane.brand, ...)` also depends on the per-locale `pane.brand` (memoised on it still). If the acceptance probe attribute disappears, those are the two suspects.
- Pane brand wording (window titles, footer items) is read live by the shell, so it changes with the new brand prop; shell-owned texts follow `uiLocale`.

## Part 3: language-neutral persisted object names (coordinator decision)

### Design
- Where a name lives: the cad object is an ephemeral bridge; its authoring state is persisted as the `cad` property set on the composed `SemioModelElement` (`model_element_from_cad_object`). That pset held `label` as free text (English words such as `Box 1`, `Place Beam 2`, `Hull`, `Slab`).
- New schema `CadObjectName` (`🧬️schema/📐️geometry/🦀️.rs`, `CadObject.label: CadObjectName`), tagged union `kind`:
  - `literal{text}`: a name the user typed (inspector rename via `patchObject` / `patchSelection` `label`, imports from files, ids of foreign elements). The only variant stored as `label` text.
  - `interaction{interactionId, ordinal, part?}`: a name the editor generated from a construction (`Place Beam` 3, polyline segment 3.2). Stored as `nameKind=interaction`, `nameInteraction`, `nameOrdinal`, `namePart`: a stable key and numbers, no text.
  - `typology{ordinal?}`: a name generated from the object's typology (catalogue add, derived hull/windows/external wall/slab). Stored as `nameKind=typology`, `nameOrdinal`.
- Display: `object_name(object, axes)` (`editor::cad::terminology`): literal as typed; `interaction` = the interaction's localized spec label (Part 1) + ordinal (`Träger platzieren 3`); `typology` = `typology_label` with `CadLabels` (new `typology_hull`, `typology_windows` en+de) + ordinal. `axes` is the viewer's `(terminology, locale)`; `CadPlayView.axes` carries it (`with_axes`, set in the two production view constructions of the editor, window chrome and render); without axes (headless exports) a generated name reads as its language-neutral key, never as a language.
- Readers/writers updated: bridge write/read (`object_name_properties`, `cad_object_from_model_element`), engine commit (all `commit_*`, `solid_object`, `bar_object`, `polyline_objects`: names built from `spec.id` + `label_count + 1`, polyline pieces via `with_part`), kit (`make_object_for_typology`, `cad_object_copy` keeps the source name instead of appending English ` copy`, `patchObject` `label` -> literal), derive-transformation (`Typology` names instead of `Hull`/`Windows`/role words), geometry import (literal), tree row, inspector row, world-instance labels.
- Commands: the commit-language approach from the previous round is removed; no command needs a locale and no `cad.engagement.invalid` fault exists. `commit_session` / `try_commit_session_entries` / `engagement_submit_entries` keep their original signatures.
- Fixtures: no committed fixture stores an object label (the forest/play model JSON has none; names derive from ids = `literal`), so no fixture changed and no migration script exists. Test constructors of `CadObject` (geometry, derive, picking, sample-scene, dwg, history, panels) were handcrafted to `CadObjectName::literal(..)` in the patches.
- Tests: `cad_object_names_round_trip_language_neutrally` (bridge: only a typed name is stored as `label` text, every variant round-trips), `generated_object_names_resolve_per_viewer_language` (en `Place Beam 3` / `Beam 2`, de `Träger platzieren 3` / `Träger 2`, `None` = key, literal untouched). Both UNVERIFIED (not compiled).

### Behaviour changes to know
- Generated names are now localized, e.g. catalogue `Beam 2` / `Träger 2`; previously the interaction commits wrote English spec labels. Duplicate (`duplicateObject`) keeps the same name instead of `<name> copy` (that suffix was an English word in the data).
- Documents written before this change store `label` text for generated names; they read back as `literal` (no migration, greenfield).
- Open risk: exact `derive` macro support for struct variants with `#[value(default)]` fields inside a `tag = "kind"` enum is assumed from `DisplayItemSpec` (same shape); first compile decides. `Cargo.lock` again needs no new edge beyond Part 1 (the geometry crate already depends on `semio-framework-value-derive`).

## Window (B8, B10) applied
- B8 applied (33 patches, all `git apply` clean) and B10 applied (5 web patches); markers `win-B8-done`, `win-B10-done` touched.
- GREEN: wasm `--lib` check of cad interaction-spec, model (geometry), inferences, interaction, kit, modes, panels, commands, editor (`🗑️generated/1102/W-b8-1.txt`, rc 0; the tagged-union `CadObjectName` derive and `LocalizedLabel` fields compiled first try); `python3 -I` language test (60 specs, 615 texts, 0 defects); `bun test` of the locale-request test (2 pass); tsc of demonstrator + ShellHost + that test: no error in a touched file (4 pre-existing errors in `wasm-bindings` and puzzle wasm files, `W-b10-tsc.txt`).
- BLOCKED, UNVERIFIED: native cad `--lib` tests (`W-b8-3.txt`, rc 1): the foreign crate `semio-framework-test-typed-input-law` does not compile (`RetainedWorkOutcome` has no `unwrap_or_else`, 2 x E0599; testkit lane). No cad crate errored. Rerun W-b8 tests once that crate is fixed. Playwright `language switch keeps panes` needs the coordinator's `test-e2e`.
- Manifests: `semio-framework-ui-locale` added to `cad-interaction-spec` and the cad engine (interaction) crate: `dependencies lock` needed.
