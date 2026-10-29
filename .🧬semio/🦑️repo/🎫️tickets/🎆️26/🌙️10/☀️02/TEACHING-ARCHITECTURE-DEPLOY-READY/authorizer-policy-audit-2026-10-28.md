# Authorizer Entry Relative-Read Audit — 2026-10-28

Read-only targeted audit requested after normal qualified build80251 failed at an eager discovery policy read. Backend owns that exact source fix and its regression. This audit made no production edits, build, import-time execution, runtime/service probe or compiler invocation.

The retained authorizer-policy-scan.mjs walked static runtime-value import/export declarations from the actual compiler entry, authorizers/📜️script.ts. It inspected 833 files / 7,840,596 bytes with a visited set and hard source-count/byte bounds; no static bare imports were unresolved. Type-only imports were skipped. Dynamic/computed imports and runtime require call graphs were not traversed, so this is not a proof about every lazy runtime path. The read-only scan handle92029 exited0. Machine output remains under generated/authorizer-policy-scan.json until cleanup.

## Conclusion

No second eager unprotected module-relative policy read was found in the inspected current Source after the backend's discovery policy edit. The compiler script's actual runScriptProgram path reads its admitted snapshot and invokes compileNxAuthorizerSnapshotV1; the snapshot/authorizer schemas and compiler policy are static JSON imports. That statement is a source finding, not an execution pass of the relocated compiler.

One concrete analogous pattern is deferred: authorizers/🧊️memo/🟦️.ts line40 reads six sibling source/schema files with readFileSync(new URL(name,import.meta.url)) inside nxAuthorizerMemoImplementationV1. It is reached by memo validation/capture implementation identity, not top-level initialization. The inspected compiler action does not call it; normal source capture invokes that path in its original source context. It would need physical-origin/captured-byte handling if subsequently invoked from a relocated bundle, but no present compiler-startup failure is established here. This was reported to backend without expanding its repair.

## Other Candidates Classified

| Source | Classification |
| --- | --- |
| Unit-cache launch UNIT_CACHE_SCRIPT_V1 | Eagerly computes a sibling script path, but does not read a policy or launch it at initialization. A deferred bundle-relative execution-path concern, not another demonstrated policy-startup failure |
| Native compiler profile roots, line115 | Deferred native profile construction includes its module and sibling schema URL. Not called by the inspected compiler action; no current failure demonstrated |
| Captured Bun loader string, line103 | Generated loader code intentionally reads its own file identity. Not a source-adjacent policy read; excluded |

Readers using physicalSourceOriginV1 and injected semioPhysicalSourceURL, and static JSON imports, were distinguished from the unprotected pattern. No compatibility loader, cache policy copy, dropped Source check or ambient fallback was proposed.

Backend reports focused relocation1test/8assertions and acyclic typing/sourcewalk2tests/66assertions passed, then launched sole qualified retry44967. Those are backend results, not results produced by this scan. Await actual authorizer passage before a frontend semantic launch.

## Capacity and Ledger

Canonical capacity at21:44:23UTC reports only177MB headroom. Qualified retry PID446177 holds3,584MB reserved/2,187MB observed; no waiters. The3,732MB semantic gate cannot fit and remains unlaunched. No new heavy request was added.

The unsupported --lang=en token was removed from the persistent-dev argv and prepared runtime documents. Commands rely on the common explicit SEMIO_LANGUAGE=en launch environment; no Source or service operation was performed for that correction. Native artifact/smoke remain passed and stopped, qualified receipt and persistent frontend remain pending.
