# result-td-graph (lane td-graph, prefix `tg`, wave 1027b): test debt of the graph suites

FINAL STATUS: 7 batches staged under `C/td-graph-tools/staged/`. All are test, fixture or suite-row changes; none touches production code. Every batch checks as `base` on live after G44, except one file of `envelope` that depends on `candidate` (by design). Each was verified on a mirror and has an apply → revert → re-apply round trip. Nothing on live was edited, no official run was made, and mirror `tg2` was dropped. Tools are in `C/td-graph-tools/` and outputs in `C/🗑️generated/td-graph/`.

## UPDATE after G45 (walks, nxoracle, scratch, candidate, discovery, envelope landed): `demand` RE-CUT on current live
- FOR coordinator: READY `python3 -I C/td-graph-tools/staged/demand/stage.py apply "$PWD" && sh C/td-graph-tools/staged/demand/verify.sh "$PWD"` → expect `VERIFY OK demand`. All 4 files are `base` on live now. Revert: `stage.py revert "$PWD"`.
- The conflict was one hunk in the styling-generator harness of `source-demand/🟦️.ts`. Resolution: peeredit2 `consumers` has the `providerReferencesV1` port stub and the `boundedScriptInputsV1` rename; both are kept. My replacement of its partial `scripts` literal by the extracted real `createScriptInputCache()` is also kept. The other 29 anchors replayed unchanged; anchors were re-recorded (30).
- Re-run from the repo root (root bunfig → pinned Bun 1.4.2), on a mirror of live plus the batch:
  - source-demand 30/0, versus 24/6 on live now;
  - native-scope 4/0;
  - 📋️scope 2/2 in the mirror: the two remaining reds are the mirror-only ENOENT ones (the mirror has no `✏️s/🧑‍💻dev` sources). The two laws this batch fixes pass there. Expect 4/0 on live; base is 2/2.
- Other checks: parse scan 3 files, 0 bad; mode law OK; apply → revert byte-exact → re-apply OK. Mirror `tg3` dropped.

## FOR coordinator (land in this order; one verify each; `stage.py` takes ROOT positionally or `--root`; revert in reverse order)
- FOR coordinator: 1. `python3 -I C/td-graph-tools/staged/demand/stage.py apply "$PWD" && sh C/td-graph-tools/staged/demand/verify.sh "$PWD"` → expect `VERIFY OK demand`: source-demand 30/0 (base 23/7), source-demand/📋️scope 4/0 (base 2/2), native-scope 4/0.
- FOR coordinator: 2. `… staged/walks/stage.py apply "$PWD" && sh … staged/walks/verify.sh "$PWD"` → derived-walks 4/0 (base 3/1), output-stamps 4/0.
- FOR coordinator: 3. `… staged/nxoracle/stage.py apply "$PWD" && sh … staged/nxoracle/verify.sh "$PWD"` → run bare without the Nx env: portable-model 7/0 (base 5/2), nx-engine 19/0 (base 14/5), admission 20/0 (base 16/4).
- FOR coordinator: 4. `… staged/scratch/stage.py apply "$PWD" && sh … staged/scratch/verify.sh "$PWD"` → suite-table laws 7/0 + 7/0. The platform and ci-report laws keep the same failure sets as base.
- FOR coordinator: 5. `… staged/candidate/stage.py apply "$PWD" && sh … staged/candidate/verify.sh "$PWD"` → candidate 2/0 inside the official envelope (base 0/3 in the envelope).
- FOR coordinator: 6. `… staged/discovery/stage.py apply "$PWD" && sh … staged/discovery/verify.sh "$PWD"` → source-discovery 6/0 (base 5/1).
- FOR coordinator: 7. This step runs AFTER td-leases `envelope` (`python3 -I C/td-leases-tools/staged/envelope/stage.py apply --root "$PWD"`) and after steps 3 and 5: `… staged/envelope/stage.py apply "$PWD" && sh … staged/envelope/verify.sh "$PWD"` → bare: candidate 2/0, settlement 1/0 (base 0/2), portable-cli 15/0 (base 14/2).
- FOR coordinator: official proof after 1-7: `bun repo run @semio-tech/repo-lib:test-build-workspace-graph-laws`. I ran the same suite runner in the official envelope on the mirror with every batch applied. Remaining reds there:
  - pinned-publication 7/12 and optional-cache 6/14, both still open (see below);
  - mirror-only: scope 2 and portable-cli 1, because the mirror lacks `✏️s`.
  
  Everything else was green, with no "Inherited build test capabilities differ". Also run `bun repo run @semio-tech/repo-lib:test-build-graph-rust` once. Expected: lifetime, assembly and source green. They are envelope-only plus checkout scratch; the mirror envelope gave 3/0, 19/0 and 4/0 with in-repo artifacts.
- Rebase if a file drifts: `python3 -I C/td-graph-tools/anchors.py apply C/td-graph-tools/anchors/<name>.json <file>` replays the anchored edits onto the current bytes. Then rebuild with `python3 -I C/td-graph-tools/mkbatch.py <batch> <live> <root-with-edits> <rels…>`, which keeps the verify.sh. `edits/{derived_walks,scratch,envelope}.py` are replayable scripts.
- Other checks:
  - Mode law (`xplat5-tools/tests/test_stage_mode.py`) passes for all 7 stage.py.
  - Parse scan (`gate-g-tools/scan.ts`): 12 TS files, 0 bad.

## Key findings
1. **Run tests from the repository root.** The root `bunfig.toml` preloads the runtime pin (Bun 1.4.2) and the orphan guard. A `bun test` started in the test's own directory uses PATH Bun 1.3.14 without them. `C/backlog-tools/live-laws.sh` does exactly that, so `a1-base.txt` and `a1-laws.txt` are distorted. Example: output-stamps is 3/1 from its directory because 1.3.14 formats `require.resolve` errors differently, and 4/0 from the root.
2. **"generations 6 failures" does not reproduce.** From the root all 19 generations files pass, 86/0. One timing law flakes under fleet load.
3. **The graph laws need short in-repo artifacts.** Admission bounds fixture paths to 256 units; the ticket `🗑️generated` dir gives 257. Portable-cli Cargo fixtures and optional-cache datasets must live inside the checkout. The runner's default sandbox lives under TMPDIR, which is outside the checkout. Batch `scratch` gives `build-workspace-graph-laws` `"scratch": "checkout"`, as `build-graph-rust` already has. For a manual run use `SEMIO_TEST_ARTIFACT_DIR=<root>/.cache/test-sandbox/<lane>`.
4. **The Nx patch refuses `cacheDirectory` unless both `SEMIO_NX_FIXED_STORAGE` and `NX_CACHE_DIRECTORY` are set.** Batch `nxoracle`: the shared graph test support declares the repository's own namespace from `nx.json` when neither variable is set.
5. **D3 (G44) dropped two `dataProducerInputRoles` rows for `✏️s/🧑‍💻dev/📋️project.json` as "vanished".** The project exists on live; it was only missing in obsrows2's mirror. The `demand` batch restores both rows.
6. **Envelope declarations must match the suite row.** A td-leases `declareBuildTestEnvelopeV1` declaration has to equal the row's capabilities. `build-workspace-graph-laws` declares none, so the files declare `capabilities:{}`. With `{artifacts:true}` the official runner refuses with "Inherited build test capabilities differ", which I reproduced.

## Changes per batch
- **`demand`**
  - `source-demand/🟦️.ts`: the extracted-declaration harness now follows the refactors. `closureCodeV1` takes the transitive closure, including module constants and destructured `observedFs` bindings. It uses real walk-view ports, the real `generatorOutputCouplingInputs`, and the real non-late `closureApiV1`/`closureHandleOfV1`. Project-assembly gets its own imports as ports. Cache literals are replaced by the extracted `createScriptInputCache`. The walk port is `boundedScriptInputsV1`, and the memo seeding uses the `{value,observations}` shape.
  - Graph fixture: the two dev rows are back. The resource-refusal case now expects the refusal text of the gate port, which ts-closure introduced.
  - `📖️source`: new `inferenceImportPortsV1`.
  - `📋️scope`: a `.ts` URL that is spawned as `process.execPath` argv[0] counts as an executable child, not a text resource. This covers the real front door `🚪️door/📜️script.ts`.
- **`walks`** (derived-walks): the only row a project may share with its TypeScript program's support is its own resolution `tsconfig.json`. The law computes that row independently. Timeouts raised to 120 s.
- **`nxoracle`**: `🧰️support` gains `declareNxOracleNamespaceV1`.
- **`scratch`**: suite row `"scratch": "checkout"`.
- **`candidate`**: the loop law targets `loadBuildGraphUnpinnedV1`, which now owns the loop; `all` rows use baseline. Dynamic `import()` and bare `import.meta` are rewritten to bindings. The attempt law binds `observationV1`, the inference producer and the real `physicalSourceOriginV1`.
- **`discovery`**: the cancellation law uses the support module's genuinely issued signal (`withCommandSourceControlV1`) instead of a spread copy of the invocation. The process module now refuses such copies with "Live Original process issuance required".
- **`envelope`**: candidate, settlement and portable-cli declare their birth.

## Open, classified and not fixed (owner decision needed)
- **📌️pinned-publication, 7/12 everywhere, a real API drift.** `loadBuildGraphV1` needs the reader store of the workspace that created the pin ("Required original cache reader unavailable"). Reuse needs a retained snapshot identity. Reset moved from the build script into the backend modules (`🔧️tools/🏗️bazel/💾️storage/🗑️reset`, Nx backend), so the text law looks in the wrong place. The tests need a redesign around the generation pin model; the graph lane should own this (same finding as obsrows2).
- **🧊️optional-cache, non-hermetic by design.** It uses the REAL repository graph store and leases:
  - deadline refusals while queued behind peer builds;
  - quota law red because the live optional store sits exactly at the 1 GiB cap (1073741825 > 1073741824);
  - the writer laws need native caller capabilities (an envelope declaration with no authored capabilities does not supply them).

  The fix is a private disposable repository and store fixture, not a test-only patch.
- **🦀️rust source, lifetime and assembly: envelope-only plus checkout scratch.** They call `receiveScriptProcessInvocation(process.env)` directly, so td-leases' declaration does not apply. They are green under the `build-graph-rust` runner. Making them bare-runnable means converting them to `withBuildTestInvocationV1` plus a declaration (a follow-up).
