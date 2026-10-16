# Nx Build Mode Strict Contracts Independent Audit

No actionable finding in the scoped 32-diagnostic repair. Read-only review of the ten listed Source files, three actual MJS bodies, ten retained logs, and original nx-source-types3 BOTH diagnostic logs. No implementation edits or execution of tests, compiler, cache, Native, network, browser, or physical effects occurred in this audit.

## Snapshot Integrity

All 23 listed SHA256 identities matched at first read and final recheck (10 Source + 3 MJS + 10 logs). No snapshot mismatch or shared-wrapper drift was observed in these specific files. Active CPU helper/Core/neutral-launcher and Fixed work elsewhere is outside this leaf audit; this report does not attribute transitive program drift to this repair.

## Source Findings

- `build-mode/🟦️.ts:111-113,146-162,167-168` keeps actual inference methods behind `testMethodV1`, whose `values/🟦️.ts:17` requires a non-proxy object and an actual own DATA function, calls with the original receiver, and returns unknown. There is no catch, output cast, manufactured invocation, control, or registration. Thrown undefined and object identity remain intact.
- `values/🟦️.ts:19-20` checks actual array shape, rejects sparse/accessor entries and proxy arrays, checks every own element as string/object, and returns the original array. Same-graph identity at build-mode line 147 therefore retains its original meaning. Nested object fields remain unknown; the external-dependency selection is compared in full against the independent expected list at line 162.
- Build-mode line 148 checks the actual commandDependencies Map before reading size. Lines 152 and 157 check model/externalNodes ancestors and resolveImport result string-or-undefined before membership. Lines 184-191 and 194 validate projection/projects/nodes/owner ancestors, dependency array/rows, required Go roots, and actual build-input arrays. Line 206 requires the manifest input array before length. Lines 170 and 202 check actual extracted lines before parsing/slicing. Required build mapping at line 123 replaces an optional indexed member with the actually authored required property.
- `build-mode/📥️contracts/🟦️.ts:33` uses an authored two-string tuple for environment-source pairs; it does not assert an unchecked external array to the tuple type. Existing model ports and all oracle assertions remain; the full model is not among the executed Source tests.
- Three adjacent declarations match every actual runtime export and function parameter count, with synchronous versus Promise returns consistent with the inspected bodies. Cargo closure identities, optional source/checksum, dependency strings and returned version agree with actual lock projection/schema. The test plugin and main projection expose dynamic results as unknown; no external package type or ambient module/any stub is introduced. Export/arity checks alone are not a proof of complete parameter semantics; the bodies were inspected independently.
- The Source test at `values/🧪️tests/🟦️.ts:13-16` adds complete export incidence and arity comparisons, independent SQLite export incidence, Ajv/SQLite DATA checks, accessor/inherited/proxy refusal, receiver and primary-failure preservation, and Cargo schema/recursive closure comparison. Its source-string law does not itself prove runtime build-mode execution.
- Ticket `typescript-artifact/📜️script.ts:58` retains the actual hash test/digest/lock/Graph-input/build-mode/runtime-authorizer/self roots and strict/noUncheckedIndexedAccess flags, adding the new pure test root. No narrower successor cohort substitutes for the original diagnostic roots.

## Evidence and Limits

Both final Source logs show isolated file 0: 4 passes/67 expectations and file 1: 3 passes/107 expectations; totals are 7 tests/174 expectations per mode, zero failures. All fourteen separately failed Source child observations are retained. Both final types3 logs have no diagnostics and contain complete metrics (prototype 845 files/7.60s; release 849 files/11.56s), consistent with the recorded successful strict exits. The logs themselves do not print an independent exit-code field, so the recorded exit-zero claim additionally depends on the producing runner/report; no rerun was performed.

The retained earlier logs establish the original 32 diagnostics in each mode. This review found their corresponding actual data/method/member repairs rather than suppression, root deletion, or weakened assertion. The file still contains unrelated preexisting JSON.parse/createRequire dynamic oracle values and existing explicit any annotations; this is a scoped repair review, not a claim that the whole test file is free of any.

No full testNxBuildModes, existing modeled Native reader/worker effects, managed Graph hash-to-completion, coupled eviction, provider/runtime/cache/compiler/Native/store qualification follows from these results. No Native authority adopter or root/control facade was introduced in this scoped repair.
