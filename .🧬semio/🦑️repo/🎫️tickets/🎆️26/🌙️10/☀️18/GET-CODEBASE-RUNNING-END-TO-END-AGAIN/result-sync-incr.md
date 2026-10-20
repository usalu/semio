# result-sync-incr (executor sync-incr, wave 1020d, resumed from 1020c agent a9e1edca51ea4e27d)

READY: set staged in `C/sync-incr-tools/` (50 anchored ops), mirror-verified (15 new tests pass, 0 regressions in 9 existing suites, tsc clean), NOT applied. Apply only after the coordinator says GO and no `show projects` / dependency sync runs.

Nothing outside `C/` was written; heavy work only in `/private/tmp/si/` (one test process at a time).

## 1. Incremental dependency sync (finished and re-verified)

New module `…/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/` (`🟦️.ts` pure core, `💾️storage/🟦️.ts` glue, `🧬️schema/🔣️.json` schema-first contract, `🧪️tests/`). Five tiers, each a sealed content-keyed receipt, so a manifest edit only costs what changed:

| tier | skips | proof |
|---|---|---|
| gate (`verifyJavascriptDependencySyncCurrentV1`, before the ownership walk) | the whole sync when the last complete sync still holds | sealed `sync:all` receipt: every scope read matches its generation receipt, every installed ledger matches its directories, no obsolete generation |
| installed ledger | the 70-84k-entry installed-paths scan per generation | per-directory seven-field stamp ledger; settled+unchanged stamp = one lstat, other directory = relist + digest; identity (dev/ino/mode) always checked; complete scan only when no ledger proves the generation, which then seals a new one |
| execution replay | the second provider walk (and the scope-read walk) | replay of the recorded physical observations; a full stamp younger than the settle window defers to a real walk |
| ownership index | readdir over the workspace tree in a new process | settled listings persisted and seeded back, trusted only under their exact live stamp |
| lock | `bun install --lockfile-only` for scopes whose produced lock equalled the published lock under identical inputs | converged-lock receipt keyed by lock/recipe/runtime sha |

Also: retry over the delta for the closed class of concurrent Source/provider changes (max 3, never on cancel/custody/unresolved Source transition), `fullFenceInterval` 32 → 64, generation receipts sealed from the scope reads the run just proved (start times per scope, settle wait 5.25 s), all long loops carry progress and cancellation.

Verification (mirror `/private/tmp/si/m`, `sh test.sh`): 15 pass / 0 fail / 369 expects. Ajv validates every sealed receipt, a Python oracle (`🐍️oracle.py`) reproduces ledger rows, verify decisions, retry/lock/replay tables, ownership projection; mutation tables (write/touch/rmtree/replace/filetolink/retarget/chmod/dirtofile) agree with the oracle. Regression `sh regress.sh`: 9 existing suites, row diff vs unpatched tree is empty except one stale test now passing (recipe version). `sh check.sh`: tsc strict over the touched files, no diagnostics.

Not measured: end-to-end sync time (needs the lease and an idle machine). Expectation from the deps4/deps5 timelines: cold sync after the recipe change below is one full install; every sync after that is the gate (seconds to a minute); one manifest edit re-installs only the scopes whose generation key moved and verifies the rest by ledger.

## 2. Hardlink fix (new this wave)

Evidence first. `incremental.test.ts` probes real Bun 1.3.14 (`--backend=hardlink|clonefile|copyfile`, isolated linker, `workspace:*` members): no backend ever gives a workspace member `package.json` a second name; only a `file:` directory dependency gains one (all backends). `git grep` finds zero `"file:"`/`"link:"`/`"portal:"` deps in tracked `package.json`. So the 15:50 nlink=2 on 1794 manifests (which include `📋️project.json`, a file Bun never reads) is NOT proven to come from Bun's backend; the audit's suspect stays unconfirmed. We therefore do both: remove the hardlink backend and install an invariant that names the offender.

Staged changes:
1. Recipe (`💾️storage/🧬️schema/📥️install.json`): `"backend":"hardlink"` → `"backends":{"darwin":"clonefile","linux":"copyfile","win32":"copyfile"}`, recipe version 5 → 6. `javascriptDependencyProducerArgumentsV1` resolves `process.platform` and throws on a missing backend or on `hardlink` ("Supported non-linking Bun install backend required"). Generation keys already hash platform + recipe, so this is one cold re-install on every machine (expected, not a bug).
2. Invariant in the source custody: `requireSingleLinkInputV1` (schema constant `singleLink.message`) in `recheckJavascriptDependencyInputsV1`. That recheck already runs at the end of input capture and after every install, so a pre-existing or Bun-made second name on any `package.json`/`bun.lock`/`bunfig.toml` input refuses the sync with `Single-link Bun dependency input required: <path>`.
3. Laws (in the incremental suite): recipe equals the fixture table, never hardlink, source contains the refusal; real `link(2)` of a manifest is refused by the invariant and passes again after the alias is removed (nlink read by the Python oracle); Bun probe above; wiring law (recheck present at capture and after installs); live law "every tracked `package.json` and `📋️project.json` has nlink 1", compared with an independent Python walk (skips outside a git work tree).
4. Tests that hardcoded `--backend=hardlink` now substitute `{backend}` from a platform table: `🧪️tests/🧫️fixtures/🔣️.json`, `💾️storage/🧪️tests/📥️install.json` (+ its test, + the stale `version toBe(4)` → 6), `🧪️tests/🟦️.ts` (portable operations + independent Node bundle), `🏗️build/🧪️tests/🕸️graph/🚀️dependency-bootstrap/🟦️.ts`. Mirror: the two producer-argument tests pass; the bootstrap test cannot load in the mirror (missing compiler module outside the mirrored closure, unrelated), so it is a text-only edit verified by tsc-free anchor match.
5. Tool `C/sync-incr-tools/nlink.py [ROOT]` (git + python, cross-platform): 2,808 tracked manifests now, 0 with nlink != 1. Run it before and after the cold sync; `check.sh` runs it first.
6. Registration: `test-javascript-incremental-sync` target in `⚡️caching/📋️project.json` and the `javascript-incremental-sync` suite in `⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts` (same shape as deps5's three suites).

If second names reappear during the cold sync, `python3 nlink.py` names the paths immediately; the next step is the `link(2)` trace from `audit-hardlink-1020.md` (needs sudo, user must run it).

## 3. Apply after GO

```
cd C/sync-incr-tools
python3 nlink.py && python3 apply.py check          # rc 1 = pending, rc 2 = drift/refused (nothing written)
python3 apply.py apply --dry-run
python3 apply.py apply --write                      # refuses while dependencies sync|lock or repo show projects runs
sh test.sh /private/tmp/si/m                        # mirror, or run the new suite live: bun repo run <caching project>:test-javascript-incremental-sync
python3 nlink.py
```
Then one cold sync (recipe changed), a second sync (expect `Store dependency sync current`), then touch one manifest and sync again (expect only that scope re-installed, `Store dependency sync stale` reason names it). Interactions: deps5 (scan/fence) is already applied live and the anchors rely on it; fixups6 is applied and its copy of the bootstrap test equals live; storage module is in every generation-receipt closure, so the first run after apply invalidates receipts once.

## 4. Open items
1. Measure the three-sync sequence above and record timings in this file after GO.
2. Source revision reader and `show projects` still own the repo-wide `nlink` refusal for `📋️project.json`; the sync only guards its own inputs. The live law and `nlink.py` cover the rest.
3. nx SDK derivative `link(source, output)` and the blob-materialize hardlink fallback from the audit are untouched (outside this ticket's set).
4. `clonefile` is not proven on Linux; the recipe only selects it on darwin.

## Files
`C/sync-incr-tools/apply.py`, `nlink.py`, `check.sh`, `test.sh`, `regress.sh`, `mint.ts`, `staged/{core.ts,glue.ts,schema.json,incremental.test.ts,incremental.fixture.json,incremental.oracle.py}`.

## 5. Round 2 (10-21 01:00): why chain17 spends 30+ min in a dependency walk, and the staged fix

READY (round 2): `C/sync-incr-tools/staged2/` (12 anchored ops, `apply.py`, dry-run default, mirror-verified 7/7 tests, tsc clean). Apply only after chain17 is green and no sync/`show projects` runs.

### Diagnosis (read-only, no signals)
- The walk is not a receipt-tier miss in the sense of a bug: `show projects` ran the sync producer (`Store dependency sync stale ... "absent"`, line 39) and then did a genuine fresh install of all 6 scopes. Evidence: `.cache/b/d/m/j` holds three batches of 6 generations (19:43, 22:59, 00:18-00:45), one new directory every ~4.5 min since 00:18, and the activity log is 6 repeats of fences + one `bun install` gap + `workspace-edges` + `javascript-installed-paths`, then the provider/binding stage.
- Why a new generation each time: the three batches are byte-identical in `bun.lock`, root `package.json`, `bunfig.toml`, patches and every member manifest EXCEPT one field. Between batch 1 and 2 only `@semio-tech/process` `package.json#exports` changed; between 2 and 3 only `@semio-tech/framework-manifest` `exports` (typegen adds `./generated/manifest`, `./generated/ui-contract`). The generation key hashes every native manifest field (install.json: "Native fields other than scripts, nested semio ... remain install inputs"), so an `exports` edit changes the key of all 6 scopes, which reinstall identical trees. About 27 min of Bun plus fences per typegen touch, and it recurs.
- Receipt tiers: all per-generation by design (ledger, sync receipt, execution replay keyed by generation id and runtime). They can only replay a generation that exists. A new id has no ledger, and the sync receipt is sealed only at the end of a complete `all` run, so a killed or in-flight run leaves `absent`. Ownership index and converged lock did not miss; the 6,000-row ownership passes cost 7-12 s each, the time is the 6 installs and about 20 full fences.
- Ready receipts are write-once (`Fresh absent ready receipt required`) and `executionSha256` really changes with member `exports`, so the generation cannot be reused in place under the concurrent-generation design; the install tree can.
- Lease: the `mode":"exclusive"` in the activity rows is a literal label of `control.publish(...)`, not a lease acquisition (no lease is requested by those calls). The real hold is the single build leader of the concurrency scheme: pid 83915 (`mit-bestand-demonstrator:test-e2e`, TEACHING-ARCHITECTURE log `2026-10-21-native-combined-neutral5.log`) prints "Another process is building the dependencies (1279 s) ... waiting for that build" and shares the lease database with pid 75010. Other commands that need current dependencies wait for the whole build; commands outside the dependencies pin (cargo) run. So the stall is system-wide for dependency consumers, as long as one build runs.

### Fix staged in staged2 (new module `…/📦️dependencies/🪞️adopt/`, 4 anchored edits in the producer)
1. Install projection: key over the staged files with install-inert manifest fields removed (list in the schema: exports, main, module, types, typings, description, license, type, imports, ... 19 fields). Proven by Bun itself: frozen install of a workspace whose members carry different values for all 19 gives the identical installed trees and an unchanged lock; an install-relevant edit does not (test `adopt bun probe`). A Python oracle recomputes the projection digest for every mutation case.
2. Adoption: a fresh generation first looks for a donor generation with an adopt receipt of the same projection digest, bun version, platform, arch and recipe, whose installed ledger still verifies; it then clones `node_modules` and `w/<i>/node_modules` copy-on-write (`/bin/cp -c -pR` darwin, `cp -a --reflink=auto` linux, in-process copy elsewhere), checks the clone has the donor's exact installed shape (entries, kinds, names digest) and skips `bun install`; any failure removes the partial clone and falls back to the installer. No change of the generation key, the stubs or the write-once ready, so concurrent readers and pointers are untouched. Verified: clone is a structural copy with zero shared inodes (Python tree digest), no symlink embeds the generation id (4,604 links checked in a live generation), edges rewrite is idempotent on a cloned tree.
3. Receipts: adopt receipts are sealed after every fresh install/adoption and once for each existing verified generation lacking one (sound: an existing id proves the current runtime made it); receipts of removed generations are retired.
4. Registration: `test-javascript-adopt` target and suite.

### Expected effect and limits
- An `exports`-only edit: 6 x (4.5 min install) becomes 6 x (clone about seconds + installed-shape scan about 6 s); the remainder is the 3-4 full fences per fresh scope (about 20-40 s each under load), so roughly 8-10 min instead of 30, not seconds. Seconds need a cheaper fence (walkperf frontier verification, deps5 open item 2) or a rewritable ready, which is a design decision for the concurrent-generation owner.
- First benefit needs donors with adopt receipts: the generations chain17 is creating now get theirs at the next verification after apply; apply to the live tree once, run one sync, then the next typegen edit adopts.
- Not measured live; the clone strategy was only run on darwin APFS (Linux `--reflink=auto` and the Windows in-process copy are untested here).
- `staged/apply.py` (round 1, first set) now reports REFUSED on 'receipts publisher body': the live producer was changed after it was applied (Codex concurrent generations). Its content is live; the file is only a record.

Apply: `cd C/sync-incr-tools/staged2 && python3 apply.py check && python3 apply.py apply --dry-run && python3 apply.py apply --write`, then `sh test.sh` (mirror) or `bun repo run <caching project>:test-javascript-adopt`.

## 6. Round 3: install-relevant generation key (staged3, supersedes staged2)

READY (round 3): `C/sync-incr-tools/staged3/` (34 anchored ops, `apply.py` dry-run default, `test.sh`, `check.sh`). Apply only after chain17 green and the coordinator's GO, with no sync/`show projects` running; staged2 must NOT be applied (staged3 contains it).

- Key: `inspectJavascriptDependencyInputsReadV1` drops the 19 install-inert manifest fields (schema list `inertManifestFields`, Bun-probe proven) from the root and member manifests before they are hashed (material, which seeds `plan.generation` through the group hash) and before the `w/N/package.json` stubs are written. Source custody still holds the full original bytes. An `exports`-only edit now yields the same generation id: no fresh scope, no install, no clone (the sync gate goes stale on the new scope-read digest, every scope is `existing`, ready text is unchanged, receipts are re-sealed).
- Ready v3: `bindings.json` loses `executionSha256` (schema `SourceOwnerReady` version 3, reader, builder, 4 test sites, fixture, SQL oracle). Execution freshness is carried by the scope-read digest (`final.scopes`) into the generation and sync receipts, and consumers recompute execution live. This also removes a real failure of conc-deps' build/publish split (ready re-derived between the two runs; any exports change in between failed with "Existing current Store ready semantics changed"). `scope`, `generation` and `bindings[].sourceDirectory` stay (conc-deps' reference test reads them). Old v2 readies are refused once (one cold rebuild, together with lock-sync).
- Clone seeding stays for true key changes (a real dependency edit, or a dependent scope rekeyed by the group hash): fresh scope clones from a verified donor with the same staged-file digest, runtime and recipe, conditions from conc-deps: donor pinned (pins only, no new lease) BEFORE the ledger replay and for the clone; a failed pin skips the donor; on any clone or shape failure the partial tree is removed before `bun install`; admission at the loop head and `frame.built.set` measurement unchanged; no collector/removal in the producer.
- conc-deps (aa390c9167f22dd9e): no veto on key, ready v3 or clone seeding; nothing of theirs reads `executionSha256`.
- Oracle that an install-relevant edit changes the key: 50+ mutation cases (dependencies, devDependencies, peer/optional, version, name, bin, os/cpu/engines, overrides, patchedDependencies, trustedDependencies, workspaces, lock, bunfig, patch bytes, added/removed file) differ from the base digest, inert edits and key reordering do not, the Python oracle agrees on every digest; the Bun probe shows an install-relevant edit changes the installed trees or fails the frozen install while all 19 inert fields do not.
- Verification (mirror, real Bun 1.3.14): staged3 suite 8/8, tsc clean, 9 existing storage/reading/generation suites identical to the unpatched mirror (73 rows, empty diff), conc-deps suite 16/16 on both, idempotent re-apply 0 pending.
- Live check after GO: apply, one cold sync (key/ready change), then touch a member `exports` and run the sync: expect `Store dependency sync current` after a short re-seal (or `stale` once, zero `bun install`, zero adopted scopes), generation ids unchanged in `.cache/b/d/m/j`.
- Limits: the key chain was verified by source law plus the projection oracle, not by running `inspectJavascriptDependencyInputsReadV1` on the real tree (needs a store context); the clone was run on darwin APFS only.
