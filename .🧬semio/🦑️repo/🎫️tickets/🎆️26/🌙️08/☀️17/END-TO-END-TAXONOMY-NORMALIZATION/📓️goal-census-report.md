# Census-fixture blocker — investigated and disproven; real blocker found, fixed, replication committed

## The coordinator's reading was wrong — say so precisely

The `remaining-package-purity-authority/🔣️.json` census fixture is **not** what blocks
`📡️replication`'s apply. Its `frozenCoordinateEvidenceContracts` wildcard (`/mappings/*/0`) already
covers both of the fixture's occurrences of this scope's moving paths (`/mappings/59/0`,
`/mappings/60/0`) — verified by instrumenting the real `lexicalTargetIncomingReferences` post-apply
check (`🧹️normalization/🟦️.ts:7317-7320`) with a one-off `DEBUG_STALE_REF` print, then running the
actual apply. Exactly **one** stale reference was reported, and it was not from the fixture:

```
[DEBUG] resolved 🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📜️script.ts
  {"structuredLocation":"unsupported-path-syntax:8:33@498","value":"🧪️vitest.config.ts"}
  🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/🧪️vitest.config.ts
```

`nxref-report`'s own hypothesis (blaming the fixture) was an unverified correlation — it found the
fixture *contains* the moving paths as strings, but never proved that occurrence was the one the
post-apply check actually flagged. It wasn't.

## Root cause 1: `resolveReferencePath` candidate ordering (real engine bug)

`📜️script.ts:8` has `runVitest(this.root, rest, "🧪️vitest.config.ts")`. `config` is resolved at
runtime relative to `this.root` (the script's own directory — `runVitest` passes it as vitest's
`--config` with `cwd: bundleRoot`), so the intended target is the **sibling** `🧪️vitest.config.ts`,
not the unrelated monorepo-root `./🧪️vitest.config.ts` (confirmed this file exists at repo root).

`resolveReferencePath` (`🧹️normalization/🟦️.ts:4679`) tried the root/coordinate-root candidate
*before* the same-directory-sibling candidate for every non-`./`/`../` token. At **plan** time the
`known` index is repo-wide, so the bare token `"🧪️vitest.config.ts"` matched the coincidental
root-level file first and the reference was silently treated as already-resolved (`unresolved=0`,
no edit generated — a false green). At **apply**'s post-check the index is restricted to only the
disposed/target paths, so the root file wasn't a candidate, the sibling was found instead, and the
now-stale reference correctly rolled the apply back. Two different index scopes, same resolver,
different (and previously undetected) answer — a real detection asymmetry, not evidence-freezing.

**Fix**: for a bare, single-segment token (no `/` anywhere — the shape a `cwd`-relative filename
argument takes), try the same-directory sibling candidate first. Every other token shape (contains
`/`, or an explicit absolute/`./`/`../` form) is untouched. This can only change resolution in the
exact ambiguous case (both a root/coordinate-root file and a same-named sibling exist).

## Root cause 2: `runVitest`'s config argument was never a recognized token

Even resolved correctly, the token was still `reference-syntax-unsupported` — nothing in
`typescriptTokens` recognized `runVitest(...)`'s third argument as a reference at all (only a small
allowlist of function names like `resolve`/`join`/`readFileSync` is scanned generically, and that
scanner takes the *first* quoted string in the call, which is wrong here — several callers pass a
literal `segments` array of quoted test filenames before the config argument). Added
`runVitestConfigArgumentTokens`: takes the **last** quoted string in a `runVitest(...)` call
specifically, matching the parameter's trailing position. This exact call shape recurs in **25**
`📜️script.ts` files repo-wide (`grep -c`), so this clears the same class for every future module
whose own `🧪️vitest.config.ts` moves, not just replication's.

## Tests (both new, fail-before/pass-after verified both ways)

- `🧪️tests/🧪️bare-reference-sibling-precedence/🟦️.test.ts` — 9 cases (dual Bun/TypeScript compiler
  extraction of the real `resolveReferencePath`): sibling preferred when both exist, root-only still
  resolves, explicit relative tokens untouched, **multi-segment bare tokens keep the old root-first
  order even when a same-named sibling exists** (proves the narrowing is exact). Reverted to the
  pristine pre-fix file: 3/9 fail with the exact predicted wrong value; restored: 9/9 pass.
- `🧪️tests/🧪️run-vitest-config-argument-tokens/🟦️.test.ts` — 9 cases: plain call captured, trailing
  config correctly picked over an earlier quoted segments-array entry, no-config call yields no
  token, the function's own default-parameter value is also captured (harmless — resolves to the
  library's own directory, no sibling there, falls through to root exactly as before). Reverted:
  1/9 (the file doesn't even parse without the new function); restored: 9/9 pass.
- Full regression: `bare-reference-sibling-precedence`, `nx-workspace-root-file-reference`,
  `reference-coverage-selection`, `json-reference-owner-lookup`, `preflight-reference-basis`,
  `reference-coordinate-progress`, `rust-physical-reference-context`, `historical-document-evidence`,
  `rust-finite-target-consumption`, `frozen-coordinate-wildcard-coverage` (5/5, direct `bun test`) —
  all green. `rust-writable-path-authority` and `historical-package-owner-identity` fail — confirmed
  **pre-existing**, reproduced identically against the untouched pristine baseline (concurrent-worker
  churn elsewhere in the same shared file; `historical-package-owner-identity`'s 2 failures match the
  exact pair already documented in `goal-refsyn-report.md`).

## Root cause 3: a new, genuine vocabulary gap, found and closed

With both reference bugs fixed, `unresolved=0` — but `apply` still rolled back, this time on the
post-apply **convergence** check, not the disposed-reference check:

```
Post-state does not converge to an empty plan: 0 operation(s), 20 finding(s);
first directory-kind-unresolved at .../📡️replication/🧫️fixtures/🧫️wire/📦️client-bye
```

All 20 findings are `directory-kind-unresolved`, one per `.bin` wire-protocol fixture the plan's own
moves split into `🧫️wire/📦️<name>/💾️.bin`. `.bin`'s `fileKindResolutionRules` classifies it
`role: "asset"`, and the only generic `📦️` catch-all directory-kind (`asset-binary-subject`) is
scoped to `parentKindIds: ["assets", "members-of-assets"]` — these fixtures aren't under `🖼️assets`,
so the very directories the engine's own plan just created were unrecognized by the engine's own
vocabulary. Confirmed **not** a one-off: the identical wire-fixture set already exists, unmigrated,
under `💻️os/📦️packages/fixtures/wire/` — this will recur for every module with binary fixtures.

**Fix**: registered `fixture-binary-subject` in `🔣️taxonomy.json` — same emoji/slug/reserved-word
pattern as `asset-binary-subject`, scoped to `parentKindIds: ["fixture-case"]` (the existing
generic "named subfolder of a `🧫️fixtures` tree" kind) instead of the assets tree. `validateTaxonomy`
via `clean taxonomy plan`: 0 problems; re-plan still `unresolved=0`.

## Applied — real command, real output

```
B=bb06c41f73f0122fbed315b7487428b976f99921
bun ./📜️script.ts clean taxonomy plan  --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --scope "🧰️framework/🔨️modules/📡️replication" --baseline "$B" --plan "$T/🗑️temp/🔣️census.json" --workers 4
[clean taxonomy plan] moves=64 roots=0 relocations=0 symlinks=0 removals=0 edits=91 regenerations=2 unresolved=0
  digest=27005f6d7bc5d5bfe02fecd94bef97b9e743a2311948bbc269ffbd14e7823784

bun ./📜️script.ts clean taxonomy apply --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --baseline "$B" --plan "$T/🗑️temp/🔣️census.json"
[clean taxonomy apply] state=committed moves=64 relocations=0 symlinks=0 removals=0 edits=91 regenerations=2
  journal=…/🧾️taxonomy-transaction/🔖️27005f6d.../🔂️attempts/🔢️000001/🔣️.json
```

Journal `state: "committed"`. On disk, zero `🦀️component.rs` / `🟦️component.ts` / `📦️glue.rs` remain
under `📡️replication`; `📜️script.ts` now reads `runVitest(this.root, rest, "vitest.config.ts")`.

## `🎠️kernel` (50 moves, 1 row) — checked, NOT the same fixture either, NOT applied

```
[clean taxonomy plan] moves=50 … unresolved=1
{'code': 'reference-syntax-unsupported', 'path': '.🧬semio/…/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📜️script.ts',
 'message': 'typescript unsupported-path-syntax:222:6@14159 ... "🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts" ...'}
```

A **different** file in a **different** ticket. It's a `paths = [literal, literal, …].map((path) =>
readFileSync(...))` array — exactly the shape `typescriptPathCollectionReferenceAuthority` exists to
trust — but one entry is `...[...].map(name => \`…${name}/🦀️component.rs\`)`, a computed spread
mixed into the otherwise-literal array. The tokenizer-based prover (`typescriptCollectionSyntax`/
`typescriptCollectionEmbeddedExpressions`) correctly refuses to trust a *mixed* literal/computed
array — widening it to partially trust such arrays is a real, judgment-heavy loosening of a shared,
actively-used prover for a one-row payoff in someone else's ticket, not a narrow fix. `📜️script.ts`
is a `fixedFilenameContracts` match (always live, never historical-evidence-exempt), so this genuinely
needs a rewrite eventually — just not one I invented unilaterally under this ticket's own file.
**Disposition: leave unresolved, not applied.** Root cause reported precisely per the brief's own
instruction to stop rather than guess.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` — `resolveReferencePath`
  reorder (root cause 1) + new `runVitestConfigArgumentTokens` (root cause 2).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — `+fixture-binary-subject`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️bare-reference-sibling-precedence/🟦️.test.ts` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️run-vitest-config-argument-tokens/🟦️.test.ts` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` (2 new routes)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` (2 new targets)
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (2 new launch entries each)
- `🧰️framework/🔨️modules/📡️replication/**` — 64 moves, 91 edits, 2 regenerations, applied and committed.
- Ticket `🗑️temp/` scratch (plan/apply logs, debug plan JSON) generated and deleted before closing out.
