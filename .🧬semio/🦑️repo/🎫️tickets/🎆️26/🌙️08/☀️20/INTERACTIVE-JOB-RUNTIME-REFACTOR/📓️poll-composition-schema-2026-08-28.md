# Required Poll Composition Schema

## Canonical Packet

The packet is `🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/{🧬️schema.json,🧬️contract.json,🧪️fixture.json,🧪️schema.json}`. Its schema is a projection of the single required existing-poll field, referencing `semio.value.resident.capacity.v1`. It is not a new full poll envelope, return drive variant, initialization export or parallel capacity schema.

The exact nested field is `composition: {bytes, slots, owners, control: {bytes, slots, owners}}`. All six values are checked integers in0..9007199254740991. WIT field order is bytes, slots, owners, control.bytes, control.slots, control.owners; WIT values are u64 and TS mapping uses checked BigInt. Owned JSON keeps the same nested names and numeric values: the required `owned_abi::PollInput.composition` has no serde default, alias, alternate envelope or string-count path. The actual existing events, command_page and budget members are unchanged by this projection. The canonical return-drive cutover remains separately required; this packet does not invent a temporary old/new union.

Control must not exceed total on any axis, and data is the exact difference. Standard JSON Schema validates the scalar shape/range; native/TS checked construction additionally enforces that relational rule. The packet includes three relational refusal vectors rather than claiming the shape validator checks cross-field arithmetic.

## Original Owner And Close Laws

Configuration equality is necessary but never sufficient authority. The host retains the original private admitted guest-domain owner and exact activation before instantiation. The guest resumes its original composition/Opening record on an exact retry, retaining partial construction. Foreign activation, equal-capacity foreign owner, changed configuration and pre-install cleanup refuse without replacing that original owner. Closing refuses new work but preserves exact already-issued cleanup under the frozen original configuration. The ten declarative binding cases are a model, not private native ownership evidence.

Return origin, drive discriminators, fixed result binary, lifecycle semantic request sequence and UI patch receipt stay unchanged. No capacity is derived from Budget, a memory ceiling, app id or equality of another runtime's numbers. Actual bootstrap allocation, all producer-export guards, bundle construction and parent UI reservation remain unmounted.

## Executed Independent Oracle

R1 was an Nx nested-shell quoting failure before parsing/tests: the `$id` property expression was shell-expanded. No semantic failure is attributed to it. R2 used an explicit literal schema id and canonical `bun x nx exec --projects=workspace -- bun -e <quoted expression>`; session25775 exited0:

```text
[DEBUG] Poll composition strict Ajv valid3/invalid8, Buffer six-u64 order3, BigInt partition3 and declarative binding-model10 PASS; native ownership and ABI not mounted.
```

Strict Ajv validates the fixture and its positive/negative shape vectors. Lodash resolves each declared field path; Node Buffer independently writes/reads each six-word little-endian layout against fixture decimal words. This48-byte Buffer is an oracle for field order and numeric preservation, **not a newly specified binary poll encoding**. BigInt checks the three control-over-total refusals. The binding model checks declared outcomes only; no callback, allocation, private identity or runtime execution is inferred.

Exact evaluated expression:

```javascript
import Ajv from "ajv"; import _ from "lodash"; import { Buffer } from "node:buffer"; const root=process.cwd(); const path="🧰️framework/🔨️modules/🎠️kernel/📥️poll/🏘️composition/"; const read=async p=>await Bun.file(root+"/"+p).json(); const capacity=await read("🧰️framework/🔨️modules/🌱️value/💾️resident/🧬️schema.json"); const schema=await read(path+"🧬️schema.json"); const fs=await read(path+"🧪️schema.json"); const f=await read(path+"🧪️fixture.json"); const contract=await read(path+"🧬️contract.json"); const ajv=new Ajv({strict:true}); ajv.addSchema(capacity); ajv.addSchema(schema); const validate=ajv.compile(fs); if(!validate(f))throw new Error(ajv.errorsText(validate.errors)); const check=ajv.getSchema("semio.kernel.poll.composition.v1"); for(const row of f.valid){if(!check(row.input))throw new Error(row.name); const bytes=Buffer.alloc(48); contract.wireOrder.forEach((path,index)=>bytes.writeBigUInt64LE(BigInt(_.get(row.input.composition,path)),index*8)); for(let index=0;index<6;index++)if(bytes.readBigUInt64LE(index*8)!==BigInt(row.words[index]))throw new Error("word "+row.name); } for(const row of f.invalid)if(check(row.input))throw new Error("accepted "+row.name); for(const c of f.partitionRefusals)if(["bytes","slots","owners"].every(axis=>BigInt(c.control[axis])<=BigInt(c[axis])))throw new Error("partition"); for(const row of f.bindingCases){let actual="refuse-retain-original"; if(row.sameActivation&&row.sameOwner&&row.sameConfiguration){if(row.phase==="empty"&&!row.cleanup)actual="initialize-original"; if(row.phase==="opening"&&!row.cleanup)actual="resume-original"; if(row.phase==="ready")actual="use-original"; if(row.phase==="closing"&&row.cleanup)actual="drain-original";} if(actual!==row.expected)throw new Error("binding "+row.name);} console.log("[DEBUG] Poll composition strict Ajv valid3/invalid8, Buffer six-u64 order3, BigInt partition3 and declarative binding-model10 PASS; native ownership and ABI not mounted.");
```

Status: schema/neutral packet ready for root review and peer mapping TDD. No production WIT, owned JSON, guest installer or host admission code changed.

## Admitted Opening Cancellation Refinement

Root review found that the first binding model refused cleanup even when an expensive partial Opening already had its exact admitted owner. The added eleventh case requires `close-original-opening`. Running the unchanged ten-case model against it produced actual RED at `binding partial-opening-cancellation` (session13861, exit1). The corrected model maps exact Opening cleanup to that transition; session6299 exited0 with the same numeric/schema checks and `declarative binding-model11 PASS`.

The declaration uses the existing `returnDrive.control.cancel` and its original admitted return identity. The Opening source must publish the existing pending result with that identity before expensive construction so cancellation can address it. This transitions the original retained Opening into Closing without replacing or detaching partial descendants. Truly unbound control-before-install still refuses. No new wire tag, synthetic owner, result variant or private-binding implementation is added. Exact original cancellation/close authority must be checked in the future native mounting; the boolean model is not that authority.

The oracle's only model change was replacing `if(row.phase==="opening"&&!row.cleanup)actual="resume-original"` with `if(row.phase==="opening")actual=row.cleanup?"close-original-opening":"resume-original"`; the output count changed10→11. The original R2 expression and output above are preserved as historical evidence.
