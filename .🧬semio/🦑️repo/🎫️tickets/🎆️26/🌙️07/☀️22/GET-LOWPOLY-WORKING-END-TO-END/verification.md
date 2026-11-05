# Lowpoly Verification

The media command roundtrip test still counted literal mesh_content, so the managed default document looked empty and could not verify current exported geometry. Its faces helper now decodes mesh_state for both the boot document and imported OBJ, PLY, and STL documents. The existing roundtrip covers every supported mesh-file format; STL permits triangulation to increase the polygon count. The import effect test continues to verify loaded quad geometry.

Reviewed the mounted managed-edit journey: ArtifactPack is in scope for encode_pack/decode_pack; its borrowed temporary config/scratch live for each synchronous build_doc invocation; snapshot comparisons borrow rather than move fields. JSON text is parsed and rewritten by serde_json before the owning decoder. The neutral contract records geometry change, source retention, pack/JSON equality, undo, and redo.

Reviewed managed duplicate and export implementations: duplication copies managed state, derives the new child handle using the fresh object id, preserves literal source text, and offsets the object transform. Export world_parts prefers managed state, independently falls back to authored source when no managed state exists, and surfaces invalid managed-state errors.

No additional compile defect found by source inspection. Runtime validation remains owned by the coordinated editor, mounted, object, and IO runs. Capacity inspection showed duplicate editor cargo waiters and an active full lowpoly target, so this verification task deliberately did not launch another compilation. The media helper change must be included in the coordinated editor test result.

## Retained Browser Probe

Added lowpoly-browser.mjs using the existing Playwright dependency and installed Chromium. It accepts --url and --out (or LOWPOLY_URL and LOWPOLY_PROBE_OUTPUT), confines output including resolved paths to this ticket generated directory, enforces the 256-character path limit, and collects readiness, title, body, accessible tree, controls, canvases, request failures, console errors, and screenshot. The browser always closes and writes its report on navigation/readiness failure. No live run started; awaiting the coordinated ready URL. Initial probe intentionally observes DOM before choosing supported edit, paint, undo, or reload UI interactions.

Additional source audit confirms exportMesh/loadMeshRequest/importMeshFile are exposed in the model action set, bridged to typed commands, listed in the retained host-only publication lane, and declared with resumable execution contracts. The retained payload validator enforces the media payload ceiling. The file picker accepts OBJ/PLY/STL and routes the returned data URL to importMeshFile; export format selection uses the same three formats as command tests.

## Activation And UI Contract

Confirmed an activation defect: the registered artifact/schema and both app dialects use s.lowpoly.lowpoly, while artifact_kind metadata and the automatic activation event used 3d.lowpoly. The model metadata now derives its id from LOWPOLY_DIALECT. Updated all three exact static descriptor values: editor artifactKinds id, editor io.artifact id, and onArtifactKind event. Added a neutral activation.json contract and Rust surface regression asserting model dialect/metadata, independently parsed checked-in descriptor, and actual plugin builder activation event. Runtime suite must be rerun after this fix.

Re-encoded the checked-in binary descriptor using public framework-os encodePackValue and verified decodePackValue plus packValueToExactJson equality against the JSON document with the independent node assertion library. The retained encode-lowpoly-descriptor.ts script records this canonical encoding operation.

UI metadata exposes Edit and Paint modes, Model world-3d window, UV canvas-2d window, and the 60/40 Model/UV paint layout. English/German localized mode, window, action, selection, and utility labels are present. World3d scenes publish mesh domain and object granularity; paint utility switches the host interaction mode to paint. UV scenes carry wireframe and paint texture within the fixed budget. Some layout titles, engagement status/placeholder text, and failed-load messages remain English literals; these are localization gaps rather than activation blockers.

A follow-up exact-id scan found the editor IO presentation independently hardcoded 3d.lowpoly. It now derives its id from the model dialect too; the editor manifest assertion and composition regression were extended coherently. No old identifier remains in the lowpoly owner/composition source roots.

The browser probe now uses an explicit persistent profile under generated output, explicit downloads/traces directories there, and process plus child TEMP/TMP/TMPDIR pointing to its generated tmp directory. Node syntax check passes after this containment change.

## Files Changed By Verification Work

- Lowpoly schema snapshot/🦀️.rs: canonical artifact-kind metadata.
- Lowpoly editor/🦀️.rs: canonical IO presentation identity and matching documentation.
- Lowpoly editor/🧪️tests/🔬️unit/🦀️.rs: current artifact identity expectation.
- Lowpoly editor/🎮️commands/📤️media/🧪️tests/🔬️unit/🦀️.rs: count current managed geometry.
- Lowpoly composition/🧪️tests/🔬️surface/activation.json: neutral activation contract.
- Lowpoly composition/🧪️tests/🔬️surface/🦀️.rs: model, static metadata and runtime event regression.
- Lowpoly composition/🧩️component/🔣️.json: canonical artifact and activation identities.
- Lowpoly composition/🧩️component/🛂️.descriptor.semio: canonical binary re-encoding verified against JSON.
- This ticket/lowpoly-browser.mjs: retained contained browser evidence probe.
- This ticket/encode-lowpoly-descriptor.ts: retained owning-codec descriptor regeneration.
- This ticket/verification.md: findings and verification status.

The probe has been syntax-checked only. No live browser has been launched by this verification task, and no live boot, canvas rendering, UI interaction, persistence, or browser success is claimed. Runtime compilation/test results remain with the coordinating task. Browser execution can resume when the ready URL is supplied.

## Browser Path Budget

The shortest default output is now the generated root itself (101 UTF-16 path units on this checkout), with single-letter profile p, temporary t, downloads d, and traces r directories (103 units each). A representative Chromium localhost IndexedDB leveldb log path is 170 units, and the temporary Playwright artifact-directory path is 137, leaving room below 256. Service workers are blocked to avoid CacheStorage path growth. Persistent profile storage, including IndexedDB, remains enabled for a reload journey. These are static path-budget checks; no browser has run. Node syntax check passes.

## Continuation Probe Audit

Fixed two concrete probe reliability issues: browser launch is now inside the report try/finally so launch failures produce evidence; readiness now waits for a visible nonzero canvas rather than resolving early on unrelated shell buttons. Console errors and failed initial HTTP status prevent a success result. Browser storage p/t/d/r directories are resolved and checked individually to reject pre-existing symlinks escaping generated output. Syntax verification passes; no browser launched. The current authored catalog router still requires generated publication before an actual hosted URL can be inferred; it exposes generate/check/preview/verify with no positional arguments. No source change to that routing is indicated.

## Boot Observation Scope

The probe now fails boot observation on any failed request or HTTP error response, with an explicit empty request-failure allowlist. It reports verificationScope=boot-observation and renderedGeometryVerified=false, even when readiness succeeds. A visible nonzero canvas alone does not prove geometry rendered; the eventual screenshot and semantic scene need inspection. Required subsequent journeys remain viewer, UV painting, host import/export, English/German locales, edit/undo/redo, and reload persistence. Syntax checked only; no live browser run.

## User-Facing Localization Changes

Added schema-first locales.json cases for explicit English/German at zero, one, and two selected faces. The Rust mode tests compare production engagement status/placeholder and failure labels to the neutral cases through serde_json serialization; a separate independent JSON-tree walk checks built-in window titles are absent so the host can use bilingual window-kind definitions. Runtime execution remains pending shared dependency publication.

Status now resolves utility names and singular/plural component names through LowpolyLabels instead of exposing raw action/granularity identifiers. The command placeholder is localized generic instruction rather than translated command samples that the typed command parser might not accept. Model and UV failed-load nodes receive the OS-resolved locale labels through the render boundary. Built-in layouts preserve structure/sizes but omit English window-title overrides. NamedLayout.label is still a plain String API; the paint layout chooser label cannot carry localized data through the current typed contract without framework work or simultaneous bilingual text. No encoding workaround or default locale was introduced.

Changed owner files: editor/🗣️terminology/🦀️.rs; editor/🎭️modes/✏️edit/🦀️.rs; editor/🎭️modes/🎨️paint/🦀️.rs; editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️.rs; editor/🎭️modes/🎨️paint/🪟️windows/🖼️uv/🦀️.rs; editor/🕹️play-app/🦀️.rs; editor/🧪️tests/🔌️mounted/🦀️.rs; editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs; editor/🎭️modes/✏️edit/🧪️tests/🔬️unit/locales.json. Source inspection confirmed all direct model/UV render callers were updated. Static composition metadata will need canonical regeneration after source builds become available. No live browser or compiler/test pass is claimed.

## Shared Preset Names And Viewer Titles

The coordinating decision resolves the earlier NamedLayout String limitation by treating preset names as shared authored content. The existing paint preset is now Paint / Malen, asserted by the neutral locale fixture. There is no separate named Model preset in the current source/descriptor; Model/Modell remains the locale-selected editor window-kind label. No default locale, localization serialization trick, or parser fallback was added.

The viewer also carried an English Model title override; it is removed so its existing Mesh/Netz kind labels can resolve with the active locale. Added viewer locales.json and a serde_json regression checking absent title plus exact English/German kind labels. Additional changed files: viewer/🎭️modes/👁️view/🦀️.rs, viewer/🧪️tests/🔬️unit/🦀️.rs, viewer/🧪️tests/🔬️unit/locales.json.

Hand-updated static composition descriptor JSON only for source-equivalent editor/viewer layout title omissions and the shared paint preset label. Re-ran the retained owning codec script successfully: encodePackValue then decodePackValue/packValueToExactJson matches the independent JSON descriptor exactly. This supersedes the earlier note that descriptor layout regeneration was still pending. Native localized UI tests and live browser journeys are still pending shared dependency publication; no runtime success is claimed.

## Final Locale And Progress API Review

Fixed a source-level compile error in the localized engagement: status granularity text had shadowed the existing granularity-control builder closure. Renamed that text value status_granularity. Fixed the remaining artifact-panel failed-load English literal to use the same OS-resolved failed_model label as the Model window.

Extended editor integration coverage with every_registered_failed_load_body_uses_the_requested_locale: malformed source forces build_doc failure, then public ArtifactEditor render projects and retires the exact returned tree for Model, UV, and artifact panel in explicit English/German. Independent serde_json traversal must find the exact neutral-case text. Also corrected a stale editor unit import of a nonexistent nested edit module to the actual modes crate root. These tests are authored but not executed.

The permanent RuntimeTestScript currently selects semio-hub-lowpoly plus editor, engine, io, mut-objects, modes, and viewer packages. The modes and viewer cfg-test roots include their new locale tests and local fixture files, and editor unit target mounts the new public render regression. Source review of the progress wrapper found operations, explicit view-state locale, render signatures, and body dirty scope consistent with current APIs; runtime behavior still requires compilation and live verification.

## Browser Journey Preparation

Added browser-journey-plan.md from the current public program-matrix and shell-interaction helpers. It identifies concrete observed-control patterns, exact app identities, action form/argument conventions, history witnesses, physical interaction requirements, and explicit create/select/transform/duplicate/edit/paint/UV/history/reload/picker/download/viewer/locale acceptance. It warns that permissive helper fallbacks and structural render digests alone cannot establish the requested result. The plan reuses public helpers on the contained context instead of launching the matrix own Metal-only browser path. No guessed DOM automation was implemented and no browser run occurred.

Localization source is settled after the artifact-panel error fix and status-closure shadow fix. Direct window render callers and seven-package runtime mounts were reviewed; exact runtime execution remains pending.
