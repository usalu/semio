# Native Resident Composition Extraction

## Actual Capacity Foundation Checkpoint (2026-08-28)

Taxonomy released the canonical package metadata. The unchanged four tests first produced the intended single missing-API E0432. The sole `resident/🦀️.rs` then gained checked private resource/capacity values with no heap, Default, global ledger or permit. The canonical native run passed4/4,0skipped,.013s; both existing wasm32-wasip2 and wasm32-unknown-unknown checks passed (.79s/.51s). Reports `📓️native-resident-capacity-r1-red-2026-08-28.md`, `📓️native-resident-capacity-r2-green-2026-08-28.md` and `📓️native-resident-wasm-r3-2026-08-28.md` preserve the exact commands and actual output. This is scalar capacity vocabulary and constructor allocation evidence only; registry admission, UI parent binding and guest initialization remain unmounted.

## Current Evidence

The actual registry constructor RED allocated 1,024 initialized slots and 4,202,496 backing bytes before admission. Its production constructor remains unchanged. Construction ordering is separately GREEN2: quarantine registration now precedes removal of the original live instance root. Neither result proves a native composition permit or final callback-shell retirement.

The canonical shared contract is `framework/🔨modules/🌱value/💾resident/{🧬schema.json,🧬contract.json}`. Its explicit safe-integer bytes/slots/owners capacity has two disjoint partitions: data equals total minus control, and control equals the declared control reserve. The TS record prices are declared logical envelopes, not native physical sizes. Its new exact-domain-record subsection is being implemented by the UI owner; no Rust duplicate of that contract is proposed.

The complete native UI resident source and the publication owner's `📓ui-resident-authority-composition-handoff-2026-08-27.md` were read. It has one process-static ledger with 64 epoch slots, root/output bits 1/2, deferred atomic returns, 8 MiB surface and 32 MiB aggregate byte limits. Existing contract and runtime static backing is already included. The last paired owner, not publication or cancellation alone, releases a reservation. This is an actual UI policy and ownership implementation, not a general composition permit.

## One Rust Authority And Crate Boundary

The canonical native source is `value/💾resident/🦀.rs`, exposed once by a small `value/💾resident/📦packages/🦀rust` package with Cargo `[lib].path = "../../🦀️.rs"`. Tests live in `resident/🧪tests/🦀.rs` and read the shared fixture one directory above. Taxonomy corrected the earlier provisional `🦀component.rs` and `🧪.rs` spellings and three-level relative path before any native compilation: two levels reach resident, while three incorrectly reach its parent value domain. Both superseded files were moved, with no compatibility copy or glue. It must have no external runtime dependencies. Native fixture parsing uses existing serde_json as a dev dependency. Package, task router and launch registration are taxonomy-owned.

UI-contract cannot depend on the top-level semio-framework because that crate already depends on UI-contract. Compiling the same source independently through several path modules would create different authority types and potentially separate statics. Neither route is acceptable. UI-contract, common Kernel and Plugin will refer to the same package identity; no forwarding ledger or second allocation pool is introduced.

The first native packet is deliberately the allocation-free capacity vocabulary and checked partition arithmetic, using the existing canonical capacity fixture and safe maximum. It is not a permit, allocator wrapper or proof of registered domain ownership. Native admission/retirement is mounted only after its exact original-owner roster and constructor failure laws execute.

## UI Policy Reuse Without Double Spending

One native composition supplies the total capacity explicitly. There is no default derived from Kernel Budget, maxPatchBytes, declarative QuotaSchema, a wire length, or the UI constants. Composition creation is an explicit host/guest construction prerequisite; it is not a new WIT export or a fabricated Open identity.

Before UI can admit a document, the composition reserves one exact UI domain envelope containing the unchanged 32 MiB aggregate maximum, the physically measured fixed metadata outside that maximum if any, and the declared slot/owner envelope. The original UI ledger owns this parent record structurally. Its existing per-surface 8 MiB check, aggregate item policy, epoch safety and root/output pairing become subdivision constraints inside that one admitted domain. They do not claim another 32 MiB from process capacity. The parent reservation cannot be returned while any UI slot, deferred return, document alias, output obligation or registered static backing remains owned.

Existing fixed backing counted within the 32 MiB UI snapshot remains within that envelope; it is not charged again as dynamic payload. Any newly added neutral record/header outside the old physical structure is separately measured and preadmitted. Metadata-initialization work is charged against the caller's step grant even when resident capacity has already been reserved. The current UI ledger is therefore reused as domain policy and exact descendant ownership, not copied or renamed into an independent neutral budget.

Other native domains, including the three Plugin registries and their Opening records, require separate parent reservations from the same composition. UI cannot lend its quota to them. The composition's declared data/control partitions remain disjoint, and control capacity cannot grow through data cancellation. No capacity is raised by this extraction.

## Structural Native Admission

An empty runtime registry allocates no directory or payload pages. Logical capacity remains 1,024. Its composition-owned record and original Opening slot precede all allocation and app construction. Directory admission, actual allocation-capacity observation, one-slot/page initialization, payload construction and final slot placement are separate retained phases. Placement refuses before taking the source. An allocator returning more capacity than requested cannot silently increase spend: the actual backing remains in the original construction slot, with publication refused and exact retirement still required.

The native record must retain exact original typed construction ownership across ordinary error and unwind. A generic `T: Drop`, callback-shaped terminal predicate, public subtraction, or `ManuallyDrop` with no recoverable parent is not sufficient. The concrete runtime owns typed registry/Opening/close roots; the neutral capability accounts and registers that exact domain envelope. Domain capability access remains private to its concrete owner. An intrinsic record-detachment observation is not a guest lifetime, native patch ACK or domain terminal witness.

Release follows actual typed descendant retirement and final physical backing release under the caller grant. A cancellation request never refunds an allocated or posted root. Parent and child finalization cannot both consume the same last item/byte grant. The final callback clock includes final owner release; a late fault retains the already-staged exact completion/ACK handoff, not a fabricated empty owner.

## Test Sequence And Scope

1. Reuse the canonical invalid-capacity and overflow fixtures in native capacity tests. Validate the same fixtures with strict Ajv and independently calculate partitions/admission with BigInt or an existing arbitrary-precision test library. Include each axis, exact safe maximum, zero, control equal total, and control greater than total.
2. Native constructor tests prove no heap directory or child slot is created by the empty capacity/registry shell. Measure actual header, slot and capacity footprints independently from logical counts. Cross-width compilation remains a separate gate.
3. Native parent/record tests exercise refused capacity before constructor entry, exact original registration before a real constructor mutation and panic, cancellation before/after allocation, actual oversized allocation retention, foreign/replayed detach refusal and zero/short final grants. A plain caught panic after completed setup does not count as constructor-unwind evidence.
4. Extract the UI parent record while retaining its existing paired-owner/deferred-return tests. Add simultaneous UI plus registry refusal and final-child/parent ordering laws against the single composition capacity.
5. Join the same registry and original Opening record to real Open, then run the existing registry RED, construction2 and lifecycle/close tests without prewarming or raising the strict 8 ms ceiling. Callback-tail quiescence and guest descendant aggregation remain separate required owners.

## Status

This is a source-grounded extraction/API plan, not native admission implementation. Root approved the canonical package path and assigned its metadata to taxonomy. The domain source currently mounts only four missing-API native tests: shared vectors, per-axis overflow, disjoint partitions, and a thread-local System allocator observation around construction. No production ledger, UI permit or Open producer has been mounted. The sole compiler has released the producer4 snapshot (actual3PASS/1 intended pool-admission RED); Rust runs remain serialized through the publication owner. All earlier logs and the current registry RED are preserved.

## Actual Capacity Source Audit

The production `plugin_exports!` macro constructs persistent `__SEMIO_PLUGIN_RUNTIME` with parameterless `PluginRuntime::new`; `extension_exports!` does the same for `__SEMIO_EXTENSION_RUNTIME`. The bundle initialization Once supplies no resident capacity. The owned poll export decodes events/command_page/Budget and then enters that same runtime; it supplies no native composition resources.

The host has two real numeric memory limits: `SharedEngineConfig::max_memory_bytes` and `BudgetLimiter::max_memory_bytes`, both currently defaulting to 512 MiB. The former configures Wasmtime pooling memory size/virtual address reservation; the latter's `memory_growing` compares one requested linear-memory extent against its bound. Neither registers the native allocator's individual roots, supplies slots/owners/control resources, or reserves concurrent Plugin and UI backing. They are therefore not an existing composition capacity source and are not converted into one here. `QuotaSchema` and Kernel Budget remain non-authority as already noted.

The actual explicit composition caller is absent in the inspected native/owned guest entry points. The full integration must introduce a required, schema-owned composition construction input through the existing host/guest initialization contract, with all authored callers updated; it must not choose a hidden total by summing old maxima or reuse a per-memory limit as available resident credit. The minimal input field/API contract must be agreed with the host owner before that ABI cutover.

## Independent Capacity Oracle

Canonical read-only invocation: `bun x nx exec --projects=workspace -- bun -e <quoted expression>`. Actual session68732 exited0:

```text
[DEBUG] Resident capacity strict Ajv/contract invalid6 PASS; Lodash partitions and overflow PASS; independent BigInt boundary matrix=25 PASS; no native permit or constructor execution.
```

Strict Ajv validates the current shared capacity schema, and Lodash checks per-axis control containment, partition subtraction and the declared overflow refusal. Twenty-five boundary pairs independently compare Lodash's numeric comparison/subtraction against exact BigInt arithmetic. This proves the fixture/arithmetic oracle only; native RED execution is still queued behind taxonomy's package registration.

Exact evaluated expression:

```javascript
import Ajv from "ajv"; import _ from "lodash"; import assert from "node:assert/strict"; const p="🧰️framework/🔨️modules/🌱️value/💾️resident/"; const f=await Bun.file(p+"🧪️fixture.json").json(); const s=await Bun.file(p+"🧬️schema.json").json(); const ajv=new Ajv({strict:true,allErrors:true}); const valid=ajv.compile(s); const axes=["bytes","slots","owners"]; const accepts=c=>valid(c)&&_.every(axes,k=>_.lte(c.control[k],c[k])); assert(accepts(f.capacity)); for(const v of f.invalidCapacities)assert.equal(accepts(v.value),false,v.name); const data=_.zipWith(axes.map(k=>f.capacity[k]),axes.map(k=>f.capacity.control[k]),_.subtract); assert.deepEqual(data,[14336,56,28]); assert.equal(_.lte(f.overflow.request,_.subtract(f.overflow.capacity,f.overflow.used)),f.overflow.accepted); const maximum=9007199254740991; let matrix=0; for(const total of [0,1,2,maximum-1,maximum])for(const control of [0,1,2,maximum-1,maximum]){const actual=_.lte(control,total); assert.equal(actual,BigInt(control)<=BigInt(total));if(actual)assert.equal(BigInt(_.subtract(total,control)),BigInt(total)-BigInt(control));matrix++;}console.log("[DEBUG] Resident capacity strict Ajv/contract invalid6 PASS; Lodash partitions and overflow PASS; independent BigInt boundary matrix="+matrix+" PASS; no native permit or constructor execution.");
```
