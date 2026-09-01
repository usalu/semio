# Scalar Declaration Final Close and Controller Release R194–R199

## Frozen Result

R199 executes all four declaration controllers successfully against seven unchanged pre/post input hashes. Raw output, full exact controller source and hash manifest are in `🧪️renderer-scalar-final-r199-2026-08-28.{txt,json}`. This is **not** a runtime decoder/reader/ledger gate.

-43 preserved scalar values:25 accepted,18 rejected, independently checked with leb128/Buffer/TextDecoder.
-8 desired fixture negatives plus u64MAX+1/leading-zero negatives reject. Unique projected IDs are semantic validation; strict JSON Schema enforces accepted↔expected and required profiles.
-91 closed ownership traces,2238 literal state/result rows,2238 matching Immer transitions.
-40 exact original child-close suffix sums.
-14 decoder construction prefixes,114 declared retirement steps,228 before/after fault checks and228 zero/short-grant checks.
-7 maintenance result/over-grant cases.
-16 original reader construction/binding/publication phases with binding before finalizer.

R194 first executed91 traces/2238 rows. R195 was an inline quoting error before the close arithmetic controller ran; R196 executed that controller successfully. R197 passed the combined packet. R198 reran after whitespace-only compact JSON formatting. R199 added an explicit ordering assertion after correcting an ambiguity in the initial proposal's separate reader-binding list: source receipt binding must occur **inside** reader construction, before its finalizer, not after the thirteen old reader phases. All earlier outputs remain retained; none is represented as a new runtime RED.

## Exact Construction Order

The original decoder has13 phases through strong roster installation, not public reader exposure. Its full domain432 is reserved before state allocation. State is linked in original pending.entry and payload.scalar before facade construction. The facade is installed in that state before record.install; progress, receipt and the same original witness are installed in their state fields before any finalizer.

The receipt initially owns the original decoder with reader=null. Only after all original state/cell/record/witness roots are retained on payload.scalar may the existing pending slot be reused for reader admission. The actual proposed reader sequence is:

```text
0..5  cell prepare/observe/claim/observe, record reserve/observe
6     reader state, including the two separately priced consumer/receipt pointers
7     original reader shell install
8     original reader witness install
9..10 original builder binding and observation
11    exact source receipt.reader binding (64)
12    exact private original binding observation (64)
13    reader finalizer (64)
14    reader publication (64)
15    scalar publication (64)
```

Every possible finalizer has an already installed original parent route: pending slot before state, payload.scalar after state, state.facade/progress/receipt/witness before their finalizers, and payload.reader plus original pending reader entry before its finalizer. Before/after thrown roots are retained in that original charged cell or the existing parent failure slot, never substituted by a string or a newly allocated retry. These are declared construction requirements and closed-model invariants, **not executed private runtime constructor laws**. There is no second record, unchecked public factory or optional unowned read path.

The exact mandatory callables and per-byte grants remain those in R187–R193. The final JSON `readerAdmissionOrder` is authoritative for ordering. Proposed charges remain decoder992/14/14, payload+8 and reader+16; none is mounted.

## Close Grants and Prefix Sums

| Scope | Turns | Logical work bytes |
| --- | ---: | ---: |
| Decoder-owned close phases |22|2048|
| Original reader ready: decoder + reader close |37|3440|
| Additional original live intrinsic read alias |5|624|
| Separate original parent page close |17|2144|

The22 decoder phases comprise9 receipt/consumer cancellation and detachment turns, then13 original-reader observation/domain/registration turns. All are64-byte turns except fixed body clearing272, record close264 and admission close296. The exact arrays are in `closePlan.decoderOwned`, not an opaque cap.

Original reader15-phase close contributes1392 bytes. Its capture turn precedes the optional five alias-close turns; alias retirement precedes its body proof. The reader's published current domain720 becomes proposed736 only through separately priced pointer words; these new words detach in the earlier consumer handshake. Existing child close work grants are not silently increased or counted twice.

The separate17-phase page close contributes2144 bytes only when that original payload page actually needs retirement. It is **not** a prerequisite fabricated by the decoder's own witness or proof that the builder/source/evidence has all retired. Its original binding guards still apply. Source/backpressure/refusal/fault prevents claiming a finite completion bound.

The source reader/page fixture paths and SHA256s are recorded with all40 suffix rows. The controller reads those actual fixtures and checks every suffix's remaining turns and byte sum. Decoder construction prefix0..13 rows distinguish unclaimed bootstrap, claimed cell, reserved unused record, installed state, shell and receipt; `readerWasNeverAdmitted` comes only from this declared original phase frontier, never a negative brand result. Before a scalar shell exists, original neutral capability retirement is separate from later typed witness retirement.

The992 decoder refund is688 at record retirement,8 at the resource-result alias and296 at cell retirement; slots/owners sum to14/14. The separately admitted original payload remains880 proposed bytes throughout this child-close model. An after-cell-refund exception therefore stays under that parent's existing failure/pending owner; it is not assigned to a refunded decoder allocation. A throw outside the canonical parent driver remains caller-owned until exact handoff. The model does not claim generic arbitrary-root disposal, physical heap accounting or whole-instance terminal authority.

## Reproducible Existing Route and Permanent Controller Proposal

The existing registered Nx target is `@semio-tech/framework-renderer-react:test-long`. Its actual `📋️project.json` calls the current `📜️script.ts test long` router, which calls the registered Vitest config. No new script, task, runtime dependency or test budget is needed.

After taxonomy admits the proposed `retained/💾️resident/🔢️scalar` schema/fixture domain, the permanent test-only controller belongs in UiDocumentStore's existing TypedWire region:

```text
OwnedResidentScalarDeclaration
  strict Ajv for frozen contract/fixture
  semantic exact-ID uniqueness, profile coverage and accepted/result equivalence
  original43 oracle values, declared field/record arithmetic
OwnedResidentScalarReceiptModel
  finite original trace rows, literal states and Immer parity
  mandatory identity/receipt/serial/cursor/latch/settle/fault laws
OwnedResidentScalarCloseModel
  actual imported child phase lists and hashes/shape
  finite suffix/prefix arrays, short grants and exact retained-fault identity
```

Proposed test-only helpers are `validateScalarDeclarationCold`, `runScalarReceiptModelCold` and `runScalarCloseModelCold`; none is exported into runtime. Loops are bounded by frozen vector/trace/phase arrays, not an increased opaque iteration limit. Scalar arithmetic remains a separate implementation parity stage.

The future selector is:

```sh
bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedResidentScalar'
```

Those tests are **not collected yet**. A fresh exact scan found no scalar-filter launch row (nor an exact renderer test-long command row) in current .vscode/launch.json. Before canonical test release, taxonomy must admit a generated seed entry for this existing Nx route, proposed name `⚖️gate🖱️ui💾️resident🔢️scalar`, group4_gate with taxonomy-assigned order. No output-only launch row, guessed ordinal or ticket-file imports will be added to permanent tests. This report requests coordination, not permission to create a competing canonical leaf.

For immediate reproduction of the frozen ticket diagnostic, the manifest contains the complete four bounded controller programs and their exact inputs. A read-only replay that refuses source drift is:

```sh
bun -e 'const r=await Bun.file(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️renderer-scalar-final-r199-2026-08-28.json").json();for(const[p,h]of Object.entries(r.after)){if(Bun.SHA256.hash(new Uint8Array(await Bun.file(p).arrayBuffer()),"hex")!==h)throw Error("Frozen input changed: "+p);}for(const x of r.results){const p=Bun.spawn(["bun","-e",x.program],{stdout:"inherit",stderr:"inherit"});if(await p.exited)process.exit(1);}'
```

## Source Boundary

Only ticket JSON/reports/logs changed during this hardening slice. UiDocumentStore remains SHA256100dae341b4112c50683105637c043a14c0149158366fc61bd1320343e34f435 from the735-test checkpoint. The latest actor caller work is separate. No canonical scalar file, imported production module, payload/reader price, native lease, live semantic parser, output publication or ACK was changed or inferred. Current proposal files are frozen for root review.

