# Architecture Course Dashboard Forms

This slice adds friendly owner-local controls for three existing targets, preserving their permanent parsers and execution authority. No new command catalog or runtime dependency is introduced. The concurrent editor retirement conflict is outside this slice; README, editor catalogs, old launch tests and the strict absence assertion are not changed here.

## Inspected Parser Contract

- `🎭️e2e/🟦️.ts:endToEndPlan` selects both topologies when neither is named; `dev` and `rehearsal` select their topology. `--serial` and `--keep` are boolean presence flags. `--proctor` consumes exactly one following process word. Other words pass through to Playwright, including `--grep`, `--project`, `--headed`, and `--workers`. The existing release-mode dashboard default must remain.
- `🚀️deploy/🟦️.ts:checkCourseStack` takes tag `latest`, HTTP port `18080`, HTTPS port `18443`, and keep=false by default. Its private flag reader consumes a separate following word; it does not parse `--flag=value`.
- `🚀️deploy/🟦️.ts:stageCourseStackBundle` takes tag `latest` and defaults to the owner package's `dist/proctor`. A selected relative `--out` resolves against the actual process cwd, which is the owner package cwd declared in project metadata.
- The registry schema currently admits `choice`, `flag`, and `text` controls. Port controls therefore use text fields matching the current string parser; this slice does not claim numeric input validation that the registry and parser do not supply.

## Verification Plan

New language-neutral vectors specify default and selected form values, exact argv word boundaries, release effects, e2e semantic plans, and Docker flag results. Ajv validates the fixture schema and the complete owner declaration against the current registry schema. Direct comparisons validate emitted argv/env; the real `endToEndPlan` checks e2e semantics and Node's independent `parseArgs` checks Docker argv. Generic Arguments remain available for additional Playwright options.

The new test include is added surgically to the existing course configuration, retaining the independent launch include and all current fields. No target workload, Docker operation, browser journey or native build is executed by these tests.

## Authored Changes

- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`: added localized topology, serial, retained artifacts, existing proctor, grep, project, headed and worker controls beside the existing release execution-mode parameter; added tag/output controls to stack bundle and tag/HTTP/HTTPS/keep controls to stack check. Defaults match the inspected parsers. Existing script/cwd/executor, forwardAllArgs, preflight, release effects, ready and compound fields are retained.
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts`: registered the new focused parameter test, preserving other current includes.
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎛️dashboard/🎚️parameters/🟦️.ts`: added fixture/owner schema checks, localization checks, exact argv/env projection, the real e2e parser comparison, independent Docker argv parsing and owner-relative bundle-default resolution checks.
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🎛️dashboard/🎚️parameters/🔣️.json`: seven language-neutral default/selected-value vectors, including paths with spaces and generic Playwright argument forwarding.
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🎛️dashboard/🎚️parameters/🧬️schema/🔣️.json`: schema for those vectors.
- This retained report.

## Executed Checks

The existing installed Vitest dependency and course configuration were used, with the script owner's existing native config-loader option:

```powershell
node node_modules/vitest/vitest.mjs run --configLoader native --config '🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts' '🧪️tests/🎛️dashboard/🎚️parameters/🟦️.ts'
```

Red phase before owner metadata changed: exit 1, eight tests executed, seven failed and one passed. The failures identified the absent parameter IDs and resulting argv; the pre-existing e2e default semantics passed. Both Ajv schema checks initialized successfully. Output: `🗑️generated/course-forms-red.log`.

Green phase after adding metadata: exit 0, one file and all eight tests passed (10.18 s runner duration; 164 ms tests, with real parser dependency imports). Output: `🗑️generated/course-forms-green.log`.

Final run after adding explicit owner-relative bundle-directory assertions and a precise independent-parser options type: exit 0, one file and all eight tests passed (13.12 s runner duration; 231 ms tests). Output: `🗑️generated/course-forms-final.log`.

Native compiler admission remains blocked separately. This evidence validates authored metadata, argument projection and parser semantics; it does not establish interactive form rendering or actual target workload execution.

## Forwarding Boundary Design

The follow-up audit found a concrete collision: the old `endToEndPlan` searched every word for topology and control tokens, including Playwright option values. For example, rehearsal plus grep `dev` selected both topologies and dropped the grep value.

Before implementation, the chosen replacement grammar is a strict course-owned prefix (`--topology both|dev|rehearsal`, `--serial`, `--keep`, `--proctor PATH`), followed by an explicit `--` forwarding boundary. Every word after that boundary belongs to Playwright and is preserved exactly, including topology words, control-looking words and another `--`. Unknown words before the boundary are errors. Bare topology words are removed from the grammar rather than kept as a legacy adapter. No table guessing Playwright option arity is introduced.

The existing dashboard parameter schema can express this without a synthetic boundary form field: put topology after the other course-owned controls; each topology choice emits its `--topology VALUE --` sequence. Playwright controls and the generic Arguments tail follow. An invocation with no forwarded tail may omit the separator; empty argv retains both-topology defaults. Duplicate owned options are rejected. A proctor value that is absent, empty or starts with `--` is rejected; a file with such a name can be expressed with an explicit relative path such as `./--keep`.

The parser's original-mode, native authority, proctor ownership and cancellation behavior are unchanged. Internal/CI callers and neutral vectors adopt the same boundary. The parent subsequently authorized surgical grammar-only edits in the README; its competing IDE paragraph remains intact, and editor retirement is still blocked by that concurrent policy conflict.

## Forwarding Boundary Implementation and Checks

The current parser now consumes only its closed prefix, stopping at the first `--` and copying the complete suffix unchanged. Its retained source has no Playwright option arity table. The proctor field help now explicitly says absolute or workspace-relative executable path, matching the existing proctor subprocess cwd; bundle output help remains package-relative.

Updated callers and assets in this follow-up:

- `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🟦️.ts`: parser and grammar docstring.
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`: owned-control order, topology prefix/boundary effects, proctor path help.
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📜️script.ts`: grammar docstrings and deploy-check's explicit serial/boundary invocation; the parent's independent TestScript work is retained.
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🎛️dashboard/🎚️parameters/🔣️.json` and its `🧬️schema/🔣️.json`: collision vectors and rejected-prefix grammar cases.
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎛️dashboard/🎚️parameters/🟦️.ts`: real parser checks, independent Node prefix oracle, and actual repo selector → Script entrypoint/router argument handoff.
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧱️local-stack/🟦️.ts`: current parser callers and precise test title.
- `.github/workflows/architecture-course.yml` and `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚀️readiness/🔣️.json`: rehearsal invocation with both build-runner and course forwarding boundaries.
- `🎓️teaching/🏛️architecture/📚️course/README.md`: grammar wording/table only; no editor retirement change.
- This report.

The schema-first red run executed 24 tests before parser/metadata changes: 19 failed and five passed. It reproduced the reserved-word collisions and invalid opaque-tail behavior (`🗑️generated/course-boundary-red.log`). Three additional rejected-prefix vectors were added before the green run.

The focused green run executed all 28 tests successfully: exit 0, one file passed, 14.36 s runner duration, 167 ms tests (`🗑️generated/course-boundary-green.log`). This includes actual `selectBuildTasksV1` stripping only the outer build-runner separator, followed by actual `runScriptMain`/`ScriptRouter` retaining the inner separator through dispatch to an observation command and the real parser. Existing Script progress output showed loading/running/complete for each dispatched vector. The observation command performs no target workload.

Selected existing caller regressions also passed: two files, four tests passed, 61 unrelated tests skipped, exit 0, 11.32 s runner duration (`🗑️generated/course-boundary-callers.log`). The selection covered current topology defaults, explicit Playwright forwarding, workflow command equality against the readiness fixture, and readiness source/dependency ordering. No compiler, Docker stack or browser journey ran. A bounded active TS/JSON/workflow scan found no remaining old bare-topology API callsites.
