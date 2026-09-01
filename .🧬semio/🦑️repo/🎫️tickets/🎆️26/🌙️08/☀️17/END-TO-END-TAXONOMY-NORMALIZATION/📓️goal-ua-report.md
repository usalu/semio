# UA slice — `🖱️ui` + `🎭️actor`, real re-measured output

Baseline: `bb06c41f73f0122fbed315b7487428b976f99921` (= HEAD, uncommitted working-tree fixes below).

## Real output

```
                 before(r1, this turn)     after(final, real pasted)
🖱️ui           moves=1083 unresolved=181  moves=1152 unresolved=114
🎭️actor        moves=71   unresolved=21   moves=71   unresolved=3
```

Neither reaches 0 → neither applied (`clean taxonomy apply` refuses on any non-empty `plan.unresolved`).

## Fixes (both in shared, concurrently-edited files — scoped, not blanket)

1. **`🔣️taxonomy.json` — `json-fixture-case` had `"inferWithoutEmoji": false`.** After a file's own
   decorative emoji equals its fileKind's registered emoji, `resolveFileKind` strips it, so the stem
   hits `matchDirectoryKind` with NO emoji evidence — `inferWithoutEmoji:false` disabled exactly the
   no-emoji branch this catch-all exists for, so it silently matched nothing. This was the whole
   cause of `🖱️ui`'s 60-file `🧵️retained/**/🧪️fixtures/<name>.json`+`.schema.json` family
   (`semantic-stem-unresolved` 104→42). Removed the flag — restores the state the prior slice's own
   report describes (a 1-file residual ambiguity between `fixture-case`/`json-fixture-case` it
   already knew about and accepted only exists when this flag is unset, which is direct evidence
   this was a regression, not original intent).
2. **`🧹️normalization/🟦️.ts` — no token extractor for a `🧪️vitest.config.ts`'s own
   `includeSource`/`coverage.include` array literals** (a different shape from the already-fixed
   `runVitest(...)` call argument). Added `vitestConfigIncludeArrayTokens`, scoped strictly to
   `🧪️vitest.config.ts` basename and to `includeSource`/`coverage.include` only — deliberately NOT
   the ordinary `test.include` glob key, which is pre-existing, populated, and never previously
   scanned (touching it would newly track ~15 already-live entries per config, out of budget to
   verify safe). Filters out glob literals (`*`) after finding `🎠️kernel`'s config pairs a glob with
   a real path in the same array — an untested change there would have created a false unresolved
   row, not cleared one.
   - Confirmed this exact shape recurs in **26** `🧪️vitest.config.ts` files repo-wide, including
     `🎠️kernel` and `◻2d` (both flagged elsewhere in this session as "exactly one blocker" —
     plausibly this same construct; not verified in their own scopes, out of mandate).
   - `🎭️actor`: cleared 18 of 21 rows (`reference-syntax-unsupported` in its own
     `📦️packages/🟦️typescript/🧪️vitest.config.ts`, 11 rows, plus 5 already-cleared foreign-ticket
     prose from `historicalDocumentEvidencePopulations` landing earlier this session + 2 others).
   - `🖱️ui`: cleared 2 of 3 rows in `📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`.

## `🎭️actor` remaining (3, all one file, correctly left unresolved)

```
🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️component.ts:329
  const paths = { output: "...", shard: "...", response: "./🟦️component.ts", result: "../🟦️component.ts" };
  const path = paths[layout.source as keyof typeof paths];
```
`output:` already resolves (its key name is in the existing generic keyword allowlist
path/file/.../output/input). `shard`/`response`/`result` are not, and adding three arbitrary
property names to that repo-wide keyword list would falsely start treating ordinary
`response:`/`result:` string values as path references everywhere — the same "judgment-heavy
widening of a shared prover for a one-row payoff" already declined elsewhere in this ticket. Left
unresolved.

## `🖱️ui` remaining (114) — by code, real pasted output

```
semantic-stem-unresolved 42
reference-syntax-unsupported 20  (incl. 2 rust-path-join-unproven inner-loop rows, 10 in root 📜️script.ts prose, 5 rust-path-join in wgpu/schema, 2 transient concurrency noise below)
directory-kind-unresolved 18
semantic-stem-ambiguous 17
package-implementation-destination-unresolved 11
generator-preview-invalid 2
reference-preimage-unreadable 2  (new this run: "preimage changed since inventory" on 2 files under heavy concurrent repo-wide editing — transient, not caused by either fix, matches the documented plugin-registry regeneration-input risk)
directory-kind-ambiguous 2
```

The 42+18+17+11 vocabulary rows are the SAME already-documented, genuinely-conflicting words the
prior `🖱️ui` slice explicitly declined to register (`document`, `dispatch`, `geometry`, `host`,
`layout`, `paint`, `runtime`, `scene`, `shell`, `surface`, `window`, plus new ones in the same shape:
`output`, `handback`, `patch`, `root`, `typed`, `metadata`, `resident/root`, `retained/instance` —
each has 2+ conflicting registered emoji elsewhere in the repo for the same English word). Verified
a sample (`output`, `metadata`, `root`, `patch`) already ARE registered — by `🎭️actor`'s own earlier
slice — with a DIFFERENT emoji than what `🖱️ui`'s on-disk directories use for the same word, so
registering `🖱️ui`'s variant would collide, not resolve. Not attempted — same reasoning as the prior
slice's §6, not re-litigated.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — removed
  `json-fixture-case.inferWithoutEmoji: false`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` — added
  `vitestConfigIncludeArrayTokens`, wired into `typescriptTokens` for `🧪️vitest.config.ts` files.
- Ticket `🗑️temp/` scratch (5 plan JSONs, 5 logs) generated this turn — deleted before closing out.

## Not attempted, recorded

- Did not re-verify `🎭️actor` a third time after the taxonomy fix (it doesn't touch actor; r2 already
  carries both fixes together).
- Did not chase `🎠️kernel`/`◻2d`'s own single-row blockers directly (not this slice's scope) — flagged
  only that their `vitest.config.ts` shares the exact construct just fixed here, worth a cheap
  re-plan by whoever owns those scopes.
- Did not touch the 10-row `📜️script.ts` (repo-root CLI) prose family or the 5 `rust-path-join`
  inner-loop rows — both already root-caused and explained as out-of-scope/expected elsewhere in
  `📓️goal-session-status.md` §7–8.
