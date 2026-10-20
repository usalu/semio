# round2-plan: ordered integration of every staged-but-unapplied set (integrate2, wave 1020d)

**STATUS 19:30: limited GO applied (every enabled ungated group, 23 ledger groups incl. pg-chain and 4 merge fixups); see `result-integrate2.md` for outcomes, fixups and deferred verification. Still open: course-B (Codex gate), lock-sync, flip, own-output, cli-zt, bzl-hermetic, graph-speed. The tables below are the plan as staged; additions made during the apply: pg-chain (after input-precision), fixup-rbe-default, fixup-rbe-probes, fixup-precision-storybook, fixup-policy-driver, bzl-incr merged copy.**

`C` = this ticket, `P` = `26/10/20/ARCHITECTURE-COURSE-ON-BAZEL`, `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `T` = `C/round2-tools`. State of this plan: 2026-10-20 18:55 (bzl-incr, bzl-default, bzl-nocache, boot-sweep and testfix-law added), nothing applied, nothing written outside `C/` and `/private/tmp/i2/`. Executor `integrate2`. Applying waits for the coordinator's "GO" (quiet tree: `show projects` is running now).

## 1. How to run it (after GO)

```
sh T/apply-all.sh                          # dry run: gates, READY state, read-only check of every group on the live tree
sh T/apply-all.sh --sim                    # proves the chain on a private overlay (/private/tmp/i2), refreshes T/touched.json
sh T/apply-all.sh --yes                    # applies the enabled groups in order; backup per group; verification; stops on the first failure
sh T/apply-all.sh --yes --only course-B --with-gated    # the gated course group, later (see 4)
sh T/apply-all.sh --yes --only lock-sync --with-lock     # dependency lock + cold sync (heavy, needs the lock slot)
sh T/apply-all.sh --yes --only flip --include flip --with-gated --flip-gate-passed   # Bazel default, only after the three products pass e2e on Bazel
sh T/apply-all.sh --undo <group>           # restore a group (refuses when a later group touched the same files or a file changed since)
```
Other options: `--with-lock`, `--flip-gate-passed`, `--only a,b`, `--skip a,b`, `--from <group>`, `--include own-output`, `--include-unready`, `--no-verify`, `--verify-heavy`, `--keep-going` (continue after a failed verification), `--ignore-running`, `--list`.
`apply-all.sh` = `python3 -I apply-all.py` (stdlib only, cross-platform). Each group runs the set's OWN apply tool on the live tree (anchored, content-hash checked, all-or-nothing, temp file + rename); the orchestrator adds: the `show projects` / dependency-sync guard, the Codex gate, a per-group backup of the files the group touches (`C/🗑️generated/round2-apply/backup/<group>/`, `ledger.json` with before/after sha256), the verification and the stop-on-failure. A re-run skips groups whose files still hold their recorded result ("applied (ledger)"). Exit codes: 2 guard, 3 refused/unknown state, 4 apply failed, 5 verification failed.
Group order, READY gate (the group's `result-<id>.md` must contain the word READY in its first 40 lines), gate for the course group and the optional own-output group are in `apply-all.py` `groups()`.

## 2. Ordered groups

Order = the order of the table. mem-gov first (coordinator: host-capacity admission before any heavy Bazel/play run). All orders listed here were proven on overlays (section 5).

| # | group | apply command (own tool) | check NOW (read only, live, 17:5x-18:07) | files (new) | verification after apply |
| --- | --- | --- | --- | ---: | --- |
| 1 | mem-gov (READY) | `python3 C/mem-gov-tools/apply.py --write` (+ `bun C/mem-gov-tools/regen-help.ts <root>`) | dry run: 8 patches + 23 new, no drift | 31 (23) + help fixture | 7 new suites (decision, sampling, sqlite oracle, gate, glue x3), `regen-help.ts --check` |
| 2 | orphan-child | `python3 C/orphan-child-tools/apply.py` (`--dry-run` for the check; the tool refuses its dry run on the live root while `show projects` runs, so the check passes `--root <live>/`) | 8 targets, anchors exactly once, no drift | 8 (4) | `descendant` suite (3), `half-born` suite (1) |
| 3 | boot-sweep (READY) | `python3 C/boot-sweep-tools/apply.py --root <live> --write` (`--undo` of its own exists; ours uses the ledger) | dry run: 23 targets resolve, no drift | 23 (17) | recovery suites under `S/🌐️platform/🥾️boot/🩹️recovery/**` (plan, heal, wiring, crash); the live drill of its result file is manual |
| 4 | testfix-defect | `python3 C/testfix-defect-tools/apply.py --write` | 13 targets "ready", 0 drift | 13 (8) | genuine-birth suites (5 + probe), `pack/absence`, course `resolution`; heavy: `testfix-defect-tools/live.sh real` (21 suites) |
| 5 | hub-bzl | `python3 C/hub-bzl-tools/apply.py --write` | adapter 133907 -> 134164 bytes, fingerprint test two bindings, 4 Go files to copy, longest path 95 | 6 (4) | Go toolchain suite (4), fingerprint suite |
| 6 | bzl-rbe | MERGED COPY `T/staged/bzl-rbe/apply.py --allow-drift` | dry run: 18 files (13 new) | 18 (13) | service suite (11), probes suite |
| 7 | demo-bzl | `python3 C/demo-bzl-tools/apply.py --write --accept-drift` | 6 files pending | 6 | `verify-independent.py` (0 FAIL), `bun law-check.ts` |
| 8 | bzl-nocache (READY; MERGED COPY) | `bun T/staged/bzl-nocache/apply.ts --write` (env `SEMIO_ROOT=<live>`; derived by `T/staged/make-nocache.py` from `C/bzl-nocache-tools`, original apply.ts + ops.json minus the course build op) | 20 of 21 ops (19 authored targets become cached actions + 3 missing reasons incl. the 3 cache-law targets; bericht:build requires demo-bzl) , 0 drift | 14 manifests | `bun C/bzl-nocache-tools/validate.ts`, `verify-independent.py`; heavy (one at a time, run twice, second must execute 0 local actions): `prove.sh light|bericht|wasm|storybook`; `prove.sh course` only after course-B |
| 9 | play-bzl | `python3 C/play-bzl-tools/apply.py --write --accept-drift` | 3 files (22257->22251, 5793->5844, 53201->53353) | 3 | `bun law-check.ts` (0 findings) |
| 10 | cache-law (NEW, `T/staged/cache-law`) | `python3 T/staged/cache-law/apply.py --write` | 3 targets pending | 1 | cache-correctness law suite (3 findings -> 0) |
| 11 | x-followup (peer x-ingress follow-up) | `python3 P/x-ingress-tools/followup/apply.py --live` | 13 ops pending, anchors OK; x-ingress main set: 24/24 present (applied 16:28) | 8 (4) | followup suite |
| 12 | course-A (course series, ungated) | `python3 C/course-bzl-tools/staged/apply-course.py --live --only x-dev,x-keys,course-bzl` | pending | 10 (6) | manifests suite, keys suite, local-stack suite (2 known red, G9) |
| 13 | docs-build | `python3 C/docs-build-tools/apply.py --write --rebase` | 30 pending, 4 deferred until the catalogue offers `remote-execution` (they land after group 5) | 1 (README) | `docs-build-tools/check.py` |
| 14 | bzl-default (READY; groups prep + setup + commands) | `python3 C/bzl-default-tools/apply.py --root <live> --group all --write` | dry run: 29 edits in 10 files, no drift | 10 | birth, portable-cli, bootstrap-cli, dependency-bootstrap, operations suites listed in `result-bzl-default.md` (touched ones run) |
| 15 | sync-incr (READY) | `python3 C/sync-incr-tools/apply.py apply --write` (rc 1 means "pending ops were applied") | 50 ops pending (its tool grew from 36; it is re-read at apply time) | 17 (6) | 5 suites |
| 16 | testfix-law (READY, 43 ops) | `python3 C/testfix-law-tools/apply.py --write` | 43 ops, 100 anchored hunks, 0 drift | 43 | edited suites (`tests()`); heavy `C/testfix-law-tools/live.sh` (34 suites, 2 jobs) |
| 17 | bzl-incr (READY) | `python3 C/bzl-incr-tools/apply.py --root <live> --write` | 16 changes (9 new module files `B/📏️incremental/` + 7 anchored edits), 0 drift | 16 (9) | incremental model/runner suite |
| 18 | parity (READY) | PREPARE `python3 C/parity-tools/build-staged.py`, then `python3 C/parity-tools/apply.py --write` | dry run: 24 files, no drift (13 new) | 24 (13) | 3 suites; then `repo verify build-parity --help` |
| 19 | regen-help | `bun C/mem-gov-tools/regen-help.ts <root>` | help snapshots equal the catalogue NOW (becomes pending after 1, 5, 14) | 1 | `regen-help.ts --check` |
| 20 | input-precision (not READY) | `python3 C/input-precision-tools/apply.py --apply` | dry run: 7 new + 4 edits (hook in project-assembly, 3 law files); tool is still being edited by its agent | 11 (7) | precision suite, cache-correctness law |
| 21 | course-B (GATED) | `python3 C/course-bzl-tools/staged/apply-course.py --live --with-graph --skip x-dev,x-keys,course-bzl` | pending; x-series all steps resolve (50 files, 10 steps), x-graph 21 ops | 50 (22) | `apply-course.py --check`, `x-integrate-tools/check-all.sh live` (heavy) |
| 22 | lock-sync (needs `--with-lock`) | manifest group of bzl-default (`python3 C/bzl-default-tools/apply.py --group manifest --write`; same edit as the frozen part of zt-e2e-fix) then `bun repo dependencies lock --all`, `bun repo dependencies sync` (cold sync, recipe changed by sync-incr), `bun repo dependencies sync` again (must say current) | 2 edits (tools `package.json` line 45, root `bun.lock`), 0 drift | 2 | the second sync is the check; `sync-incr-tools/nlink.py` before and after |
| 23 | flip (OPTIONAL + GATED: `--include flip --with-gated --flip-gate-passed`) | MERGED COPY `T/staged/bzl-default-flip/apply.py --group flip --flip-gate-passed --write`, then `regen-help.ts` | 32 edits in 13 files, 6 optional absent; merged copy: 12 files | 12 + help fixture | `regen-help.ts --check`, birth/portable-cli/CLI suites, doc laws; then rebuild the repo MCP server |
| - | own-output remainder (OPTIONAL, gated, skip) | `sh C/own-output-tools/staged/apply.sh --yes` | 1 file pending | 1 | - |
| - | cli-zt, boot-sweep, pg-chain, bzl-hermetic, bzl-nocache, graph-speed | placeholders: active only with `<id>-tools/apply.py`, a READY `result-<id>.md` and a touched-list (add the set to `sim.py`, run `--sim`) | cli-zt: `plan.json` not built; boot-sweep: no READY; pg-chain: edits not final; bzl-hermetic, graph-speed: nothing yet; bzl-nocache: result file exists, no apply tool | - | - |

Totals of the groups that exist as tools today: about 270 distinct files incl. lock-sync and flip (293 changed paths in the final overlay incl. repeats). Longest absolute path 182 UTF-16 units (limit 256).

## 3. Why this order (dependencies found)

- mem-gov first by request. It patches the shared adapter/operations files with anchored edits; the catalogue anchors of bzl-rbe did NOT survive it (both append to the tail of the `flags` and `operations` arrays), hence the merged copy of bzl-rbe (group 5).
- boot-sweep right after orphan-child (coordinator: hardens recovery, early); both edit the native storage module and the cleanup command with anchored edits that commute (proven). bzl-nocache after demo-bzl (its `bericht:build` op requires demo-bzl's output fix; without it the op reports PENDING and the other ones still apply) and before cache-law (it also gives the 3 retire/cargo targets their reason, so cache-law becomes a no-op; it stays as the standalone fix). The course `build` op of bzl-nocache is DROPPED in the merged copy: x-site/x-proctor/reconcile (course-B) own that target and both sets rewrite it (whichever is second refuses: x-site "hard anchor drifted", reconcile "target already has metadata"). Reverse: if course-B is applied first, the original bzl-nocache would refuse that op as drift, the merged copy is immune.
- orphan-child, testfix-defect, hub-bzl, demo-bzl, play-bzl, x-followup are disjoint from everything else except where the matrix says so.
- bzl-rbe before docs-build: the 4 README edits of docs-build are conditional on the catalogue offering `remote-execution`.
- docs-build uses `--rebase` (course-B's x-release edits the same README; every anchor still matches exactly once, so the order is free; proven both ways).
- bzl-default (prep, setup, commands) after bzl-rbe and docs-build (its own result says so; its README anchors do not overlap docs-build's), bzl-incr before parity (anchored edits of the operations catalogue/schema before the whole-file rebuild).
- lock-sync = the `manifest` group of bzl-default (the same edit as the frozen part of zt-e2e-fix, which is therefore superseded) followed by `dependencies lock --all` and sync-incr's cold sync; it is a separate heavy group (`--with-lock`) because it needs the dependency lock slot and a quiet machine. Undo of this group after the lock is refused by design (the lock rewrote the files).
- flip is the final, optional, gated group (`--include flip --with-gated --flip-gate-passed`): its help-snapshot fixture edits expect 24 occurrences of the Nx-default sentences but 38 exist once mem-gov, bzl-rbe, parity and bzl-incr add operations, so the merged copy `T/staged/bzl-default-flip/` (derived from the current original by `T/staged/make-flip.py` at every start of `apply-all`) drops that one file and the group's post step regenerates the fixture from the flipped catalogue (`regen-help.ts`, checked: 38 entries x 2 languages, 0 remaining "policy default (nx)").
- parity before input-precision before course-B: parity rebuilds 11 whole files from their then-current text (so it must come after every anchored set that edits those files: catalogue, operations schema, build script, bazel script, root manifest), input-precision regenerates its rules from the live manifests at apply time (so it follows the manifest changes of parity).
- regen-help after the last catalogue change (mem-gov, bzl-rbe, parity each add operations); it is also the post step of group 1.
- course-A (x-dev, x-keys, course-bzl) does not depend on the inference files. The rest of the course series (x-inputs, x-site, x-release, x-paths, x-proctor, reconcile, x-graph) is one chain that must stay in series order: x-site/x-proctor anchor on the `check` inputs x-inputs rewrites, and `reconcile` edits a test file x-inputs creates (proven: applying x-site/x-proctor before x-inputs is refused). So the whole chain is gated (group 17).
- own-output remainder: skip. Codex repaired the refusal live and froze the shell manifest; the remainder would add `sourceInputObservation: freshness` to 4 more shell targets and `dependsOn typegen`.

## 4. Gate: Codex ticket 26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY (course-B and own-output)

Codex froze five graph/inference files (shell manifest, owner-discovery, admission schema, graph fixture, source-demand test) for its native regression session 82722 (`workspace:test-workspace-build --configuration=release --skip-cache`; live now as pid 98426) and its native sessions 63416, 86151, 96264. course-B changes inference (`script-inputs/🟨️.mjs`, `⚡️caching/📥️inference/🟨️.mjs`), the admission schema + graph, the Bazel adapter and build script, and invalidates every graph receipt once. `apply-all.sh` therefore refuses course-B/own-output unless `--with-gated` is given, and with `--with-gated --yes` it additionally refuses while a process whose command line contains `test-workspace-build` or `native-build-regression` runs ("GATE CLOSED", seen at 18:07 with pid 98426). It cannot see sessions 63416, 86151, 96264 (no pid known): the coordinator must confirm them terminal before passing `--with-gated`.

## 5. Overlap / conflict matrix (files touched by more than one group) and how each was resolved

| file | groups | resolution |
| --- | --- | --- |
| `L/🏗️build/⚙️operations/🔣️catalog.json` | mem-gov, bzl-rbe, bzl-incr, parity, bzl-default flip (docs-build reads it) | all three append flags/operations at the same array tails. bzl-rbe re-staged (merged copy `T/staged/bzl-rbe/edits.py`: anchor on the object before the tail, `\n    }\n  ]\n}\n` at end of file, so it applies before or after); mem-gov and parity as authored (parity rebuilds from the current text). Both orders proven, differing only in array order |
| `L/🏗️build/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json` | mem-gov, parity, bzl-default flip | generated: regenerated by regen-help after the last catalogue change; flip's own edits of it are dropped (merged copy) |
| `L/🏗️build/⚙️operations/🧬️schema/🔣️.json` | mem-gov, parity, bzl-incr | mem-gov anchored, parity rebuilt after it |
| `L/🏗️build/📜️script.ts` | mem-gov, bzl-rbe, bzl-incr, parity, course-B (x-release), bzl-default | anchored sets commute (proven); parity rebuilt last |
| `L/🏗️build/🔧️tools/🏗️bazel/📜️script.ts` | mem-gov, hub-bzl, parity, course-B (x-release, x-paths) | same |
| `📋️project.json` (root) | bzl-rbe, parity | parity rebuilt after bzl-rbe |
| `L/⚡️caching/📋️project.json` | bzl-rbe, play-bzl, bzl-nocache, cache-law (sync-incr reads) | block-anchored edits; play-bzl and bzl-rbe base hashes drift by the other's edit, so both run with `--accept-drift`/`--allow-drift` (anchors still required exactly once; their hashes only guard staging time) |
| `🧰️framework/🛍️products/🦑️repo/README.md` (and the product README) | docs-build, bzl-default, course-B (x-release), flip | docs-build `--rebase`; bzl-default's anchors do not overlap; any order |
| `S/📦️packages/🟦️typescript/📋️project.json` (course), proctor rust `📋️project.json` | course-A, course-B | anchored, series order kept inside the wrapper; course-A first is proven |
| course `📋️project.json` (`build` target) | bzl-nocache (course op), course-A, course-B | course op dropped from the merged bzl-nocache, course-B owns it |
| root `📋️project.json` (`build`, `prepare`, `cpp`, `build-storybook`) | bzl-nocache, bzl-rbe, parity | disjoint targets; parity rebuilt last |
| native storage `S/🟦️.ts`, `S/🧹️cleanup/🎛️command/🟦️.ts` | orphan-child, boot-sweep | anchored, commute (proven both ways by the chain) |
| shell `📦️packages/🦀️rust/📋️project.json` | own-output (skipped) | Codex froze it |

Overlay proofs (`T/sim.py`, byte-copy overlays under `/private/tmp/i2/ov-*`, tool copies with the live root replaced; each run = the set's own write mode): C sets then course series = course series then C sets = split course-A/course-B = course-all (`diff -r` identical); mem-gov first and mem-gov late both apply (catalogue array order differs, 3 files: catalogue, help fixture, a path literal of the overlay); course-B last equals course-B before mem-gov/parity. All 207 changed json/ts/mjs/cjs/py files of the final overlay parse (`bun build --no-bundle`, `json`, `ast`). Not provable on an overlay: the verification suites that need a mounted image or the real tool store.

## 6. Checks recorded now (read only, live tree, no drift anywhere)

- All 17 groups above report `pending`, none drifted, none refused; regen-help is `applied` (becomes pending after 1/5/14). `C/🗑️generated/round2-apply/dryrun-live-2.txt`, per-set outputs in `check/*.txt`.
- Peer series (`P/x-integrate-tools/apply-all.py --check`): 9 steps pending (50 files), peer-ops applied, its `x-graph` step skipped because it looks for `x-graph-tools/staged/apply.py` (the script is `x-graph-tools/apply.py`): use `apply-course.py --with-graph` (done in group 17). x-graph alone: 21 ops pending. x-ingress main set: 24 ops present (applied 16:28); follow-up 13 pending. x-paths: pending (the 15:52-16:19 incident was reverted).
- Already live and not part of round 2: policy-admit, integrate round 1 groups A-G (371 files), boot-rebind (schema has `bootSession`), x-ingress main set, the 16:45 own-output refusal repair (Codex).

## 7. Not covered (no staged apply form) and open items

1. fnsplit4a residue: not a staged set. The shape pass over the 60 kept phase-split directories found 11 with emoji findings (duplicate doc emoji / folder emoji; list in `result-integrate.md` 6.3) and `plist2.txt` lines 27-49 were never processed. Needs an executor (rename the duplicate doc emoji of the sibling modules, run `fnsplit2-tools/shape.sh`).
2. zt-e2e-fix frozen part: now carried by group lock-sync (bzl-default `manifest`, identical edit). Original note: (remove `"@semio-tech/framework-filesystem": "workspace:*"` from `L/⚡️caching/🚀️bootstrap/🛠️tools/package.json` and the matching `bun.lock` line, then `bun repo dependencies lock`): still present live (lines 45 and 54); `python3 C/zt-e2e-fix-tools/apply.py --include-frozen` does the edits; needs a dependency lock, so it is the coordinator's call after the series.
3. Expected, not READY yet (placeholders exist, they activate on `<id>-tools/apply.py` + READY result): bzl-hermetic, graph-speed. Unready sets: input-precision, cli-zt, pg-chain (their agents were still editing at 18:07; `apply-all.sh` re-reads each tool at run time, so a resumed agent's newer staging is picked up, but touched-lists and the overlay proof must be refreshed: `sh T/apply-all.sh --sim --include-unready --with-gated` before GO).
4. Pre-existing reds that will show in verification: local-stack proxy cases (G9, marked known red); the Bazel aggregate index, `parsers`, `seal` and Nx-comparison suites (see `result-integrate.md` 6.6) are not in any group's verification list.
5. After the series (coordinator, heavy, one at a time): `bun repo show projects` (new graph hashes), `tools setup nx|bazel`, `bun repo verify build-parity --help`, `bun repo remote-execution status`, `sh P/x-integrate-tools/check-all.sh live`, then the cold runs (`course-bzl-tools/course-run.sh`, `play-bzl-tools/cold-run.sh`, `demo-bzl-tools/cold-run.sh`).
