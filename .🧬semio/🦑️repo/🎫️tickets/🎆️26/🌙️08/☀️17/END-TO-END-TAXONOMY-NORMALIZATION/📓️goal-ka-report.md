# `🎠️kernel` and `🖼️assets` — investigated, both correctly stay unresolved; no apply possible

No source files changed (one temporary `[DEBUG]`-prefixed instrumentation line was added to
`invokeGeneratorPreview` in `🧹️normalization/🟦️.ts` to prove root cause 3, then reverted — `grep -n
DEBUG_GENERATOR_PREVIEW` on that file now returns nothing). No `apply` was run: every scope stayed
non-zero `unresolved`, and `apply` refuses on any unresolved decision.

## `🎠️kernel` (50 moves, 1 row) — refined diagnosis, same disposition as the prior slice

The prior census slice attributed the block to `typescriptPathCollectionReferenceAuthority` (the
tokenizer-based **for-of** proof) refusing a mixed literal/computed-spread array. That mechanism is
real but not what actually decides this row: the array in
`FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/📜️script.ts` is consumed via `paths.map(...)`, never a
for-of, so the for-of authority was never in play for it.

The function that actually governs this shape is the separate **regex-based `.map`-only fallback**
in `typescriptTokens` (`🧹️normalization/🟦️.ts`, the `for (const declaration of
content.matchAll(/\bconst\s+(...)\s*=\s*\[...\]\s*(?:\.map\b|;)/gu))` block). It backs off
completely — for every declaration in the file — whenever `typescriptCollectionHasForOf(content)` is
true anywhere in the whole file. This file has ~10 unrelated for-of loops elsewhere, so the fallback
never even attempts `paths`.

Confirmed this is **intentional, not a proxy bug**: `🧪️tests/🧪️typescript-path-collection/🔣️.json`
has a case named exactly `independent-map-in-for-of-source-conservatively-suppressed` — a `.map`
array plus an unrelated `for (const key of values)` elsewhere in the same file, with `expected: []`
— matched against an independent TypeScript-AST oracle in the sibling test
`🟦️.ts` ("neutral map-only boundaries match independent AST for-of detection without fallback").
This is deliberate, already oracle-verified conservatism, not an unread proxy.

Separately, the mixed-array shape itself (`...[...].map((name) => \`.../${name}/🦀️component.rs\`)`
spliced into an otherwise-literal array) would *also* have blocked the for-of authority had this
array been for-of-consumed — the prior slice's read of that mechanism is correct in isolation, it
just isn't the one this row goes through.

**Disposition: leave unresolved, not applied.** Two independent, already-tested conservative refusals
converge on the same row; loosening either is a general, judgment-heavy change to a shared prover for
a one-row, one-file payoff outside this ticket. Files read only, nothing changed.

## `🖼️assets` (1 089 moves) — composition shifted to 6 rows; all 6 investigated

### Rows 2–5: the generated wgpu `🟨️frame-worker.js` bundle — engine intent established, no small sound fix found

Traced why a **generated** artifact is scanned as an ordinary hand-authored reference source at all:

- `repositoryReferenceCandidatePaths` (the sole choke point that decides which files get scanned
  for path-bearing tokens) excludes a path only via `isExcluded`, which reads `taxonomy.exclusions`.
  `loadTaxonomy` builds `exclusions` **only** from `pathExclusions`, and the schema loader asserts
  `pathExclusions` must be *exactly* `{compose, temp/compose}` — two hardcoded entries, full stop.
- `generatorContracts[*].outputRoots` (the schema's own declaration of which paths are
  generator-owned, `tracked`/`ignored`, byte-verified via `checkTarget`/preview) is **never
  consulted** by `repositoryReferenceCandidatePaths`, for any contract, tracked or ignored. This is
  not specific to `wgpu-frame-worker`'s outputRoots pointing at post-move destinations (as recorded
  before) — even a generator whose outputRoots exactly match the file's *current* path (e.g.
  `🛂️manifest/🤖️generated/🟦️manifest.ts`, `plugin/📇️registry/🤖️generated/🟦️playgrounds.ts`) gets no
  exemption today. The engine's *intent* (regenerate-and-verify, not hand-rewrite) is legible from
  the generatorContracts machinery itself; the reference-scanning path simply never wires into it.
- A narrow fix restricted to `wgpu-frame-worker` would need to map the file's **current** path to a
  **post-projection** outputRoot. `projectionActivation.{sourceManifestPath,destinationManifestPath}`
  looked like a ready-made pair for this, but the transposition is structural
  (`…/engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu` ↔ `…/engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust`)
  and `outputRoots[5]`'s path (`…/🎯️targets/🧊️wgpu/🧵️frame-worker/🤖️generated/🟨️.js`) isn't under
  either manifest directory — it's a new sibling folder the 📺️renderer/wgpu projection itself creates.
  Recovering it soundly needs the same move-destination computation the blocked 📺️renderer/wgpu scope
  already owns (~161–179 blockers) — the exact chain the prior brief said not to re-enter.
- A *general* fix (exempt every candidate whose taxonomy-computed destination lands inside any
  tracked `outputRoots`) is sound in principle but is a repo-wide change to the single reference
  scanning choke point every scope depends on, for a payoff local to one file — the same
  "judgment-heavy loosening of a shared, actively-used prover" the kernel row was correctly declined
  for.

**Disposition: leave unresolved, not applied.** Root cause is precise and general (a real, load-bearing
gap: generator ownership is invisible to reference scanning) but the sound fix is the 📺️renderer/wgpu
projection landing first, not a local patch here.

### Row 1 (python, ticket fixture): unchanged, on record, not retried

Already investigated and disproven as narrowable in an earlier slice (embedded-package boundary
correctly blocks it); not re-attempted per that slice's own note.

### Row 0: `generator-preview-invalid` for `plugin-registry` — new since the prior 5-row count, root-caused, not a taxonomy defect

Reproduced 3× byte-identical. Instrumented `invokeGeneratorPreview`'s `spawnSync` result with one
temporary `[DEBUG]` line (added, run once, reverted — confirmed absent by grep):

```
[DEBUG] invokeGeneratorPreview {"id":"assets-build","inputBytes":0,"status":0,"signal":null,"stdoutLength":2592393,"stderrLength":0}
[DEBUG] invokeGeneratorPreview {"id":"plugin-registry","inputBytes":1761536,"status":null,"signal":"SIGTERM","error":"SystemError: spawnSync bun ETIMEDOUT","stdoutLength":0,"stderrLength":0}
```

`plugin-registry`'s preview command is killed by the engine's own hardcoded `timeout: 60_000` in
`invokeGeneratorPreview` (`🧹️normalization/🟦️.ts:6888`) with a real 1.76 MB projected-input payload
(1089 moves + 54 edits for this scope). A manual run of the same local `📜️script.ts
preview-generated` with no projected input completed in 8 s, so the command itself is not hung; it is
simply slower than 60 s once fed this scope's real, large projection, consistent with the
already-documented finding (`goal-session-status.md` §12) that `plugin-registry`'s input set spans
the whole repository (40 232 paths) and is the structural risk flagged for every bulk apply. This is
the same known class surfacing one step earlier (plan-time preview) rather than apply-time
invalidation. Not a taxonomy/reference defect, not something to patch by loosening a shared 60 s
timeout repo-wide; the documented real fix (narrow `plugin-registry`'s input scope) is unchanged and
out of scope here.

**Disposition: leave unresolved, not applied. Environmental/structural, reproducible, root-caused, not
retried further.**

## Net result

Neither scope reached `unresolved=0`; no `plan`/`apply` was executed to completion; no moves landed.
Both scopes' blockers were re-verified as correct refusals (or, for row 0, a known, reproducible
infrastructure limit) rather than defects this slice should patch. Files touched: none (net diff on
`🧹️normalization/🟦️.ts` is zero from this slice — instrumentation added and removed within the same
investigation). `🗑️temp/` scratch plan JSONs and one stray log from this slice were generated and
deleted before closing out; nothing else in `🗑️temp/` was touched (it holds many other slices'
in-progress files).
