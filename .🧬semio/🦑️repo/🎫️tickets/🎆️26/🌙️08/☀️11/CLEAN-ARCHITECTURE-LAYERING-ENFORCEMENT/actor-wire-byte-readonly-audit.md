# Actor Wire Byte Boundary Read-Only Audit

Local source inspection only. No tests, runtime probes, GUI actions, or source edits. Paths below are relative to the repository root.

## Actual Carrier Evidence

- WIT owner `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:10` declares `pack = list<u8>`. Installed generated binding code `node_modules/@bytecodealliance/jco-transpile/vendor/wasm-tools.js:8552` lifts byte lists as `new Uint8Array(memory0.buffer.slice(ptr3, ptr3 + len3 * 1))`. The checked-in generated jcoprobe callback bundle also lifts bytes with this expression at line 8243. These inspected outputs establish Uint8Array with copied Wasm memory as an actual published carrier, not object/base64 shapes.
- Portable fixture `🎭️actor/🧪️tests/📨️effect-wire-routes/🟦️.ts:28–38` explicitly rehydrates `{base64}` fixture storage into Uint8Array before invoking wire decoders. Fixture base64 is not evidence for accepting strings at the live boundary.
- Actor component codec leaf `🎭️actor/📮️shard-client/🧬️component-codec/🟦️.ts:19–29` owns Uint8Array pack/spr/success/fault types; the fault branch at 61 also requires Uint8Array.
- The installed binding lowering path at wasm-tools.js:8923 accepts a buffer/view for its internal byte copy. That does not establish arbitrary `{kind:"bytes",value}` or `{data}` as lifted turn outputs.

## Confirmed Source Gaps

In `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts:30–46`, `coerceWireBytes` accepts unchecked number arrays, bytes/data wrapper objects, and base64 strings. Array conversion silently wraps/truncates negative, fractional, nonnumeric, and >255 values. No byte cap precedes array/string allocation. Its failure diagnostic stringifies the entire input before slicing, so diagnostic work itself is unbounded and can throw on cyclic objects.

Same-realm Uint8Array is returned by identity. Cross-realm Uint8Array is recognized with ArrayBuffer.isView plus its tag at line 32 and wrapped with a new view over the same buffer; that preserves byte region and aliases custody rather than copying bytes. It creates a new view identity. This is useful carrier recognition, but its required custody behavior needs an explicit contract, especially for shared or mutable backing buffers.

`coerceTurnResult:90–95` accepts private receipt bytes only via instanceof Uint8Array, so it rejects the cross-realm byte carrier that coerceWireBytes admits. Integration test `💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts:267–279` deliberately requires exact raw turn and receipt object identity and rejects arrays/string/wrappers. Do not silently copy receipt owners while resolving cross-realm recognition.

The function silently replaces malformed effects/uiPatches/presence fields with empty arrays, retains mutable input arrays/objects, and admits NaN/Infinity nextWake as numbers. WIT next-wake is option<u64> (plugin schema:1321); actual shard normalization must be checked before changing the renderer-facing field. WIT lifecycle/ui-patch receipts are typed records/variants (schema:661–669,1326–1327), while shard private receipt carriers are encoded bytes; these are distinct boundary types.

## Consumers and Legacy Reliance

All calls inside the actor wire-turn source were inventoried at lines 119,175,273,379,382,388,433,450,498,513,534,809,868,886. Two calls explicitly synthesize array fallback `[]` at 534 and 809; replace these with the canonical empty carrier when narrowing acceptance. Other inspected calls forward unknown WIT payloads and do not establish reliance on wrapper/base64 input.

The WGPU plugin bridge imports this source directly at line 102, invokes turn coercion at 246 and lifecycle routes 649–684, and converts an outgoing backbone payload to number[] at 816. ShellHost imports a separate PluginRuntime coerceWireBytes at 820 and uses it at 6932–6933. PluginRuntime retains a second implementation at 3648–3665 accepting arrays/objects/base64, without the cross-realm check. Its private turn coercion starts at 639. Consolidate these implementations through the owner facet; no inspected live producer proves object/base64 shapes are necessary. Complete external caller dependence remains an open probe, not an asserted absence.

## Facet Ownership Inventory

Existing actor package root `🎭️actor/📦️packages/🟦️typescript/🟦️.ts` exposes TurnScheduler and scheduler types only. Reexport existing wire-turn definitions through one explicit facet; do not duplicate them: WireVariant, WireUiPatch, WireTurnResult (54–64); TypedOperationToken/AckEvent/Page/PageScan (240–307); PatchOp and RetainedSurface (366–397); ShardTurnCarriers (571); InboundRequestDrive/Turn/Answer (716–735); SpawnedJobPort/Drive (782–789). Not every internal renderer tree type necessarily belongs in the public ABI.

Schema owners already exist for WIT actor/pack/effect/turn types, actor component codec requests/answers, lifetime/patch receipt codecs, retained turn and page interfaces. Public byte handling should reference those existing owners and separate encoded shard receipts from lifted WIT receipt records. The renderer-facing optional WireVariant/TurnResult types are handwritten interpretation types, not authoritative generated WIT declarations.

## Open Runtime Probes

Run real actor binding/shard output with nonempty pack and receipts; verify constructor/realm, byteOffset/length, identity after bridge transport, and lifetime through subsequent Wasm calls. Test invalid/fractional/oversized carriers before allocating, cross-realm receipt acceptance without custody substitution, shared/detached buffers, nonfinite/u64 wake handling, and malformed turn rejection. No claim here that these probes passed.

## Extended Repository Callsite Inventory

An `rg -a` inventory over repository TS/TSX/JS/MJS, excluding dependencies, tickets, generated output and cache, found the following direct symbol sites. No named import alias or `.coerceWireBytes`/literal computed-member use was found in that inspected source scope. Dynamic property names, reflection, ignored build output and non-source consumers remain outside this textual proof.

| Owner | Direct input consumers | Input contract |
| --- | --- | --- |
| Actor wire-turn | 119 shell payload; 175 leftover frame payload; 273 typed-operation page payload; 379/382 patch node; 388 patch props; 433 extension payload; 450 answer payload; 498 export items; 513 video program; 534 job input; 809 job completion value; 868 effect pack field; 886 effect params pack | Raw WIT variant/pack boundary, except explicitly synthesized `[]` at 534/809. Fixtures rehydrate bytes before these calls. |
| WGPU plugin bridge | import 102, call 816 | Raw WIT send-message payload becomes `Array.from(...)` for first-party friendly Effect. This outgoing array is intentional. |
| PluginRuntime | 663 command-ingress fault; 928 decodeWirePack; 1049 load-document pack/spr; 2989 reserved-job input; 3032 spawn-job input; 3615 presence update; 4203 decoded foreign-step payload | Mixed ownership: raw WIT in fault, patches/effects, spawn jobs and presence; nested pack value at 4203 requires the pack codec's byte representation contract. |
| ShellHost | import 820; calls 6932/6933 | **Friendly** `effect.loadDocument.pack/spr`, declared readonly number[] by Kernel Effect. These arrays are required by the current first-party API, not a legacy jco shape. |
| PluginRuntime test surface | export object 4573; test destructure 36 | Test exposure of existing function, no direct symbol invocation found in the inspected test file. |

The inventory's full PluginRuntime source path is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`; WGPU and ShellHost paths are under that same engine as identified above. The root search found no additional direct coerceWireBytes symbol sites in S/Hub or other domains. This is a bounded textual inventory, not a runtime call graph.

PluginRuntime `decodeWirePack` callers are patch fields 879,882,885,888,896,899,902,905 and effect/params helper paths 1036/1043. Its direct coercer at 1049 projects raw WIT docPack/spr into friendly number arrays. Reserved job handlers at 2989/3032 read `WireVariant` spawn-job fields. Presence at 3615 decodes packed update bytes. `decodeForeignStep:4193–4205` first decodes an enclosing pack, then converts the nested payload; verify actual pack-value byte projection before narrowing that nested conversion.

Kernel `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` declares intentional friendly array byte fields: loadDocument pack/spr (1193), sendMessage/publishEvent payload (1249–1250), blobWrite bytes (1251), httpRequest body (1253), documentWrite ops (1255), cacheDerive input (1259), storageWrite bytes (1276). SpawnJob input is explicitly Uint8Array (1268). Therefore removal of arbitrary number arrays at the raw WIT ingress must preserve separately validated conversion of the existing friendly array API; a blanket “all arrays are legacy” change would break ShellHost.

The previous consolidation recommendation means consolidate **the raw WIT rule** through its existing owner. It does not justify routing friendly KernelEffect arrays through a strict raw-jco-only validator. Handcraft the two `[]` fallbacks and any raw-wire test factories to Uint8Array; retain fixture `{base64}` storage rehydration, which already feeds the canonical runtime carrier.

### Cross-Realm Custody

After reliable cross-realm Uint8Array recognition, returning the actual `raw` view (with a TypeScript narrowing assertion) preserves original view identity, buffer identity, byteOffset and byteLength. Constructing `new Uint8Array(raw.buffer,...)` preserves the byte region but changes view identity. Where receipts bind exact owner identity, accept only the actual recognized view and preserve it; do not normalize it into a second view or copy. Shared backing memory, detached/resizable buffer validity and descriptor access must be assessed under the existing ingress/custody contract rather than guessed from `instanceof` or a spoofable display tag. Runtime realm/custody probes remain pending.
