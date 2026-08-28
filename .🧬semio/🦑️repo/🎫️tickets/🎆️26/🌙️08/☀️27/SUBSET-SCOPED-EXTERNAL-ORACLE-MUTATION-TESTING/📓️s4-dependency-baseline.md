# 📓️ S4 — Dependency baseline regeneration after the owner-contribution discovery fix

## 1. Measurement before touching anything

```
bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts dependency
```

exited **0** even on the stale, committed baseline:

```
[dependency] ecosystems=4 entries=233 production-reachable=151 test-oracle=31
... 31 test-oracle lines ...
[dependency] production-debt png (oracle png-png-1-2-mutate) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt zip (oracle zip) reachable from 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml — owner 🧰️framework/🛍️products/💻️os/🖥️host
[dependency] production-debt image (oracle image-tiff-6-0-mutate) reachable from ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️component.rs, 🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs — owner 🧰️framework/🔨️modules/🗺️surface
[dependency] production-debt brepjs (oracle brepjs-occt) reachable from ✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🟦️brep-implementation.ts — owner ✏️s/🔌️plugins/📐️cad
```

**Why the script command didn't already fail**: `DependencyScript.run()` (📜️script.ts:892-916) calls
`loadClassifiedBaseline(repoRoot)` (📜️script.ts:111-159), which reads the committed
`🔒️dependencies.json` and then *live-merges* every oracle-linked package the registry currently
reports as missing, defaulting it to `kinds: ["test-oracle"], productionReachable: false`. The
`verdict = ratchetDependencies(sorted, sorted, registry)` call then compares this merged set
**against itself**, so it is structurally always `ok`. The committed file's own staleness never
surfaces through `dependency` — only through the unit tests in `🧪️index.test.ts` that read
`🔒️dependencies.json` directly off disk, unmerged. Confirmed by running those tests before
regenerating:

```
bun test .../🧪️index.test.ts -t "dependency ratchet"
  error: ifcopenshell is linked by oracle ifcopenshell-ifc-2x3-any-differential but is absent from the dependency baseline

bun test .../🧪️index.test.ts -t "cross-language oracle hosts"
  error: python:ifcopenshell is on a generated host's import path but is absent from the dependency baseline
```

Both match the failures named in the ticket brief exactly, and both come from a single missing
package: `python:ifcopenshell`.

## 2. What the baseline must contain (read, not guessed)

- `loadClassifiedBaseline` (📜️script.ts:111) — for every already-committed entry it keeps the
  entry's own `kinds` (only remapping legacy 4-value kinds via `classifyLegacyKind`,
  📦️index.ts:1997), but **overwrites** `oracleIds`/`capabilities` from every oracle in the registry
  that links that package name (`oracleLinkedPackages`). For any oracle-linked package **absent**
  from the committed file, it appends a brand-new entry with `kinds: ["test-oracle"]` and
  `productionReachable: false` — never `true`, regardless of what the code actually does.
  `externalOracleHostPackages` (📦️index.ts:2034) is merged the same way for host packages named
  directly in a contribution's `oracleHostPackages` (no local `path`).
- `ratchetDependencies` (📦️index.ts:2079) is shrink-only on `productionReachable` count and forbids
  (a) any new production-reachable dependency versus the baseline, (b) any new test dependency whose
  oracle isn't registered.
- `classifyLegacyKind` / `isProductionClass` (📦️index.ts:1997-2014) are only relevant to entries still
  carrying the legacy `runtime|build|test` vocabulary; none of the current file's entries needed
  remapping.

## 3. Regeneration

```
bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts dependency write-baseline
→ [dependency] baseline rewritten with 233 classified entries
```

## 4. Before / after

| | Before (HEAD) | After (regenerated) | Δ |
| --- | --- | --- | --- |
| entries | 232 | 233 | **+1** |
| ecosystem: rust | 85 | 85 | 0 |
| ecosystem: js | 70 | 70 | 0 |
| ecosystem: go | 60 | 60 | 0 |
| ecosystem: python | 17 | 18 | **+1** |
| kind: production-runtime | 104 | 104 | 0 |
| kind: production-build | 60 | 60 | 0 |
| kind: repository-tooling | 41 | 41 | 0 |
| kind: test-runner | 18 | 18 | 0 |
| kind: test-oracle | 30 | 31 | **+1** |
| **productionReachable = true** | **151** | **151** | **0 — did NOT go up** |

`git diff --stat -- 🔒️dependencies.json` → `1 file changed, 165 insertions(+), 79 deletions(-)`.
The insert/delete counts look large relative to the +1 entry because `loadClassifiedBaseline` sorts
by `ecosystem.localeCompare` then `name.localeCompare`, and the previously-committed file was sorted
differently (case-sensitive-ish ordering putting `Masterminds/...` before lowercase names); most of
the diff is pure reordering churn from the new canonical sort, not content change. The genuinely new
content is:

1. **One wholly new entry**: `python:ifcopenshell@0.8.4.post1`, `kinds: ["test-oracle"]`,
   `productionReachable: false`, `oracleIds: ["ifcopenshell-ifc-2x3-any-differential",
   "ifcopenshell-ifc-4-any-differential"]`, `users` merges both the direct oracle link and the
   `externalOracleHostPackages` host-path contribution from `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json`
   (the two merge paths in `loadClassifiedBaseline` collapsed into the same entry, as designed —
   confirmed no duplicate `python:ifcopenshell` rows exist).
2. **Enrichment of already-classified `test-oracle` entries** that previously carried only a handful
   of `oracleIds`/`capabilities` because most contributions were invisible before the naming fix.
   Examples: `rust:lopdf` gained 8 more `oracleIds` (`lopdf-pdf-1-4-a-mutate`,
   `lopdf-pdf-1-7-a-mutate/e/h/ua/vt/x-mutate`, was 3 ids → now 10), `rust:quick-xml` gained
   `quick-xml-xml-1-0-valid-mutate`. These are metadata-only changes on packages that were already
   `test-oracle`/`productionReachable: false` — they do not change any count in the table above.

**Test-oracle packages classified for the first time**: only `python:ifcopenshell` (1 package). No
other new `test-oracle` rows were added — everything else in the 31-entry test-oracle list was
already present in the 30-entry pre-regeneration list under the same name.

**Production-reachable delta**: **0**. The ratchet's production-reachable count did not move in
either direction. This makes sense given how `loadClassifiedBaseline` treats new oracle-linked
entries — they are always injected as `productionReachable: false`, so the newly-visible
`ifcopenshell` contribution could not have raised the count even if the real code were reachable
from production (it isn't, per `externalOracleHostPackages`/oracle-only usage). No packages account
for an upward move because there was none.

## 5. Remaining failures — do not paper over

`bun ./📜️script.ts dependency` now **exits 0** cleanly.

However, the stricter unit test suite in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🧪️index.test.ts` still has
**one genuine remaining failure**, unrelated to what `write-baseline` can fix:

```
🔒️ dependency ratchet > the committed baseline classifies every ecosystem it tracks and keeps oracles out of production
error: brepjs is linked by oracle brepjs-occt but is absent from the dependency baseline
  [
-   "test-oracle",
+   "production-runtime",
  ]
```

(A follow-on run without stopping at the first failure, done via a scratch analysis script — not a
repo edit — confirms this is exactly **two** rows, both from the single `brepjs-occt` oracle:
`js:brepjs` and `js:brepjs-opencascade`, both `kinds: ["production-runtime"]`,
`productionReachable: true`.)

**What this means**: `brepjs`/`brepjs-opencascade` are genuine, pre-existing production runtime
dependencies of the CAD plugin (`✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/package.json`, also used
by `✏️s/🔌️plugins/🧩️puzzle`) — this was already true in the committed baseline **before** this
regeneration (`git show HEAD:🔒️dependencies.json` shows the identical `kinds`/`productionReachable`
for both packages prior to my `write-baseline` run). What changed is that the naming-bug fix now lets
the `brepjs-occt` oracle contribution register at all, so this specific self-test assertion — which
requires every oracle-linked package's baseline `kinds` to equal exactly `["test-oracle"]`, with no
exemption for recorded production debt — has a linked package to check for the first time, and it
fails. This is the same shape of newly-VISIBLE pre-existing condition called out for `ifcopenshell`,
except here the pre-existing condition is a real contradiction rather than a mere gap: `brepjs` is
simultaneously (a) declared `productionReachable: false`/test-only in this ticket's own
`📓️w4-brepjs-qualification.md` plan, and (b) already imported from production code
(`🟦️brep-implementation.ts`) and consequently listed in the `dependency` script's own
`production-debt` printout with an owner (`✏️s/🔌️plugins/📐️cad`) and a `reachableFrom` path. The
oracle registry's `productionDebt` field records this honestly; `write-baseline` cannot and must not
silently reclassify a real production-runtime import as `test-oracle`, since that would hide, not
fix, the contradiction. This is a genuine oracle-purity/architecture question for
`✏️s/🔌️plugins/📐️cad`'s owner to resolve (e.g. by moving the OCCT wrapper usage behind a
test-only oracle boundary, or by extending this specific assertion to exempt recorded
`productionDebt` the way `DependencyScript.run()`'s own `leaked` check and the "recorded production
debt" describe block already do) — it should stay red, not be swallowed here.

Two other tests in `describe("🔒️ recorded production debt")` initially reported
`this test timed out after 5000ms` / `60000ms` under bun's default per-test timeout. Re-running the
same suite with `--timeout 120000` (the whole-repo `oracleImportsInProduction` source scan is simply
slow in this environment) produced **3 pass / 0 fail / 417 expect() calls** — these were environment
timeout artifacts, not real regressions, and are not reported as failures.

## 5b. Full-suite sanity check — unrelated pre-existing failures (flagging, not fixing)

For thoroughness I also ran the entire `🧪️index.test.ts` file (`bun test ... --timeout 120000`,
295s, 2168 `expect()` calls: **68 pass / 7 fail**). Six of the seven failures are **unrelated to the
dependency baseline** and were not caused by `write-baseline` — I did not touch anything but
`🔒️dependencies.json`, confirmed by `git status --porcelain` showing only that file plus this
report as changed. They are almost certainly caused by the **same root-cause fix** named in the
ticket background (the `🔣️component.json` → `🔣️.json` naming bug), because that fix makes many more
contributions/fixtures/test files newly visible to discovery, which trips *other* shrink-only
ratchets and frozen-contract checks that were themselves calibrated against the old, empty-oracle
world:

- `🔍️ discovery and contract > every committed case satisfies the frozen contract` — now reports 84
  breaches vs. an expected 1: dozens of `✏️s/🔌️plugins/🗄️stdio/.../🧪️oracle/🔣️.json#...` fixtures
  marked "non-reproducible", plus test-discovery-count overages in `🧰️framework` (59 vs baseline 35),
  `.🧬semio` (9 vs 0), `✏️s` (7 vs 1).
- `🔍️ discovery and contract > the migration backlog is a shrink-only ratchet, never a growing
  allowlist` — `expect(count).toBeLessThanOrEqual(baseline...)` fails, 9 > 0 for some area.
- `🧹️ clean safety > no tracked fixture, source file or compose path is ever a clean candidate` —
  `expect(existsSync(join(repoRoot, "compose"))).toBe(true)` fails (received `false`).
- `🚫️ oracle purity > narrowing a run to one case must not make other cases' adapters look like
  production source` — timed out at 60000ms (possibly another environment-timeout artifact like the
  two in §5, not re-verified with a longer timeout since it is out of scope here).

These all live in baselines/contracts owned by the test-discovery and migration-backlog machinery,
not by `🔒️dependencies.json`, and fixing them would mean editing files explicitly off-limits to me
(`🧪️oracle/🔣️.json` contributions, and anything under
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/**`). **Flagging for the ticket owner rather than
touching them.** The seventh failure is the `brepjs` one already analyzed in §5.

## 6. Summary

- `🔒️dependencies.json`: 232 → 233 entries, test-oracle 30 → 31, production-reachable 151 → 151
  (unchanged — no upward ratchet).
- New test-oracle package: `python:ifcopenshell@0.8.4.post1`.
- `bun ./📜️script.ts dependency` exits **0** after regeneration.
- One genuine remaining self-test failure in `🧪️index.test.ts` (`dependency ratchet` describe block):
  `js:brepjs` / `js:brepjs-opencascade` linked by oracle `brepjs-occt` are classified
  `production-runtime` (correctly, given real production imports) rather than `test-oracle`, which
  the strict "every oracle-linked package must be `test-oracle`" assertion does not currently exempt
  even though the registry records it as `productionDebt`. Left red on purpose — not a baseline
  regeneration problem, and not something in scope for me to fix (would require editing either the
  test file or the plugin's own dependency, both outside my writable scope).

## Files touched

- `/Users/ueli/Documents/semio/🔒️dependencies.json` — regenerated via
  `bun ./📜️script.ts dependency write-baseline` (only modification made).
- This report.

Scratch files used for verification (not committed, not part of the restricted test module):
`/private/tmp/claude-501/.../scratchpad/dep-before.txt`,
`/private/tmp/claude-501/.../scratchpad/dep-after.txt`,
`/private/tmp/claude-501/.../scratchpad/check-all-oracle-links.ts` — a read-only script importing the
existing library functions (`loadOracleRegistry`, `oracleLinkedPackages`,
`externalOracleHostPackages`) to enumerate every kind/productionReachable mismatch in one pass,
confirming the `brepjs` finding is exhaustive (exactly 2 mismatches, 0 missing host packages).
