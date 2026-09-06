# Real GIS JCO Describe in the Dedicated Child

## Verdict

The first honest browser execution claim is narrow and now mechanically specified: execute the **freshly materialized, policy-bound GIS closed actor** in the dedicated Chromium Worker and invoke its WIT export `describe.describe()` with no arguments. Its returned byte list is **not** byte-identical to the fresh `packages/gis/descriptor.semio` snapshot: the guest emits the canonical descriptor with all three artifact hash fields empty, while the materializer patches the final staged descriptor with raw-component, extracted-core, and descriptor-self hashes. The oracle must compare the result to the canonical staged descriptor after blanking all three typed hash fields, then verify the final staged hash bindings separately.

This proves a real GIS component made it through component materialization, JCO closure, verified child transfer, activation, and one actual WIT call. It does **not** prove rendering, a `reactor.poll` turn, host effects, document access, Map persistence, or collaboration.

The ongoing native materialization target is the authority for the actual GIS import list. This audit did not run it and does not treat the ticket's synthetic generation-stage files as a GIS result.

## Exact callable shape

The declared component ABI is unambiguous:

* `interface describe` exports `describe: async func() -> list<u8>` in [the component WIT](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1300).
* `world actor` exports the `describe` interface alongside `reactor`, `jobs`, and `checkpoint` at [lines 1319–1325](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1319).
* The existing JCO actor API confirms the nesting: it calls `describe.describe()` at [the generated wrapper seam](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts:587).

So the dedicated child call is exactly:

```ts
await child.invoke(["describe", "describe"], [])
```

It must accept only a `Uint8Array` with an exclusive, fixed backing buffer. The child already rejects aliased/resizable values and bounds a result at 1 MiB at [its wire measurement boundary](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts:19); however, the trusted catalog currently permits a descriptor through 4 MiB. Before this particular law exists, either set the first describe-specific output bound to the trusted descriptor maximum or reject an admitted descriptor above the child result maximum. Comparing only a prefix, accepting a string, or silently enlarging the generic child output limit would be a false proof.

## Expected descriptor facts

The browser result's primary oracle is typed, canonical equality with the descriptor captured in the same fresh producer lease **after blanking all three hash fields on the staged form**. The WIT documents that `describe()` produces the packed `PackageDescriptor` written as `descriptor.semio` at [lines 1292–1301](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1292). The trusted loader already has a canonical decode/reprojection check at [trusted-catalog](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1179).

After equality, decode those exact bytes using that existing canonical descriptor decoder and assert these source-backed neutral facts:

| Fact | Current authority |
| --- | --- |
| package/plugin/version | `semio:gis` / `gis` / `0.1.0` from [GIS assembly](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🦀️.rs:27) |
| role and dependencies | plugin role and no dependencies; the materializer candidate checks the corresponding fresh descriptor before generation publication |
| artifact dialects | `s.gis.gismap` / `gis.map` and `s.gis.gisterrain` / `gis.terrain`, from [the identity corpus](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🧪️fixtures/🪪️artifact-identity/🔣️.json:1) |
| GIS Map selected surface | `s.gis.gismap@1/*#editor`, role `editor`, renderer target `wasm`, window kind `gis2d-main`, from [the trusted producer target](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5409) |
| declared app fleet | Map editor/viewer plus terrain editor/viewer, source-registered at [GIS assembly](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🦀️.rs:10) |

The **staged** descriptor's wasm hash must equal the same fresh component hash and its final descriptor bytes must match the materialized descriptor snapshot/hash. The runtime result instead must carry exactly three empty hash strings. A static hash, a hand-authored descriptor fixture, a fixture actor, or an extracted temporary core is not an oracle.

## Current child profile and required local port

The real materializer consumes the retained fresh GIS component directly into `buildClosedBrowserActorArtifactV1`, captures the observed import interfaces, and stages the resulting closed actor at the fixed GIS path [during materialization](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5378). Its actual `importInterfaces` decides which branch is possible:

1. If only the two Semio interfaces are present, the current deny-all host port is enough for a describe-only call, provided `describe()` itself does not request an effect.
2. If any allowed Preview2 interface is present, the current child source cannot activate it: [the worker's host port](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts:20) exposes no `wasi` member, whereas the closed-bundle wrapper requires `port.wasi.nowNs` and `port.wasi.write` before instantiation. This is a present source gap, not a reason to widen the policy.

The smallest safe extension is a **per-worker, private** `BrowserWasiPort` that implements only the existing port contract at [WASI lines 1–15](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:1):

* `nowNs()` returns a non-decreasing `u64` derived from the worker-local monotonic `performance.now()`, clamped against a retained `lastNow`. It must not use `Date.now()`, a Hub time endpoint, or the plan clock. The same private monotonic source should provide the pure `nowMs()` import; current `Date.now()` at [worker line 21](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts:21) unnecessarily gives wall-time information.
* `write(stdout|stderr, bytes)` accepts only the already bounded synchronous writes, makes a local copy and immediately zeroes it. It must not post a message, update Shell/plugin state, access a document, or print host-visible diagnostics. `createBrowserWasiActivation` itself enforces the 64 KiB single-write and 1 MiB aggregate bounds and calls the port synchronously.
* `exit(status)` marks only this child activation closing (via a queued local retire); it must not turn a guest exit into a successful describe result or inform a Hub route.
* The current private `pure.log` and `pure.traceSpan` should be bounded local no-ops/counters for this proof, while all `host-async` calls stay denied. This permits a component's pure diagnostic path without granting an effect bridge.

No filesystem, network, random, environment values, command arguments, terminal handle, caller URL, receipt, document content, Hub callback, or global resource map is needed. A materialized interface outside the existing fourteen-item Preview2 vocabulary remains a pre-publication rejection.

## Smallest Chromium acceptance law

Add the law beside the existing real Chromium containment owner in [Hub's `BrowserActorChildWorkerContainmentCheckScript`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3918), but give it a separate fresh-materialization fixture/target rather than adding GIS bytes to `child/🧪️fixtures/🔣️.json`.

Suggested name: `trusted_fresh_gis_closed_actor_describe_runs_in_chromium_without_host_effects`.

1. Use the just-created immutable trusted generation, the native materializer's own retained descriptor snapshot, and its accepted `browserActor` row. Assert the GIS actor SHA, byte length, component hash, descriptor-byte hash, policy hash, and sorted observed import interfaces before worker construction.
2. Reserve one child using the exact actor hash/length and private document runtime key. Fetch or hand off the closed actor only through the selected, authenticated body path; transfer it once; require the parent buffer detached and the child rehash/`loaded` acknowledgement to match the catalog row.
3. Invoke only `["describe", "describe"]` with `[]`. Require a single `Uint8Array`, exact byte length and SHA equality with the same generation's `packages/gis/descriptor.semio`, then canonical-decode it and assert the neutral table above. Enforce an invocation deadline and the descriptor/result bound before accepting it.
4. Assert zero host dispatches, zero effect reply/cancel messages, zero document read/write/HTTP/blob effects, zero Shell/plugin view-state writes, zero Map outbox/WAL/store changes, and no public module URL request after the one admitted actor-body transfer. This is intentionally a descriptor execution law, not a renderer or persistence law.
5. On normal close, activation abort during load, actor-byte/hash substitution, descriptor substitution, and expired/revoked reservation, require rejection or pre-load retirement; then require worker termination, buffer wipe/detachment, and child capacity `{actors: 0, bytes: 0}`. Do not let a rejected `describe` result be decoded as a descriptor.

The existing synthetic containment fixture does exercise tight-loop termination and local WASI output, but it is not evidence that the current GIS component imports, instantiates, or describes successfully. In particular, its purported `port.wasi` calls must not be reused as proof until the source worker itself supplies the local WASI port described above.

## Nonclaims and next boundary

Even on a green law, the actor's required `host-async` interface remains deny-all and the call never exercises `reactor.poll`. The next separate production slice is an authenticated, scope-bound host bridge for an actual Map turn and then the Store/WAL fixed-three approval owner. This describe law must not be used to enable arbitrary actor invocation, rendering, or actor body delivery outside the private reservation.

## Current local-WASI implementation addendum

Root's TDD has now landed a worker-local `wasiPort`: [the worker](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts:21) admits only `loading`/`active` use, uses `performance.now()` for nanoseconds, caps its own output at 65,536 bytes / 128 writes, and sends `exit` to local fault/retirement. The limits are schema-bound at [the child schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts:1). This narrows the 1 MiB / 64 KiB-per-write generic WASI activation budget, which is safe as capability reduction.

Three corrections are necessary for the proposed **no-host-effect real GIS describe** law:

1. `wasiPort.write` currently decodes and sends guest bytes to `console.log`/`console.warn` at [worker line 31](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts:31). The current generic containment test explicitly expects those console values at [Hub script line 4102](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:4102). That is bounded but observable guest-output leakage. The real GIS path must instead use an activation-private synchronous copy-and-zero discard sink and assert no console output; retaining diagnostics can remain a separately named test-only port, not the actor production port.
2. The same worker still supplies pure `nowMs` with `Date.now()` and rejects `log`/`traceSpan` at [worker line 33](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts:33). `describe` is expressly a build-time, pure-import call, so a genuine component can legitimately use these pure operations. Use the same bounded worker-local monotonic source for `nowMs`, and bounded local no-op/counters for log/trace, rather than assuming the GIS implementation never calls them or granting wall-time/console access.
3. The generic child result bound remains 1 MiB, while the trusted catalog's descriptor admission cap is 4 MiB. The first real law must reject a selected descriptor exceeding the transport's exact 1 MiB output budget before child loading, or introduce a separate descriptor-only, owned 4 MiB transport/limit with matching schema and hostile `max+1` tests. It must never truncate, compare a prefix, or make a verified 4 MiB descriptor silently unexecutable.

The fresh materialization and Chromium runs cited by root were still active when this addendum was written; no real GIS import list, closure, descriptor result, or rendered surface is claimed here.

## Correction: Current Pure-Clock And Diagnostic-Port Semantics

Status: read-only source review after the worker-local WASI addition. No
browser or native command was run for this correction.

### `pure.now-ms` is currently a Unix-wall-clock contract, not a monotonic timer

The WIT type alone is only `s64`
([`📜️.wit:911-914`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit#L911)),
but current first-party implementations establish its live semantics:

* the native guest fallback returns `SystemTime::UNIX_EPOCH` milliseconds
  ([`🌐host/🦀️.rs:1047-1055`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs#L1047));
* the existing browser JCO host returns `BigInt(Date.now())`
  ([`📦️packages/🟦️typescript/🟦️.ts:907-918`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts#L907)); and
* the OS reactor reads it once per poll to stamp and expire shared presence
  peers ([`⚛️reactor/🦀️.rs:1846-1851,1881-1885,1968-1973`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs#L1846)).

Replacing the child `hostPort.nowMs` with `performance.now()` would therefore
silently make a real browser guest disagree with native presence TTL and
wall-clock ordering. No HLC-specific caller was found in this narrow plugin
call graph, but the presence expiry caller alone makes that replacement an
incompatible semantic change. Retain epoch milliseconds for `pure.now-ms`.
The separate WASI monotonic-clock port may correctly use worker-local
`performance.now()`; it is a different WIT clock with different consumers.

The first real GIS law should instead require only an in-range signed 64-bit
epoch-millisecond result and should not make it an authority input: no plan,
lease, revalidation, document or socket comparison may consume a guest-returned
clock value. A future clock split needs a new WIT name and twin schemas, not a
silent host substitution.

### Bounded console diagnostics are not a current authority leak

The current child output path decodes at most the schema's bounded total and
calls `console.log`/`console.warn` only
([`child/🧵️worker.ts:21-35`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts#L21)).
It neither sends a `MessagePort` payload nor has a Hub/document/network/storage
bridge. The only observed consumer in the current Chromium containment target
is Playwright's page diagnostic listener, which deliberately asserts those two
strings ([`Hub 📜️script.ts:3959-3964,4102-4104`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L3959)).

Accordingly, there is no source-backed cross-document, Hub, or plugin-view-state
leak to classify as P0. The bytes are nonetheless developer-console-visible,
so an operator who treats guest diagnostics as sensitive can select an explicit
discarding diagnostic policy later. That is a product/privacy policy choice;
it is not required for the present no-document/no-network/no-storage execution
claim. The real GIS `describe` law may additionally assert it emitted no
diagnostics, but must not require removing this bounded diagnostic feature to
establish actor containment.

## Correction: Runtime `describe()` Is A Hash-Blanked Canonical Descriptor

Status: read-only source trace. No GIS materialization, native target, or
Chromium invocation was run for this correction. The previous literal
byte-equality wording in this report was wrong.

### Current byte transformation

The guest implementation constructs the descriptor and deliberately sets
`wasmSha256`, `coreWasmSha256`, and `descriptorSha256` to empty strings before
the first-party Pack encoder runs
([guest `describe_plugin`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs#L137-L155)).
The native emitter then invokes that exact component export through its owned,
pure-import execution path ([`execute_describe_owned`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/🦀️.rs#L410-L424)).
It typed-decodes the returned Pack and changes **only** these fields:

1. `hashes.wasmSha256` becomes the SHA-256 of the captured raw component;
2. `hashes.coreWasmSha256` becomes the SHA-256 of the separately captured core;
3. `hashes.descriptorSha256` is derived by canonical-encoding the descriptor
   with that field empty, then is inserted into the final canonical Pack.

That is the full mutation at
[`describe_component:426-465`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/🦀️.rs#L426-L465).
It does not patch `manifest.version`, package/plugin identity, execution,
contributions, or another semantic descriptor field. The existing native
freshness utility independently documents and implements the same all-three
typed blanking transform
([`descriptor_bytes_with_blank_hashes`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs#L29514-L29532)).

Therefore the real-child law must not compare the child result's length, SHA,
or bytes directly with `packages/gis/descriptor.semio`; all three necessarily
differ from the final staged form. It must also not patch the child answer from
catalog values before comparison: that would turn the asserted guest fact into
a host reconstruction.

### Required real-GIS oracle

For the selected immutable GIS generation, perform the following in order:

1. Read the staged descriptor once through the trusted generation's bounded
   regular-file/receipt path, retain those bytes in memory, and require their
   exact `byteLength` and SHA-256 to equal the selected `receipt.descriptor`.
   The actor row must already bind that same receipt via
   `sourceDescriptorByteSha256`, and its `sourceComponentSha256` must equal the
   selected component receipt.
2. Before starting the child, typed-decode the retained staged Pack, require
   canonical re-encoding to equal the retained bytes, and verify its final
   hashes: raw equals the selected component SHA, core equals the receipt core
   SHA, and a re-encoding with **only** `descriptorSha256` blanked hashes to its
   final self hash. These are catalog/materializer facts, not guest facts.
3. Invoke only `describe.describe()` and require one bounded `Uint8Array`.
   Typed-decode it, require canonical re-encoding to equal the returned bytes,
   and require each of its three `hashes` members to be exactly `""`.
4. Clone the retained staged Pack with the same first-party Pack vocabulary,
   blank **all three** hash members, canonically re-encode it, and require exact
   byte equality with the returned result. All non-hash fields are consequently
   covered by one byte comparison without a JSON-number or key-order adapter.
5. Only after all four steps accept the call; then normal close must retire the
   actor/source and restore child capacity to zero. This remains describe-only:
   no renderer, Map, document, WAL, or host-effect claim follows.

The current TypeScript catalog verifier cannot be used unchanged for step 3:
it correctly requires every catalog hash to be 64 lower-case hex
([`validateCatalogDescriptorPair:2609-2621`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts#L2609-L2621)).
Add one small explicit runtime-describe normalizer beside that codec instead of
loosening catalog verification: strict/canonical Pack decode; exact descriptor
shape; exactly the three empty runtime hashes; clone through `clonePackValue`;
blank all three staged hashes; canonical Pack encode. It must not route through
`packValueToExactJson` or `JSON.stringify`, which is not the Pack identity
contract.

Suggested law name:
`trusted_fresh_gis_closed_actor_describe_matches_hash_blank_staged_descriptor`.
Its minimum hostile rows are: a guest result with one populated hash; a
non-canonical/trailing Pack result; a staged non-hash mutation such as
`manifest.version` or `packageId` with otherwise valid hash fields; a staged
raw/core/self-hash inconsistency; and a selected actor whose source component
or descriptor-byte hash differs from the selected receipt. These exercise both
independent halves of the binding rather than accepting a synthetic blank
descriptor.

### Capture and lifetime boundary

`produceFreshComponentV1` captures the raw component first, creates the core
from a private staged copy, invokes the real emitter, then captures the emitted
JSON/Pack pair before any generation stage
([`produceFreshComponentV1:417-463`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L417-L463)).
`captureFreshComponentInputs` validates that exact final pair against those
same captured raw/core bytes and retains the final Pack snapshot
([lines 317-339](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L317-L339)).
`stageFreshComponentInputs` writes that Pack unchanged as
`descriptor.semio`, checks its SHA against the snapshot, and records that final
SHA/length in the receipt; it does no post-stage descriptor patch
([lines 385-413](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L385-L413)).

The GIS materializer derives the browser actor from the one-shot private raw
component lease and records both source identities before actor bytes are
staged ([Hub materializer lines 5383-5403](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5383-L5403)).
Thus the child law must use the selected generation's retained descriptor
bytes, not the metadata-only `FreshComponentReceiptV1`, an old work directory,
or a later source-tree reread. The shared stable reader protects the captured
input by lstat/fstat device-inode-size-time identity before and after a bounded
read ([`readStableBuildFile:29-49`](../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts#L29-L49)); once the
trusted loader has accepted and copied the selected stage, the comparison must
continue from that in-memory owner.

This is GIS-specific. The current rotation deliberately changes the **Stdio**
label and recomputes its descriptor self hash
([Hub rotation lines 5588-5603](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L5588-L5603)); it supplies no GIS closed actor and is not evidence for a
generic direct byte-equality rule. No source shows a GIS version patch after
the emitter: the requested version is read from the verified final descriptor
and carried unchanged in the fresh receipt.
