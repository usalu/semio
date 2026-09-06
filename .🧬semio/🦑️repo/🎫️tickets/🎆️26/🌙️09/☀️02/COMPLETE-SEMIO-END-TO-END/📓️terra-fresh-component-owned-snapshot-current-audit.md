# Fresh Component Snapshot Ownership And Bootstrap Profile Audit

Status: read-only current-source audit on 2026-09-06. No product files were
changed and no build was run. This corrects the older catalog frontier: Hub now
does have a concrete two-package Stdio/GIS producer at
[`🌎️hub/📦️packages/🦀️rust/📜️script.ts:4635`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L4635).
It is not yet a safe source of bytes for the next catalog/derived-actor record.

## Decision

Repair `produceFreshComponentV1` before attaching a browser actor or another
catalog payload. The producer must pass one **owned, verified component and
descriptor snapshot** to an immediate build-side consumer. That consumer writes
the generation from those bytes; it does not reopen Cargo output, descriptor
output, or the staged pathname. Keep the current Hub materializer as the sole
fixed Stdio/GIS coordinator rather than introducing a second GIS producer.

The required boundary is:

```text
fresh Cargo output --one bounded handle read--> component snapshot
  -> JCO/extractor and descriptor emitter consume private copied input
  -> one bounded handle read of core and descriptor outputs
  -> strict JSON/pack verifier over descriptor snapshot + component/core digests
  -> immediate trusted-generation callback writes exactly the snapshots
  -> receipt/profile metadata only; buffers are wiped in the producer finally
```

The callback may return a metadata-only `FreshComponentReceiptV1`; no caller
gets an artifact pathname or a byte owner after the callback returns. The Hub
can calculate its package summary and bundle from that metadata and the bytes it
just copied into its private staging root.

## P0: Descriptor Verification And Publication Currently Use Different Bytes

`produceFreshComponentV1` reads and verifies descriptor JSON/pack at
[`plugin/🖨️describe/.../📜️script.ts:353-368`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L353), then zeroes both admitted buffers. It subsequently reopens and copies the
descriptor pathname at [`:369-370`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L369). The copy's SHA-256 is put into the receipt but is never compared with the
SHA-256 of the verified descriptor bytes. `freshCopy` only compares the source
path's size before and after the copy ([`:285-314`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L285)); a same-size replacement therefore passes.

A descriptor changed between verification and `freshCopy` can be the bytes that
the Hub stages at `packages/<plugin>/descriptor.semio`, while its bundle says
only that the different staged hash is authoritative. This breaks the promised
descriptor-to-raw component admission before a catalog loader has a chance to
reverify it. Component staging happens to have a post-copy SHA comparison
([`:360`, `:369-371`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L360)); descriptor staging has no equivalent fence.

### Bounded repair

Replace `freshCopy(source, ...)` for both component and descriptor with two
different operations:

1. `freshReadOwned(path, limit, stage, control)` opens a regular,
   non-symlink file once, takes an fstat identity/size before and after its
   bounded handle read, checks cancellation per 64 KiB chunk, and returns the
   exact `Buffer` plus SHA-256. The path must be checked again after closing to
   reject a replaced path rather than silently attest a later object. It must
   reject changed mode, device/inode where those fields exist, size, or mtime
   across the admitted read. A renamed path that leaves the opened old object
   intact is safe only because no later operation reopens the path.
2. `freshWriteSnapshot(destination, bytes, ...)` writes that exact buffer with
   `wx` and fsync, returning its computed digest/length. It neither stats nor
   reopens a source path. The producer compares its output digest with the
   snapshot digest before returning.

Capture the raw component before running JCO. Give JCO and the descriptor
emitter a private work-root copy made from that component snapshot, not the
shared Cargo output pathname. Capture the extracted core through the same
bounded-handle primitive before using its digest in descriptor verification.
Finally, verify `descriptorJsonSnapshot` and `descriptorPackSnapshot`, then
write the exact packed descriptor snapshot into the Hub stage. The existing
`finally` must wipe all admitted buffers, including component/core/descriptor,
after the consuming callback returns or throws.

This is stricter than merely adding
`stagedDescriptor.sha256 === hash(descriptorPackBytes)`: that comparison fixes
the observed descriptor substitution but still lets JCO and the emitter read a
later mutable Cargo output after its original digest was captured.

## Current Hub Consumer And Profile Shape

`materializeTrustedStdioGisBundle` creates empty private build/stage roots,
sequentially produces fixed `stdio` and `gis` requests, and writes the bundle
before atomically renaming its generation ([`Hub script:4641-4743`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L4641)). Its profile framing correctly includes both component SHA-256/BLAKE3 and descriptor SHA-256, every native-codec row, and the GIS target fields
([`:4486-4523`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L4486)). The fixed profile is:

| Package | Stored identity | Codec/target role |
| --- | --- | --- |
| GIS / `semio:gis` | version, component SHA-256+BLAKE3, descriptor SHA-256 | exact Map + Terrain codecs; sole writable Map editor target |
| Stdio / `semio:stdio` | version, component SHA-256+BLAKE3, descriptor SHA-256 | exact 26-codec, zero-target package |

The profile generation therefore invalidates an old plan when either staged
component/descriptor identity or a native-codec row changes. This is a useful
existing seam for adding a later `browserActor` record; it does not itself make
the producer's current double-read safe.

There is a second source-authority boundary at
[`trustedBootstrapSourceCodecs`:4625-4631](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L4625): it parses two repository JSON files after both fresh components are
produced. Those rows are framed into the generation, but are neither admitted
from the verified descriptor nor captured under the producer's snapshot.
Before catalog sealing, read each codec declaration once through the same
regular-file/size identity primitive; strict-validate the expected 26+2
canonical rows and retain the parsed immutable rows until bundle encoding. Do
not claim that raw component/descriptor verification attests an independently
reopened codec declaration.

## Small API Change

Keep the exported request/control types if other build tooling needs them, but
do not let the trusted catalog use the general path-returning producer shape:

```text
produceFreshComponentV1(
  fixed build authority,
  fixed request,
  fresh target root,
  control,
  consume: (FreshComponentVerifiedSnapshotV1) => Promise<ReceiptMetadata>
) -> Promise<ReceiptMetadata>
```

`FreshComponentVerifiedSnapshotV1` is module-private/opaque. Its consuming
methods expose only:

* `writeComponent(destination)` and `writeDescriptor(destination)`, each
  single-use and exact-byte/fixed-name;
* component SHA-256/BLAKE3, descriptor SHA-256, lengths, core SHA-256,
  canonical descriptor-derived identity and WIT exports.

It exposes neither arbitrary source/stage path nor a public `Buffer`. The Hub
callback is the only current consumer: it writes
`packages/{gis,stdio}/{component.wasm,descriptor.semio}` while holding the
fresh snapshots, then returns the existing receipt fields. A future derived
browser bundle builder must receive the same component snapshot (or a separate
single-use `copyComponentToPrivateInput`) before it can add an actor row. It
must never receive `repoRoot` or a component path supplied by a client/plan.

The receipt remains metadata-only after callback completion. This keeps the
current materializer's immediate staging but prevents a retained raw byte owner
from escaping into an unrelated catalog producer.

## First Laws

1. **Same-size descriptor replacement.** Inject a replacement after
   `verifyFreshCatalogPackageV1` but before publication. Require rejection,
   zero stage files, and no bundle/current publication. Repeat a raw component
   replacement between Cargo output capture and JCO; reject before JCO is given
   a different pathname.
2. **Snapshot equality.** A valid component/descriptor pair produces stage
   bytes whose SHA-256, BLAKE3 where applicable, and byte lengths exactly equal
   the admitted snapshot and receipt. Mutating a staged file after callback
   write must be detected before generation rename.
3. **Core/codec source split.** Replace the extracted core or either codec
   declaration after its preflight but before use. Core replacement must deny
   descriptor verification; codec replacement must deny before profile encoding
   and leave no generation. Include 26/2 cardinality, duplicate row and
   malformed hash mutations.
4. **Derived payload precondition.** A browser-actor builder receives an
   exact GIS component snapshot and descriptor SHA binding; attempted use after
   snapshot finalization, alternate component path, or a callback failure emits
   no actor file/bundle row/marker.

These are producer and generation-assembly laws only. They do not claim a
trusted catalog loader, protected actor body route, worker containment, or
browser activation.

## Implementation Order

1. Implement the snapshot/descriptor P0 in the plugin describe script and add
   the first two laws.
2. Change the Hub materializer to consume snapshots directly and make codec
   rows one-time strict snapshots before profile encoding.
3. Add the mandatory derived-actor schema to the bundle/profile encoder only
   after those inputs are owned and immutable; use the materializer callback,
   not a separate GIS producer.
4. Then wire the verified immutable catalog, plan/lease projection, protected
   body route and private worker in the previously recorded derived-actor
   packet.
