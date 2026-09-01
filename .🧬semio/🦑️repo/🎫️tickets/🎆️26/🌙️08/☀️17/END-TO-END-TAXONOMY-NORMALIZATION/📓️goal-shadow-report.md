# Package-Boundary Shadow Over-Broadening — Report

## Your distinction is correct

Confirmed against `historicalDocumentEvidence`/`historicalEvidenceBoundaryOwns` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`: the four
`historicalDocumentEvidencePopulations` are a closed, exactly-validated set. Three
(`ticket-report`, `cursor-plan-snapshot`, `dev-prompt-log`) have leaf grammars requiring a literal
`.md` extension — prose by construction. Only `ticket-workspace`'s `^.+$` leaf admits any file,
including real code/scripts. The old code ran `historicalEvidenceBoundaryOwns` unconditionally over
the OR of all four populations, so one `Cargo.toml` shadowed a sibling `📓️` report exactly as it
shadowed a sibling `.rs`/`.mjs` — conflating "could be code" with "is prose."

## Fix

`historicalDocumentEvidence` now tests the `ticket-report` population separately. If a path matches
`ticket-report`, it is exempt as soon as it clears negative (1) (`fixedFilenameContracts`) —
`historicalEvidenceBoundaryOwns` is never consulted for it. Every other population match (in
particular a `ticket-workspace` match — evidence snapshots, scratch scripts, generators) still runs
the full boundary-owns check, unchanged. Negative (1) is untouched for every population.
Deliberately did **not** extend the carve-out to `cursor-plan-snapshot`/`dev-prompt-log`: an
existing, already-passing test (`"dev-prompt-log honors both existing negatives…"`) requires a `.md`
transcript embedded inside a package boundary under `💬️prompts/` to still block — narrower scope
than the `📓️` case you named, so left alone.

## Regression canary — still blocked (unchanged)

`generate_w1_a_gltf_create_scene.mjs` and `derive-dwg-fixture.c` (both `FULL-STDIO-…-MUTATIONS`
ticket, `ticket-workspace` population, sibling of that ticket's `Cargo.toml`) — still block, verified
by the pre-existing unit cases and by the engine-level fixture test, both still green, unmodified.

## Tests added (fail-before/pass-after, verified both directions)

1. Unit case: the SAME ticket's real, tracked `📓️w1-gltf-create-scene-frozen-audit.md` (sibling of the
   same `Cargo.toml` that keeps `.mjs`/`.c` live) — now exempt.
2. Engine case (`planTaxonomy`, real git fixture): `embedded-pkg/📓️note.md` planted beside
   `embedded-pkg/Cargo.toml` — move not blocked — while `embedded-pkg/lib.rs` beside the same
   manifest still blocks.

Red (temporarily neutralized `isNarrativeReport` short-circuit): both new assertions failed exactly
as expected (`Expected: true/false, Received: false/true`), 5/7 passed — no other test moved. Green
(restored): `bun nx run @semio-tech/repo-lib:test-historical-document-evidence` → **7 pass, 0 fail, 77
expect() calls**. Neighboring suites unaffected: `test-preflight-reference-basis` → 30/30 pass;
`🧪️package-boundary-classification` (via `🧪️index.test.ts -t "package boundary"`) → 13/13 pass.

## Real before/after (pasted, `--scope`, `--plan` under `🗑️temp/`, baseline `bb06c41f73f0122fbed315b7487428b976f99921`)

```
actor : moves=71  unresolved=106 → 28   (all reference-syntax-unsupported; zero 📓️-report rows remain)
kernel: moves=50  unresolved=23  → 7    (all reference-syntax-unsupported; zero 📓️-report rows remain)
assets: moves=1089 unresolved=5  → 5    (unchanged — its 1 non-wgpu row was never a 📓️ report; the
                                          extract_positions.py canary from goal-last6-report.md stays blocked)
```

No scope reaches `unresolved=0`. Remaining `actor` (28) and `kernel` (7) rows are legitimate:
production source (`📤️return/📨️response/🟦️component.ts`, root `📜️script.ts`) and other tickets'
`📜️script.ts`/`.dependency-cruiser.cjs` files — the latter protected by `fixedFilenameContracts`
(negative 1) regardless of this change, unrelated to the shadow defect. Verified by grepping each
plan's `unresolved` for any `/📓️*.md` path: zero in all three scopes, both before-context (§ above)
and after.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` — `historicalDocumentEvidence` (logic + docstring), `historicalEvidenceBoundaryOwns` (docstring only).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️historical-document-evidence/🟦️.ts` — 2 new cases (real-repo unit case + engine fixture case), both proven red-then-green.
