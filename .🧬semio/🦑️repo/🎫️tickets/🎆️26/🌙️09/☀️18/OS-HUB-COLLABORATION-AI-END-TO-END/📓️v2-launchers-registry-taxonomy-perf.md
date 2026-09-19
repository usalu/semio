# V2 — Launchers, registry data drift, taxonomy performance (2026-09-19)

Slice V2 of ticket 26/09/18 OS-HUB-COLLABORATION-AI-END-TO-END. Follow-ups handed over by
`📓️v1-verification-gates.md` §6 gaps 1, 2, 3, 6.

Every number below was produced by running the command on this machine; captures are in
`🗑️generated/v2-*.txt`. No cargo was run by this slice.

---

## 0. Headline

| # | Item | Before (V1) | After (V2) |
|---|---|---|---|
| 1 | playground variants without a conformant React dev launcher | **23 of 65** (46 of 65 counting the wgpu twin) | **0 of 65**, and **0 of 65** for wgpu |
| 1 | launcher law reporting | aborted on the first bad variant (`aggregator`) | reports **every** mismatch, both renderers, with port |
| 2 | registry vitest (`test long`) | 44 pass / **1 fail**, 7 files | **48 pass / 0 fail, 7 files, exit 0** (45 at my first green run; peers added 3 tests since) |
| 3 | `plugin-registry check` | 1836 | **1836** (re-run after all V2 edits: `v2-check-after.txt`, exit 1); `check-generated` **exit 0**. Re-censused: **no pure-data-drift family remains** (§3) |
| 4 | `verify taxonomy report --scope <subtree>` | **crashed** before producing a finding | **exit 0 in 1 m 45 s**, with live progress output |
| 4 | `verify taxonomy` progress reporting | none (silent for ~1 h 50 m) | phase/current/total line every 5 s |
| 1b | `🧹clean🧩️taxonomy❄️frozen-markdown-coordinates` / `…🕰️historical-json-source-encoding` launch rows | **missing** | registered; that suite went 17 pass/3 fail → **18 pass/2 fail** |

---

## 1. One React and one wgpu dev launcher for every playground variant

### 1.1 What was actually wrong

V1 measured "23 of 65 variants have no launcher matching `-- <variant>`". Measuring the whole
contract (command **and** env **and** port, for both browser renderers) with
`🐍️v2-launcher-audit.ts` showed the real surface:

```
$ bun 🐍️v2-launcher-audit.ts <repo>            # 🗑️generated/v2-launcher-audit-before.txt
playgrounds=65 reactLaunchers=74 wgpuLaunchers=68
=== react: 46 of 65 variant(s) without exactly one conformant launcher ===
=== wgpu:  46 of 65 variant(s) without exactly one conformant launcher ===
=== ports: 134 distinct, 0 collision(s) ===
```

Three independent root causes, all in the seed→generator contract:

1. **The port env variable was dead in 38 of 47 curated rows.** Rows carried `BLOCK_2D_PLAY_PORT`,
   `CAD_JS_RENDERER_PLAY_PORT`, `FEM_3D_PLAY_PORT`, … . The only port variable any dev server reads is
   `S_OS_PORT` — `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:150` (`process.env.S_OS_PORT ?? 6066`),
   `📺️renderer/…/🧊️wgpu/🌐️server/📜️script.ts:12`, and `📚️library/🟦️.ts:2739`
   (`frameworkOsPlaygroundDevEnv`). Outside `.vscode/*` the `*_PLAY_PORT` names appear in exactly one
   live file (`✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📋️project.json`) and otherwise only in closed
   tickets. Those launchers were binding the catalog default, not the variable they advertised.
2. **`SEMIO_PLUGIN` was written as the plugin id, but is read everywhere as the *variant*.**
   `🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts:54` sets `SEMIO_PLUGIN: variant`;
   `🏗️builder/🌐️vite/🟦️.ts:26` and `🧪️tests/⚖️parity/🏃️execution/🟦️.ts:206` read it as the variant.
   Only `🚀️launch/🟦️.ts:170,173` and the law at `🧪️tests/🚀️launch/🟦️.ts:69` treated it as a plugin id —
   which is why `norm`'s 15 variants all matched one another (`din16798 = 15`) and the 7 `demonstrator`
   variants all matched the `demonstrator` row.
3. **Commands were hand-written per row and drifted from the variant id**: `-- block 2d`,
   `-- wfc 2d grid`, `-- procedural 3d`, `-- 5d`, `-- trinity jack`, `-- wires`. These resolve at
   runtime (`resolveFrameworkOsPlaygroundPlugin` accepts aliases, `📚️library/🟦️.ts:2720`) but they are
   unmatchable by any registry-side law and make each row a separate thing to keep in sync.

### 1.2 Root fix — the registry owns the whole dev launcher, the seed owns only presentation

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts`:

| file:line | change |
|---|---|
| `🚀️launch/🟦️.ts:51-58` | `DevLauncherEntry` reduced to `{ namePrefix, order, wgpuOrder?, env?, users? }`. `command`, `reactEnv`, `wgpuEnv`, `reactServerReadyAction`, `wgpuServerReadyAction` are **gone** — a seed row can no longer disagree with the variant it launches. |
| `🚀️launch/🟦️.ts:60-61` | new `DEV_SERVER_READY` — the one `serverReadyAction` template (was 2 hand-copied variants differing only in whether `0.0.0.0` was allowed). |
| `🚀️launch/🟦️.ts:63-67` | new exported `playgroundDevCommand(variant)` = `bun nx run workspace:dev -- <variant>`. |
| `🚀️launch/🟦️.ts:69-74` | new exported `playgroundDevEnv(playground, renderer, port)` = `{ S_OS_PORT, SEMIO_PLUGIN: variant, SEMIO_RENDERER, SEMIO_APP? }`. |
| `🚀️launch/🟦️.ts:109-125` | `renderEntry` takes the `PlaygroundEntry` and composes registry env + the row's `env` extras. |
| `🚀️launch/🟦️.ts:141-163` | `renderUserEntry` starts from the same registry base, so the multi-user `s` rows stop being a separate env dialect. |
| `🚀️launch/🟦️.ts:179-183` | `defaultDevLauncher` is now only `{ namePrefix, order, wgpuOrder }`. |
| `🚀️launch/🟦️.ts:218-228` | the synthesis pass no longer skips a variant that has a curated row; it synthesises **whichever renderer's name is missing**, reusing the curated `order`. This is what gave the 6 `demonstrator` variants (`aggregator`, `aussuchen`, `bearbeiten`, `generator`, `koordinator`, `verfolgen`) their first wgpu launcher — `wgpuLaunchers` 68 → 74. |
| `🚀️launch/🟦️.ts:2-14` | docstring states the invariant: exactly one react + one wgpu launcher per variant, registry-owned. |

`.vscode/🧩️launch.seed.jsonc`: the `devLaunchers` table rewritten for all **47** rows by
`🐍️v2-normalize-launch-seed.ts` (kept in this folder; it re-reads the seed immediately before writing
so a peer's concurrent skeleton edit survives). `namePrefix` was re-derived from
`playgroundLaunchNamePrefix` so the seed now agrees with the name the generator emits (it was stale —
`🎬️animateplay`, `🏚️mitbestand🪵️aussuchen` — and silently corrected downstream by
`refreshDevLaunchNames`). Seed 3561 → 2803 lines; no `configurations` row was touched by that script.

`🧪️tests/🚀️launch/🟦️.ts:57-84` — the law, rewritten:
* renamed to "registers one react **and one wgpu** dev launcher for every playground variant";
* collects **all** mismatches into an array and asserts `mismatches.join("\n") === ""`, so a failure
  names every bad variant instead of dying on the first;
* checks the full contract (`command === playgroundDevCommand(variant)`, `SEMIO_PLUGIN === variant`,
  `SEMIO_RENDERER`, `S_OS_PORT === ports[renderer]`) for both renderers;
* excludes the `👤️<n>` multi-user rows, which are additional launchers for the same variant by design;
* asserts non-zero discovery twice (`playgrounds.length > 0`, `pool.length > 0`) so it can never pass
  on an empty set.

### 1.3 Result

```
$ bun 🐍️v2-launcher-audit.ts <repo>            # 🗑️generated/v2-launcher-audit-after.txt
playgrounds=65 reactLaunchers=74 wgpuLaunchers=74
=== react: 0 of 65 variant(s) without exactly one conformant launcher ===
=== wgpu:  0 of 65 variant(s) without exactly one conformant launcher ===
=== ports: 134 distinct, 0 collision(s) ===
```

Ports: the registry is the port authority (`[[package.metadata.semio.playground]]` per crate); the
audit enumerates all 134 declared dev/wgpu/user ports and finds **0 collisions**, so no new port had to
be allocated and no peer's port was taken.

`.vscode/launch.json` was regenerated by the registry's own generator, twice, with non-zero discovery
each time: `plugin registry catalog refreshed (60 plugin crates, 65 playgrounds, 54 framework
packages)` (`🗑️generated/v2-registry-generate.txt`, `v2-registry-generate2.txt`).

**Not done here:** the nx `activate-*`/`serve-*`/`dev-*` targets themselves were already uniform —
`resolveNxInvocation` (`📚️library/⚡️caching/🚀️bootstrap/📜️script.ts:262-282`) derives
`dev-<variant>-<renderer>-<profile>` and the matching `activate-…` watch target from the catalog, so no
`📜️script.ts` / `📋️project.json` target needed adding. What was missing was only the launch surface.

### 1.4 Also fixed: two missing `4_gate` launch rows

`🧪️tests/🕰️historical-json-source-encoding` asserts that both frozen-evidence gates are registered in
`.vscode/🧩️launch.seed.jsonc` **and** `.vscode/launch.json`; both rows were absent (verified absent in
the pre-image `🗑️generated/v2-seed-before.jsonc` too, so not caused by this slice). Added
`🧹clean🧩️taxonomy❄️frozen-markdown-coordinates` (order 410.18) and
`🧹clean🧩️taxonomy🕰️historical-json-source-encoding` (order 410.19) to the seed skeleton in the shape
the fixture declares. That suite went **17 pass / 3 fail → 18 pass / 2 fail**
(`🗑️generated/v2-frozen-evidence-tests.txt`, `v2-historical-json-after.txt`).

---

## 2. Registry vitest suite

V1 left one failing test — the launcher law above. With §1 landed:

```
$ cd …/🔌️plugin/📇️registry && bun ./📜️script.ts test long      # 🗑️generated/v2-registry-test.txt
 Test Files  7 passed (7)
      Tests  48 passed (48)
   Duration  47.85s
exit=0
```

Root-fixed, not skipped: the assertion still exists, is stricter than before (both renderers, exact
port, exact command) and now passes on real data.

Note for whoever runs it: the default `test` level is killed by a 15 s budget before the suites finish;
`test long` is the honest invocation (V1 §3.6 found the same).

---

## 3. `plugin-registry check` — re-census, and why no data-drift family is left

```
$ cd …/📇️registry && bun ./📜️script.ts check                   # 🗑️generated/v2-check-before.txt
plugin taxonomy tree violations (area(s) "✏️s/🔌️plugins" is "clean"):
1836 finding(s)
```

The gate reached its taxonomy audit (i.e. the "catalog is stale" pre-gate was green) because the
generator had just been run.

### 3.1 Every remaining family, normalised (1836 findings)

| count | shape | kind of work |
|---|---|---|
| 892 | `<leaf> is not reachable from Cargo manifest <manifest>` | Rust `#[path]` mounts — code + cargo |
| 615 | `surface "<s>" is missing 🎚️config/🧬️schema/…` / `👥️presence/🧬️schema/…` | ≈3 100 handcrafted schema leaves |
| 133 | `artifact "<a>" subset "<s>" is missing 📚️examples/ \| 🚪️io/ \| 🧬️schema/ \| 🏅️standards/` | directory-lane migration |
| 61 | `mode "<m>" is missing required child "<c>"` | directory-lane migration |
| 33 + 12 | `plugin root is missing 🎮️commands/🦀️.rs` / `plugin-root is missing …` | taxonomy-owner decision (V1 §6 gap 3) |
| 32 + 24 + 22 | `… example "<e>" is missing 🧪️tests/ \| 🖼️assets/ \| 🟦️.ts \| 🦀️.rs` | authored example content |
| 4 + 2 + 1 + 1 + 1 + 1 + 1 | singletons (`🎪️demonstrator` plugin-root `🦀️.rs`, `📐️cad 🪟️windows/🎚️config`, redundant `🔌️plugin` contract, undeclared subset path) | V1 §6 gaps 4–5, owned by B3d |

Per plugin: `🗄️stdio` 1071, `📕️norm` 201, `🏗️fem` 57, `➗️mathematical` 44, `🗒️note` 41, `🀄️wfc` 39,
`🎞️animate` 33, `🖍️draw` 31, `🎬️sequence` 24, `🌊️flow` 24, `🧱️block` 23, `🧩️puzzle` 19, rest ≤ 18.
**Non-`🗄️stdio` total: 765.**

### 3.2 The honest finding: the brief's "pure data drift" families do not exist any more

I was asked to fix the non-`🗄️stdio` families that are *pure data drift* — descriptor/registry/catalog
JSON, launch/port rows, stale names — family by family. **There are none left in the 1836.** The only
data-drift family in V1's census was the 17 `missing module target` false positives, which V1 itself
root-fixed. Every one of the 1836 remaining findings is one of:

* a **filesystem-structure migration** (a directory lane or an authored leaf that does not exist), or
* an **unreachable Rust source file** needing a `#[path]` mount and a cargo compile to verify, or
* a **taxonomy-owner policy decision** (the `🎮️commands` rule).

Spot-checked rather than assumed: `✒️writer`'s
`🗿️artifacts/✒️writer/…/📚️examples/🎬️demo-session/🦀️.rs` exists on disk (with `🟦️.ts`, `🖼️assets`,
`🧪️tests` beside it) and `grep -rn "demo-session" --include=*.rs ✏️s/🔌️plugins/✒️writer` finds **no
`#[path]` mount anywhere** — the gate is right and the fix is Rust, which this slice is explicitly not
to build.

Creating ~200 `📌️.empty.md`-only lanes for the 133 + 61 directory families would move the number but is
structural work inside plugins that slices B3a–B3d hold open right now; doing it blind from here would
collide with them and would not be verifiable without their builds. **Deliberately not done**; recorded
with exact counts and owners instead.

### 3.3 `🗄️stdio` — would a generator produce those ~3 100 schema files? No.

Asked to determine whether the `🗄️stdio` schema-lane family should come from an existing generator.
Checked the two candidate pipelines:

* **Generator contracts** (`📚️library/🔣️taxonomy.json` `generatorContracts`, **23 contracts**, 19 with a
  `previewTarget` enumerated by `🧪️tests/🚀️launch/🟦️.ts:18-27`): `actor-typegen`, `async-typegen`,
  `shell-typegen`, `ui-contract`, `ui-axes`, `schema-entity-catalog`, `graph-catalog`,
  `framework-manifest`, `plugin-registry`, … Every one declares explicit `outputRoots`. Enumerated
  them all: **exactly 7 outputRoots lie under `✏️s/🔌️plugins`, and all 7 belong to
  `external-step-assets`** (the `📐️step` AP214 `.stp` fixtures). **Exactly one contract has a
  `🧬️schema` outputRoot — `schema-entity-catalog` — and its three outputs are
  `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/{🟦️.ts,🦀️.rs}` plus a Go CLI leaf, none
  under a plugin.** So **no contract's `outputRoots` covers `✏️s/🔌️plugins/*/…/🎚️config/🧬️schema/` or
  `👥️presence/🧬️schema/`.** Those lanes are authored
  input, not generated output — which is also why they are tracked and why the validator
  (`🗿️taxonomy-validation/🟦️.ts:576-611`, `assertAppSchemaOwner`) requires all six
  `TAXONOMY_SCHEMA_FILENAMES` leaves (`🦀️.rs`, `🟦️.ts`, `🔗️.graphql`, `🔣️.json`, `🛰️.proto`, `📜️.wit`)
  to be present, rather than checking a generator receipt.
* **The plugin build** (`🔌️plugin/🏗️build/📋️plan/🟦️.ts`, `🏃️execution/🟦️.ts`) produces wasm components
  and descriptors; it consumes schema leaves, it does not emit them.

**Therefore a generator-based fix would mean introducing a new generator contract**, and its precise
shape would be: a `surface-schema-projection` contract whose `ownerPath` is `✏️s/🔌️plugins`, whose
`inputPatterns` are each surface's existing `🎚️config/🦀️.rs` + `👥️presence/🦀️.rs` Rust config/presence
types, whose `outputRoots` are the per-surface `🎚️config/🧬️schema/` and `👥️presence/🧬️schema/`
directories with `inclusion: "ignored"`, and whose preview emits the six language projections of each
type from the one Rust declaration — exactly the shape `ui-contract`/`schema-entity-catalog` already
use for framework types. That flips the family from "3 100 handcrafted files" to "one generator +
`bun nx run workspace:generate`", and it makes `assertAppSchemaOwner` a `check-generated` freshness
question instead of a presence question. It is a **schema-first design change owned by the taxonomy
owner**, not something a verification slice may land unilaterally; recorded here as the concrete
recipe, not attempted.

---

## 4. `verify taxonomy` — where the time goes, and what is fixed

### 4.1 Profile (measured, `🐍️v2-taxonomy-profile.ts`, `🗑️generated/v2-taxonomy-scope-registry.txt`)

The verb **already accepted `--scope`** (`📜️script.ts` `runTaxonomy`), so none had to be added — but
the scope only bounded half the work:

```
scope = 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry
inventory = 6.8 s      entries = 104        violations = 13
plan      → plan/incoming-coordinate-roots  0 … 72 049      (whole repo)
          → plan/incoming-candidates        0 … 75 975      (whole repo)   ≈ 65 s
          → then CRASHED
```

Three distinct problems, in order of cost:

1. **The plan phase ignores the scope.** `incomingReferenceSnapshot`
   (`🧹normalization/🟦️.ts:3462-3480`) triggers a whole-repository reference scan as soon as *any*
   in-scope entry has `sourcePath !== normalizedPath`. It then (a) derives **72 049 ancestor
   directories** from the 75 975 candidate paths and `lstat`s `<dir>/.git` for every one of them
   (`referenceCoordinateRoots`, `:3382-3416`), and (b) reads, hashes and parses **every file in the
   repository** (`referenceCandidatesWithProgress`, `:3452-3462`). This is semantically justified —
   any file anywhere can reference a moving path — but it means a 104-entry scope pays a 76 000-file
   bill, and the unscoped run pays it *on top of* inventorying all 75 975 entries. That is the
   ~1 h 50 m V1 measured; it is a linear-but-enormous constant, not an accidental quadratic.
2. **Every generator contract with a `compilerInputManifest` was inventoried unconditionally**, before
   the scope guard that decides whether the contract is even relevant (old
   `🧹normalization/🟦️.ts:7693`, guard at `:7698`). So a scoped run paid for it — and, worse, it
   **threw**: `Current compiler input manifest compiler input bytes differ: bun.lock`. Peers added 7
   root dependencies during this fleet's run (V1 §5.3), `bun.lock` changed, and the recorded
   `🧑‍💻dev/📤️distribution/🧾️manifest.json` (a *generated build output*) no longer matches it. One
   stale build artifact aborted the entire gate before a single finding — the same "one failing
   assertion hides everything" shape as V1 §3.6.
3. **A fresh `RegistryCatalogInputView` per contract.** `generatorInputPaths` built its own view when
   none was passed (`:6748`), so the view's memoised `lstat` cache was discarded once per generator
   contract inside one `generatorPlanning` pass.

### 4.2 Fixes

| file:line | change |
|---|---|
| `📜️script.ts:8208-8221` | `verify taxonomy report\|enforce` now passes a `progress` callback to `verifyTaxonomy` and prints `[verify taxonomy progress] <operation>/<phase> <current>/<total> <path>` on every phase change and at most every 5 s. Parity with `verify taxonomy implementation` (`:8194`). The gate is no longer indistinguishable from a hang. |
| `🧹normalization/🟦️.ts:7685` | one `registryCatalogInputView` built per `generatorPlanning` pass… |
| `🧹normalization/🟦️.ts:6746-6748, 6806, 7693, 7715` | …and threaded through `generatorInputInventory` → `generatorInputPaths` (new optional `view` parameter, already supported downstream), so all 23 contracts share one `lstat` cache. |
| `🧹normalization/🟦️.ts:7696-7707` | `compilerInputs` is computed **only** when the contract can actually be planned (`compilerOutputVerification \|\| outputProblem \|\| outputMutation \|\| patternMutation`), mirroring the gating `catalogInputs` already had. `patternMutation` was hoisted so `inputMutation` keeps its exact meaning. |
| `🧹normalization/🟦️.ts:7698-7706` | that computation is wrapped: a stale/unreadable compiler-input manifest now records `violation("generator-compiler-input-manifest-stale", <manifestOutputPath>, …)` and `continue`s, instead of throwing out of the whole verb — the same shape as the neighbouring `generator-activation-invalid` handler. |

### 4.3 Result

```
$ bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/…/🔌️plugin/📇️registry
[verify taxonomy progress] inventory/setup 0/1 …
[verify taxonomy progress] plan/incoming-coordinate-roots 0/72049 .agents
[verify taxonomy progress] plan/incoming-candidates 0/75975
[verify taxonomy report] clean=false errors=12 warnings=0 scope=🧰️framework/…/📇️registry
… 12 × directory-kind-unresolved
exit=0            67.09s user  24.74s system  1:44.57 total
```

Was: crash with no finding. Is: a complete, scoped, observable run in 1 m 45 s that produces 12 real
findings (unregistered semantic kinds for the registry's own `🧪️tests/*`, `🧫️fixtures/*`,
`🧬️schema/*`, `🚀️launch/🏷️name-prefix` directories). Capture: `🗑️generated/v2-taxonomy-scoped-registry.txt`.

**Not verified:** the unscoped whole-repo `verify taxonomy report`. With no scope,
`compilerOutputVerification` is true by definition, so it still reaches the `bun.lock` mismatch — which
now yields **one finding instead of an abort**, but the run itself still costs the ~1 h 50 m of §4.1
item 1 and I did not spend the host on it under the running fleet. The honest unblock for the
`bun.lock` finding is to rebuild the distribution bundle
(`@semio-tech/framework-os-dev:check-distribution` / its preview target), which regenerates
`🧑‍💻dev/📤️distribution/🧾️manifest.json` against the current lockfile.

### 4.4 The 28 dead `frozenCoordinateEvidenceContracts` — do **not** delete them

Re-measured: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:720`
`frozenCoordinateEvidenceContracts` now holds **41 entries, 13 whose file exists, 28 whose file does
not** (27 under `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`, one under
`26/08/23/END-TO-END-TESTING-REFACTOR/w14-audit`, one under `26/08/17/END-TO-END-TAXONOMY-NORMALIZATION`).

**The contract format does not allow removal without falsifying history.** Evidence, not opinion:

* `🧪️tests/🕰️historical-json-source-encoding/🟦️.ts:74` asserts the contract set minus one id has
  exactly `historical.originalContracts.count` members **and** `:75-76` asserts
  `sha256(canonicalJson(original))` equals a frozen digest in
  `🧫️fixtures/🕰️historical-json-source-encoding/🔣️.json`. Deleting 28 entries means rewriting the very
  digest whose purpose is to prove the set was not edited.
* `🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts:93` asserts the same set has length 38 with one id
  filtered out.
* `🧪️tests/🕰️historical-json-source-encoding/🟦️.ts:80-100` asserts the **physical bytes, size and mode**
  of `…/w14-audit/fixtures2.json` — one of the 28 — still on disk.

Running both suites confirms the current state (`🗑️generated/v2-frozen-evidence-tests.txt`):

```
historical-json-source-encoding: 17 pass 3 fail   (→ 18 pass 2 fail after §1.4)
  ✗ expected length 38, received 40        # a peer ADDED two contracts during this fleet's run
  ✗ ENOENT lstat …/w14-audit/fixtures2.json
frozen-markdown-coordinates:  fail — frozenMarkdownCoordinateEvidenceContracts gained
  frozen-ticket-app-schema-facets-* rows the fixture does not declare
```

So the set is *already* out of agreement with its own seals, in both directions: files deleted at
ticket-close (28) and contracts added by peers (2 JSON + several Markdown). A silent delete of the 28
would make the count law pass again while destroying the only record that those bytes were ever
frozen — which is exactly what the gate exists to prevent. **Deliberately not deleted.** The correct
owner action is an explicit, recorded retirement affordance (a `retired: { reason, ticket }` field, or
moving the row into a `retiredFrozenCoordinateEvidenceContracts` table whose digest is sealed
separately) so that the deletion is itself evidence. That is a schema change for the evidence-contract
owner; the two seal fixtures and the four suites listed above move with it.

---

## 5. Honest gaps

1. **`plugin-registry check` is still 1836** and this slice did not reduce it, because no family in it
   is the data drift the brief expected (§3.2). Every remaining family is named, counted, owner-assigned
   and spot-checked above. The single biggest honest lever is the `🗄️stdio` 615-file schema family, and
   §3.3 gives the exact generator contract that would replace it.
2. **Unscoped `verify taxonomy report` was not run to completion** (§4.3). It is provably not hung and
   now prints progress; it is still ~76 000 files of inventory plus ~76 000 of reference scanning, and
   the host is saturated.
3. **`bun.lock` vs `🧑‍💻dev/📤️distribution/🧾️manifest.json`** is a live data break caused by peers adding
   root dependencies. It is now a finding rather than a crash, but it is not fixed — the fix is a
   distribution rebuild, not an edit.
4. **`verify interactivity apps` is red for an unrelated reason** and never reaches its launch-coverage
   check: `📜️script.ts:8991` declares `INTERACTIVITY_ALL_APP_REQUIRED_GATES = [] as const`, and the
   self-test at `🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts:34` indexes
   `INTERACTIVITY_ALL_APP_REQUIRED_GATES[0].name`, so it throws before any discovery
   (`🗑️generated/v2-interactivity-apps-before.txt`). Left alone — it is neither in this slice's four
   items nor safe to guess the intended gate list. Note that the same gate's launch-coverage check
   reads `devLaunchers[...].command` with a fallback of `bun ./📜️script.ts dev <variant>`
   (`📜️script.ts:9166`), which no generated row has ever matched; after §1.2 removed `command` from the
   seed the fallback is the only path, so that check needs the fallback corrected to
   `bun nx run workspace:dev -- <variant>` when somebody un-breaks the self-test.
5. **A peer left a `[DEBUG]` log** at `…/📇️registry/🧪️tests/🚀️launch/🟦️.ts:116` during this slice. Not
   mine, not removed (slice K1 owns hygiene).
6. **Runtime vs tests.** Everything in §0 was produced by running the command here. What is *not*
   runtime-verified: no dev server was actually started from a regenerated launch row (the host is at
   its concurrency limit and other slices own the boot recipes). The claim that `S_OS_PORT` is the live
   port variable is grounded in the four reader sites cited in §1.1, not in a booted server.

---

## 6. Files changed

| File:line | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts` (2-14, 44-74, 109-125, 141-163, 179-183, 196-204, 218-228) | registry-owned dev command/env/serverReadyAction; `playgroundDevCommand` + `playgroundDevEnv` exported; `SEMIO_PLUGIN` = variant; per-renderer synthesis so every variant gets both launchers |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🚀️launch/🟦️.ts:57-84` | launcher law reports every mismatch, covers both renderers, checks command+env+port, asserts non-zero discovery |
| `.vscode/🧩️launch.seed.jsonc` | `devLaunchers` table normalised (47 rows, 3561 → 2803 lines); two missing `4_gate` frozen-evidence rows added |
| `.vscode/launch.json` | regenerated by `…/📇️registry/📜️script.ts generate` (never hand-edited) |
| `📜️script.ts:8208-8221` | `verify taxonomy` emits phase progress |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts:6746-6748, 6806, 7685, 7693-7707, 7715` | shared catalog input view across generator contracts; lazy compiler-input inventory; stale compiler-input manifest becomes a violation instead of an abort |
| `.🧬semio/…/📓️v2-launchers-registry-taxonomy-perf.md` | this report |
| `.🧬semio/…/🐍️v2-launcher-audit.ts` | full launcher/port audit (all mismatches, both renderers, port-collision check) |
| `.🧬semio/…/🐍️v2-normalize-launch-seed.ts` | one-shot seed `devLaunchers` normaliser (re-reads immediately before writing, peer-safe) |
| `.🧬semio/…/🐍️v2-taxonomy-profile.ts` | phase-by-phase taxonomy timing probe |

No baseline, seal, digest or fixture was rewritten to make a failure go away. The one seal this slice
*could* have rewritten — the frozen-coordinate-evidence digest — is deliberately left intact (§4.4).

Captures (delete with the ticket): `🗑️generated/v2-launcher-audit-before.txt`,
`v2-launcher-audit-after.txt`, `v2-registry-generate.txt`, `v2-registry-generate2.txt`,
`v2-registry-test.txt`, `v2-check-before.txt`, `v2-check-after.txt`, `v2-check-generated.txt`,
`v2-taxonomy-scope-registry.txt`, `v2-taxonomy-scoped-registry.txt`,
`v2-interactivity-apps-before.txt`, `v2-frozen-evidence-tests.txt`, `v2-historical-json-after.txt`,
`v2-seed-before.jsonc`, `v2-launch-before.json`.
