# H26: "Bounded dense Source owner rows required (reference b8a87efb)"

Root cause (outage, landed 02:3x CET): `ensureBuildScopesV1` (H23, "on-demand scopes on the Nx path", `🏗️build/🚀️bootstrap/🟦️.ts`) passed the root `package.json` `workspaces` (member packages) to `javascriptScopeOwnersV1` as the declared dependency scopes. Every project root of a `run-many` closure therefore became its own owner (69 > `maximumScopes` 64) and `prepareJavascriptDependencies("sync", {kind:"scopes", owners})` threw in `captureJavascriptDependencyOwnerRowsV1` (storage `🟦️.ts`) as an Internal error. It surfaced in the workspace stage of every worker after the dependency receipt went stale (H8's per-scope receipts only gate the stale path); workers whose scope receipt was current returned early, which is why some stayed Ready.
Evidence: diagnostic message (landed first) printed `length 69, maximum 64, preview [".","🧰️framework/📦️packages/🟦️typescript",...]` on m01/m06/m12/m15.

Fixes:
1. `javascriptDeclaredScopesV1(root)`: root plus top-level folders whose package.json declares `semio.workspace` (6 on this checkout); `ensureBuildScopesV1` uses it. Test: `🏗️build/🚀️bootstrap/🧪️tests/🎯️closure` (6/6, new real-checkout test: declared scopes are owners, members map onto them, <= maximumScopes).
2. The throw in `captureJavascriptDependencyOwnerRowsV1` now names length, maximum, extra keys, preview of the first rows and caller frames (no longer opaque).
3. `🗂️workspaces/🟦️bun/🟨️.cjs` scopesPlan refuses more than `maximumScopes` declared scopes with a roster-class edit naming count and directories; schema `PhysicalLimits.maximumScopes` 64; test `🗂️workspaces/🟦️bun/🧪️tests/🔢️scope-capacity` (69 refused, 64 admitted; with closure test 6/6 pass).

Coordinator whole-graph read (H9): not affected (no scoped sync there; walk reports 6 scopes, 319 packages).
Unverified: workers re-running the stage with the fix (m01 in backoff, m06/m12 running); a 1-minute parse error window (my docstring contained a comment terminator) hit `bootstrap/🟦️.ts` before I fixed it. Remaining: the error is still an Internal error class for other row overflows (message is now explicit).
