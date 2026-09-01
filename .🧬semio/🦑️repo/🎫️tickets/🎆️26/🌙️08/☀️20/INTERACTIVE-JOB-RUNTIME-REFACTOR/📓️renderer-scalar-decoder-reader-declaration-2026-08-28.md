# Scalar Decoder and Original Reader Declaration

This first-stage report is historical. The exact reader callables, work phases and executable closed-model refinement are superseded by [R187–R193](📓️renderer-scalar-hardening-r187-r193-2026-08-28.md); current ticket JSON carries that refinement. No runtime/price cutover follows from either report.

## Status and Scope

This is a **ticket-only proposal**, not production source, an admitted decoder, a released API or a price change. The four JSON declarations are in `📄️scalar-decoder-declaration`. They describe one scalar algorithm at a time over the original field-owned input. Symbols, complete strings, containers, Surface backing, typed projections, output publication and both transport/semantic ACKs remain separate work.

The current resident source was reread: Payload21 fields at resident/component27, Reader11 at31, reader admission692–720, byte reading741–744, facade754–759. The existing reader increments its cursor and returns a newly constructed byte result. It has no private scalar consumer/receipt association. Builder.reader is already occupied by its real reader; the existing312-byte Payload domain cannot silently fund a decoder pointer.

## Fixed Census and Proposed Prices

The contract lists every field, not an aggregate placeholder:

| Record | Fields | Logical bytes |
| --- | ---: | ---: |
| ScalarState |32|272|
| ScalarFacade |1|24|
| ScalarProgress |1|24|
| ScalarWitness |2|32|
| ScalarReadReceipt |8|80|
| Total domain |5 records|432|

Each record is16+8×field count. Domain432/5/5 plus original neutral record264/3/3 and admission296/6/6 yields **992 bytes /14 slots /14 owners**. No per-instance closures, callbacks, generators, arrays, maps, WeakMaps or scratch backings are proposed. Module functions/private brands are shared, not one allocation per decoder.

Two independently priced parent changes are proposed, **not mounted**: Payload.scalar adds one word (312→320, total880/15/15); Reader.scalarConsumer/scalarReceipt add two words (160→176, total736/12/12). The80-byte receipt is funded once in the decoder record; the reader's pointer words are funded in its own record. Current production prices remain312 and160. These proposals require a coordinated source/catalogue/fixture release before any runtime cutover.

The logical model does not bound arbitrary exception graphs, physical JS object headers, allocator overhead or temporary engine values. The first unknown exception remains quarantined under its original charged owner; it is not declared deallocated.

## Concrete Proposed Interface

Only the original privately branded payload drives these methods; no public decoder constructor, record/cell accessor, arbitrary reader factory or structural retirement callback:

```ts
payload.beginScalarDecoder(grant): ScalarAdmission
payload.startScalar(decoder, profile, grant): ScalarStep
payload.advanceScalar(decoder, grant): ScalarStep
payload.consumeScalar(decoder, exactResultSerial, grant): ScalarStep
payload.closeScalarDecoder(decoder, grant): ScalarStep
```

Names are proposed, not exported. ScalarAdmission exposes the original facade only after both directions of reader binding are observed. Recovery during construction is through original payload.scalar and its preexisting pending slot, never a caller-returned handle alone. ScalarStep forwards an existing child result on a child turn; decoder-owned turns expose the preinstalled ScalarProgress view. That view is explicitly a mutable latest-step view, not a historical receipt or publication authority. A stale reference cannot authorize another read or consume. Any existing child transient result allocation remains that child's source-accounting obligation, not a claimed decoder admission.

Scalar ready state exposes only a primitive number or bigint and checked u64 result serial. It holds that primitive until the same parent consumes the exact serial. Starting another profile while a result is outstanding rejects. Supported primitive profiles are UI value tag, u64 natural, safe53 natural, one UTF8 codepoint and UI finite f64. A profile selector is an algorithm choice from the trusted parent at idle, **not** permission to interpret an arbitrary field as a valid whole native packet. Existing native identity safe53 rejection remains unchanged.

## Original Installation and Recovery

The JSON lists13 decoder construction grants, then the separately admitted original reader's13 grants, then exact binding/observation/publication64-byte turns. Every phase consumes its own grant; no prepare+construct+install reuse.

The record and cell are linked through the existing original pending slot before domain allocation. State is installed into both slot.entry and payload.scalar before facade construction/finalization. Facade, progress, receipt and witness install themselves into the original state before any fallible finalizer. An after-install throw recovers the identical shell; no duplicate allocation retry. The one pending slot is cleared only after its decoder fields are retained on payload.scalar, permitting the real reader admission to reuse it. Otherwise reader admission would deadlock behind its own decoder.

Reader construction privately binds the original consumer/receipt before exposure. The existing unrestricted reader.advance/builder.beginRead consumer boundary must be narrowed in the actual future cutover; a correct caller sequence alone is insufficient. No change to it is made by this proposal.

## Read, Latch and Parse Boundaries

The preallocated eight-word ScalarReadReceipt binds exact original reader, decoder, phase, checked serial, kind, items, bytes and byte value. Its sole concrete source writer must install a scalar byte outcome before returning that byte. It is not a caller-supplied receipt or a setter accepting arbitrary roots.

The intended transaction is: source scalar read → later64-byte observation/latch → separate one-byte parse → later64-byte receipt settlement → possible scalar result publication. No caller-controlled offset, borrowed source buffer, or read-ahead array exists. Reading into the receipt and advancing the source scalar cursor are one source-owned scalar transaction, not an adapter adding work after an arbitrary child completes.

Reader maintenance may perform alias/page work instead of reading a byte. Such a turn forwards the exact child result, including blocked/rejected and raw over-grant counts, with no appended wrapper charge. Completion bookkeeping occurs on the following grant. The scalar receipt is not a fabricated byte result for a maintenance turn. A throw after maintenance or a scalar mutation permanently faults forward execution, retaining the original owner; the design does **not** claim rollback or clean retry after unknown mutation. Tests must prove the canonical source transaction itself before adopting this boundary.

Only an exact reader await-page/await-seal phase may select the original builder for the next separate turn. A generic blocked result is not converted into success. The decoder is admitted and bound before first page/EOF so it can drain the single256-byte window while the source remains incomplete. Source consumption, scalar completion, field EOF and publication are distinct facts.

Natural accumulation prechecks each next digit and the tenth u64 byte before arithmetic, preventing an oversized intermediate BigInt. Safe53 is checked against exact bigint before Number conversion. UTF8 produces a codepoint, not an uncharged string. f64 uses two32-bit words and arithmetic, not an allocated DataView backing; the independent oracle uses Buffer.readDoubleLE. Negative zero/nonfinite rejection is specifically the existing UI profile, not a generic codec-wide float ban.

## Two-Sided Close and Fault Ownership

The same two-word ScalarWitness changes phase; no second closure proof is allocated. Stop new work, observe/discard an outstanding scalar receipt/latch under explicit grants, and prove input-quiescent without pretending reader terminal. The reader replaces its exact consumer reference with the same witness; only after that private observation may the decoder remove receipt dependencies. Reader settlement clears its two original pointer words. The decoder then drives the existing exact reader/page/alias/builder-binding retirement and separately observes reader terminal.

The original payload pending slot captures the decoder before payload/backlinks/body are cleared. Only actual body emptiness and no unknown fault permit its domain-terminal witness. Payload.scalar unlink, neutral record detach/observation/close, admission alias/cell close and final parent observation remain separate grants. Original-vs-foreign/retired pairing is checked at every step; negative match alone is not a never-installed authority.

Constructor, read, parse, receipt, detach and settlement wrapper faults retain the first exact unknown root under the original state/cell before rejection. Same-value replay is inert. A distinct second exception remains caller-owned/rethrown, not overwritten. Revocation forbids new operational admission but never revokes the original close authority. No new actor-name/instance-ID lookup participates in close.

## Declaration Oracle and Limits

R182 was an inline command quoting error before execution; preserved separately, no test outcome. R183 actual inline diagnostic exits0 and prints:

```text
[DEBUG] {"strictAjv":2,"recordCensus":5,"vectors":43,"accepted":25,"rejected":18,"plannedWindowShapes":3,"immerChargeTransitions":3,"runtimeDecoderExecuted":false}
```

Strict Ajv validates both JSON documents; declared record arithmetic is checked independently. Existing @webassemblyjs/leb128 decodes/re-encodes canonical naturals, Buffer reads f64, and fatal TextDecoder with ignoreBOM preserves UTF8 codepoints. The43 neutral scalar rows agree with those independent oracles. Immer checks a small declared admission/fault-preservation/refund arithmetic model. These are **declaration/oracle results only**, not a production parser test or actual neutral-ledger reservation.

The three planned window shapes place a multibyte scalar at byte255 using preceding valid primitive units; no seek or synthetic offset is assumed. Only their arithmetic shape was checked. Actual reader/window/split, constructor loss, child-grant and cancellation tests remain missing-method/runtime RED work after approval. Neither this diagnostic nor the immutable JSON proves physical capacity, source-page continuation, all-profile parity, clean unknown-fault retirement or any native/UI ACK.

## Capacity and Progress Accounting

The proposed delta over the existing one-window path is992 decoder bytes +8 payload pointer bytes +16 reader pointer bytes =**1016 additional logical bytes**,14 additional slots and14 additional owners. This is not the full field/response footprint. Existing original source/response, builder, evidence, page, intrinsic reader and control charges remain simultaneous and must be admitted by the same composition ledger. No quota is increased; if those exact available resources do not fit, construction blocks before allocation. Variable typed output and still-unmounted source metadata must not be omitted from a later end-to-end capacity proof.

For an input unit of b successfully parsed bytes, decoder-only normal work has four scalar phases per byte (read, observation/latch, parse, settlement), plus start/publication/consume:4b+3 turns once the reader is ready. This excludes actual reader/page/alias/builder maintenance and admission; those transitions must be derived from their original fixture phase lists, not hidden inside this bound. b is at most10 for natural-u64,8 for f64,4 for one UTF8 codepoint and1 for tag. A malformed/truncated input may finish earlier; expected fault observation is a separate phase, not an invented byte.

Runtime tests must assert monotonic parsed offset, source serial, remaining scalar bytes and finite admission/retirement phase progress. A blocked source/page is external backpressure, not counted as arbitrary permitted stutter or a reason to synthesize EOF. The future cancellation loop bound is the sum of the declared decoder close phases and actual child close phase bounds for that exact prefix; no opaque increased iteration cap is proposed.

## Exact Declaration Diagnostic

This is an inline ticket diagnostic, not a new script file or a registered production/test command. The prospective permanent test belongs in the existing renderer test registration only after runtime/schema approval.

```sh
bun -e 'import Ajv from "ajv"; import {produce} from "immer"; import assert from "node:assert/strict"; import {Buffer} from "node:buffer"; import * as importedLeb from "@webassemblyjs/leb128/lib/leb.js"; const leb=importedLeb.default??importedLeb; const dir=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📄️scalar-decoder-declaration";const c=await Bun.file(dir+"/🧬️contract.json").json(),f=await Bun.file(dir+"/🧪️fixture.json").json();const ajv=new Ajv({strict:true,allErrors:true});for(const [schema,value] of [["🧬️schema.json",c],["🧪️schema.json",f]]){const check=ajv.compile(await Bun.file(dir+"/"+schema).json());assert(check(value),JSON.stringify(check.errors));} for(const r of c.records)assert.equal(r.bytes,16+8*r.fields.length);assert.equal(new Set(c.records.map(r=>r.name)).size,5);const sum=c.records.reduce((a,r)=>a+r.bytes,0);assert.equal(sum,c.domain.bytes);assert.deepEqual(c.total,{bytes:sum+c.intrinsic.record.bytes+c.intrinsic.admission.bytes,slots:5+3+6,owners:5+3+6}); const oracle=v=>{const bytes=Buffer.from(v.hex,"hex");if(v.profile==="ui-value-tag"){if(bytes.length!==1||![1,2,5,6,7,12,16,18].includes(bytes[0]))throw Error("tag");return String(bytes[0]);}if(v.profile==="utf8-codepoint"){const decoded=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true}).decode(bytes);const points=Array.from(decoded);if(points.length!==1)throw Error("extent");return String(points[0].codePointAt(0));}if(v.profile==="ui-f64"){if(bytes.length!==8)throw Error("extent");const n=bytes.readDoubleLE();if(!Number.isFinite(n)||Object.is(n,-0))throw Error("domain");return String(n);}if(!bytes.length||bytes.length>10||(bytes[bytes.length-1]&128))throw Error("extent");const decoded=leb.decodeUIntBuffer(bytes,0);if(decoded.nextIndex!==bytes.length)throw Error("extent");const raw=Buffer.from(decoded.value);if(raw.length>8&&raw.subarray(8).some(x=>x!==0))throw Error("u64");const fixed=Buffer.alloc(8);raw.copy(fixed,0,0,8);const n=fixed.readBigUInt64LE();const encoded=Buffer.from(leb.encodeUIntBuffer(fixed));if(!encoded.equals(bytes))throw Error("canonical");if(v.profile==="natural-safe53"&&n>BigInt(Number.MAX_SAFE_INTEGER))throw Error("safe53");return String(n);}; let accepted=0,rejected=0;for(const v of f.vectors){let actual=null;try{actual=oracle(v);}catch{}assert.equal(actual,v.expected,v.id);v.accepted?accepted++:rejected++;} for(const w of f.windowPlans){const start=w.prefix.reduce((n,p)=>n+p.hex.length/2*p.repeat,0);assert.equal(start,255);assert(start+w.tail.hex.length/2>w.crosses);} const before={used:{bytes:0,slots:0,owners:0},held:false};const admitted=produce(before,d=>{for(const k of ["bytes","slots","owners"])d.used[k]+=c.total[k];d.held=true;});const faulted=produce(admitted,d=>{d.fault="exact-root";});assert.deepEqual(faulted.used,admitted.used);const closed=produce(admitted,d=>{d.held=false;for(const k of ["bytes","slots","owners"])d.used[k]-=c.total[k];});assert.deepEqual(closed,before);const census={strictAjv:2,recordCensus:c.records.length,vectors:f.vectors.length,accepted,rejected,plannedWindowShapes:f.windowPlans.length,immerChargeTransitions:3,runtimeDecoderExecuted:false};console.log("[DEBUG] "+JSON.stringify(census));'
```
