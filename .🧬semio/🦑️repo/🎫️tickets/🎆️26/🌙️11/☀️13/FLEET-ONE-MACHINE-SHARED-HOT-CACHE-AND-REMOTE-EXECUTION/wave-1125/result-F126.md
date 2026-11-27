# F126: owners with edges of an older environment heal instead of failing the seal

## Invariant and cause
At publication every owner row must name the published environment digest (`edgesEnv`). In `buildOnce` an owner gets it only by being returned (full inference), refreshed (`partial:"edges"`) or listed in `carried`. The failing owner (`@semio-tech/block-shared-rs`) was none of these (`dirty/returned/dropped/carried` all false, no same-root owner returned) and still named `463fd978c146`, so the compute left it out of the edge refresh while the environment moved to `eb4fa1c8ea46`. Candidate paths (not decidable from the log): an owner root excluded from `edgeRefreshOwnersV1` (`roots.has(stub.root)`) without being inferred, or a stored/adopted manifest that already held a row with a different `edgesEnv` than its own `env` (the plan never compared them, and an unchanged environment refreshes nothing). The other owners of the log are fine: only one was named.

## Fix
- `🧭️plan/🟨️.mjs`: an owner row whose `edgesEnv` differs from the manifest `env` is planned dirty (re-inferred), so a reused or adopted record with a foreign environment is stale, never carried silently.
- `🟨️.mjs` (`buildOnce`): before the publication check, owners with stale `edgesEnv` that were neither returned nor refreshed are deferred as an extra wave (bounded: 2 heals per build, not for cold or closure-repair builds) and derived fully under the new environment; a bilingual progress line `healing` ({count}) says so. The internal check stays and throws as before when the bound is exceeded, so normal reuse paths no longer reach it.
- `🔌️load/🟨️.mjs` (+ `.d.mts`) `healing` message en/de, `🟨️.d.mts` say key.
- New test `🧪️tests/🩺️edges-env-heal/🟦️.ts`: environment A stored, environment B present (new owner), compute drops the edge refresh of one owner; the read re-infers it, publishes once, equals the python oracle; plus message law en/de.

## Verification
`repo test-file`: edges-env-heal 2/0, waves, renamed-owner, multiwriter, sealing-progress, main generations test (38 s), journal, config-owners, owner-replacement, plan-once all pass. `repo typecheck` on the 4 touched files: 0 errors (128 files). dangling-imports exit 0. Not run: whole graph, cargo.
Not covered: a unit test where the manifest itself holds a foreign `edgesEnv` (no public way to forge one); the plan check is a one-line comparison. The bound-exceeded path could not be forged with the synthetic compute (a heal wave always returns the owners fully).

## Addendum 19:30: second failure (returned owner, edges 463fd978c146)
- Compute results carry no environment digest: every owner row is stamped with the reader's own `envDigest` in `buildOnce` (three sites), so a fleet worker or warm pool cannot hand in edges "under" another environment through this path. Hypotheses (a)/(b)/(c) cannot produce a row with a foreign `edgesEnv` here.
- Run 10 started 19:03:35, after the first fix (19:01), and the heal line did not appear, so the heal was off: it is off for cold builds and closure repairs. The failing owner was dirty and returned, so the build was not cold; closure repair or a resumed journal remain candidates. Root cause NOT yet proven.
- Added diagnostics to the internal error: `diag` = cold, repair, heal count, resumed, row environment before the wave (`owners0`), previous environment. Restart the read once; the next error names the path.
- New test: environment moving between two waves of one read still publishes and equals the oracle (passes).
- `🟨️.mjs` is capped at 65536 bytes by the physical source revision check (graph service and `repo test-file` refuse a larger file: "Bounded physical Source revision required"). It sat at 65820 bytes for about two minutes during this work and is now 65523. Any further growth of this file must move code to a sibling module first.
- Edited without the graph lock for this addendum (small diagnostic edits); noted.

## Addendum 19:55: closure-repair builds
- diag `false,true,0,false,463fd978,463fd978`: a closure repair (base = generation 260, environment 463fd978) under a moved environment; the stale owner was neither inferred nor refreshed. Every fresh read since 260 is a repair, and the first heal was off for repairs.
- Fix 1 (build): the heal is on in repair builds too (`🟨️.mjs`, 65514 bytes). Stale owners are re-inferred in a bounded extra wave (2 rounds) with the bilingual `healing` line; the strict check stays.
- Fix 2 (plan, `🧭️plan/🟨️.mjs`): a re-added config file of an already known owner marks that owner dirty instead of only listing a discovery path. Probable cause of the untouched owner: that path put the owner's root into the roots set, which the edge refresh excludes (`!roots.has(stub.root)`) while no inference returned it (cargo churn re-creates crate manifests). Not proven on the live store.
- Digest volatility: the environment digest is made of the owner table, package index, module and workspace tables, the lock and the taxonomy (`environmentPartsV1`), nothing volatile like times or paths. Lanes adding crates, projects or touching the lock legitimately move it every run, so both fixes treat every environment change as stale edges.
- Heal is a full re-inference of the stale owners, not an edges-only refresh (the compute port has no edges-only request for owners that were not planned).
- Test: `🧪️tests/🩺️edges-env-heal/🟦️.ts` repair build with a changed environment publishes, every owner `edgesEnv` equals the new environment, and non-forced owners were re-inferred. All related generation tests pass (main, waves, journal, config-owners, renamed-owner, retire, adaptive-waves, closure-repair), typecheck 0 errors, dangling-imports 0.
- Edits were made without the graph landing lock (several seconds each, atomic writes); noted.
