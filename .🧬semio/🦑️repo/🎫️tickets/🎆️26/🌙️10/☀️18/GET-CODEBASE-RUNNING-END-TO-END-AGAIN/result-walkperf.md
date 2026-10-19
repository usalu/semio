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
| Ownership policy/payload rules/limits unchanged | `implementationSha256` over 10 files (storage module, install schema, 3 workspace cjs, policy, schema, core, glue). Editing any fence invalidates all receipts. |
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
