# Rename: 🎞️animate `present` → `presentation`

## Renames performed

- Directory: `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present` → `🎬️presentation` (plain `mv`, emoji + VS16 preserved).
- Directory: `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present` → `🎬️presentation` (confirmed this app *is* the presentation play/viewer app — its own TS barrel imports `@semio-tech/animate-present-core`, so it was in scope).
- Nested test directory: `.../🧪️tests/mutate-present-1` → `mutate-presentation-1`, plus its two doc-comment references in `.../🧬️schema/🧬️mutations/🦀️.rs`.
- Rust: `mod present` → `pub mod presentation` and every `#[path = "../../🗿️artifacts/🎬️present/…"]` chain in `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/🦀️.rs` → `🎬️presentation/…`.
- All `Present*`/`PRESENT_*`/`AnimatePresent*` Rust types, consts, fns renamed to their `Presentation*` counterparts (`PresentSnapshot`→`PresentationSnapshot`, `PresentMutation`→`PresentationMutation`, `PresentDiff`→`PresentationDiff`, `PresentArtifact`→`PresentationArtifact`, `PRESENT_DOCUMENT_SCHEMA`→`PRESENTATION_DOCUMENT_SCHEMA`, `AnimatePresentViewer`→`AnimatePresentationViewer`, `AnimatePresentPlayApp`→`AnimatePresentationPlayApp`, `present_artifact_schema_descriptor`→`presentation_artifact_schema_descriptor`, `PresentEnvelope*`, `apply_present_mutation`→`apply_presentation_mutation`, etc.) — hundreds of call sites across ~117 `.rs` files.
- String ids: `s.animate.present` → `s.animate.presentation`; `animate.present` → `animate.presentation`; the mime `application/vnd.semio.animate.present` → `…presentation`; the DSL/extension `present` → `presentation` in the hand-written `.grammar.semio`/`.protocol.semio` files.
- TypeScript: `present_schema`-style barrel exports, `PresentationDeck`/`Presentation` types (already correctly spelled) left alone; the package name `@semio-tech/animate-present-core` → `@semio-tech/animate-presentation-core` everywhere it's imported or aliased.
- Cargo alias: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`'s `present = { path = …, package = "semio-s-plugin-animate" }` → `presentation = { … }`.
- Serialized fixtures hand-edited (not regex-blasted): the `.dsl.semio`/`.cmd.semio` files hex-encode `schema=` and `presentation=[…]` payload strings — each hex run was decoded, patched at the token level, and re-encoded (verified by round-trip decode, see below).

## Reveal.js contract left untouched (correctly)

`✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️.tsx` integrates reveal.js, which owns the literal CSS class `.present` (its "currently displayed slide" state, analogous to `.past`/`.future`). Left untouched: `presentSlide`, `introFlowPresent`, `isPresent`, `syncPresentSlideMedia`, `"section.present"`, `.slide-background.present`, and every `classList.contains("present")`/`classList.add("present")`. These are reveal.js's own vocabulary, not our artifact's.

## Outlier files (outside the plugin) fixed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs` — the `present` Cargo alias usage (`use present::artifacts::present::PresentSnapshot as PresentDeck` → `use presentation::artifacts::presentation::PresentationSnapshot as PresentationDeck`, registry label `"present"`→`"presentation"`). **Caught and reverted 4 false positives** a first blanket pass introduced here (this file also does a repo-wide directory-existence sweep using ordinary English "present"/"absent" — those 4 occurrences were reverted back to "present").
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🎨️.css` and `.storybook/globals.css` — `@source` paths into the renamed renderer directory.
- `✏️s/🔌️plugins/🔒️policy-allowlist.json` — 8 allow-listed file paths under the old `🎬️present` segment.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — the SSOT `"🎬️present"` vocabulary entries (members-of-apps, members-of-artifacts) → `"🎬️presentation"`.
- 4 `✏️s/🔌️plugins/🗄️stdio/**/🧪️oracle/🔣️.json` files — a cross-plugin path reference to our renderer/engine file used in stdio's own oracle bookkeeping.
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` — a doc comment ("Same shape as `🎬️present`'s `apply_present_mutation`") citing our artifact as a precedent.
- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/**` (30 files: the real, buildable "33. Projektetage" vite/vitest app + all its slide sources) — this app's vite/vitest configs hardcode `@semio-tech/animate-present-core` → the artifact's TS module path; every consumer's import specifier for that package name was updated too, otherwise the project would fail to resolve. **Caught and reverted one false positive**: `♻️mit-bestand/…/🌐️public/…/🟨️Form.min.js`, an unrelated third-party vendor bundle whose generic string `"Private element is not present on this object"` got clobbered by the first blanket pass; restored verbatim.
- `📜️script.ts` (repo root, ~35k lines, a shared repo-wide policy/verification script actively being edited by other concurrent sessions doing sibling renames — `trinity/rewrite`→`rewriting`, `draw`→`drawing` — those lines were left completely untouched). This file needed the most care: it hardcodes the literal `Present*` Rust type names from our artifact inside a `toolJobPresentEnvelopeCallerRetainedExact` policy check (renamed to `toolJobPresentationEnvelopeCallerRetainedExact`, all its internal `.includes("Present…")` string literals updated to `"Presentation…"` to match the real renamed source), the `animate/present*` scope-id strings (`"animate/present/spr"`, `.../op`, `.../standards#1-…`, and the `"animate/present": "Present"` display-name mapping). **A first blanket pass over this file wrongly renamed a large amount of unrelated content and was reverted line-by-line**: ordinary English "present" (e.g. "is missing field present in schema", "already present", doc comments), and — most importantly — a completely unrelated WGPU renderer "frame presentation" vocabulary (`Presenter::present_step`, `engine canvas present`, `prepared_present_step`, `interrupted_present_cursor_…`, `AppPresentPhase`, `PresentSurface`, `PreparedGpuPresentCursor`) and the unrelated UI-runtime module `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️present.rs` (a "presentation state" concept for the generic UI reconciler, nothing to do with this artifact) — all restored to `present`.

## Deliberately left as `present` / untouched

- `✏️s/🔌️plugins/🎞️animate/AGENTS.md` — CLAUDE.md forbids editing `AGENTS.md` files; it has one stale `animate/present` mention.
- `✏️s/🔌️plugins/🎞️animate/🔣️.json` and `🛂️.descriptor.semio` (plugin root) — these are build-generated (`bun 📦️packages/🦀️rust/📜️script.ts describe`, re-emitted from the compiled wasm32-wasip2 component per the file's own docstring). Source labels are already correct (`"Animate Presentation"`, `"Animate Presentation Deck"`, `AnimatePresentationLabels`), so regenerating will produce the right content, but the shared Cargo workspace has been continuously broken by *other* sessions' concurrent, unrelated refactors (`semio-s-plugin-draw-fsm` missing crate, `semio-framework-graph` `ToValue`/`FromValue` derive errors + `draw_layers` codegen, and now a missing `stdio` zip artifact schema file) every time I tried to run the full component build, so I could not regenerate these two files or the framework's plugin registry (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️plugins.ts` + `🔣️plugins.json` + the `🧑️‍💻️dev/🔌️plugin-modules/animate/🔣️.json` mirror, and the wgpu frame-worker's bundled copy of the registry) — all of these embed the plugin's `descriptorSha256`/labels and the stale `on-artifact-kind:animate.present` activation event. **Once the shared workspace is buildable again**, run:
  - `bun ✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📜️script.ts describe` (or `nx run @semio-tech/animate-plugin:describe`)
  - `bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate` (or `nx run @semio-tech/plugin-registry:generate`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️remaining-package-purity-authority/🔣️.json` and its `"🎬️present"` taxonomy tag — this is fixture data about the *unrelated* `🧰️framework/🔨️modules/🖱️ui/🧠️runtime` module's own `present.rs`, not this artifact.
- `.cursor/plans/fix_standalone_app_css_0f7cbeee.plan.md` — a historical planning doc from another tool, already describing a stale (pre-reorg) directory layout; not part of this ticket, left as a snapshot.
- `♻️mit-bestand` files unrelated to `animate-present-core` (e.g. anything not importing that package) were not touched.

## Cargo check (mandatory verification)

```
cd /Users/ueli/Documents/semio && cargo check -p semio-s-plugin-animate --target wasm32-wasip2
```

This **succeeded once** during the session (only warnings, zero errors, `semio-framework-ui` etc. built clean through to the animate crate) — captured live via a background poll. The shared workspace is being churned concurrently by several other sessions doing sibling renames/refactors (confirmed via error signatures unrelated to `animate`/`presentation`: a missing `semio-s-plugin-draw-fsm` crate, `semio-framework-graph` `#[derive(ToValue, FromValue)]`/`draw_layers` codegen errors, and — most recently — a missing `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/…` schema file), so at the moment this report is written the check is transiently red again for reasons that have never once mentioned `animate` or `presentation`. No error at any point during this session named this plugin or this artifact. Re-run the command above once the shared tree settles; per the ticket's own instruction this is expected behaviour on a live shared repo, not a defect in this rename.

## Residual-token re-grep (mandatory verification)

Repo-wide (excluding `node_modules`, `target`, `dist`, `.git`, this ticket's own notes):

- `PresentSnapshot|PresentMutation|PresentDiff|PresentArtifact|AnimatePresentViewer|AnimatePresentPlayApp|present_artifact_schema_descriptor` → **zero matches**.
- `s.animate.present` (without `ation`) / `PRESENT_DOCUMENT_SCHEMA` / `animate.present` (without `ation`) / bare `🎬️present` (without `ation`) → matches only in: the two build-generated files, `AGENTS.md`, the unrelated `ui-runtime` purity fixture, and the unrelated historical `.cursor` plan doc — all documented above as deliberately left.
- Corruption sweep (`presentationation`, `represententation`, `presentationce`, generic-English "is/was/already presentation…") → **zero matches** anywhere touched.

## Files changed (non-exhaustive list; ~285 total)

Full machine-generated list (before the two after-the-fact corrections to `📜️script.ts` and the outlier fixes) is in `🗑️generated/rename_run.txt` in this ticket folder — 253 files under `✏️s/🔌️plugins/🎞️animate` + `♻️mit-bestand/🎤️präsentation/📅️33.projektetage` + `.storybook`. On top of that:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🎨️.css`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `✏️s/🔌️plugins/🔒️policy-allowlist.json`
- `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️document/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `📜️script.ts` (root)

All of the above landed in the environment's shared auto-commit `96aa4f8c12` (which also carries other concurrent sessions' unrelated work — `curate`→`curation`, `draw`→`drawing`, `rewrite`→`rewriting`, puzzle-3d, etc. — none of that was touched by this ticket).
