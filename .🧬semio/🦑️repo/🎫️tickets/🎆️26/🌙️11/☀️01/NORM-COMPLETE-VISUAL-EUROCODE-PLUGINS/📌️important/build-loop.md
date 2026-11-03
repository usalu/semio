# 🔁 Build Loop (Mandatory for All Agents)

The repo MCP tools abort after 1800 s without progress, and the host (12 cores, 15.8 GB RAM) queues builds under memory pressure. Do **not** use `mcp__repo__test`/`mcp__repo__run`/`mcp__repo__cargo` for builds. Use the `repo` CLI through Bash with `run_in_background: true`:

```bash
cd /home/deck/semio && SEMIO_LANGUAGE=en repo run '<project>:<target>' > "$TICKET/🗑️generated/<you>/<name>.txt" 2>&1; echo "exit $?" >> "$TICKET/🗑️generated/<you>/<name>.txt"
```

- Projects: `@semio-tech/norm-enXXXX-rs` (Rust: `test`, `check`, `build`), `@semio-tech/norm-enXXXX` (TS), `@semio-s/plugin-norm-app-surface`. For others, read the `📋️project.json` files directly.
- You are notified when the background command exits; while waiting, keep writing code/tests for the next step. Never start a second copy of the same run.
- `[capacity] waiting … memory pressure` lines are normal: wait, do not cancel. Prefer `check` before `test`.
- Never kill other sessions' processes. Never run cargo/nx/bazel directly.

## Queue etiquette (memory is the bottleneck)
- Never run `repo show projects` (it reserves about 6.4 GB and blocks the queue). The project names are listed above; for others read the `📋️project.json` files directly.
- Run at most one build or test of your own at a time. Run `check` first, and `test` only once `check` is green.
- Batch your edits: compile after a meaningful chunk, not after every file.

## Incident 2026-11-02 (resolved)
Builds that failed in `build:dependencies` with "Bun installation owner is a symlink: CLAUDE.md" failed because of an untracked root symlink, which has now been removed. Simply re-run the build. Orphaned MCP builds were killed and the repo MCP server was restarted, so memory is free again.

## Single compile owner for shared crates (from 2026-11-02 01:00)
The integrator agent alone builds and fixes the compile of the shared crates: `@semio-s/plugin-norm-app-surface`, `@semio-s/plugin-norm-structural-scene` and `@semio-tech/norm-compliance-rs`. Package agents (WP-01..WP-11) must not start builds of these crates any more. Finish your code, then write compile concerns into `📌️important/requests.md` addressed to `integrator`. Artifact agents keep building their own `@semio-tech/norm-enXXXX-rs` project.

## One build per artifact (from 2026-11-02 01:05)
Throughput is about 6 builds per hour, so duplicate builds of the same project starve everyone.
- Each `@semio-tech/norm-enXXXX-rs` project has exactly one builder: its **norm-completion agent**. It writes every run to `🗑️generated/build-enXXXX/<n>.txt` (n increments) and appends the outcome to `🗑️generated/build-enXXXX/latest.md`.
- **Visual agents** of that artifact do not start `-rs` builds. If you have one queued, cancel it (stop your own background shell). Read `build-enXXXX/` logs instead, fix errors in your own files, and append "ready for rebuild: <reason>" to `latest.md`. The norm agent batches your fixes into its next run.
- Every build compiles all edits of both agents, so before starting a run, both agents' pending edits must be saved.

## Incident 2026-11-02 02:00 (fixed)
- `Cargo dependency lacks authored member authority: ✏️s/🧪️tests/🧪️mutation-vectors/...`: the `✏️s/Cargo.toml` `member-manifests` glob did not cover `🧪️tests/**`. It is fixed; re-run.
- `Unknown project in the closure` for new crates: every new Rust crate needs `📋️project.json` + `📜️script.ts` next to its `Cargo.toml`, like its siblings. The integrator is adding them for structural-scene, compliance and app-surface. Artifact norm agents must check this for the scene crate of their own artifact.
- `SIGKILL in build:graph-plan`: out of memory while planning the graph. Simply re-run.
- Never pass `--language` to `repo run`. Use the `SEMIO_LANGUAGE=en` environment variable.

## Shared crate project (2026-11-02 01:30)
The norm workspace crates (compliance, app-surface, results-config, structural-scene, registry contract) build and test through **`@semio-tech/norm-artifact-contract-rs`** (`📇️registry/🧬️contract/📦️packages/🦀️rust/📋️project.json`). The names `@semio-s/plugin-norm-app-surface`, `@semio-s/plugin-norm-structural-scene` and `@semio-tech/norm-compliance-rs` do not exist, so do not add project files for them. Only the integrator runs this project.

## Incident 2026-11-02 (ingress) - `check` cannot succeed, use `build`
`Original compiler ingress requires the repository launcher and authentic enforcing store` after `synchronize` is a framework defect, not ours: the artifact Rust `check` target runs a raw `cargo check` that bypasses the issued compiler launcher (no `RUSTC`), so `.cargo/config.toml`'s ingress refuses it. It affects every `@semio-tech/norm-enXXXX-rs:check` and `@semio-s/plugin-norm-mechanics:check`. Our crates, paths and locks are fine (`synchronize` legitimately rewrites `Cargo.lock`).
- Do not run `:check` for these projects until the integrator announces the framework patch here. Use `:build` as the compile gate (then `:test`). Details and the patch proposal: `🔍️audits/ingress-blocker.md`.

## MANDATORY from 2026-11-02 03:00: use `:build`, not `:check`
The artifact `:check` target can never succeed until the framework `CheckScript` is patched (see `🔍️audits/ingress-blocker.md`). Builders must use `SEMIO_LANGUAGE=en repo run '@semio-tech/norm-enXXXX-rs:build'` (or `@semio-tech/norm-artifact-contract-rs:build`, `@semio-s/plugin-norm-mechanics:build`) as the compile gate, then `:test`. Cancel any queued `:check` run of your own.
- Update: the framework patch is applied (artifact Rust `CheckScript` now uses the issued launcher). It is compile-untested. The next norm `:check` (the artifact's single builder) is the validation: if it still prints the ingress error, fall back to `:build` and report to `integrator`; if it reaches rustc, `:check` is usable again.

## Incident 2026-11-02 03:10 (fixed)
`semio-framework-plugin-retained-command (Unbalanced Rust source delimiter)`: a staged edit by another session had added a stray `}` in `🧰️framework/.../🧵️retained-command-job/🦀️.rs`. The coordinator removed it in the working tree. Simply re-run the build.

## Source boundary per crate (found 2026-11-02 03:35) - READ BEFORE ADDING CODE
"Rust crate Source extent or boundary exceeded before compiler" is a PER-CRATE budget, not a per-file cap (the `maximumBytes` in the message is the budget remaining for that crate when the file was read). Each crate's cfg-selected closure (all `mod`/`#[path]` files plus `include_str!`) must stay within **262144 bytes, 32 files, 4000 lines (comments and blank lines count)**. Splitting a module into sub-module files does NOT help and raises the file count; move code into a sibling crate (new `Cargo.toml` under `📦️packages/🦀️rust`, member in the plugin `Cargo.toml`, path dependency, `pub use` in the old crate) when a crate passes about 230 KB / 28 files / 3500 lines. Check before every build: `python3 -I "$TICKET/🧰️scripts/source-boundary-scan.py" ✏️s/🔌️plugins/📕️norm` (prints crates over the limit; `-v` lists all). Generated tables must be compact and size-guarded in the generator (see `🧰️ndp/codegen.py`). Rule details: `🔍️audits/source-boundary.md`.

## Oracle libraries available (2026-11-02 03:40)
structuralcodes 0.7.3, concreteproperties, sectionproperties 3.10.2, blue-prints, groundhog and sympy 1.14.0 are installed in `/home/deck/semio/.venv` (locked in `uv.lock`), together with numpy, scipy, shapely, PyNiteFEA and anastruct. Run oracles through the shared `python_oracle` helper or with `/home/deck/semio/.venv/bin/python -I`, not with bare `python3`. Upgrade the oracles to real third-party cross-checks where the library covers the clause (see `🔍️audits/oracle-strategy.md`).

## HOLD artifact builds until the "shared crates green" line appears (2026-11-02 04:25)
Every `norm-enXXXX-rs:build` currently fails in source admission on the app-surface facade ("Missing or ambiguous Rust module: store_ownership_macros", from the `#[path]`-mounted split). The integrator fixes this. Artifact builders must not start new builds until the integrator appends `shared crates green` here. Keep coding meanwhile.

## shared crates green (2026-11-02 04:34)
`@semio-tech/norm-artifact-contract-rs:build` exit 0 (compliance, ndp, scene + foundation, app-surface facade and six crates). Artifact builders may resume `:build`. `:test` of the contract project is running next (log integrate/contract-test.txt).

## Shared-crate compile errors (from 2026-11-02 05:30)
Artifact builders may fix small, type-directed rustc errors in the shared crates themselves (wrong `SurfaceKind` path, `.copied()` vs `.cloned()`, missing bounds or imports). Re-read the file first, make a minimal edit, and log it in `requests.md` under "shared fixes". Larger changes still go through the integrator. Fixed so far: viewport3d lowering/window, diagrams2d canvas-lowering/window (coordinator).

## Single fixer for the shared app-surface crates (from 2026-11-02 06:15)
Every artifact build currently stops on the same errors in `semio-s-plugin-norm-app-surface-panels` (CheckStatus::UnverifiedBasis/Info arms, FieldSpec::choices, chip_id moves, UiFixedList annotations). One dedicated fixer agent owns the app-surface sibling crates until it appends `shared crates green (artifact-validated)` here. Artifact builders must not edit app-surface files and must not start new builds until that line appears; keep working on your own crates meanwhile.

## "NX Funded Nx graph cache capacity exhausted" (2026-11-02 06:40)
`repo cache status` shows nothing to evict (Nx at 860 MiB of 1.28 GiB, graph at 1.78 of 3.37 GiB), so `cache gc` does not help. The error comes from concurrent graph generations: re-run the build once (after the app-surface hold lifts) and do not start parallel duplicates.

## MANDATORY build command from 2026-11-02 07:00 (Nx graph capacity)
"Funded Nx graph cache capacity exhausted" means every Nx graph namespace is held by a concurrent build, and the framework refuses at once by default. Always build with the authored wait knob:
```bash
cd /home/deck/semio && SEMIO_LANGUAGE=en SEMIO_NX_GRAPH_CAPACITY_WAIT_MS=5400000 repo run '<project>:build' > "<log>" 2>&1; echo "exit $?" >> "<log>"
```
At most one build per agent, and no parallel duplicates. The app-surface panels fixes are already in the tree (see `🔍️audits/panels-fix.md`), so the hold is lifted: artifact builders run their `:build` now with this command.

## Silent "Owned Nx invocation failed (1)" = host out of memory (2026-11-02 09:15)
No code defect: bun/Nx processes are OOM-killed (`journalctl --no-pager | grep -i 'out of memory' | tail`), or the compile refuses with "Insufficient physical compiler initial headroom" after 30 min. Check `free -m` (start builds only with more than 4 GiB available, at most about 2 builds fleet-wide), then re-run. To see task errors: `repo run '<project>:build' --output=static` (or `--verbose`) prints all task output; failures are between `> nx run <project>:build:prototype` and `NX   Running target ... failed`; the daemon log is `.cache/b/o/m/nx/graphs/<hash>/<id>/daemon-error.log`. Details: `🔍️audits/nx-silent-failure.md`.

## CENTRAL BUILD RUNNER (from 2026-11-02 09:20), supersedes per-agent building
The silent exit 1 failures are OOM kills: about 10 concurrent builds exhausted the 16 GB host (see `🔍️audits/nx-silent-failure.md`). From now on **no agent starts `repo run` builds or tests** except the central build runner.
- The runner builds at most 2 projects at a time, round-robin. It starts one only when `free -m` shows more than 4 GiB available, and always uses `--output=static` with the mandatory env vars.
- Request a build by appending `- ready for rebuild: <what>` to `🗑️generated/build-enXXXX/latest.md` (or `integrate/latest.md` for shared crates). The runner writes `build-enXXXX/<n>.txt` and appends `- run <n> (<target>): exit <code>, <k> rustc errors` to `latest.md`.
- Artifact agents read those logs, fix errors in their own files, and request again. Order: `:build` until green, then `:test`.
- Cancel any build of your own that is still queued.

## Known compile pitfall: `list_delta!` with `key: id` gives E0117 (updated 2026-11-02)
`key: id` implements the framework `Keyed` trait on the row inside the diff crate, which is an orphan-rule error whenever the row lives in the model crate. The macro stays unchanged on purpose: many non-norm crates (bim, procedural, ...) rely on that `Keyed` impl. For norm artifacts ALWAYS emit a local public marker `pub struct <Delta>Key;` with `impl protocol::list_delta::KeyOf<Row> for <Delta>Key` and the form `list: Vec<Row>, key: String = by <Delta>Key`. All norm diff crates and the generators (en1992-geometry-diff, en1993-entity-gen, en1993-storey-model-gen, en1996_gen_diff, field-adder/new_collection*.py) emit it now. Details: `🔍️audits/wave2/list-delta-fix.md`.

## CORRECTION (2026-11-02 09:50): the 04:34 "shared crates green" line is retracted
`norm-artifact-contract-rs:build` does not compile the app-surface panels crate, so it does not prove the shared crates compile for artifacts. Its `:test` exited 1 (cargo status 101, no diagnostics captured). Shared crates are only proven once an artifact `:build` passes them. Runner: queue `@semio-tech/norm-artifact-contract-rs:test` with `--output=static` so the failing tests become visible.

## 2026-11-02 14:20 builds reach rustc (infra)
"Insufficient physical compiler initial headroom" was the rust cache partition full of idle unit-cache lanes (nothing collected them before rustc starts). Collected with the framework collector; `build-runner/cache-guard.sh` keeps >= 9 GiB free. en1999 already shows rustc errors; read `build-enXXXX/<n>.txt` (`error[`). Details: `🔍️audits/wave2/infra.md`.

## 2026-11-02 15:50 "Admitted Cargo artifact plan Source changed" is retried now
Concurrent edits by other agents no longer kill a build outright: the framework re-plans and re-runs (up to 4 attempts, unit cache reused), and the runner re-runs a project immediately (up to 3 times) if the error still escapes. Keep edits of one file atomic (write once, not in many small saves). Details: `🔍️audits/wave2/infra.md`.

## Incident 2026-11-02 22:50 - "lacks authored member authority" for ndp crates (fixed)
- Cause: `semio-s-plugin-norm-ndp-model` had `workspace = "../../../../../../.."` (7 ups, resolves to `✏️s/🔌️plugins`); needed 6 ups to reach `📕️norm`. The crate was then in no owner's membership and `synchronize` also silently dropped it from `📕️norm/Cargo.toml` members. Fixed key and re-added the member line.
- Rule: `package.workspace` must resolve to the owning workspace root, and the crate must be in that root's `members`. Check with `python3 TICKET/🧰️scripts/check-cargo-authority.py` (no cargo needed) before building a new crate.
