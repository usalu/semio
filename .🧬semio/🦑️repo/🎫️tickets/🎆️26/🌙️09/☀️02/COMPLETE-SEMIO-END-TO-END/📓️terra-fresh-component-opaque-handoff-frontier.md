# Fresh Component Opaque Handoff Frontier

Status: read-only audit on 2026-09-06. No Cargo, build, or product source was
changed. This reviews the implemented fresh-input capture/staging prerequisite
and specifies the next callback handoff only; it does not claim a browser actor
is catalogued, a staged generation is revalidated, or codec rows are immutable.

## Current Slice Is Coherent

The implemented producer now captures the raw component before JCO receives a
private staged copy ([`describe/📜️script.ts:351-360`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L351)),
captures its extracted core before the descriptor emitter receives its own copy
([`:361-375`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L361)),
then verifies descriptor JSON/Pack against those retained digests
([`:308-330`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L308)).
`freshStage` exclusively creates, boundedly writes, fsyncs, and removes a
partial stage ([`:279-305`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L279)).
The finalizer erases retained raw/core/descriptor inputs and deletes the private
work directory ([`:393-400`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L393)).

That matches the five registered `fresh-component-staging` laws described in
[`root-fresh-component-snapshot-staging.md`](./📓️root-fresh-component-snapshot-staging.md).
The exported receipt remains deliberately metadata-only at
[`FreshComponentReceiptV1:33-41`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts#L33);
it cannot reopen a component path or manufacture raw loader bytes.

## Smallest Callback-Owned Handoff

Do not return raw/core/descriptor arrays in the receipt, add a path field, or
give the callback `stageRoot`/`workRoot`. The next producer boundary should be
a **required, one-shot callback**, so the only current caller (Hub bootstrap at
[`hub/📜️script.ts:4662-4670`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L4662))
must consciously derive any actor before producer retirement. This is a
greenfield signature replacement, not an optional/legacy overload:

```ts
export type FreshComponentDeriveV1<T> =
  (input: FreshComponentComponentLeaseV1) => Promise<T>;

export type FreshComponentProducedV1<T> = Readonly<{
  receipt: FreshComponentReceiptV1;
  derived: T;
}>;

export async function produceFreshComponentV1<T>(
  repoRoot: string,
  request: FreshComponentRequestV1,
  freshTargetRoot: string,
  packageStageRoot: string,
  control: FreshBuildControlV1,
  derive: FreshComponentDeriveV1<T>,
): Promise<FreshComponentProducedV1<T>>;
```

`FreshComponentComponentLeaseV1` is an exported class/interface with no public
constructor and no paths. It exposes one `consume` operation only:

```ts
lease.consume(async componentBytes => /* derive and return */)
```

The producer constructs it only after descriptor verification and after the
component/descriptor stage hashes equal the retained snapshot. `consume` may be
called exactly once; it gets a **new bounded copy** of the verified raw
component, not `snapshot.componentBytes`, and always zeroes that loan on both
resolve and reject. The producer separately zeroes its source snapshot in the
existing finalizer. It exposes neither core nor descriptor Pack: the closed
browser actor builder needs only the component, and broader bytes would enlarge
authority without a current consumer.

The copy matters. A `Uint8Array` is mutable even behind TypeScript `Readonly`.
Giving the callback the producer's retained component lets it mutate memory
whose digest has already been admitted. The current browser builder defensively
copies its input at
[`browser-bundle/📜️script.ts:247-250`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts#L247),
but the handoff must remain correct for a throwing or future derivation too.

Hub's GIS callback should call the existing
`buildClosedBrowserActorArtifactV1(componentBytes, { cancelled, progress })`
directly; it must then compare the builder's independently computed
`artifact.componentSha256` to `receipt.component.sha256` before retaining the
artifact result. The actor builder already emits that field
([`browser-bundle/📜️script.ts:326-332`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts#L326)).
This comparison belongs beside the callback in Hub, not in a generic
`T`-returning producer, since a generic caller can falsely label arbitrary data
with a SHA string.

The callback result remains in memory until the Hub's subsequent strict actor
record/staging code owns it. It must not write an actor file into the producer's
stage directory. On callback error/cancellation, the existing producer removes
component/descriptor files and the outer Hub materialization catch removes the
whole private `stageRoot` ([`hub/📜️script.ts:4744-4751`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L4744)).

## Exact First Laws

Add these schema-first rows to the existing `fresh-staging` fixture and keep
the actor-specific final one in the Hub/browser integration fixture.

1. **verified-component-loan-is-one-shot**: callback runs only after descriptor
   verification/stage equality; a second `consume` rejects, and no public
   receipt contains a byte/path capability.
2. **loan-is-independent-and-zeroed**: retain a callback alias, mutate it, and
   resolve. The staged component still hashes to the original receipt; after
   callback settlement the alias is zeroed. Repeat with a callback rejection
   and prove both loan/source erasure and no component/descriptor stage file.
3. **cancel-before-and-after-derive**: cancel before lease consumption and from
   a derive checkpoint. The callback result is not returned, the initial
   producer's two stage files are removed, and the caller's outer stage cleanup
   receives the rejection. No actor output path is supplied to the callback.
4. **GIS-actor-binds-loaned-component**: feed the canonical fixture through the
   real closed-browser actor builder; require
   `artifact.componentSha256 === receipt.component.sha256`. A callback result
   made from a deliberately altered loan must be denied at the Hub comparison,
   even though the staged component remains valid.
5. **no-reopen-after-capture**: rename/replace all original component/core and
   descriptor source paths after capture but before consumption. The callback
   sees the original component SHA and the receipt/staged component still agree.

The producer-only first three can use a bounded fake derivation; they do not
need a costly JCO/Cargo run. The fourth is the actual browser/JCO acceptance
and belongs in the existing registered Hub/browser gate.

## Boundaries Still Explicitly Open

No immediate regression in the implemented capture/finalizer itself was found.
Two adjacent paths remain intentionally outside this slice and must not be
mistaken for input closure:

* `trustedBootstrapSourceCodecs` directly reopens mutable GIS/Stdio JSON source
  paths after component production ([`hub/📜️script.ts:4626-4632`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L4626)).
  It needs the next strict bounded canonical codec-row capture before its values
  are allowed into `profileSummary`/`generationId`.
* Before `rename(stageRoot, generationRoot)`, the Hub only fsyncs directories
  and does not reread/hash the staged component/descriptor against the receipt
  ([`:4720-4741`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L4720)).
  The existing-generation branch does verify SHA after reopen, but the new
  generation path needs the planned final whole-generation regular-file,
  bounded-digest fence. The callback handoff must not be treated as that fence.
