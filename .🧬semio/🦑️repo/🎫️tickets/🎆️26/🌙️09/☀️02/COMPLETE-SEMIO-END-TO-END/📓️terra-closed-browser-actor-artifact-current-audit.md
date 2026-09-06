# Closed Browser Actor Artifact: Current Audit

## Scope and verdict

This is a source-only review of `buildClosedBrowserActorArtifactV1` and its fixture wiring. I did not run a build or fixture. Root reports the source artifact gate as GREEN and the canonical derived-artifact activation as GREEN; that qualifies the stated fixture paths, not a trusted catalog, plan, or worker installation.

The builder has a useful lower-level derivation boundary: it copies the component before hashing and code generation, pins the direct JCO package version to `1.27.0`, accepts only the fixed pure/host-async/first-party-WASI import inventory, constrains JCO output names and aggregate size, applies the existing closed-graph checks, and returns the closed ESM SHA-256 and byte length. The actor-factory law exercises the resulting ESM, including a real core-Wasm oracle, and the canonical actor-import fixture writes and activates the derived bytes at [`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📜️script.ts:120).

It is not yet sufficient as the next strict catalog bridge. Three missing identities prevent a correct insertion into the current plan/lease contract, and provenance/ownership still need tightening before the record can be catalog-admitted.

## P0 — the current lease cannot represent the derived ESM

The build result at [`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:85) has only:

```text
source component SHA-256, output SHA-256, output byteLength,
policy string, derived import interfaces, mutable output bytes
```

The current execution target needs **both** SHA-256 and BLAKE3 plus length for the served component ([`🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1173)). More importantly, its parser requires those execution-component digests to equal the package's source component digests ([`🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts:1254)). The closed ESM necessarily has different bytes from the source WebAssembly component. Serving it from the existing `/execution-target/component` route by changing the plan/package digest would also break descriptor binding; serving it with the old digest fails the browser verifier ([`🧵️backbone-worker.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:750)).

Do not overload `package.component*` or weaken the equality check. Add one catalog-owned execution payload record, selected only from the verified source package:

```text
semio.os.closed-browser-actor-artifact/v1
  sourcePackage:
    componentSha256, componentBlake3, descriptorByteSha256
  policy:
    id, policySha256
  payload:
    mediaType: text/javascript
    sha256, blake3, byteLength
  importInterfaces: sorted exact strings
```

The plan/lease needs an explicit `execution` payload distinct from its source `package`. It must bind `execution.sourcePackage` byte-for-byte to the selected descriptor/package and bind `execution.payload` to the authenticated body endpoint. Only the trusted catalog compiler computes the missing payload BLAKE3 from the exact output bytes and seals the record. The browser verifies the payload fields before import; it continues to parse the raw package descriptor against `sourcePackage`. This preserves the current source-component/descriptor relation without pretending an ESM is the package component.

The catalog compiler, plan signature, protected manifest, component-body response, and private worker relay must all carry this new record as one unit. A caller cannot provide a derived digest, policy, imports, module URL, or body selector. Until then, this builder is a fixture/build capability only.

## P0 — provenance is only a label, not a complete fixed policy

`codegenPolicy: "semio.os.browser-jco-1.27.0-jspi.v1"` and the child assertion at [`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:48) pin the direct JCO version. They do not bind:

- JCO's resolved package content or its transitive transpiler/shim graph. The lock currently records JCO `1.27.0` and an integrity string at [`bun.lock`](/Users/ueli/Documents/semio/bun.lock:845), but the builder neither checks a lock digest nor records it.
- the Bun version and TypeScript version used by `closedBrowserActorBundle`; both affect emitted ESM/AST admission, while the policy label says only JCO/JSPI.
- the precise codegen/closure policy source and the ordered fixed WASI-async/admitted-interface lists.
- the executable and ambient environment used for `spawn("node", ..., { env: process.env })` in [`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:42). `PATH` and `NODE_OPTIONS` remain ambient authority.

Make the policy a first-party canonical, hashed object rather than a string. It should contain the exact JCO package identity/integrity, complete lockfile digest, Bun and TypeScript versions, closure-policy source digest, fixed interface and JSPI operation arrays, and all output bounds. Construct an absolute, fingerprinted Node tool and sanitized child environment (no inherited `NODE_OPTIONS`, loader hooks, or caller-selected executable). The trusted catalog stores and recognizes the policy digest, not a free-form policy name. A source digest or policy constituent change requires a new policy revision and a new catalog build; runtime only verifies the sealed artifact record.

## P1 — parent-side generated-manifest and scratch ownership

The child validates JCO output names before emitting JSON, but the parent parses that JSON as an unchecked cast ([`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:67)) and then uses child-provided filenames in `join(evidence, name)` ([`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:76)). `lstatSync` followed by `readFileSync` is also a check/use pair. The local JCO child is currently the only writer, but this is exactly the boundary the catalog would trust.

Before any read, independently parse and require: exact JCO version; a unique sorted imports array whose every item is in the fixed policy; 2–65 unique filenames; exactly one `browser-actor.js`; each remaining name matches the core regex; and no path separator, `.`/`..`, or duplicate. Open inside the private scratch directory, verify the opened regular file identity/size before and after bounded read, and reject a changed inode/size. Recheck aggregate bytes in the parent; do not rely solely on child assertions.

`snapshot.fill(0)` at [`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:86) does not retire `component.wasm`, JCO's source/core files, or codegen logs written under `evidenceRoot`; those survive success, cancellation, and failure. Split caller-visible test evidence from production scratch ownership. The catalog compiler should obtain a private scratch-owner capability, ingest verified output bytes, then retire the whole scratch directory on every terminal path. It should expose no raw input path or generated core path to its caller.

The returned outer object is frozen, but its `Uint8Array` is mutable. Mutating `artifact.bytes` after return leaves `artifact.sha256` stale. Keep bytes in an opaque single-use build owner (or copy+rehash immediately in the catalog compiler) and make catalog sealing consume that owner. A sealed catalog record contains digests/length/interfaces only; its body store owns its own freshly hashed byte copy.

## P1 — bounds and cancellation laws still needed

The child runner polls cancellation every 100 ms and terminates its process tree ([`🟦️.ts`](/Users/ueli/Documents/semio/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1874)), which is a sound base. The current `cancel-after-codegen` vector cancels only after JCO has returned, before closure ([`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:495)); it does not prove cancellation while the external codegen child is live, nor scratch retirement on that path. The 64 MiB byte limit bounds individual material, but peak memory includes source snapshot, JCO child allocations, generated disk files, decoded cores, ESM string, and returned bytes; catalog admission needs a one-at-a-time retained build slot and a declared aggregate memory reservation before copying the component.

Add these schema-first vectors:

1. A controlled codegen port/process fixture that blocks after writing the snapshot; cancellation must terminate it, return the cancelled terminal result, and leave no scratch entry.
2. A hostile manifest with `../`, duplicate core, extra file, unsorted imports, wrong version, and replacement-after-stat; each denies before closure and never reads outside scratch.
3. Mutable returned-byte mutation followed by catalog ingestion; the ingestion must detect/recompute the payload hashes and reject a stale receipt.
4. Cross-process deterministic rebuild under the same sealed toolchain policy, comparing the full derived record and ESM digest. The present repeat test is valuable but runs in one toolchain/process context.
5. Catalog-plan round trip: only source package plus catalog-sealed derived payload reaches the protected target body; changing either source descriptor/component hash, derived SHA/BLAKE3/length, policy digest, or imports causes stale/integrity denial before ESM import.

## Boundary retained

The current tests establish build output closure and actual fixture activation, including canonical JCO imports. They do not prove trusted catalog admission, descriptor-to-derived-payload binding, plan/lease evolution, protected payload transport, production actor-worker containment, or browser sandboxing. No production trust claim should be made from the builder alone.
