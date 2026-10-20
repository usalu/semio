# Browser Boundary 2026-10-21

Parent ticket owns lifecycle. Build skill applied from .agents/skills/build/SKILL.md.

## Contract

Language-neutral cases and JSON Schema under runtime/📇️catalog precede implementation. The cases declare the full authored pane mapping, invalid variants, expected host/assets, exact pure catalog source graph, and exact brand graph/imports. Esbuild targets a browser and evaluates the compiled catalog against the same cases as the direct module; Ajv validates the cases. No new package manifest or runtime library is required.

## Validation

A read-only repo exec request was cancelled during host memory admission; no commands bypassed admission. Focused runtime target and browser graph validation remain pending.

## Changes

- New runtime/📇️catalog/🟦️.ts imports only authored JSON and owns host/assets/panes constants plus pane selection.
- 🪧️brand.ts imports this pure catalog. Its imported build runtime is no longer part of the browser source graph.
- runtime/🟦️.ts reexports the pure API while retaining component closure, generated build targets, and deployment layout in the build-only module.
- New runtime/📇️catalog/🧫️cases.json, 🧬️schema/🔣️.json and 🧪️tests/🟦️.ts exercise all eight mappings, five invalid variants and the exact bundler graphs. Runtime execution agent wires this helper into the existing owner test.

No package manifests, dependency locks, AGENTS files, build commands, or git state were changed by this extraction. Focused validation remains queued behind the parent E2E admission rather than introducing a competing command.

## Full Registry Admission Follow-Up

The page now authors maxRows: 512, the explicit maximum approved by registry schema x-semio-admission. The previous 128-row authority could not admit the current 70 targets + 1 host + 156 playground rows (227 total). Other limits remain finite and unchanged: 2,097,152 JSON bytes, 4,096 edges, 65,536 work units, 30,000 ms deadline. Registry string/list limits remain enforced by the original admission implementation; no parser or policy was weakened.

Language-neutral fixture/schema now record the complete composition contract, expected current census and denial at 128 rows. The helper uses TypeScript AST to verify the page's actual numeric authority, invokes actual composeSpecificOsCatalogV1 against its generated registry, validates the complete admitted rows with Ajv, asserts final progress/census and every runtime pane variant, and expects the old 128-row call to reject. This full composition exercises all existing bytes, work, edge, ID, string, list, URL and reference checks. Ajv uses the registry's existing strict:false convention to accept the schema's x-semio-admission annotation.

The brand graph bundler now uses the production Vite define import.meta.vitest = undefined. Its exact source graph still excludes every test module and build module.

No additional build/test command was queued; parent owns the already waiting full test and E2E validations.

## Portable E2E Artifact Paths

Installed Playwright source was read without executing private APIs. workerProcessEntry.js bounds a test-folder stem to 60 characters, then appends project and retry; the prior output root was 172 UTF-16 characters, so retry error-context.md could reach 266 characters. util.js persists path attachments through attachments/name-SHA1.ext; body attachments remain in memory. index.js writes automatic error-context.md and trace.zip directly.

The E2E consumer now writes under ticket/🗑️generated/e/<UUID32>. Removing the UUID's fixed hyphens is reversible and retains all 128 identity bits; no truncated hash or shared directory is used. Current ticket output root is 153 UTF-16 characters. Schema-authored policy reserves 102 characters for installed runner suffixes and refuses longer roots before starting Playwright. The current worst declared worker trace is 254 characters; error context under a maximum 60-character stem with chromium and retry2 is 246 characters.

Case titles now use concise semantic identities (pane roster, overview chrome, each pane boot, aussuchen preview/undo-redo, energie run-cancel, statik undo-redo, funding logos). Detailed expectations and attachments remain. Browser health checks actual public testInfo.outputDir/outputPath for trace.zip, error-context.md and p.png. There are no private testInfo mutations. Four snapshot sites (eight boot cases plus three final workflows) now save p.png through the public screenshot path option and attach its returned body, retaining actual image files without an attachment-copy/hash path.

New modules under 🔨️modules/🧪️e2e/📁️artifacts own policy, JSON Schema, language-neutral cases, path implementation and regression helper. The existing runtime catalog owner invokes the helper. It validates policy/cases with Ajv, checks complete UUID identity, distinct sessions, invalid identities, 256-character boundary/refusal, title roster via TypeScript AST, fixed config retries/workers, installed runner's naming cap and declared direct/worker suffix bounds. Actual public Playwright output paths are checked by every served E2E case. No additional command was queued; parent owns the pending unit and served runs.

## Capture API For Remaining Workflows

The acceptance spec defines the local helper capturePaneArtifact(page, paneId, name = "p.png"). Names must match a short semantic PNG filename (one initial lowercase letter, at most fifteen following letters/digits/hyphens, .png). It checks the full output path, writes the screenshot and attaches the same buffer. All four existing snapshot sites now call this helper. Future workflow cases should use concise plain-ASCII titles of at most 24 characters and add their exact title to artifacts/🧫️cases.json.

The authored suite cap is 32 cases with one worker and at most two retries. The declared retry trace fixture therefore reserves worker95/recording191, whose current ticket path is 255 characters. The 102-character suffix reservation covers that bound; the earlier 254 figure covered only the original15-case roster. A runtime regression checks the exact roster, cap and retry/worker configuration.

Capture helper remains local to the single acceptance spec; its signature is shared among cases without exporting a third-party Page type.
