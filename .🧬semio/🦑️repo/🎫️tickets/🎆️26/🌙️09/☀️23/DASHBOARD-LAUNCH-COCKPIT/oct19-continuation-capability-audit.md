# Continuation Capability Audit

This bounded read-only review examines current permanent root tools, the developer-tools debugger declaration, and the architecture-course script router, e2e parser, deployment argument handling and project target declarations. It excludes historical ticket compatibility, native installation work and another catalog-writer trace. No source, process, Git state or storage was changed. No commands that build, publish, deploy or run workloads were executed.

## Concrete Friendly-Control Gaps

### Course End-to-End Topology and Debug Controls

Current permanent `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🟦️.ts:18` documents the supported dev/rehearsal topologies, serial scheduling, retained run directory, selected proctor executable and Playwright arguments. Its real parser at lines86–92 extracts `--proctor`, topology words, `--serial` and `--keep`; other words are forwarded to Playwright. A developer narrowing a failure to the development topology or keeping artifacts needs these current options.

The corresponding package project's `test-e2e.metadata.semio.dashboard.parameters` currently exposes only the release-default execution-mode choice. It provides no localized topology, serial, keep, proctor or test-filter control. `forwardAllArgs:true` means the user can express those modes through the generic Arguments field after independently finding the CLI vocabulary. This is a concrete discoverability/form gap in an existing permanent operation, not an absent executor or a historical helper compatibility requirement.

Recommended owner-local form: topology choice (both/dev/rehearsal), serial and retain-artifacts flags, existing-proctor path, and the commonly supported Playwright filters. Keep unknown admitted Playwright options in Arguments. Shared language-neutral vectors should project exact argv and independently compare the current parser's semantic plan.

### Course Docker Bundle and Probe Options

Current permanent `🚀️deploy/🟦️.ts:1062` declares `docker-stack-bundle [--tag <tag>] [--out <directory>]`; lines1070–1072 actually select the image tag and resolve the output directory. The matching target has no dashboard parameter declaration. A developer staging a named release into a selected handoff directory must know these raw flags and that a relative output path resolves from the owner package cwd. Merely exposing the target does not communicate those choices.

The same file at lines853 and866–869 declares/parses docker-stack-check's image tag, HTTP port, HTTPS port and retained-stack flag. The corresponding target also has no parameters. A developer avoiding an occupied host port or retaining a failed probe must discover `--http-port`, `--https-port` and `--keep` outside the dashboard form.

Both targets retain `forwardAllArgs:true`; therefore these operations are technically expressible, and this audit does not claim functional loss. Appropriate owner metadata would expose localized tag, workspace output directory, bounded numeric ports and retain controls with exact argument-vector tests. This extends current permanent target forms beyond the already known generic source-picker limitation.

## Checks That Did Not Establish a New Defect

Root build-help, build-task and build-affected expose their backend/mode argv through current neutral tool declarations. Developer debugging declares profile, program, cwd and literal JSON arguments with actionable descriptions. The course router retains permanent targets for the inspected deployment and e2e verbs; no missing current target was found in this bounded set.

The e2e target's release default is not itself an incompatibility with a dev-only topology: the current course mode owner rejects prototype mode only for release intent and permits release for dev intent. No unsupported claim of a mode mismatch is made.

No new confirmed argv/cwd execution bug was established. This audit's findings concern discoverability and typed task-specific forms, with technical expressibility through Arguments retained. Native runtime acceptance remains outside this read-only evidence boundary.

## Focused Review of New Owner Forms

The current owner now supplies the missing e2e controls, bundle tag/output and stack tag/ports/retention. All parameter titles have English/German keys, supplied descriptions are bilingual, topology defaults to both with no argv, flags default false, and optional proctor/filter/project/workers have no default. The e2e release-default effect is preserved. Bundle defaults project `--tag latest --out dist/proctor`; the owner cwd makes that the same package-local directory as the permanent function's no-flag default. Stack defaults project `--tag latest --http-port 18080 --https-port 18443`; keep adds only `--keep`. These two target forms add no environment effects. Readiness and compound declarations remain outside these added parameters and were not rewritten.

The new neutral cases/schema plus owner-form tests inspect declaration IDs/localization, compare exact argv/environment, run the actual e2e parser, and independently parse Docker option vectors with Node parseArgs. This auditor read those tests but did not duplicate the executor's reported eight passing cases. The tests currently project their own metadata loop rather than execute the installed native form. The native resolver rejects unknown choices, invalid boolean flag values and NUL input; text-valued port/worker fields have no numeric/range constraint, so they are not bounded numeric form validation. Their actual downstream parsers still own final acceptance. An omitted optional field has no argv; explicit empty-string native form behavior is not established by these vectors.

Proctor selection skips building the checkout proctor and passes the selected string to `launchProctor(..., run.repoRoot, ["serve"], ...)` at e2e233, unlike the package cwd used for the owner script. The form currently calls this an optional executable path without specifying that execution cwd; a workspace-relative path hint would make that behavior clearer. There is no file-existence assertion in the metadata itself.

## Confirmed E2E Parser Collision

The new grep/project controls expose a real current parser defect. `endToEndPlan` at e2e86–92 detects topology names anywhere in argv and removes every matching token, including values belonging to Playwright options. Exact read-only reproduction extracted and Bun-transpiled the current function, retaining its actual body and current topology constants; `[DEBUG]` output confirmed:

| Selected form argv | Actual current result |
| --- | --- |
| `rehearsal --grep dev` | topologies become dev+rehearsal; Playwright gets dangling `--grep` |
| `dev --project rehearsal` | topologies become dev+rehearsal; Playwright gets dangling `--project` |

This changes both topology intent and filter semantics. The current positive vectors use a noncolliding grep and project desktop, so do not catch it. The parent received the concrete failure immediately. The permanent parser needs option-value-aware topology handling and language-neutral collision vectors, including flags whose values equal dev/rehearsal. No source fix was made by this audit. The native/runtime proof boundary and known concurrent editor-writer conflict remain separate.

## Parser Repair Handoff

The course owner received exact collisions and boundary recommendations. The intended clean grammar is a closed prefix (`--topology both|dev|rehearsal`, `--serial`, `--keep`, `--proctor PATH`) followed by an explicit `--` whenever a forwarded tail exists. Empty argv retains both defaults; bare prefix topology names are rejected. The suffix is opaque, including unknown/future options, owned-looking words, dev/rehearsal values and later `--` tokens. Prefix duplicates, unknown/bare words, invalid/missing topology, and missing/option-looking proctor values are rejected. Such a filename can be expressed with a relative ./ prefix. Node parseArgs is an independent oracle for the closed prefix; it must not interpret or enumerate the Playwright suffix.

The bounded production caller inventory found generic test-e2e forwarding and package script127's deploy-check serial call. Existing local-stack tests194–209 also exercise the old bare topology and mixed-word syntax. These exact current callers/tests were sent to the course owner for conversion. The dashboard topology control is to emit the boundary after all owned controls and before Playwright fields. This handoff is not itself proof that the source has been repaired; source review follows the owner's completion signal.

## Final Bounded Source Review After Repair

The settled source now implements the closed prefix directly: only topology/serial/keep/proctor are recognized before the first `--`; duplicates and unknown prefix words throw; topology admits both/dev/rehearsal; missing, empty or double-dash-leading proctor values throw. At the first separator, the function slices the entire remaining vector unchanged and stops interpreting it. Empty argv retains both defaults. This fixes the confirmed collision without guessing a Playwright option vocabulary. The proctor field now documents absolute or workspace-relative executable paths.

Current metadata orders serial/keep/proctor before topology; every topology choice emits `--topology <choice> --`, and grep/project/headed/workers plus extra Arguments follow. Release mode remains an environment effect, not a tail word. Neutral cases include grep dev, project rehearsal, owned-looking suffix values and a later literal separator, with negative duplicate/unknown/missing-value cases. The independent Node parseArgs oracle receives only the prefix. The dedicated test additionally runs real selectBuildTasksV1 and ScriptRouter/runScriptMain plumbing to prove the inner separator survives argument forwarding; it does not claim native Original admission.

Read-only caller review confirmed deploy-check now passes `--serial --`, the CI workflow selects `--topology rehearsal --`, readiness fixture matches that CI invocation, and local-stack parser expectations use the new grammar. Generic test-e2e continues forwarding its complete segment vector. No additional concrete defect was found in this bounded final review. The course owner reports 28 passing dedicated tests; this auditor did not rerun that suite. Earlier text/numeric-validation and installed-native proof boundaries remain as stated, and the concurrent IDE writer conflict is separate from this parser/form repair.
