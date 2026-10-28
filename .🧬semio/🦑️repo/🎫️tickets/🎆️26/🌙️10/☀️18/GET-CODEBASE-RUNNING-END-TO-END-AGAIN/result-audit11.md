# result-audit11 (lane audit11, prefix ab, wave 1027b): pre-landing adversarial review of the G47 queue

STATUS 19:25 FINAL. I reviewed 35 batches and the sealed flip, base→new against the live tree of 18:45-19:05.
- 31 LAND (several only after a rebuild; see the order below).
- 2 FIX FIRST with an audit11 copy, both verified on mirror `abm`: nolegacy `reapi` and edit30 `warmderive`.
- The flip: LAND, but use audit11's `post-flip.sh` and add the mirror default.

Nothing live was touched. No official run, no ticket tool, no sub-agent. Mirror `abm` is dropped and no process of mine is left.

## Verdicts

| batch | verdict | one-line reason |
|---|---|---|
| compliance `fwprocess` `fwmodules` `libother` `libtools` `libbuild` | LAND | Comment-only: the TypeScript printer with comments removed, the Python AST without docstrings and the Rust text without comments are identical base→new (`audit11-tools/neutral.mjs`). There are two declared exceptions: the lock schema's German now uses real umlauts, and the memo edit law drops `[DEBUG]` texts that nothing matches. |
| compliance `libgraph` | LAND after `rebuild_all.sh` | Comment-only, but 5 of its 11 rows DRIFTED on live (peer edits to project-nodes, owner-discovery, script-inputs, generations load and generations). |
| compliance `lang` | LAND after rebuild (same group as `names`) | One row DRIFTED (`📜️script.ts`: a peer already landed the `bazelStarlarkRulesV1(hermetic)` change) and one row is already NEW. No production code matches any removed English text (§lang). The serialized witness preload `installWorldWitnessV1` is untouched. |
| compliance `names` | LAND after rebuild | 16 renames. `git grep` outside the ticket finds no reference to an old name except in the 4 rewritten files. The moved files are byte-identical apart from `flatReferenceFactIndexV1`. No suite row, schema `$id`, doc or taxonomy file names the old folders. The jobserver test row DRIFTED (a peer's `cargoScratchBaseV1`). |
| nolegacy `lease` | LAND (after compliance) | A registry entry without standing can no longer break a running session (§lease scenario). The base is compliance's `new` for the routing file. |
| nolegacy `reapi` | **FIX FIRST → audit11 copy** | The lane copy refuses every command that fills `output_files`/`output_directories`, even when `output_paths` is set. REAPI 2.1 itself says those fields "will be ignored" in that case, so a conformant 2.1 client gets INVALID_ARGUMENT. RED on the lane worker (status 3), GREEN with the copy (16/0). |
| nolegacy `nobaseline` | LAND (own group, membership change) | Every reference found by `git grep` (`📒️baseline`, `OriginalResourceBaseline*`, `issuer-baseline-*`, `maximumLegacyKeyUtf8Bytes`) is in the batch, except the allowlist row, which is in nl-`lang`. The removed `issuer-drain-failure` line sits inside the `issuer-drain` branch, so it is dead code. |
| nolegacy `lang` | LAND after compliance (rebuild with `rebuild.py`) | Its 7 drifts are exactly compliance's `new` bytes (checked hash by hash). The closure loader's serialized functions get `refuse` as a parameter, so none of them references an outer identifier. |
| schemafirst (15 batches, `law` first) | LAND after compliance + nolegacy, via `rebuild_all.sh` | Every numeric literal (≥ 100) that a code row loses reappears as a schema value. The one apparent miss (`1023`) is `sweepBlockProgressRows-1`. There is no TDZ: `observationBlockPolicyV1` (line 843) is declared before its use (878). One cold round is expected (§schemafirst). |
| edit30 `warmderive` | **FIX FIRST → audit11 copy** | (1) Unsound: the parsed-file table is keyed without the compiler options. A tsconfig change with unchanged sources reuses the old SourceFile, whose cached `imports` miss what the new options synthesize (`importHelpers`→`tslib`, `jsxImportSource`), and `moduleDetection`/`jsx` decide module augmentations. The warm answer then differs from a cold one. RED: the corpus step "importHelpers on" answers wrongly on the lane copy. (2) A superseded deriver keeps about 1-2 GB for up to 10 min after every tool edit or landing. |
| edit30 `nxbuild` | LAND | Nextest builds from the same planned metadata the assert step already reads. It takes effect only when that metadata's `workspace_root/Cargo.toml` is the request's manifest. The real-Nextest law shows one Cargo call and identical binaries. |
| edit30 `witpub` | LAND (hot file: rebase with `witpub/edit.py` after compliance `lang` and schemafirst `bazelscript`) | The witness comes from the capture manifest right after every capture, and a planned file the capture lacks fails closed. `.witness` is excluded from plan reuse. No reference to the removed rule or runner remains outside the batch. |
| cisource `cisrc` | LAND (order with cicover `ci1`, rebuild the second) | Test, fixture and schema only. The manifest already declares the input. |
| cbudget `shard` | LAND, optional, after `clock` | The fixture is real Vitest output and the Python model reproduces it. LOW: the partition law returns early (and passes) when the course config path vanishes. |
| sealed flip (`flip.py`, `post-flip.sh`) | LAND, with audit11 `post-flip.sh` + mirror default | `flip.py` is sound: a dry run on live gives exactly one anchored line and keeps the file mode. `post-flip.sh` swallows the promote exit code, so a blocked promotion still prints `POSTFLIP exit 0` (§flip). |

## FOR coordinator (ready lines)

Order (each step re-checks `stage.py check` = base right before apply; this queue overlaps on about 30 files, see §overlaps):

1. **Compliance group.**
   - Run `sh C/compliance-tools/rebuild_all.sh`, then `sh C/compliance-tools/apply_all.sh /Users/ueli/Documents/semio apply`.
   - Verify each batch with `VERIFY_ROOT=… sh C/compliance-tools/staged/<b>/verify.sh`.
   - Revert with `… apply_all.sh … revert`.
2. **Nolegacy group.**
   - Rebuild: `python3 -I C/mirror-tools/mirror.py create nl-main` (fresh, compliance now live), then `python3 -I C/nolegacy-tools/rebuild.py`.
   - Apply `lease`, `nobaseline` and `lang` from `C/nolegacy-tools/staged/<b>/stage.py apply --root …`.
   - **For `reapi`, apply `python3 -I C/audit11-tools/staged/reapi/stage.py apply --root /Users/ueli/Documents/semio` INSTEAD of the lane copy.** It has the same 4 rows and the same bases (`check` = base on live 19:05). Verify with `VERIFY_ROOT=<mirror> sh C/audit11-tools/staged/reapi/verify.sh` (abm: 16 pass / 0 fail; verify now passes `--timeout 120000`, without which the ByteStream test hits bun's 5 s default). Revert with `… stage.py revert --root …`.
   - `nobaseline` deletes 9 files (a membership change): keep it out of graph-heavy groups or accept one re-inference.
3. **Schemafirst group.** Run `sh C/schemafirst-tools/rebuild_all.sh /Users/ueli/Documents/semio`, then `sh C/schemafirst-tools/apply_all.sh /Users/ueli/Documents/semio apply` (`law` first).
4. **Edit30 group.**
   - `nxbuild`: lane copy.
   - `witpub`: rebase per the lane (`witpub/edit.py <fresh root> apply` + `make-stage.py`), because compliance `lang` and schemafirst `bazelscript` both edit `📜️script.ts`.
   - **`warmderive`: use the audit11 copy.**
     - Rebuild on a fresh mirror after steps 1-3 with `python3 -I C/audit11-tools/staged/warmderive/rebuild.py <fresh mirror root>`. It runs the lane's anchored scripts plus audit11's anchors; byte-exact reproduction was proven on abm.
     - Then `python3 -I C/audit11-tools/staged/warmderive/stage.py apply --root /Users/ueli/Documents/semio`.
     - Verify with `sh C/audit11-tools/staged/warmderive/verify.sh <mirror root>` (abm: warm laws 7/0, tsc 0 new, suites 70/2 = the lane's known set, `VERIFY OK`). Revert with `… stage.py revert`.
5. `cisrc` (rebuild after cicover `ci1` if that lands first). `shard` only after `clock` is proven.
6. **Flip.** Run `python3 -I C/self-g-tools/flip.py apply`, then **`sh C/audit11-tools/staged/flip/post-flip.sh /Users/ueli/Documents/semio`**. It must print `promote exit 0` and `POSTFLIP exit 0`. Apply `C/audit11-tools/staged/flip/mirror-seal-live.patch` to `C/mirror-tools/mirror.py` (one line, `env.setdefault("SEMIO_REPO_SEAL","live")`) BEFORE the flip.

Product proofs (one cold round covers steps 1-4, since every batch moves the captured implementation and several move the graph implementation digest):
- `course:build`, `course:test`, `print-rs:test`, dashboard build, then course hot ×2 (2nd: memo served + `[bazel] plan reused`).
- warmderive: the lane's two course edit probes (`edit-probe.py wdA …`, `wdB …`). The 2nd edit must be served without a `build:graph-derive` child. After the probes, `ps -A | grep '♨️warm/⌨️main'` must show at most one deriver, and none ~15 s after a tool file is touched.
- witpub/nxbuild: one print edit probe (no `RepoInputsWitness` action; the `cargo:build` first unit about 2.4 s earlier).
- `bun repo leases status` (holders read `unknown` only without a live entry).

## Findings with scenarios

### reapi (MEDIUM, conformance): refuse only a pure 2.0 command
- REAPI v2 proto, `output_paths`: "If `output_paths` is used, `output_files` and `output_directories` will be ignored!" (fetched from bazelbuild/remote-apis main).
- The lane's worker: `if(command.output_files.length>0||command.output_directories.length>0)return fail(3,…)`, which also covers a command that fills both. Its own test even pins that case as refused.
- Scenario: a 2.1 client that fills the deprecated fields for 2.0 servers as well (allowed by the spec) gets INVALID_ARGUMENT on every action. Bazel 8.4.2 fills only `output_paths` against a server whose high version is ≥ 2.1, so Bazel is not affected today. The executor is still the repo's REAPI server.
- Copy: `if(command.output_paths.length===0&&(…))` refuses only a pure 2.0 command, which is still no 2.0 support. In the test, the third refused case is now `{outputFiles, outputDirectories}`, and the served action fills `outputPaths` plus deprecated fields that must be ignored (still one `out` tree, empty deprecated symlink lists).
- RED on the lane worker: `Expected 0, Received 3`. GREEN with the copy: 16/0.

### warmderive (HIGH soundness + MEDIUM residency)

**(1) Option-blind parse reuse.**
- `reuse.sourceFile` keys a parsed file by `fileName, languageVersion, impliedNodeFormat, jsDocParsingMode` and is active even when the store is NOT armed.
- TypeScript 5.9 builds a SourceFile under `getCreateSourceFileOptions`. That includes `setExternalModuleIndicator` (from `moduleDetection`, `jsx` and `module`), and `collectExternalModuleReferences` caches `file.imports` on the object (`if (file.imports) return;`) together with the synthetic imports of `importHelpers`/`jsxImportSource`.
- Scenario:
  - Add `"importHelpers": true` to a tsconfig and change no source.
  - The memo re-asks the derived question (the config row is stale), and the warm deriver answers.
  - `main.ts` is reused with its old `imports`, so `tslib` is missing from the program's files and packages, while a cold child would include it.
  - The memo now serves a typecheck closure that lacks `tslib`, so a later `tslib` change does not re-key the target.
- Fix:
  - The table belongs to `(config, JSON of the effective compiler options)`, and another option set starts empty.
  - Script-inputs passes `JSON.stringify(options)`; `configFile` is a non-enumerable option, so it is not serialized.
  - Law, store level: parse counts under alternating option sets (3).
  - Law, corpus: a step "the configuration switches on importHelpers while every source keeps its bytes" (armed=false, plus a `tslib` package). It is RED with the lane store (`guarded.wrong` = that step; store law 1≠3) and GREEN with the copy.

**(2) The deriver outlives its implementation.**
- The endpoint is per (checkout, identity), and the identity moves on every edit or landing of any captured tool file.
- The lane's deriver checks its closure only at start, so the superseded one keeps about 1.0-1.2 GB (up to the 2 GiB limit, which is checked only after an answer) for `idleMilliseconds` = 10 min. No run of that identity ever asks it again.
- Under the fleet that means: every tool landing, every agent tool edit, and every mirror (its own root) that runs course edits adds a resident about 1 GB outside the capacity door. Example: 3 checkouts × 2 identities within 10 min ≈ 6 GB on the 16 GB target.
- Copy:
  - After the closure is proven byte-identical, the deriver records size, `mtimeNs` and inode of every closure file.
  - Every `warm.checkMilliseconds` (15 s, new schema key with EN+DE text, typed in `GRAPH_MEMO_POLICY_V1`) the idle deriver re-stats them and stops at once when one moved, or when RSS is above `maximumResidentBytes`.
  - Law: stamps hold, then fail on a rewrite and on a deletion; the wiring law pins the tick.
- What stays (MEDIUM follow-up for edit30, not blocking with (2) fixed):
  - The deriver is still not booked at the capacity door; the door sees it only as foreign memory.
  - It is not listed in `repo leases status` / fleet status.
  - The derived line still says "child process".
- Is a resident of ~1 GB acceptable under the 16 GB target and AGENTS (maximum control/performance)? Only bounded as in the copy: at most one live deriver per checkout's current implementation, gone within 15 s of being superseded and at most 10 min idle. Even then it must be visible and admitted. That is the lane's next step: book a resident reservation at the door and release it on exit.

**(3) Interaction with the flip (MEDIUM, a loss of benefit, not a soundness issue).**
- The deriver runs from disk, and its start check compares the run's captured closure (`root` = the checkout) with the files on disk.
- In `sealed` mode the run executes the promoted snapshot. Any peer edit since the last promotion makes the closure fail, so no warm deriver starts (exit 3), and a running one stops at the next tick.
- With the flip, warm answers therefore happen only while live equals the promoted snapshot. The wdA/wdB proof must run right after a promotion with no peer edits in between.
- Follow-up: start the deriver through the sealed entry with the run's snapshot digest.

**TypeScript program reuse otherwise sound.**
- `oldProgram` is passed only when every previous resolution probe holds. TypeScript itself refuses structure reuse on option changes, and the answer keeps the previous rows that held, so it is a superset.
- Low residual: the deriver runs `typescript` from `node_modules` on disk, while the run's captured loader checks installed packages for drift. A TypeScript upgrade between them is not detected by the closure check.

### lease: can an existing registry entry without standing now break running sessions?
- Every writer (the TS registry `flush`) writes `standing:leaseStandingSnapshotV1(…)`, which never returns undefined, through tmp+rename. No Rust or Go code writes this registry (`git grep` of `lease-registry`/`SEMIO_LEASE_REGISTRY_DIRECTORY`).
- Measured on live 18:5x: 422 entry files, **0 without standing**.
- The new `validEntry` makes a reader (with `prune`) DELETE a standing-less entry of another live process. The only producer would be a process still running pre-standing code (HEAD 10-25 has no standing), and none can be running.
- Even then:
  - The writer rewrites the entry every heartbeat.
  - The holder reads `unknown` and is never terminated. Base would terminate such a "legacy" holder after 4 h, so the new behaviour is safer.
  - Physical locks are separate from the registry.
- Mixed generation during the landing: an old parent rejects a new child's queue line with `holderState:"unknown"` (schema enum), so that progress line is omitted (cosmetic, `return null`).
- The status/fleet schema, Python oracle and texts follow. No Rust/Go consumer of the holder states exists.

### lang (compliance + nolegacy)
- 123 removed English error texts were checked for production matchers (`includes`/`match`/`toThrow`) outside the two batches. The only pinned ones are `Bazel session slot required` (sessions test 302, storage test 47), and it still comes from `requireBazelSlotV1`, which runs before the changed `readBazelSlotStateSyncV1`.
- `📖️status` now imports the build texts (a catalog, not a tool module), so its leaf promise holds.
- New `.mjs`→`.ts` imports (witness, phases, cutoff, native-manifests, provision, closure-read-worker) run under Bun or Node 24 (type stripping).
- The witness preload serializes only `installWorldWitnessV1`, which neither batch touches.

### schemafirst
- Value check: `audit11-tools/moved_values.py`.
- Re-keying is as the lane says:
  - No format stamp moves.
  - The graph implementation digest moves (revision, observation, generations, graphcore, buildmisc).
  - Compliance's docstring batches move the same files anyway, so one group gives one cold round.
- Before 1150 s-class re-inference is blamed on this group, check the g44 baseline.

### flip (MEDIUM, procedure)
1. `post-flip.sh` runs `SEMIO_REPO_SEAL=promote … | grep | cut`. The exit code is lost, so a blocked promotion (exit 1, `path:line:col`) still ends with `POSTFLIP exit 0`. Copy `C/audit11-tools/staged/flip/post-flip.sh`: captures `rc`, prints `promote exit N`, and sets `bad=1` (`sh -n` OK).
2. Mirrors carry no `.🧬semio/🦑️repo/⚡️cache` (git-visible files only), so a mirror's first `bun repo` promotes its own snapshot. In `sealed` mode, a batch applied to the mirror AFTER that first command is then never run by `bun repo` on the mirror: a silent false green for any lane that verifies through `bun repo` there. `mirror-seal-live.patch` makes `mirror.py run` default to `SEMIO_REPO_SEAL=live` (the lane rule "editors run live", made mechanical).
3. The standing promote rule is manual. A forgotten promote means the verdict tests the old snapshot (a false GREEN). Suggest that `land.py apply` ends with a promotion and fails the landing on exit 1.
   - LOW: `land.py sealed-flip check` maps to `flip.py status`, which exits 1 in live mode, so the base state reads as a failure.
4. A nested `repo` started mid-run reads the pointer anew, which the lane acknowledges, so the gate must promote only between runs. Peers' in-flight runs on live are not protected. This was worse in live mode, so it is not a regression. Follow-up: pass the parent's snapshot digest to nested runs.

## Overlaps (shared files across the queue; whoever lands second rebuilds)
- compliance × schemafirst: 21 files (observation, generations ×5, graph core, memo derive, arguments, cache-gc ×2, sampling, decision, sessions, port, `📜️script.ts`, linux oracle, precision, inputs, project-nodes, native-manifests, source-analyses, script-inputs).
- compliance × nolegacy: 8 (stacked, bases = compliance `new`).
- edit30 `warmderive` × compliance `libgraph` × schemafirst `inference`: `🧭️script-inputs/🟨️.mjs`. `warmderive` × schemafirst `memo`: memo `🟦️.ts` + schema.
- `📜️script.ts`: compliance `lang` × schemafirst `bazelscript` × edit30 `witpub`.

## Method
- Diffs and live status: `C/audit11-tools/diffs.py` (output `C/🗑️generated/audit11/*.diff`, `status.txt`).
- Drift inspection: `drift.py`.
- Comment-only proof: `neutral.mjs`, using the TypeScript printer with `removeComments`.
- Removed-text matcher hunt: `removed_texts.py` + `git grep -F -f`.
- Schema value check: `moved_values.py`.
- Manifest rehash: `rehash.py`.
- Reference hunts with `git grep` outside `.🧬semio` only.
- Mirror `abm` (default subtrees, 16,454 files): both audit11 copies applied, the RED proofs swapped the lane files in and restored them, guard clean on every run. Dropped at the end.
- Logs (sweepable): `C/🗑️generated/audit11/`.

## Files
- `C/audit11-tools/staged/reapi/` (manifest, files, base, stage.py, verify.sh).
- `C/audit11-tools/staged/warmderive/` (+ `rebuild.py`; lane tools kept).
- `C/audit11-tools/staged/flip/{post-flip.sh,mirror-seal-live.patch}`.
- `C/audit11-tools/{diffs.py,drift.py,neutral.mjs,removed_texts.py,moved_values.py,rehash.py}`.
