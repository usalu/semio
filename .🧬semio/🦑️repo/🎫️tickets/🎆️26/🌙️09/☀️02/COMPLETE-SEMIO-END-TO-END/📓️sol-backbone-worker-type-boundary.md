# Backbone Worker Type Boundary

## 2026-09-07 retained byte and fixture ownership

Renderer typecheck receipt `typecheck-95009.txt` reported 17 diagnostics in `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts`. The worker now copies every generic typed-array input that crosses Web Crypto, Fetch, or `Response` into an owned `ArrayBuffer`; no `SharedArrayBuffer`-compatible alias crosses those browser APIs. Byte equality accepts `ArrayLike<number>` and compares by index, so owned typed arrays do not require allocation merely to compare.

Replication mutation payloads remain `unknown` at the shared envelope boundary. The two local helpers now accept that exact type and delegate validation to the existing canonical `encodePackValue` implementation, which already rejects values outside the closed Pack grammar. This removes the three false `unknown -> PackValue` assignments without an assertion or unchecked cast and without a second encode/decode pass.

The stale worker fixtures were repaired at their proof boundary. The session-stamping fixture constructs a complete validated `DocumentExecutionTargetLeaseFieldsV1`; the GIS peer corpus is narrowed by an AJV generic type guard before use and its UI node is checked as `UiNodeRecord`; the bootstrap finish spy declares its receiver; rejected document-authority promises return `Error` only after an `instanceof Error` check. The Web Crypto cancellation seam now observes the `BufferSource.byteLength`, independent of whether the caller owns an `ArrayBuffer` or a view.

The registered renderer typecheck remains RED on diagnostics outside the worker (tutorial/renderer/Flow/repository-library owners), but emits no `backbone-worker.ts` diagnostic. The focused registered OS command initially reproduced one `hash-abort` fixture regression caused by the newly owned digest buffer; after fixing the seam it is GREEN: one file, 78/78 tests, 21.69 seconds. This is a controlled Node/Vitest worker result, not a genuine browser, Hub, GIS component, or rendered-scene claim.
