# Storybook And Session Output Independent Audit

## Scope

Read-only Terra review of the completed Storybook move lane and the playground-session output correction. I read the correction packet, `📓️sol-storybook-2026-09-12.md`, the live contracts, source, receipts, registered routes, and the fresh working-tree state. I made no source, schema, manifest, Git, AGENTS, ticket-lifecycle, or persistent generated-output changes. Disposable evidence is under `🗑️generated/terra-storybook-session`.

## Storybook evidence

The active graph currently has exactly 107 `🧪️.story.tsx` leaves. A direct set comparison against `🧫️fixtures/🧫️storybook-discovery/🔣️.json` found 107 fixture paths, 107 live paths, and no missing or extra path (`🗑️generated/terra-storybook-session/story-leaf-set.json`). The installed `storybook/internal/csf-tools` parser independently accepted all 107 sources and reproduced all 333 title/export/derived-id identities with no errors (`csf-direct.json`). This bypasses the live taxonomy loader deliberately, so it is evidence for the moved source identities, not for the whole registered route.

`.storybook` contains only the two native configuration leaves at its root: `main.ts` and `preview.tsx`. The former `stories/` directory is absent. All active configuration helpers, including project annotations, have anonymous leaves in semantic subtrees. The live taxonomy makes the two exact native paths `adapter-source` with `package-glue` validation and retains no `storybook-vitest-setup` fixed-name contract. The registered fixture checks the canonical `.../🧪️.story.tsx` suffix, parser identities, roots/aliases and Coda rejection. Both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` contain the generated `🧹clean🧩️taxonomy🧪️storybook-discovery` Nx command, so the registered route is launch-seed-derived.

The two retained inactive Coda inputs remain byte-preserved:

| Input | SHA-256 |
| --- | --- |
| `CodaValidationTree.source` | `92ffaf4f12dad54ffd39e93586c9dccc43c30588ab40eb005593bbf0be946a8c` |
| `CodaTrees.source` | `ebe1545ff6081520d520e2a60d50a750fb515293eb77c42d681eaa19cec7e585` |

The focused Nx route was attempted with `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, and a ticket-owned `NX_WORKSPACE_DATA_DIRECTORY`. It currently stops while creating the project graph: the renderer WGPU boot module imports a missing `🔌️Ports/📡️interactive-jobs.ts`, and the graph also reports `npm:@asamuzakjp/css-color` without a source project. Calling the registered repo-library script directly likewise stops before its tests because the live taxonomy rejects the concurrently edited `wgpu-frame-worker` package-generation/input-pattern order. These are current framework/taxonomy integration limits, not an observed Storybook source-identity regression. The full Storybook browser build was intentionally not repeated; its owner is actively changing that runtime boundary.

## Session-output evidence and defect

The live per-variant registry route has 12 staged session roots. Every root contains `🎮️playground-session/🟦️.ts`, its `.nx-artifact.json` receipt names precisely that one artifact key and owner `playground-session:<variant>`, and none retains the old `🟦️session.ts` leaf (`session-static.json`). Read-only native imports of the existing `note` and `puzzle5d` entries expose the matching variants and the expected session fields (`live-session-imports.json`). The dev Vite config constructs its `virtual:semio-playground-session` alias from `dist/sessions/<selected variant>/🎮️playground-session/🟦️.ts`; the activation/preparation consumer paths use that same artifact key.

**P1 — the session ownership test is not isolated and must not run concurrently.** At `🧪️tests/🎮️playground-session/🟦️.ts:56`, the test invokes the real dev `generate playground-session` command, which writes the live canonical source. It subsequently calls `stagePlaygroundSession(row.variant, absolute(fixture.staging.rootPath), projection)`, where `fixture.staging.rootPath` is the live registry `dist/sessions` root. `SEMIO_SESSION_OUTPUT_TEST_ROOT` protects only the Bun/esbuild scratch outputs, not either shared writer. Thus the test’s claim that it proves canonical immutability itself races and mutates both production-like outputs.

Required narrow correction: give the fixture test ticket-owned injected canonical and registry-session roots, and render/stage solely in those roots. Factor the Vite session-path selection into an exported pure resolver that accepts a session root; production Vite passes its live registry root and the test passes its isolated root. Keep the existing read-only canonical `check`/`preview` and native imports as separate acceptance evidence. Do not back up and restore shared outputs, since that races other workers.

The registered read-only `generate playground-session check` and a direct preview attempt currently stop during `loadCatalogTaxonomy` on the same `wgpu-frame-worker` ordering error noted above. The isolated WGPU cache-input oracle also could not load its real entry because its current browser-boot import lacks `🔌️Ports/📡️interactive-jobs.ts`. Neither failure exercises the fixed session writer or proves a session failure; both must be rerun after the concurrent WGPU source/taxonomy lane converges.

## Disposition

The Storybook relocation and current staged-session receipt topology have direct positive evidence. The live-output-mutation test defect is actionable and blocks treating the session fixture as an independent oracle. Registered Storybook/session and WGPU oracle completion remain limited by the current shared WGPU import/taxonomy breakage.

## Follow-up Go And Tool-Policy Spot Check

With separate ticket-owned Nx workspace-data/artifact roots and `--skip-nx-cache`, I ran the two exact registered routes after the Storybook/session work.

- `@semio-tech/repo-lib:test-go-dispatch` reached both test bodies but failed because `canonicalGoPlan()` cannot load the current invalid WGPU taxonomy. The public compiler-input test failed at that load; the cancellation test then correctly timed out waiting for the readiness file that cannot be produced before planning. This does not reproduce the former detached-process leak or test the corrected cancellation behavior.
- `@semio-tech/repo-lib:test-package-body-policy` completed 46 tests and 144 expectations successfully, including the corrected multiline-arrow and `BundleScript` delegation vectors. Its three remaining failing tests each call `loadCatalogTaxonomy()` and fail only on the same two WGPU ordering messages. It therefore provides a live partial confirmation of parser precision, but cannot confirm the prior 49/154 full result while the shared taxonomy is invalid.

Exact logs: `🗑️generated/terra-storybook-session/nx-go-dispatch.log` and `🗑️generated/terra-storybook-session/nx-tool-policy.log`. These are in-flight WGPU/taxonomy causality, not final Go cancellation or tool-policy defects.
