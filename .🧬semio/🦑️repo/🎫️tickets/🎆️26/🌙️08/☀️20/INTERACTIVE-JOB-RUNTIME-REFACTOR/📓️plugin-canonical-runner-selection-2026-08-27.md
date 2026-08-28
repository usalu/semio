# Plugin Canonical Test Runner Selection

The existing Plugin test target now delegates normal tests to the existing central level resolver and budgeted nextest runner. Explicit --no-run before the libtest separator remains Cargo compile inventory. No central runner, timeout, thread, stack, coverage default, launch target, or production synchronization was changed. Existing target registration remains valid.

Schema-first six neutral argument cases cover exact filters, exhaustive/no-fail-fast expression, plain and level-qualified no-run, and separator ownership. Actual selector results are checked against fixed expected argv and Node parseArgs' independent no-run/separator result; Ajv rejects modified fixture modes.

R1 was only an Nx nested-quote invocation failure, not test execution. R2 reached the intended missing-selector ReferenceError after schema validation. R3 executed all six cases successfully. Full outputs follow.

## R1

```sh
set -o pipefail
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false SEMIO_COVERAGE=0 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- bun -e 'import {pluginTestRunnerSelfTests} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts"; console.log("[DEBUG] plugin runner cases="+pluginTestRunnerSelfTests());' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-runner-red-r1-2026-08-27.txt'
```

```text
1 | mport {pluginTestRunnerSelfTests} from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️mod
                                            ^
error: Expected string but found "/"
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[eval]:1:41

1 | port {pluginTestRunnerSelfTests} from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modu
                                             ^
error: Expected ";" but found "Users"
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[eval]:1:42

1 | TestRunnerSelfTests} from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️p
                                                         ^
error: Invalid flag "D" in regular expression
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[eval]:1:54

1 | estRunnerSelfTests} from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️pl
                                                          ^
error: Invalid flag "o" in regular expression
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[eval]:1:55

1 | stRunnerSelfTests} from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plu
                                                           ^
error: Invalid flag "c" in regular expression
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[eval]:1:56

1 | unnerSelfTests} from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin
                                                              ^
error: Invalid flag "e" in regular expression
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[eval]:1:59

1 | nnerSelfTests} from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/
                                                               ^
error: Invalid flag "n" in regular expression
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[eval]:1:60

1 | nerSelfTests} from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/�
                                                                ^
error: Invalid flag "t" in regular expression
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[eval]:1:61

1 | ests} from /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️pa
                                                                        ^
error: Unexpected 🧰
    at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[eval]:1:69

Bun v1.3.14 (macOS arm64)
Error: Command failed: "bun" "-e" "import {pluginTestRunnerSelfTests} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts"; console.log("[DEBUG] plugin runner cases="+pluginTestRunnerSelfTests());"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: 1,
  signal: null,
  output: [ null, null, null ],
  pid: 81033,
  stdout: null,
  stderr: null
}

```

## R2

```sh
set -o pipefail
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false SEMIO_COVERAGE=0 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- bun -e 'import {pluginTestRunnerSelfTests} from '\''/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts'\''; console.log('\''[DEBUG] plugin runner cases='\''+pluginTestRunnerSelfTests());' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-runner-red-r2-2026-08-27.txt'
```

```text
14 |   const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
15 |   assert(validate(fixture), JSON.stringify(validate.errors));
16 |   const level = process.env.SEMIO_TEST_LEVEL, coverage = process.env.SEMIO_COVERAGE;
17 |   try {
18 |     for (const row of fixture.cases) {
19 |       const selected = pluginTestInvocation(row.args);
                            ^
ReferenceError: pluginTestInvocation is not defined
      at pluginTestRunnerSelfTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts:19:24)
      at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/[eval]:1:199

Bun v1.3.14 (macOS arm64)
Error: Command failed: "bun" "-e" "import {pluginTestRunnerSelfTests} from '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts'; console.log('[DEBUG] plugin runner cases='+pluginTestRunnerSelfTests());"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: 1,
  signal: null,
  output: [ null, null, null ],
  pid: 81370,
  stdout: null,
  stderr: null
}

```

## R3

```sh
set -o pipefail
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false SEMIO_COVERAGE=0 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- bun -e 'import {pluginTestRunnerSelfTests} from '\''/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts'\''; console.log('\''[DEBUG] plugin runner cases='\''+pluginTestRunnerSelfTests());' 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-runner-green-r3-2026-08-27.txt'
```

```text
[DEBUG] plugin runner cases=6

```

