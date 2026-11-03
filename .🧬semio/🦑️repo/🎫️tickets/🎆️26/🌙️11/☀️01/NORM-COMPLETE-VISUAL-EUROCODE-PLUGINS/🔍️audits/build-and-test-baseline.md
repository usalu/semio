# 🧪 Build and Test Baseline

Scope: Eurocode artifacts under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/` (EN 1990–1999), the norm app-surface project, and how their oracles and feature tests are wired. No source code was modified.

## Status

**No baseline was measured.** Every `repo` MCP call that needs the build scheduler stalled, so no pass/fail or duration numbers exist yet.

- `mcp__repo__show_projects`: failed after 24m49s waiting on `dependency bindings` lease. Its raw stderr is in `🗑️generated/` (`show-projects.txt`).
- `mcp__repo__leases_status` and `mcp__repo__fleet_status`: no response for 1800 s each, then aborted by the MCP idle timeout.
- A background `repo show projects --language en` (Bash, timeout 1500 s) was still in "waiting for capacity" when this report was written. Its output goes to `🗑️generated/show-projects.txt`.
- The `semio` MCP server reported "failed to connect" at session start. It is not needed for the repo tools above.

### Why the fleet is blocked (from `repo doctor --only=leases,dependencies,processes` and `repo leases status --all`)

- Stale exclusive holder: pid `66263` (`repo dev-site`, teaching course `typescript` vite build, owner session `a2m6c56sjvr`) has held `semio.compiler.cpu/v1/0`, `semio.compiler.cpu/v1/1` and `compiler-state:v1:11` for about 249,862 s (~69 h). Its CPU is 1.1 %. Doctor flags it as the main queue source.
- Stuck vite: pid `1268651` (teaching `vite --mode prototype`) has held its command for about 168,774 s at 0.3 % CPU.
- Waiting on capacity: 8 repo processes, including our fleet's `repo run @semio-tech/norm-en1991-rs:test`, `@semio-tech/norm-en1990-rs:test`, `@semio-s/plugin-norm-app-surface:test`, plus `repo show projects` and `repo dependencies lock --all` runs.
- Memory floor: the capacity queue needs about 6.4 GB per request against a 1.98 GB floor with 5 running jobs ahead.
- Lock-order violations: 20 recorded (out-of-order `compiler-capacity` requested while `javascript-dependency-bindings` is held shared).
- Dependency scopes `.`, `♻️mit-bestand`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech` have `package.json` newer than their published generation.
- Doctor verdict: `degraded: work runs, but something slows it` (0 errors, 10 warnings, 3 notes).

Not done on purpose: killing pid `66263` or `1268651`, and `repo doctor --fix --only=leases`. Both belong to other sessions or shared state. Doctor's own suggested owner action is to ask the owner whether the teaching build is still progressing. The leases-only fix doctor offers is safe in its own description (it removes only rows of dead processes and folders of gone lease directories), but it changes shared state, so it is left for the human owner or the lead to run.

## Project inventory

Names come from `package.json` and `Cargo.toml` manifests. Rust names were also confirmed against the live process table where noted. Targets listed are from the TS `package.json` scripts; the Rust test targets were not listed because `show_targets` never returned.

| Artifact | TypeScript project (manifest) | Rust crate (manifest) | Rust test project (process table) |
|---|---|---|---|
| EN 1990 | `@semio-tech/norm-en1990` | `semio-s-artifact-norm-en1990` | `@semio-tech/norm-en1990-rs` (seen) |
| EN 1991 | `@semio-tech/norm-en1991` | `semio-s-artifact-norm-en1991` | `@semio-tech/norm-en1991-rs` (seen) |
| EN 1992 | `@semio-tech/norm-en1992` | `semio-s-artifact-norm-en1992` | `@semio-tech/norm-en1992-rs` (seen) |
| EN 1993 | `@semio-tech/norm-en1993` | `semio-s-artifact-norm-en1993` | `@semio-tech/norm-en1993-rs` (inferred, not seen) |
| EN 1994 | `@semio-tech/norm-en1994` | `semio-s-artifact-norm-en1994` | `@semio-tech/norm-en1994-rs` (seen) |
| EN 1995 | `@semio-tech/norm-en1995` | `semio-s-artifact-norm-en1995` | `@semio-tech/norm-en1995-rs` (inferred) |
| EN 1996 | `@semio-tech/norm-en1996` | `semio-s-artifact-norm-en1996` | `@semio-tech/norm-en1996-rs` (seen) |
| EN 1997 | `@semio-tech/norm-en1997` | `semio-s-artifact-norm-en1997` | `@semio-tech/norm-en1997-rs` (inferred) |
| EN 1998 | `@semio-tech/norm-en1998` | `semio-s-artifact-norm-en1998` | `@semio-tech/norm-en1998-rs` (inferred) |
| EN 1999 | `@semio-tech/norm-en1999` | `semio-s-artifact-norm-en1999` | `@semio-tech/norm-en1999-rs` (inferred) |

Other norm projects in the same family, for the record: `@semio-tech/norm-din16798`, `norm-din18599`, `norm-din4108`, `norm-iso16757`, `norm-vdi3805`, and the shared `@semio-tech/norm-js`.

Shared surfaces:

- Norm app-surface: `@semio-s/plugin-norm-app-surface` (seen in the process table; Cargo crate `semio-s-plugin-norm-app-surface`, which lives at `📇️registry/🧬️contract/🖥️app-surface/📦️packages/🦀️rust/`).
- Compliance kernel: Cargo crate `semio-s-plugin-norm-compliance` (under `⚖️compliance`).
- Results config: `semio-s-plugin-norm-results-config`.
- Structural scene: `semio_s_plugin_norm_structural_scene`.

The `show_projects` listing was not captured, so project names beyond the manifests above are unverified. Filtering for `norm`, `en199x`, `app-surface` and `compliance` gives the list above.

TS test scripts are of the form `"test": "repo run @semio-tech/norm-<std>:test"`. The `package.json` for EN 1993 is the reference case.

## Oracles and feature tests

### Oracle manifests

- Per artifact: `🗿️artifacts/<std>/…/🔮️oracles/🔣️.json`.
- Shared: `✏️s/🔌️plugins/📕️norm/🔮️oracles/🔣️.json`. Its six entries are the only ones with real third-party libraries: `structuralcodes`, `concreteproperties`, `sectionproperties`, `blue-prints`, `sympy`, `groundhog`. They are tied to concrete, section, steel, symbolic and geotechnical clauses.
- Every per-artifact entry except iso16757 has `package: ""`, `testOnly: true`, and kind `verified-native-second-implementation` (mutations) or `cross-semio-implementation` (evaluate/compliance). Their rationale says no third-party library reads these formats. The oracle is a second Python implementation.

### Python oracle files

- Harness: `🔮️oracles/🏃️harness/🐍️.py`, tests `🔮️oracles/🏃️harness/🧪️tests/🐍️.py`, tolerance `📏️tolerance/🐍️.py`, numerical adapter `🔌️adapters/numerical/🐍️.py`.
- Execution: `🔮️oracles/🏃️execution/🐍️.py`, plus `verbs/`, `carrier/`, `document/`.
- Per artifact: `🔮️oracles/⚖️compliance/🐍️.py` (compliance/evaluate) and `🔮️oracles/🧬️snapshot-schema/🐍️.py` (schema, for some artifacts). The EN 1993 mutation oracle is `🧪️tests/🔩️mutate-en1993-1/🐍️.py`, which the EN 1993 manifest says has 49 kinds and 65 committed vectors.

### Third-party libraries actually imported (grep over norm `.py` files)

- `jsonschema` (7 imports) in the iso16757 compliance oracle and the snapshot-schema oracles.
- `numpy` (2) and `scipy` (1) in the harness numerical adapter and the structural-scene diagram-kit oracle.
- `shapely` (2) in the viewport3d oracle of the app-surface.
- `pytest` only in harness tests.

Declared but not imported in any `.py` file (grep for `structuralcodes`, `concreteproperties`, `sectionproperties`, `sympy`): the four declared libraries are not used by any Python file. `blue-prints` and `groundhog` were not checked.

### Numeric comparison

- Mutation oracles compare by `ordered-json-v1` (exact ordered JSON), not by numeric tolerance.
- Numeric tolerance exists in `🔮️oracles/🏃️harness/📏️tolerance/🐍️.py`, but it was not read in detail. Whether any Eurocode number is checked with a tolerance was not established.

### Feature tests

- 21 `.feature` files in the repo: 15 are `🧪️tests/mutate-<std>-1/🥒️.feature` (one per artifact: the 10 Eurocodes, plus din16798, din18599, din4108, iso16757 and vdi3805), 3 under app-surface `🪟️visual-windows` (checks, properties, viewport3d), 1 under `⚖️compliance/⚡️incremental-evaluation/🧪️tests/🥒️scenarios/`, and 2 under structural-scene.
- No `*-evaluate` feature file exists for any Eurocode artifact. The compliance oracles (`en1993-1-compliance-independent` and similar) therefore have no matching `.feature` file in this repo. This is a coverage gap against the brief's "every clause" goal and AGENTS' "language-agnostic test per feature" rule.
- The EN 1993 mutation feature (`🥒️.feature`) has 3 scenarios, tagged `@capability-en1993-1-mutate @oracle-en1993-1-python-independent @comparison-ordered-json-v1 @mutations-en1993-1-any`.
- AGENTS rule "validate against a third-party library" is not met by any Eurocode artifact's own oracle. EN 1993's manifest says so explicitly. The only third-party-backed oracles are in the shared root manifest for concrete, section, symbolic and geotechnical clauses.

## Recommended edit-to-test loop (once the scheduler is free)

Not yet verified. Commands are the ones the repo tools accept.

1. Check the queue first: `mcp__repo__fleet_status` with `language: "en"`. Stop and wait if the `semio.compiler.cpu` lease is held by a process that is not progressing.
2. Per artifact, TS unit tests (fast path, mirrors the `test` script): `mcp__repo__test` with `project: "@semio-tech/norm-en1993"`, `language: "en"`, `test_level: "fundamental"`. Use `bail: true` to stop on the first failure. Use `skip_cache: true` only when a test must re-run without changes.
3. Rust test for the same artifact: `mcp__repo__test` with `project: "@semio-tech/norm-en1993-rs"` (name inferred from the pattern; verify with `mcp__repo__show_project`).
4. App surface: `mcp__repo__test` with `project: "@semio-s/plugin-norm-app-surface"`.
5. Full set, if time allows: `mcp__repo__run_many` with the list of `@semio-tech/norm-en199x` and `-rs` projects, or `mcp__repo__affected` for changed projects only.
6. Use the `_rs` and TS targets for the EN 1993 artifact first, since that is the one this ticket's brief has in view.

Keep output in `🗑️generated/` under the ticket folder. Do not run `cargo`, `nx` or `bazel` directly.

## Broken or suspicious targets

- No target was verified broken, because no target ran.
- `show_projects`, `show_targets` and `show_project` must be run again before anything else. The 24-minute failure of `show_projects` is a scheduler stall, not a target error.
- Process table shows `repo run @semio-tech/norm-en1991-rs:test --output=static` and `@semio-tech/norm-en1992-rs:test --output=static`. The `--output=static` flag is supported by the repo `run` command.
- The `semio` MCP server failed to connect at session start. This does not affect `mcp__repo__*` tools.

## Next actions for the lead

1. Ask the teaching owner (session `a2m6c56sjvr`, pid `66263`, pid `1268651`) whether their vite builds are still progressing. Either side can end them. Until then the compiler-cpu lease stays exclusive.
2. Decide whether to run `repo doctor --fix --only=leases` (safe per doctor) and `repo dependencies ensure` (rebuilds only moved scopes).
3. Rerun the baseline (EN 1993 TS, EN 1993 Rust, app-surface) once the queue drains.
4. Add an `*-evaluate` feature for each Eurocode artifact, and a third-party oracle for EN 1993 (or record the explicit AGENTS exception in the manifest).
