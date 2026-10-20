# Result walkperf (2026-10-19 23:05 - 2026-10-20 00:15)

Executor `walkperf`. Task: make `ensureBuildEnvironmentV1` / the dependency read path verify an unchanged tree cheaply (generation receipt) and fall back to the complete walk only on mismatch.
Outcome: **designed, implemented and tested, STAGED, not applied.** Nothing under the HARD FREEZE paths, no `package.json`/`bun.lock`, no `📜️script.ts`, no taxonomy file was edited. Apply after the sync exited green (`C/walkperf-tools/apply.sh`, it refuses while a sync or `deps4-*.sh` runs).

## Where the time really goes (verified from the code, not from the progress names)

`bun repo run` -> `ensureBuildEnvironmentV1` (`🏗️build/🚀️bootstrap/🟦️.ts`) -> `inspect(false)` -> `currentOwner()` **twice**. One `currentOwner()` is:
1. `withCurrentBunWorkspaceOwnershipV1(all)`: tree derivation + fence + final validate = **4 full directory-tree passes** (75,348 directories, 322 state probes, 312 manifests per pass; measured 617,189 operations, 36-58 s per call at load 45-107).
2. `readJavascriptDependencySourceOwnerScopeReadV1`: input plans (314 files, 4.4 MB), binding observation, then the provider walk `readJavascriptDependencySourceOwnerExecutionV1` **twice** (before/final), each ~12 min under load 35 (deps4 figure; I did not re-measure it: it needs a live Store context).
So one ordinary command = 8 tree passes + 4 provider walks (~25-50 min, matching env-pass). `withJavascriptDependencyReadStorageV1` repeats the same pair around every consumer body. `dependencies:workspace-edges` is the producer's link-rewrite stage; it is not part of an ordinary command (it only appeared because the sync was not green yet and `inspect` fell into `material-required` -> full producer).

## Design (schema-first)

Contract: `staged/schema.json` (-> `🧾️generation/🧬️schema/🔣️.json`): `Protocol` constants (bounds, stamp/anchor fields, settle 5 s, mismatch classes, tiers, implementation file list), closed `Receipt` schema, row formats of the two evidence files.

Artifacts in the Store partition `generation/` (immutable, content addressed, written under the existing exclusive custody only):
- `<sha256>.frontier.ndjson` workspace topology: one row per walked directory `["d",path,dev,ino,mode,mtimeNs,ctimeNs,settled,projectionDigest]` and per probed path `["s",path,kind,dev,ino,mode|"missing"]` (75,670 rows, 20 MB).
- `<sha256>.execution.ndjson` the provider walk's complete physical observation set `["o",location,kind,anchor,stamp,target,full]` (the exact `observations` map the walk already keeps and replays).
- `<scopeKey>.<receiptSha256>.receipt.json` sealed receipt (canonical-JSON sha256, 193 KB): implementation digest, runtime (bun/node/platform/arch + the walk's own `runtimeSha256`), recipe/ownership digests, per scope input sources (path, kind, dev/ino/mode, sha256), manifest text digests, binding pairs + observation digest, ready receipt fingerprints, execution scope fingerprints, evidence references, `scopeReadSha256`.

Verification `verifyGenerationReceiptV1` (cheapest first, every tier returns a typed mismatch, only cancellation throws): implementation -> runtime -> open conditions -> bindings -> ready -> inputs -> manifest texts -> frontier -> execution -> recompute the walk's own `sha256` and require it equals the sealed one. The result has the same `{required, receipts, sha256}` shape as `ScopeRead`, so callers are unchanged.
- Progress + cancellation: `control.check()` every 256 operations, `control.tick()` (publishes `dependencies:generation-verify:<n>`, yields, re-checks custody) every 4096.
- Locking: verification only runs under the caller's existing **shared** store custody (`withRepositoryCacheStoreV1` / reader lease), reads nothing exclusively, holds it for seconds instead of minutes. The exclusive writer is used only to publish: at the end of `dependencies sync` (producer, best effort, never fails the sync) and once from `inspect(true)` when no valid receipt exists (peers queued behind it find the receipt and skip every walk).
- Racy timestamps: a frontier directory whose ctime/mtime is younger than `walk start - 5 s` is flagged volatile (always re-listed and compared by projection digest); a `full` provider observation that young makes the capture **defer** (nothing is published). Directory stamps are only a cache key; tolerated churn is decided by re-listing and comparing the projection digest (the exact projection the walk itself retains), bounded by `maximumRelist` 4096 (`membership-churn` -> fallback).

## Soundness: each invariant of the old path, and what covers it now

| Old invariant | Now |
|---|---|
| Ownership policy/payload rules/limits unchanged | SUPERSEDED by walkperf2 (below): the sealed runtime import closure (175 files), not a hand list of 10. |
| Complete current membership (scopes, 1,415 packages) incl. refusal of symlink source dirs, real-directory requirements, collection/opaque exclusions | Replay of the walk's own retained observations: every listed directory (stamp, else re-list + projection digest incl. entry kinds, so a new symlink/dir/package.json changes it), every state probe (kind, dev, ino, mode), every manifest text digest. Any difference -> full walk, which then throws exactly as before. |
| Install inputs (manifests, lock, bunfig, patches, ancestry, missing stays missing) and the generation derived from them | Every source re-read and sha256-compared, dev/ino/mode re-checked, absent stays absent (same predicate as `recheckJavascriptDependencyInputsV1`). Generation is a pure function of these bytes + runtime constants. |
| Tool identity (bun/node/platform/arch, installer recipe) | `runtime` tier (same `javascriptDependencySourceOwnerRuntimeV1`, now exported). |
| Ambient resolution conditions closed (NODE_OPTIONS, conditions, preserve-symlinks) | Re-checked per verification (`conditions-open`); regex pinned to the storage source by a law test. |
| Source endpoint links + physical generation ancestry, before/after equality | Same `observeJavascriptDependencyBindingsReadV1` output compared by digest; verify runs before and after the witness/consumer and the two `sha256` must be equal. |
| Ready receipt (`bindings.json`) exact | Same reader, `fingerprint` (stat + sha256 of bytes) compared; expected text is a pure function of unchanged inputs. |
| Provider identity (Native == prospective Source instance, manifest/version, unowned fallback refusal, missing-to-root) | The complete observation trace is replayed (same stamp/anchor formulas, readlink targets). Equal answers to every query the deterministic walk made => equal result. `execution.scopes` fingerprints are carried in the seal. |
| Store identity/custody (`context.recheck`, forged/foreign controls) | `readReady` goes through `readJavascriptDependencyBindingReceiptReadV1` (rechecks the Store); every tick re-checks `context.check`; bootstrap still calls `store.recheck()` and the unchanged parser witness (typescript/@iarna-toml bytes). |
| Cancellation + progress | Preserved (above). |

Residual, accepted, not weaker than before: (a) the walk never hashed installed package contents either (only provider manifests); (b) stamps of settled directories/providers are trusted at nanosecond ctime (as the walk's own replay already does); (c) a receipt is integrity-sealed, not authenticated (same trust level as the ready receipts in the same private partition); (d) device number changes (reboot/remount) make rows stale -> one fallback + republish.

## Changes (all staged under `C/walkperf-tools/`, applied by `apply.sh`)

New (5): `staged/core.ts` -> `…/📦️dependencies/🧾️generation/🟦️.ts` (pure engine, imports only node + the repo validator), `staged/glue.ts` -> `…/🧾️generation/💾️storage/🟦️.ts` (Store/ready/bindings ports, publish, retire), `staged/schema.json` -> `…/🧾️generation/🧬️schema/🔣️.json`, `staged/test.ts` -> `…/🧾️generation/🧪️tests/🟦️.ts`, `staged/fixtures.json` -> `…/🧾️generation/🧪️tests/🔣️.json`. All paths < 130 UTF-16 units.

Anchored patches (22 ops, each anchor must occur exactly once; idempotent), defined in `walkperf.py`:
- `…/💾️storage/🟦️.ts` (9): export `javascriptDependencySourceOwnerRuntimeV1`; candidate walk returns its `observations`; execution result carries them; `ScopeRead` result carries `evidence`; `withJavascriptDependencyReadStorageV1` wrapped by `readJavascriptDependencyGenerationElseV1` (receipt first, unchanged complete walk as the `complete` branch; the function's text laws still hold: 2 `ScopeRead(`, 2 `ReaderScope(`, `custody.recheck()`); producer end publishes one receipt per requested scope (`publishJavascriptDependencyGenerationReceiptsElseV1`, failure only logs `[DATA]`).
- `🏗️build/🚀️bootstrap/🟦️.ts` (3): `inspect` tries the receipt before `currentOwner()`; the confirmation walks become receipt re-verifications; if no receipt matched, `inspect(false)` returns `receipt-required` right after the witness so the existing exclusive `inspect(true)` publishes (no new catch-with-return, `!publishReceipt` branch and the tail are byte-identical, so the existing AST law still binds).
- `🗂️workspaces/🟦️bun/🟨️.cjs` (7) + `🟦️.ts` (3): `projectListing` extracted (single source of the projection rule, used by the walk and the verifier), `readListingProjection` exported, custody gets `observed()`.

## Verification (all against a scratch mirror outside the repo; the real tree was only read)

Mirror = `python3 walkperf.py mirror <dir> --copy <test dirs>`: real directories along the patched paths, symlinks to everything else, patched/new files as real files.
- New suite: `bun test <mirror>/…/🧾️generation/🧪️tests/🟦️.ts` -> **9 pass, 0 fail, 143 expects**. It covers: 29 fixture cases (`fixtures.json`: match after touch/irrelevant file/hidden/opaque dir/source edit; each mismatch class: membership, manifest, input, binding, ready, provider (edit, ambient appears, link retarget), evidence tamper, implementation, runtime, conditions, churn, seal), tampered/schema-invalid receipts, deferral of unsettled timestamps, cancellation + progress (4,200 directories), glue verify against real `JavascriptDependencyReadContextV1` custody and real `observeJavascriptDependencyBindingsReadV1`/`readJavascriptDependencyBindingReceiptReadV1`, retirement. Cross-checks with third parties: **Ajv** validates the sealed receipt against the schema; an independent **Python 3** implementation (`os.scandir`, `hashlib`, `json`) reproduces every directory projection digest and the receipt seal from the fixture rules.
- Mutation check: removing the manifest-text check, the listing comparison, the provider stamp check or the input byte check each fails the case table; removing the link-target check does not (redundant with the link's full stamp, kept to mirror the walk).
- Existing suites, baseline (real tree) vs patched (mirror), same minted caller envelope: `🗂️workspaces/🟦️bun/🧪️tests` 61/61 pass both; bootstrap AST law `Bootstrap publishes missing parser receipts…` pass, `neutral bootstrap parser receipt…` pass, phase law `graph bootstrap authenticates its explicit phase` pass; storage source laws (`actual owner separates readonly inspection…`, `all actual installed receipt consumers…`, capacity, policy) pass; `💾️storage/🧪️tests/🟦️.ts` 20 fail both, `👓️reading` 38 fail both, **identical failing sets** (pre-existing: stale launch argv `--dry-run`, missing package `semver`, cascades of "Original process admission closed").
- `tsc --noEmit --strict` over core, glue, patched storage, patched bootstrap and the new test: 0 errors. `bun build --target=bun` of core/glue/storage/bootstrap OK, `node --check` of the cjs OK.
- `apply.sh apply --dry-run`: lists 4 patches + 5 new files; refuses with the running sync/`deps4-*.sh` processes (observed), refuses when a target sha differs from `bases.json` (checked with a poisoned base), `--rebase` applies anchors onto the current file.

Not run: any `bun repo` command, the real `ensureBuildEnvironmentV1`/producer/`withJavascriptDependencyReadStorageV1` against the live Store (needs leases and a green sync; the glue is covered with real custody objects but not the lease owner), Windows/Linux, `declared execution checks actual provider instances` (cannot load `semver` here, fails identically at baseline; the S2/S3 patches use only local identifiers, tsc-checked).

## Microbenchmark (real tree, read-only; `bench/bench.ts`, load average 45-107 on 10 cores)

`bun bench/bench.ts <mirror core> <mirror cjs> <repo> <repo>/.cache/b/d/m <library>`; it runs the patched cjs walk once, captures a receipt from it and verifies it.
- Old: one `withCurrentBunWorkspaceOwnershipV1` = 36.5 / 56.1 / 58.4 s, 617,189 operations (x2 per command, plus 4 provider walks).
- Capture (once per generation): 3.3-6.0 s; evidence 20.5 MB frontier (75,670 rows) + 3.8-4.7 MB execution (20,886 observations, real set is bounded by 131,072).
- Verify, full receipt (all tiers accepted): **6.0 s and 12.0 s** in the two complete runs (median of 3 repetitions each; machine load swung 45-107, the earlier run with a bench bug stopped early and is not counted); tiers: implementation 1 ms, bindings 44-89 ms (307 links), ready 1 ms, inputs 27-66 ms (314 files, 4.4 MB), texts 30-39 ms (312), **frontier 2.7-4.5 s**, **execution 2.2-3.4 s** (20.9k rows; ~14-20 s loaded at the 131k bound).
- Cost model: 21-24 µs per `lstat` measured at this load (typically 2-4 µs idle), so the tree pass is bound by ~75k + ≤131k lstats: **estimated 0.7-1.2 s on an idle machine** (estimate, not measured). Against 8 tree passes + 4 provider walks this removes >95 % of the work and holds the shared lease for seconds, which is what starved `native-fixed-image:.cache/b/d`.
- Peer effect: the sync's exclusive waits end once ordinary commands stop holding shared custody for 10+ minutes each.

## Apply and validate (coordinator, after the freeze lifts and the sync exited)

1. `sh C/walkperf-tools/apply.sh apply --dry-run` then `sh C/walkperf-tools/apply.sh apply` (add `--rebase` only if peers moved a target; the anchors are checked either way).
2. `C/walkperf-tools/run-tests.sh real` (explicit file paths, sequential, minted envelope).
3. One `bun repo run <any cheap target>`: expect the stderr/progress to show a single slow pass, `.cache/b/d/m/generation/` populated; the second run must show only `dependencies:generation-verify:*` progress and finish in seconds. A deliberate mismatch (edit a member `package.json`) must fall back to the complete walk, then republish.
4. `dependencies sync --all` should now end with 6 receipts (`[DATA] … not published` on stderr otherwise).

## Open items

1. Wire `🧾️generation/🧪️tests/🟦️.ts` into a project target and the launch seed (AGENTS command registration); frozen files (`project.json`, seed) not touched.
2. Taxonomy admission of `🧾️generation` (+ `💾️storage`, `🧬️schema`, `🧪️tests` members) not checked (no `bun repo`).
3. Remaining slow consumers: `readJavascriptDependencyCurrentBindingsV1` (diagnostic) and `💾️storage/🔒️enforcement/…/📤️publication/🟦️.ts` still call `ScopeRead` with a custody walk; they can reuse the same `verifyJavascriptDependencyGenerationReceiptV1`.
4. While no receipt exists the first command pays 3 passes instead of 2 (publish happens in `inspect(true)`); persistent deferral (provider manifest younger than 5 s at walk start) would repeat that, it logs `[DATA] … deferred`.
5. Possible compaction if the 20 MB frontier matters: parent-index/front-coded paths (~8 MB); the cost is dominated by lstat, not parsing.
6. Rerun `declared execution checks actual provider instances` once `semver` is installed again.

## Hygiene
No file under `🧰️framework/…/📚️library/` or any other repo path was modified; scratch mirror, minted envelopes, logs and bench output lived under the session scratchpad and are not kept. No debug logs added, no sub-agents, no git command, no `bun repo`, no cargo. Tools kept in `C/walkperf-tools/`: `walkperf.py`, `apply.sh`, `bases.json`, `run-tests.sh`, `gen-schema.py` (generates `staged/schema.json`), `bench/bench.ts`, `staged/*`.


---

# walkperf2 (2026-10-20 01:00 - 02:00): sound receipt (audit-verify-1019h section 5)

Executor `walkperf2`. HARD FREEZE respected: only `C/walkperf-tools/` (staged copies, scripts, bench) and this file were edited; nothing under the library, no `package.json`/`bun.lock`, no `📜️script.ts`, no taxonomy file. Still STAGED, not applied. Scratch mirror `/private/tmp/claude-501/walkperf2-mirror` (outside the repo, symlink farm plus real files for the closure).

## Verdict on each audit finding

| Finding | Result |
|---|---|
| 5.1 `implementationFiles` covers 10 of ~156 closure files | FIXED. The list is gone. A receipt now seals the actual runtime import closure of six entry points, computed by the repo's own closure reader `captureBuildSourceClosureV1` (`🏗️build/📥️source/🔏️closure`: Bun transpiler imports, literal `./x.json|cjs|ts` assets, module-owned sibling `🧬️schema` directories, no-follow ancestry, bounded). Entries (schema constants, `Protocol.implementation.entries`): the storage module, `🗂️workspaces/🟦️bun/🟨️.cjs` and `🟦️.ts`, the generation core and glue, and the patched bootstrap caller `🏗️build/🚀️bootstrap/🟦️.ts`. Result: **175 files, 2.1 MB**, including `requireNativeStoragePathV1` (`…/📋️contract/🟦️.ts`), `withArtifactRelativeDirectoryV1` (`📁️filesystem/🧾️observation/📁️descriptor`), the schema validator, store opening/paths/layout, native caller authority, the three policy/schema files the audit named, both patched files, `🔏️closure` itself. |
| 5.2 producer-side inputs unsealed | FIXED, with a correction of the premise (below). New `producer` section in the receipt, new tier and mismatch class `configuration-changed`. |
| 5.3 stamp trust on non-POSIX / bind mounts | NOT done (open item 1). |
| 5.4 binding endpoints | Holds. Residual clarified: the recorded pair set is one pair per package path of the scope (`javascriptDependencyBindingsV1` maps scope root + every member), so an unrecorded extra `node_modules` link can only appear at a new member, which changes membership (new `package.json` -> `membership-changed`). I tried the fixture "binding added at a fixture member without a pair": it matches because the fixture records a pair for one member only; I did not keep it (it would assert a property of the fixture, not of the real pair set). |

### Correction to audit 5.2 (verified in the storage module)
The producer does NOT read the real `~/.npmrc`, `~/.bunfig.toml` or `$XDG_CONFIG_HOME/.bunfig.toml`:
- `javascriptDependencyProducerEnvironmentV1` clears every `BUN_*` and `NPM_CONFIG_*` variable and `HOME`, `USERPROFILE`, `XDG_CONFIG_HOME`, `XDG_CACHE_HOME`, `TMPDIR`, `TMP`, `TEMP`, `NODE_ENV`, `NODE_OPTIONS`, then sets them to Store-fixed directories (`bun-home`, `bun-config`, `bun-cache`, `bun-global`, `bun-bin`, `tmp`).
- `prepareJavascriptDependencyStoreEnvironmentV1` throws `External installer configuration retained` when `bun-home` holds `.bunfig.toml` or `.npmrc`, or `bun-config` holds `.bunfig.toml`.
- The installer runs in a Store stage directory (staged manifests + the canonical project `bunfig.toml`), not in the workspace, and the project `bunfig.toml` must be absent or exactly the canonical text (`javascriptDependencyProducerConfigurationV1`).
So the reachable inputs are Source bytes (already sealed in `inputs`), the Store-fixed dirs and the surviving ambient variables (proxies, certificates). Sealing is nevertheless done for every level, because it is cheap and removes the argument:

`producer.files` (path label `w:` workspace, `s:` Store, `a:` absolute; kind `missing|file|other`, byte count, sha256; symlinks are followed, an unreadable file is `other`): `.npmrc` in the workspace root and in every required scope directory; `~/.npmrc`, `~/.bunfig.toml`; `$XDG_CONFIG_HOME/.bunfig.toml` and `~/.config/.bunfig.toml`; Store `bun-home/.npmrc`, `bun-home/.bunfig.toml`, `bun-config/.bunfig.toml`. (Scope-level and root `bunfig.toml` are already sealed byte-for-byte in `inputs`.)
`producer.environment` (name + sha256 of the value, never the value): every variable whose upper-cased name starts with `BUN_` or `NPM_CONFIG_` (covers `BUN_INSTALL_CACHE_DIR`, `BUN_INSTALL`, `NPM_CONFIG_REGISTRY`, any case), plus `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY`, `NO_PROXY`, `NODE_EXTRA_CA_CERTS`, `NODE_TLS_REJECT_UNAUTHORIZED`, `SSL_CERT_FILE`, `SSL_CERT_DIR`; ignored (set by `bun run` itself, would make receipts flap between launch contexts): `NPM_CONFIG_USER_AGENT`, `BUN_INSPECT`, `BUN_INSPECT_CONNECT_TO`, `BUN_INSPECT_NOTIFY`. Law tests pin the prefixes and the Store roles to the installer recipe (`ProducerRecipeProtocol.clearEnvironmentPrefixes`, `home`, `config`) and to the storage source text.
`runtime` gained `revision` (`Bun.revision`) next to the bun version, node version, platform, arch and the walk's own `runtimeSha256`.

## Design

Receipt (`staged/schema.json`, regenerated by `gen-schema.py`, deterministic): `implementationSha256` is replaced by `implementation = {entries, evidence}`, new `producer`, `runtime.revision`. New content-addressed evidence `<sha256>.closure.ndjson`, rows `["f", path, sha256, bytes]` sorted by UTF-8 path bytes; the evidence digest is the sealed digest of the implementation (covered by the receipt seal). Verify tier order: implementation -> runtime -> producer -> conditions -> bindings -> ready -> inputs -> manifest texts -> frontier -> execution -> seal.
- Verify does not re-resolve imports: it hashes every sealed row (lstat must be an ordinary file, size bound, sha256 and byte count equal). A new file can only enter the closure by editing a closure file (all imports in the closure carry explicit extensions; the Python scan found none without), so a content check of the sealed rows is complete. Entry-list equality is checked too. Cost on the real closure: **8-21 ms** for 175 files (measured, load 40-70).
- Capture runs the repo reader once per entry and unions the files (3.1-4.5 s at load 40-70, once per publish; needs the caller's `ScriptInvocation`, taken from `context.invocation`).
- Loaded-code race (new, not in the audit): a receipt only vouches for the code that actually ran. Capture defers (`implementation-unsettled`, nothing published) when any closure file has mtime/ctime later than `min(walk start, process start) - 5 s`. A process therefore never seals an edit that happened after it loaded its modules.

## Changes (all in `C/walkperf-tools/`)
- `staged/core.ts`: types `GenerationImplementationV1`, `GenerationProducerV1` (+ file/variable), closure evidence rows, `configuration-changed`, implementation tier by sealed rows, producer tier by canonical-JSON equality (mismatch path names the first differing file or `env:<NAME>`), unsettled-implementation deferral, `runtime.revision`.
- `staged/glue.ts`: `captureJavascriptDependencyGenerationImplementationV1` (repo closure reader), `javascriptDependencyGenerationImplementationFileV1`, `observeJavascriptDependencyGenerationProducerV1`, `JavascriptDependencyGenerationSubjectV1` (root, entries, ambient, loadedAtMs; injectable so laws run isolated), capture/verify/publish take the subject (default = live), retire keeps closure evidence.
- `staged/schema.json` + `gen-schema.py`: `Protocol.implementation` (entries, extraFiles, suffix, bound), `Protocol.producer` (levels, variables), tiers, class list, `Implementation`/`Producer*` defs, `ClosureRows`.
- `staged/test.ts`, `staged/fixtures.json`: see below.
- `walkperf.py`: `mirror --closure` (real files for the closure + full sibling schema dirs, empty dirs preserved: the reader refuses symlinked ancestors/schema dirs), `closure` (prints the closure of the staged tree), `overlap [DIR...]`; the 22 anchored patches are unchanged (storage 9, bootstrap 3, cjs 7, bun ts 3).
- `bench/bench.ts` (new tiers `base`, `implementation`, `producer`; args `... <closure-list.json> [implementation-root]`), `run-tests.sh` (mirror needs `--closure`).

## Verification (all run; mirror = `python3 walkperf.py mirror /private/tmp/claude-501/walkperf2-mirror --closure`)
- `SEMIO_TEST_ARTIFACT_DIR=<realpath dir> sh C/walkperf-tools/run-tests.sh mirror <mirror>`: generation suite **12 pass, 0 fail (990 expects, ~25 s)**; `🗂️workspaces/🟦️bun/🧪️tests` 61/61 (with a `/var`-symlink TMPDIR it fails 5 tests with "ancestor must be a real directory: /var": set `SEMIO_TEST_ARTIFACT_DIR` to a realpath); bootstrap AST law, parser receipt schema, phase law, storage source laws: 1+1+1+4 pass; `💾️storage/🧪️tests` 20 fail / 26 pass in the patched mirror vs 21 fail / 25 pass on the real tree in the same minutes (patched failing set is a subset; pre-existing: stale argv, missing `semver`, admission cascades).
- New fixture cases (62 total, 33 new): closure entry / transitive import / dynamic-import target / commonjs asset / json asset / module-owned schema edited -> `implementation-changed`; closure file removed, replaced by a symlink -> `implementation-changed`; identical-bytes rewrite, touch, unrelated sibling edit, file added next to the closure -> match; entry list changed; closure evidence tampered -> `evidence-invalid`; bun revision changed -> `runtime-changed`; workspace `.npmrc`, `~/.npmrc` (also as symlink, also unreadable), `~/.bunfig.toml`, `$XDG_CONFIG_HOME/.bunfig.toml`, `~/.config/.bunfig.toml`, Store `bun-home/.npmrc`, `bun-home/.bunfig.toml`, `bun-config/.bunfig.toml` appear; `BUN_INSTALL_CACHE_DIR`, `BUN_INSTALL`, `NPM_CONFIG_REGISTRY` (upper and lower case), `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS` set -> `configuration-changed`; ignored `NPM_CONFIG_USER_AGENT` and unrelated `PATH` changed -> match. The existing `bunfig appears` case still falls back via `inputs`.
- Real-closure law (`generation receipt: editing any file of the real closure…`): captures the real 175-file closure with the repo reader, copies it to a temp root, then appends a byte to EACH of the 175 files in turn and requires `implementation-changed` naming exactly that file; an unlisted sibling file does not matter; deleting or symlinking `requireNativeStoragePathV1`'s module falls back; restoring matches.
- Closure cross-checks: every input of an **esbuild** bundle (metafile, `bundle`, per entry) is in the sealed closure; an independent **Python** import/asset/schema scan is a subset too; the named fences (12 files from the audit), all 7 files of the old hand list and the three policy/schema files are sealed. Third-party schema validation (Ajv) accepts receipt and the producer observation; Python still reproduces frontier digests and the receipt seal.
- Mutation checks (edit the mirror's core, rerun): skipping the closure row comparison fails the case table and the real-closure law; skipping the producer comparison fails; skipping the entry-list comparison fails. Restored: green.
- `tsc -p` (strict, repo tsconfig + bun types) over core, glue, test, patched storage and patched bootstrap (884 files in the program): 0 errors. `bun build --target=bun` core and glue: OK. Docstring emoji unique per file (core 27, glue 15).
- Microbenchmark (real tree read-only, load 56-65, `bench/bench.ts`, 3 repetitions): old tree walk 53.4 s (617,389 ops, x2 per command); capture 5.4 s; verify tiers (median ms): base 0.3, implementation 8.3-11.5 (first cold sample 210), producer 0.1-0.4, bindings 16-141, ready 0.2-3, inputs 29-104, texts 11-20, frontier 1.9-5.0 s, execution 2.8-3.5 s; **full receipt 3.6 s / 7.2 s / 7.2 s** (median 7.2 s) vs 53.4 s for one walk, so the closure + producer tiers add ~10 ms. Raw lstat pass over the 75,373 directories: 1.3 s.
- `apply.sh check`, `apply --dry-run --ignore-sync --rebase`: 4 patches + 5 new files, bases unchanged (storage 8eb2e6dc…, bootstrap a85f28a5…, cjs 9dfda486…, bun ts 91df73ef…). Real `apply` refused while the new deps4 loop (pid 38014) runs: correct.
- Not run: any `bun repo` command, the real publish/verify against the live Store, Windows/Linux/devcontainer.

## Interaction with lease-fair (and other staged tools); apply order
`python3 C/walkperf-tools/walkperf.py overlap C/lease-fair-tools C/tsc-stage-tools C/bzl-stage-tools C/cargo-pressure-tools` (re-run it at apply time; lease-fair had no result file yet when I looked, files read from `lease-fair-tools/bases.json` at 01:30):
- **No target file is shared** with lease-fair (12 targets, all under `🧰️framework/🔨️modules/🏃️process`), bzl-stage (5), cargo-pressure (4).
- **tsc-stage** patches the same storage module (one anchored op, base 8eb2e6dc…): I applied both op lists in both orders onto the current file, every anchor occurs exactly once -> they commute. walkperf must then be applied with `--rebase` (anchored) because the strict sha check refuses a changed storage file.
- **lease-fair targets inside the walkperf closure**: `🔒️leases/🟦️.ts`, `🧭️routing/📥️invocation/🏃️process/🟦️.ts`, `…/🧬️schema/📈️progress.json`. Not a conflict (its progress schema addition `queueWait` is additive, `lease-queue:` prefix; the glue only imports `checkResourceLeaseControl`/`ResourceLeaseControl`, still exported; walkperf's `dependencies:generation-verify:<n>` resources use the generic message), but any change to these files after a receipt was published makes that receipt `implementation-changed` once (one complete walk, then republished).
- **Order**: (1) stop the deps4 loop; (2) remove deps4 `[DEBUG]`; (3) apply every other staged library change (lease-fair `apply.sh apply`, tsc-stage `apply.sh apply`, tax-stage etc.); (4) `walkperf.py apply --rebase` (dry-run first), then `run-tests.sh real` with `SEMIO_TEST_ARTIFACT_DIR` set to a realpath; (5) restart the loop. Publish happens in the next `inspect(true)` or at the end of the sync, i.e. after all closure edits. Any closure edit afterwards (also by peers: 126 of the 175 files are process modules) costs one complete walk, never an unsound accept.

## Open items
1. Audit 5.3: refuse to trust stamps when `ino` is 0 or on unqualified platforms/mounts (capture-side deferral is a 3-line change in `captureGenerationReceiptV1`; needs a Windows/devcontainer run to validate). Not done.
2. Closure breadth is the price of soundness: an edit anywhere in the 175 files forces a walk. If that turns out too chatty during development, narrowing must be done by removing entry points or splitting modules (visible in `walkperf.py closure`), never by a hand list.
3. A localized progress label (EN/DE) for `dependencies:generation-verify:` in `📈️progress.json` belongs to lease-fair's progress schema owner; currently the generic message is shown.
4. `producer` observes ambient files and variables at publish time (after the walk/install); a change during the install window is not seen. Same trust level as the complete walk (which never read them).
5. Open items 1-6 of the first walkperf report stand (launch registration, taxonomy admission of `🧾️generation`, remaining `ScopeRead` consumers, frontier compaction, `semver` test).

## Hygiene
No debug logs added, no sub-agents, no git command, no `bun repo`, no cargo. `bases.json` was rewritten once by `walkperf.py bases` with identical content (verified). Scratch (mirror, art dir, closure list, bench output) lives under `/private/tmp/claude-501/` and is removed at the end of the task.
