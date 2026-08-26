# Puzzle Fill Retained Preview JSON Second Adversarial Audit

## Verdict

**RED — the three previously reported admission/fallback defects are remediated, but the renderer still admits payloads that the declared language-neutral schema forbids.** Specifically, the root and diagnostic objects declare `additionalProperties: false`, while the parser rejects extra keys only for `candidateGhost`. No production source or script was edited in this audit.

## Inputs Reviewed

- Root `AGENTS.md`.
- The prior RED report: `📓️terra-puzzle-fill-retained-preview-json-fresh-adversarial-audit-2026-08-26.md`.
- The implementation report: `📓️sol-puzzle-fill-retained-preview-json-implementation-2026-08-26.md`.
- The available ticket plan, `PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/📓️p9ac-owned-planar-booleans.md`. No other original-plan attachment was present in this ticket at audit time.

## Original REDs — GREEN

### RED-1: fixed eight-item `candidatePage`

Resolved. `World3dHost/🟦️component.tsx:1168-1170` requires `Array.isArray(diagnostic.candidatePage)` and `diagnostic.candidatePage.length !== 8` rejects all non-eight-item pages. The renderer law at `index.test.ts:2765-2766` rejects both seven and nine entries.

### RED-2: malformed `candidateGhost`

Resolved. `World3dHost/🟦️component.tsx:1117-1128` accepts only `null`, or a non-array object with exactly six required ghost fields, typed strings, a nonnegative safe integer, and finite `[3]`/`[4]` tuples. The law at `index.test.ts:2781-2795` rejects arrays, arbitrary objects, missing/wrong fields, invalid indexes, bad tuple lengths, non-finite values, and an extra key.

### RED-3: locale/terminology fallback

Resolved. Puzzle3d and Puzzle5d now accept only `en`, `en-US`, `de`, and `de-DE`, and both use `Terminology::parse(...)?`:

- Puzzle3d `terminology/🦀️component.rs:104-118`.
- Puzzle5d `terminology/🦀️component.rs:71-92`.

Their four accepted matrix cells and unsupported locale/terminology rejection are asserted in each terminology module. The active render callers convert `None` into `PluginAssemblyError("ui.localization.unsupported", ...)` (`Puzzle3d 🦀️component.rs:2636`; `Puzzle5d 🦀️component.rs:3451`). Engagement, measures, and context-menu callers suppress their optional output on the same unrecognized input rather than selecting English/native. The normal defaults are explicit recognized values (`native`, `en-US`) in each config module.

No compile-obvious Option drift was found by source inspection: every production resolver caller either handles `Some(...)`/`None`, maps `None` to an assembly error, or is an explicit test default `expect`. Compilation was not run.

## Remaining Blocker

### RED-4: parser does not enforce the schema's `additionalProperties: false`

The fixed fixture declares `additionalProperties: false` for the root, `ghost`, and `diagnostic` objects at `precompute/🪣️fill/🧪️fixtures/🔭️preview-json.schema.json:5,34,47`.

The parser enforces exact keys only for the ghost (`World3dHost/🟦️component.tsx:1122`). It has no root-key or diagnostic-key census. Consequently, a payload consisting of an otherwise valid root and `fillBuildPreview`, plus either a root `extra` property or `fillBuildPreview.extra`, passes the field-by-field parser despite being invalid against the declared schema. The renderer law checks an extra ghost property only (`index.test.ts:2792`); it has no root/diagnostic extra-property rejection law.

This is a fail-closed contract gap. Either remove those schema assertions or, as the retained fixed-shape contract requires, reject any root/diagnostic key outside the schema's property sets and add laws for both rejected cases.

## Safe Verification

| Check | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on the seven remediation leaves | GREEN, exit 0. |
| `jq empty` on `preview-json.schema.json` and `preview-json-law.json` | GREEN, exit 0. |
| Scoped `git diff --check` across the nine Rust sources, renderer, and renderer test | GREEN, exit 0. |
| Negative token census for the old short-page condition, locale/terminology fallbacks, and three removed aggregate fill serialization routes | GREEN, no matches. |
| Positive token census for strict page length, strict ghost record, Option resolvers, and active-render assembly errors | GREEN, all expected paths present. |
| `rustfmt --edition 2021 --check` on all nine claimed touched Rust sources | RED: the two large app roots already produce unrelated formatting diffs (Puzzle3d `🦀️component.rs:1894,2643`; multiple Puzzle5d root locations). The seven remediation leaves are formatted; this audit does not claim the nine-file check is clean. |

Cargo, Nx/Bun/Vitest, Wasm, browser, cache, and runtime checks were not run. The audit therefore makes no compilation or runtime-behavior claim.
