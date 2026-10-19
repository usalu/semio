# Native System FFI Typing Repair

The current-source TypeScript program reported four TS2345 errors at the actual Windows privilege-check implementation. `ReturnType<typeof dlopen>` erased Bun FFI's generic symbol declarations, leaving the four security API call parameter tuples as `never`.

The repair introduces a typed local `openSecurity` factory around the unchanged actual `advapi32.dll` symbol declaration and types the owned handle as `ReturnType<typeof openSecurity>`. The factory is called at the existing location inside the same `try`. No `any`, unsafe FFI function cast, host-policy change or authority bypass was added. The exact reversible text transform reproduced the full before-image byte-for-byte when undone, and a per-file before-image guard protected concurrent work.

## Verification

- RED: the root agent's actual current-source TypeScript run reported four TS2345 diagnostics for `OpenProcessToken`, `LookupPrivilegeValueW`, `GetTokenInformation`, and `AdjustTokenPrivileges`; recorded in temporary `current-source-typecheck-final.log`.
- GREEN: after the typing repair, the root agent reran the full current-source TypeScript program using the actual authored package configuration, controlled public workspace exports and existing declarations. It exited **0**, reporting **zero diagnostics over 3,174 reads**, in temporary `current-source-typecheck-complete.log`. This is the current-source no-emit diagnostic, not a canonical managed build pass.
- The existing bounded source regression `actual fixed opener refuses unavailable host privilege before first lease or cache mutation` passed: **1 pass, 1 filtered out, 0 fail, 4 assertions**. It verifies refusal before a lease or cache effect; the test's real host FFI case was filtered out.
- The current-source neutral diagnostic in `native-typing/📜️script.ts` evaluated the two actual authored host functions with modeled FFI only. All **14** existing language-neutral privilege inventory/check cases matched the normative fixture, Ajv schema and independent in-memory SQLite oracle, including required token/library closure counts. Its console explicitly logs modeled FFI and no native privilege/cache effects. The first diagnostic attempt exposed a diagnostic-only BigInt equality bug; replacing JSON-stringified equality with Node's `deepStrictEqual` fixed the harness, then all 14 cases passed.

No actual native privilege API, storage allocation, compiler build or host-policy modification was performed for this typing repair. The final canonical admission command remains a real pre-allocation refusal as documented in `final-native-admission.md`.

## Changed Files

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🗂️system/🟦️.ts`
- Ticket input `native-typing/📜️script.ts` (retain as requested input).
- This report.

Temporary test output lives under this ticket's `🗑️generated` directory and belongs to final ticket cleanup.
