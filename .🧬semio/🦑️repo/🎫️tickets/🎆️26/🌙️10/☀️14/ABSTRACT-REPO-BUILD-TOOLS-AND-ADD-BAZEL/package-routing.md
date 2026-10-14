# Package Working Directory Routing

The real repo-lib package command `bun run test-build-surfaces --help` initially failed with `Script not found "repo"`: Bun resolves the root `repo` package script from the current package, and packages do not declare that script. No target ran in this red check (`🗑️generated/package-repo-wrapper-red.txt`).

The bounded correction changes 675 exact script values in 237 non-root package manifests to `bun --cwd "<portable relative repository root>" repo <unchanged arguments>`. Root package scripts remain unchanged. Each original and proposed command was parsed by the owned literal tokenizer and the existing independent string-argv implementation; every argument after `repo` is identical, and Node path resolution verifies the selected cwd. The edit verified every current script against its observed original value and checked that no other parsed manifest field changed. A post-edit oracle checks all 675 exact resulting values (`package-routing-before.txt`, `package-routing-write.txt`, `package-routing-after.txt`; detailed transient receipt `package-routing-before.json`, all under generated).

The one proven programmatic cwd defect was DslScript: its router root is the testing domain. Its existing `runCmd` now uses `this.repoRoot`, preserving its command arguments, environment, wire, stdio and orchestration budget. The bounded independent caller census found the other actual programmatic neutral invocations already use their repository root; project-local `bun ./📜️script.ts` execution remains unchanged.

The permanent closed build-surfaces schema and language-neutral fixture contain POSIX/Windows path contexts, spaces, Unicode, explicit tool selection and literal argument boundaries. The test compares the owned tokenizer with string-argv, resolves both platform path semantics, validates all tracked package wrappers, and uses the existing TypeScript AST oracle to check the real DslScript cwd. TDD red failed on a real tracked package's `bun repo` prefix; green checked 671 tracked wrappers (four additional wrappers are current untracked authored manifests covered by the 675-value exact source oracle).

Actual Bun 1.3.14 workspace bin experiments are retained in `bin-probe/📜️script.ts` and its language-neutral descriptor. An outside-package bin pointing to the existing root entry was not installed; an inside-package positive control was installed and worked, and package-only projected staging also lacked the outside entry. The explicit cwd solution requires no bin, shim, dependency or new production script. The `--routing` mode independently runs root package scripts from nested space/Unicode package paths and verifies literal tool/data arguments, forwarded caller arguments, root cwd and byte-identical original invocation wire under the original 120-second control. Its two real executions passed. Bun's own package-shell handling drops an authored empty `""` argument; that third-party observation is separate from this cwd change, whose existing wrapper suffixes were preserved. The permanent independent parser vector retains the empty argument case.

Validation:

- Focused package law: 1 test, 3383 assertions, green in 0.399 seconds (`package-routing-green.txt`).
- Complete actual build-surfaces suite: 11 tests, 31,418 assertions, green in 0.584 seconds (`package-routing-surfaces.txt`).
- Actual declared repo-lib wrapper from its package cwd: `bun run test-build-surfaces --help`, exit 0, real neutral CLI help, 0.151 seconds (`package-repo-wrapper-green.txt`). This proves package entry routing without executing the target or loading a graph.
- Actual independent Bun fixture: both nested package paths passed (`package-routing-bun.txt`, `package-routing-bun.json`).

No current selected/canonical graph or backend runtime acceptance is claimed by these checks. Final genuine selected runtime, nested public runtime and all-pin planner/Stdio checks remain held for their explicit publication releases.

Independent read-only review separately parsed all 675 resulting values with Python shlex, compared their argument suffixes with the pre-edit receipt, and resolved their relative cwd to the repository root: zero mismatches. The current DslScript field and permanent tracked/fixture oracle scope were also reviewed clean.

## Current default Nx regression follow-up

The actual selected20 default Nx runtime reached all11 tests and exposed three newly authored non-root bare wrappers after the earlier correction. A fresh focused run reproduced the current package law RED. Only three exact script values were corrected: process TypeScript package test, CAD aec-building TypeScript package test, and CAD Rust package verify-cad-document-contract. Each now uses its package-relative explicit repository --cwd. Independent declaring-owned string-argv parsing verifies every original argument suffix byte value, and Node resolves each authored cwd to the actual repository root. All other manifest bytes and fields were preserved; the four valid root package bare wrappers remain unchanged.

The existing permanent package routing law then passed1 test/3,403 assertions in873 milliseconds, checking675 tracked wrapper values. Exact RED/GREEN logs are generated/package-routing-current-red.txt and package-routing-current-green.txt; generated/package-routing-current-corrections.json retains the three exact before/after values. Core received source-settled immediately for actual default Nx retry. This focused proof does not itself claim the selected backend runtime is green.
