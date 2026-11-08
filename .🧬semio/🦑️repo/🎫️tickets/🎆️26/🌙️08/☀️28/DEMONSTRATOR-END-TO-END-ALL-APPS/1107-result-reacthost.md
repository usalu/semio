STATUS: Direct full vitest GREEN (reacthost-d105: 231 files, 3294 tests; later edits re-proven d106 37/37). `repo run ...:test --run` (reacthost-102) FAILED rc 1 in graph inference, not in a test: `Unknown selected named source input: reads` (reacthost-102.txt line 10463). Demonstrator vitest 100/100, e2e check GREEN 8/8, tsc only the 4 TS2307. Browser run UNVERIFIED.

Logs in `C/🗑️generated/1105b/`.

## Post-merge drift fixed (11-07)
- `shared-border.ts` (ui WindowChrome): `instanceof Element` threw `ReferenceError: Element is not defined` from a MutationObserver callback outliving the jsdom window (uncaught error fails vitest runs). Now realm-independent `isElement` (nodeType 1), 4 sites.
- Stepper (merge made the group a shared-border frame, no `overflow-hidden`): test `puzzle3d-settings-document` asserts the frame attribute `data-shared-border-frame` instead.
- Media export now refuses an empty descriptor (`media-export.descriptor-capacity`): host-faithful fixtures give a non-empty descriptor (`MediaTransportHost` lifecycle test and its Chromium fixture); zero-byte terminal expectation includes the descriptor. d103: 19/19.
- tsc: `MediaType` was not exported by `@semio-tech/framework` (renderer `🎬️media/📤️file`): exported type from the manifest module. Repo setup command used the removed `runCmdStatus("subst.exe")` port; now `nativeTemporaryWindowsDosDeviceV1()`. tsc102: only the 4 TS2307 (puzzle-wasm x2, flow-core x2) plus the setup pair, now fixed (rerun pending).

## Open
- `framework renderer wgpu generated worker` package-integration (3 tests, d101): declared wgpu browser closure is stale (media lifecycle imports `io/schema/refusal` and `failure-owner`, JSON import). Fix is `repo run` of the wgpu package `generate-browser-closure` target; needs a slot. Also bun warns `Duplicate key "retirementFailure"` in `🚪️io/🧬️schema/🔣️.json:661` (io schema owner, not host).
- e2e selector drift list from `C/1107-audit-e2e.md`: not present yet.

## Audit fixes applied (11-07, from 1107-audit-e2e.md)
- F1: `expect.json` koordinator `rowText` per language (en `Box \d+`, de `Quader \d+`); `probe.ts` selects by `ctx.language`.
- F2: `probe.ts` acts in every language unconditionally (env gate removed).
- F3: en `demonstrator.landing.title` = "Designing with Existing Stock · Demonstrator".
- F4: from code `Play / Pause` is rendered when the results window is not live: probe `pausedTokens` accepts `Word` or `Word / X`.
- F5: no producer of the ` · ` separator found in tool-run code (not confirmed): `🧫️cases.json` finalized/aborted now match the label with `\b`, with or without a suffix.
- Verified: `run.sh --check` GREEN 8/8, probe tsc clean, demonstrator vitest 6 files/100 tests (demo-d102).

## Not run
`repo run ...:test --run` (reacthost-101) and the wgpu `generate-browser-closure` target never got a slot (slots 0/0); the queued waiter was killed at turn end. Rerun when slots open: `C/1105b-tools/reacthost/run.sh 101 @semio-tech/framework-renderer-react:test --run`, preceded by the wgpu package `generate-browser-closure` target. UNVERIFIED.

## 11-07 later
- io schema `🚪️io/🧬️schema/🔣️.json`: duplicate `retirementFailure` in the `receiving` variant `properties` (identical content, compact line 174 vs pretty copy); removed the compact one, JSON parses, bun warning gone.
- There is no `generate-browser-closure` target (module `browser-closure` is unwired; its error text names a nonexistent target). Ran the generator through `1105b-tools/reacthost/closure-gen.ts` (generate + check OK, reacthost-closure1.txt); taxonomy.json and wgpu project.json regenerated; package-integration 31/31 (d104).

## 11-07 a11y audit (1107-audit-a11y.md) fixes
- F3 recheck: HEAD's en `demonstrator.landing.title` WAS German (identical to de); my change to "Designing with Existing Stock · Demonstrator" is correct (the audit read the file after my edit). Added `demonstrator.brand.name` (en/de); all 8 `windowTitle`/`shortWindowTitle` en cells are now English ("Designing with Existing Stock · X", "DwES · X"), de keep "Entwerfen mit Bestand · X"; short titles are full words (no `Verf`/`Bearb`/`Gen`).
- Findings 6/7/8/10: `ENTWERFEN_MIT_BESTAND_NAME` deleted; navbar label and h1 use `demonstratorText("demonstrator.brand.name", locale)`; logo svg is `aria-hidden` (h1 names the app); owner brands and shell `document.title` fallback use `DEMONSTRATOR_NEUTRAL_TITLE` ("Designing with Existing Stock / Entwerfen mit Bestand"), no default language.
- Finding 9: vite html `title` per language via the framework's `SemioHostHtmlTexts` (joined en · de, no `<html lang>`, as the framework spec says); `🌐️.html` and the locale test case match. I did not add `lang="mul"` (framework convention: the app sets lang once it knows the reader's).
- Finding 3: beginner neighbour names are actions ("Show {{label}} above" / "{{label}} oben anzeigen", all 4 directions), no gesture in the accessible name.
- Finding 11: after picking a language the landing h1 (`tabIndex=-1`) receives focus (only when the picker was shown); finding 12: picker spans block-stacked with no literal " / ", buttons in `role="group"` labelled by the heading.
- Finding 16: new `📐️cad .../renderer/🗣️labels/🟦️.ts` (en+de via `registerUiTranslationBundles`, `useCadReplLabels()` called at the top of every pane); all listed texts, the `aria-label`s, "Selection", "N selected" and both select placeholders are localized; "Clear" button has the field-specific name via `aria-label`.
- Tests updated: shell-brand, pane description titles, locale html case. demonstrator vitest 6 files/100 tests (demo-d105); e2e check GREEN 8/8; tsc103 only the 4 TS2307.
- Not done / UNVERIFIED: CAD renderer in-source tests (hookorder) not run (needs the cad project test run); findings 13 (footer link as anchor), 17 (sourcing "Catalog" label), 18 (fem terminology German brand data), 19, 4, 15 (browser) left.

## wgpu contributions cut (bundle lane request)
`loadPluginModule` now takes `consumedTopicsFor(appId)` instead of the receiver's `consumes` row; `pushScopedContributions` (wgpu plugin-bridge `🧲️contributions-and-backbone`) calls it with the app id, and the frame-worker's `consumedTopics(id)` (`🧵️state`) resolves it through `consumedTopicsForApp(catalog rows, contributionManifests, id, appId)`, the same cut as the React shell. d106 (package-integration + bridge/contributions tests) 37/37, closure generator reports no drift, tsc104 only the 4 TS2307. Whether the demonstrator reaches this path (wgpu target) is not browser-verified.

## reacthost-102 result
rc 1 after about 2 h 20 min: graph inference of project 648 of 651 threw `Unknown selected named source input: reads` (admission `visit`, via `declaredSourceInputs` -> `inferProjectInputs`; the failing project is not named in the log). I found no project.json or tracked json naming a `reads` input; the only git-modified project.json files are the wgpu package (closure regen, entries only) and a deleted `value-retained-clone-protocol-rs` project.json (not mine). Owner: graph/actinfra. Rerun reacthost-103 once that is fixed.

## reacthost-103
Killed on the coordinator's order (graph verification held 4.6 GB, starving the compile frontier): run.sh 7879, slot.sh 7883, repo bun 7909 and its children 9640, 10972, by recorded pids. No rc, no verdict. Rerun on the coordinator's word at the quiet point: `C/1105b-tools/reacthost/run.sh 104 @semio-tech/framework-renderer-react:test --run`. Direct vitest proof stands (d105).

## No-cargo pass (11-07)
- CAD renderer own tests run directly: `1105b-tools/reacthost/vtc.sh <tag> [filters]` (config `vitest.cad.config.ts` includes the renderer in-source suite): cad-c2 `📐️cad` renderer 73/73 pass incl. the hook-order suite and the new label hooks; the full react corpus plus it 232 files/3367 tests (cad-c1).
- Audit a11y: finding 13 done (funded-by credit is a real `<a href target=_blank rel=noopener noreferrer>`, shell-brand test updated); finding 4 done (beginner wording for grid, paneWaiting, paneFailed in en+de). Left for a cargo-capable lane (Rust edits in plugin crates, need schema work and compile): 17 (sourcing port label "Catalog" is a plain String on `MediaPortSpec`), 18 (`terminology_document("reuse", ["Entwerfen mit Bestand", ..])` in 10 plugin manifests: locale-free breadcrumb data), 19 (example data, acceptable). 15 needs a browser.
- Demonstrator vitest 6 files/100 tests (demo-d107); e2e `--check` GREEN 8/8. No e2ecases changes in the acceptance suite yet (only my cases.json edit is modified); will answer that lane when it asks.
- Host proof for the quiet point: `C/1105b-tools/reacthost/proof.sh 104` (closure generate+check, then `repo run @semio-tech/framework-renderer-react:test --run` through slot.sh). Not started; waiting for the coordinator.
