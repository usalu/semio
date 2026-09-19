# K2 — Test infrastructure defects (ticket 26/09/18)

Six test-infrastructure defects sibling slices found but did not own. Every claim below was executed;
captures are in `🗑️generated/k2-*.txt`.

| # | item | status |
|---|---|---|
| 1 | `vitest-configuration-ownership` "machine-specific `cacheDir`" | DONE — diagnosis **refuted**, real drift fixed, gate 7/7 |
| 2 | orphaned test suites census + registration | WIP |
| 3 | `4_gate` launch families | DONE — 34 rows added, census 0 missing in both files |
| 4 | `foundation-source-check` `:371` `HUB_DEV_BINARY_TARGET` | DONE — gate 11/11 |
| 5 | `documentId`→`artifactId` descriptor rename | WIP |
| 6 | three `🔋️energy` `sim::` unit tests | WIP |

---

## 1. `vitest-configuration-ownership` — the `cacheDir` diagnosis is wrong

**Verified state.** U2's `📓️u2-…md` §6.5 reports "42 of 43 owners already mismatch their fixture
`projectionSha256` … the projection includes `cacheDir`, which `repoCacheDirectory()` resolves to a
machine-specific absolute path *outside* `repoRoot`, so `portable()` cannot relativize it".

Both halves are false:

1. `repoCacheDirectory()` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts:8`) is
   `join(repoRoot, ".🧬semio", "🦑️repo", "⚡️cache", …)` — **inside** `repoRoot`, so `portable()`
   rewrites it to `./.🧬semio/…` like every other repo path.
2. The 42/43 number is an artifact of U2's own probe. `🐍️u2-vitest-projection-hash.ts:41` calls
   `projectionHash(config, resolve(repoRoot, owner.configurationRoot))` — an **absolute** root — while
   the gate (`🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts:104`) passes the **relative**
   `owner.configurationRoot`. Every row therefore hashed differently except the single owner whose
   `configurationRoot` is `"."` (`resolve(repoRoot, ".") === repoRoot` → `portable()` → `"."`). Hence
   exactly 43 − 1 = 42 "drifted".

**Measurement.** `🐍️k2-ownership-projection.ts` replays the gate's own inputs for *every* owner (the
real gate short-circuits on the first rejected `expect` inside `Promise.all`) and additionally scans the
canonical projection string for any absolute path. Capture `🗑️generated/k2-ownership-census.txt`:

```
owners=43 rootBad=1 nameBad=0 hashBad=8 machineBad=0
```

`machineBad=0` — **no owner's projection contains a single machine-derived path**. The real defects were
one `configurationRoot` regression and eight content drifts from peers' legitimate config edits.

**Fixes.**

- `🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts:53-59,111-113` — the projection is split out as
  `behavioralProjection()` and the gate now asserts `ABSOLUTE_PATH_IN_PROJECTION` matches nothing, i.e.
  no quoted value in the hashed projection may open with a POSIX root or a Windows drive. This is the
  root-level guard for the class U2 feared; it did not exist before, which is why nobody could tell the
  difference between a machine-dependent hash and ordinary drift.
- `🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts:75-88` — `routerRoot()` now also accepts the
  `📦️packages/🟦️typescript` bundle of the nearest semantic ancestor. `*Script` classes are routinely
  extracted out of `📜️script.ts` into sibling owners (`🧑‍💻dev/🧪️tests/🏃️execution/🟦️.ts:42`); the old
  walk found no `📜️script.ts` above such a file, fell through to `repoRoot`, and reported a correct
  `runVitest` selection as `wrong`.
- `🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json` — the `🧰️framework` owner's
  `configurationRoot` was still `🧰️framework/📦️packages/🟦️typescript`; O1 deliberately re-rooted that
  config at `🧰️framework` (`🧰️framework/🧪️tests/🎚️config/🟦️.ts:12`, with the reason in its docstring:
  `includeSource` globs cannot walk upwards, so the suite collected zero tests). Fixture follows source.
- Eight `projectionSha256` rows regenerated from the gate's own hash function (there is **no** owning
  generator for this fixture — `projectionSha256` appears nowhere but the fixture, the schema and the
  test; the honest regeneration path is a replay of the gate, which `🐍️k2-ownership-projection.ts --write`
  is). Later edits in this slice moved two more, regenerated the same way.
- Three `runVitest` selections that pointed nowhere (see §2 — they are the same defect class as item 2).

**Proof.** `🗑️generated/k2-ownership-after.txt`:

```
 7 pass   0 fail   526 expect() calls   Ran 7 tests across 1 file. [35.44s]
```

Owner count 43 → 46 (schema `minItems`/`maxItems` and the two count assertions moved with it).

---

## 2. Tests that run nowhere

_in progress — census script and results below_

### 2.1 Found through the ownership gate's `runVitest` law

| owner | defect | fix |
|---|---|---|
| `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📜️script.ts:26` | selected `"🧪️vitest.config.ts"`, **a file that does not exist** — `nx run …projektetage:test` could never have run | repointed at `../../🧪️tests/🎚️config/🟦️.ts` |
| `✏️s/🔌️plugins/🀄️wfc/🧪️tests/🎚️config/🟦️.ts` | a real, registered owner absent from the fixture | fixture row added |
| `🧰️framework/🛍️products/🎤️presentation` (×2) | both vitest configs sat at the predecessor path `<package>/🧪️tests/🟦️.ts`, inside `📦️packages/` | relocated to the semantic owners (below) |

### 2.2 `@semio-tech/presentation-react` — a 147-test suite that collected **nothing**

`🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (was
`…/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts`) aliased `@semio-tech/ui-react` to
`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` — the **inverted**
path; the real entry is `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`.
The whole in-source file failed to transform, so `bun ./📜️script.ts test` reported
`1 failed | 2 passed (3)` with **11 tests** and a green-looking "only one file failed".

Root fixes:

- the `ui-react` alias corrected, plus the missing `@semio-tech/ui-react/test` and
  `@semio-tech/animate-presentation-core` aliases (the latter is what the projektetage spec needs), and
  the projektetage spec alias repointed from `🟦️.ts` (a browser entry that only side-effect-imports) to
  `🔖️spec.ts`.
- `🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx:66` — the **production** module
  imported `act` (a React test utility) from `@semio-tech/ui-react`, which has never exported it. Moved
  into the `import.meta.vitest` block as `await import("@semio-tech/ui-react/test")`.

Result: **147 tests collected (was 0), 136 pass, 11 fail.** The 11 are genuine, pre-existing product
failures in a suite that has never run; classified in `🗑️generated/k2-presentation-react-failures.txt`
(6 × `presentation.chapters` undefined, 4 × `*.closest` on a null disposition, 1 × PDF canvas state).
`@semio-tech/presentation` itself: **2 files / 56 tests, all pass** (`🗑️generated/k2-presentation-tests.txt`).

---

## 3. The `4_gate` launch family

**Verified state.** K1 restored one row and reported "at least eight" missing families. The real number
is **34**. `📜️script.ts:9003-9009` documents that commit `6f33e313da` (2026-09-15) emptied
`INTERACTIVITY_ALL_APP_REQUIRED_GATES` *and* deleted the `⚖️gate…` rows from both launch files; the
per-gate fixtures and their tests were never updated, so each of those gates has been asserting a row
that does not exist.

**Measurement.** `🐍️k2-gate-launch-rows.ts` walks every JSON under `📚️library` and `🌎️hub`, collects
each object carrying a `4_gate` launch registration, and derives the command the owning test asserts
(`launchCommand` → `nxCommand` → `bun nx run @semio-tech/repo-lib:<target> --skip-nx-cache`). Before:
`declared=35 present=1 missing=34`.

**Fix.** `.vscode/🧩️launch.seed.jsonc` — 34 configurations appended to the `configurations` array
(re-read immediately before the write; Z1/V2 hold the same file). `.vscode/launch.json` regenerated with
`bun nx run @semio-tech/plugin-registry:generate` (`🗑️generated/k2-registry-generate.txt`, exit 0,
"`.vscode/launch.json regenerated`"). Never hand-edited.

**Proof.** `🐍️k2-gate-launch-census.ts` (`🗑️generated/k2-gate-census.txt`):

```
declared=35 missing=0
.vscode/🧩️launch.seed.jsonc: 4_gate rows = 37
.vscode/launch.json:         4_gate rows = 37
```

Every target referenced exists: 25 `@semio-tech/repo-lib:test-*` targets verified present in
`📚️library/📦️packages/🟦️typescript/📋️project.json`; `@semio-tech/value-resident`,
`@semio-tech/value-resident-rs`, `@semio-tech/ui-host-rs` and the two `os-hub` socket-grant targets are
real projects.

Owning gates re-run (`🗑️generated/k2-gate-tests.txt`, `k2-gate-failures.txt`):
`🔤️taxonomy-leading-grapheme` **9/9 pass**. Five other gates still fail, but **none** of their failing
tests is the launch-registration test (`🫙️artifact-empty-facet-authority`, `♻️taxonomy-pattern-compiler-reuse`,
`🕰️historical-json-source-encoding` — failures are semantic/oracle ones);
`🪶️artifact-empty-facet-authoring` and `📍️draw-destination-observation` need `SEMIO_TEST_ARTIFACT_DIR`
which only their nx target sets. `🛟️transaction-recovery-authority`'s launch assertion is unreachable
behind a pre-existing broken router-path assertion at `:94`
(`expect(router).toContain('🧪️tests/🧪️' + row.id + '../🛟️transaction-recovery-authority/🟦️.ts')` — a
malformed concatenation). All of these are pre-existing and outside this item.

---

## 4. `foundation-source-check` at `:371`

**Verified state.** Reproduced: `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts:83` exports
`HUB_DEV_BINARY_TARGET` and `🌎️hub/📦️packages/🦀️rust/📜️script.ts:107` imports it, but the owner's row
in `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json` listed neither.

**Fix.** That fixture row — `HUB_DEV_BINARY_TARGET` added to `declarations` (source order, after
`hubBinaryPath`) and to `rootImports` (the router's import-statement order, after `freeLoopbackPort`).

**Proof.** `🗑️generated/k2-foundation-source.txt`:

```
 11 pass   0 fail   262 expect() calls   Ran 11 tests across 1 file. [3.45s]
```

---

## 5. `documentId` → `artifactId` descriptor rename

_in progress_

## 6. `🔋️energy` `sim::` unit tests

_in progress_

## 7. Honest gaps

_in progress_

## 8. Files changed

_in progress_
