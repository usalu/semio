# Result F18 (Original-required cluster of red C tests)

## Finding: there is no single root cause
Re-measured today, the "Original ... required" cluster is mostly gone or splits into independent causes. Measured first errors (one file at a time, `repo test-file`, load gate 38):
- Suite default envelope is `run @semio-tech/repo-lib:test-<suite>` (`🪪️native-birth`), but laws of the publication family need the `tools setup all` Original AND a private repository (the live store is fenced by `🚧️fence`). The harness had no way to give a private invocation another build command: `privateRepositoryInvocationV1` hard-coded `show projects`. Fixed (seam below).
- Authored native-owner-command capabilities drifted behind the caller schema (`codeRoot` required): owner-cmd-policy, test-body (E57 fixed only native-caller). Fixed in two files; remaining literal authors: `🧪️tests/🧫️fixtures/📬️test-body`, `📣️progress` fixture, `📥️invocation/🧪️tests`, `🏃️execution` tests (cargo, build), `🛤️runtime-and-path-routes`, `📥️original-control` (not checked, grep list).
- Several files of the F7 red list are already green (owner 11/0, layout, recovery, child) or fail for other reasons (value drift: `🔒️bounded` 16 GiB vs 48 GiB; `📐️phases` stage order; prepared-worker 1 ms deadline rounding between receipt and wire).
- Not tests at all: the `📜️script.ts` rows (asset-publication, graph-purpose, native-dependencies purpose, nx purpose, cargo child, `♻️roles.ts`) are child programs; real discovery (`discoverProcessTestFilesV1`: imports the test API) never runs them, only `repo test-file` does and reports a false red ("Closed fresh standalone declared asset entry required"). Proposed fix, not landed (needs the Python routing oracle + `file.helper` text/test changes): `routeTestFilesV1` refuses a file under a `🧪️tests` dir that discovery would not count and no suite names.
- Extracted-function harness drift (E47) is the largest remaining class: native-build 2 left, test-body, budget parent.

## Landed (see `T/landings.md` "Wave 1125 F18")
1. `registerIsolatedBuildTestV1` (process testing invocation) stretches policy body and file scope with `stretchedBudgetMillisecondsV1`; `📏️capacity` test control spans (the 20 s phase control was the ~19 s "physical body cutoff") use it too.
2. `PrivateRepositoryWorkspaceV1.invoke(body,{arguments})`: build command of the private caller is selectable (default `show projects`).
3. native-build law uses `repository.invoke(...,{arguments:["tools","setup","all"]})`.
4. `codeRoot` added to authored capabilities (owner-cmd-policy, test-body).

## Before -> after (pass/fail, one file at a time)
- `📏️capacity` 11/2 -> 13/0
- `📋️owner-cmd-policy` 5/6 -> 11/0
- `🏗️native-build` 3/4 -> 4/3 (remaining: source-extraction oracle, `mac:absent`)
- `📦️owner` 10/2 -> 11/0 (peer or load, not mine)
- `📬️test-body` 0/9 -> 0/9 (codeRoot gone; next: extracted-function harness, "Live Original process issuance required")
- unchanged: `🔒️bounded` 6/1, `📐️phases` 8/1, `prepared-worker` 1/2, `🏪️store` not rerun.
Not run: the process testing invocation test file and the literal-deadline ratchet (load 44-65 at the end); the stretch edit does not change any literal count.
