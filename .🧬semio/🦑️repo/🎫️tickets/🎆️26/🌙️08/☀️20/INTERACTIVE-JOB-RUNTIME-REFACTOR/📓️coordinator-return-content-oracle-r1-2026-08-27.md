# Coordinator Canonical Return Content Oracle R1

## Actual Format Gate

Exit0. Strict Ajv validated both declaration and fixture schemas. Independent webassemblyjs ULEB plus Node Buffer matched six record frames, three scalar UI operations, and the exact11-byte Unicode surface. Ordered10record/11opcode declarations were checked. Thirteen total direct checks; no mounted parser, native encoder, source ownership or8ms certificate is inferred.

```text
[DEBUG] return-content-declaration-oracle {"checks":13,"schemas":2,"recordFrames":6,"scalarOperations":3,"utf8SurfaceBytes":11,"presenceDeclared":"presence","scope":"schema/declaration/format only; no mounted parser or native encoder proof"}
```

All four captured content files were stable:

```text
8dc9bfde06eb9f0ba89154a10ced8a9cd199d111abbac3284cca12a9f87baeca  🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧬️wire.json
e364d3d8f488dd4bfeb47fb6a60bcc952f7d104e5c92506d7be6453fd7f1a283  🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧬️schema.json
aeef90fd6e8379c50bd7bdf02f7e6a14c6ebdf7c826ad993d5ac169475ccec44  🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧪️fixture.json
b73e18d7f3a7aa16113f686987c558f509354e677002ebc9094a5da05544b38f  🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧪️schema.json
```

## Source Review and Assigned Refinement

Read all four content files and full canonical report. The versioned length-delimited section order includes render-plane presence and preserves per-operation PACK fields, direct u64 children, and exact original issued UI receipt. Records may cross pages; independent semantic storage must be admitted and populated before raw input ACK, never borrowing the previous page while waiting for another.

The global `pack` primitive wording still described all values as generic wire-value documents. Actual Plugin reactor constructs `Effect::SendMessage` from `protocol::encode_app_frame` and renderer exchange bytes, forwarding payload unchanged. The native owner is refining this to opaque exact bytes with per-field semantic dialect and adding actual Invocation/presence vectors. This oracle does not resolve that semantic gap. Demonstrator owns outer TS framing; UI owns typed operation and independently retained payload storage. No whole-result compatibility path is approved.

## Reproducible Read-Only Command

```sh
SEMIO_RETURN_CONTENT_ORACLE_EVAL='(async()=>{const {readFileSync}=await import("node:fs");const {Buffer}=await import("node:buffer");const {default:Ajv}=await import("ajv");const {strictEqual,deepStrictEqual}=await import("node:assert");const base="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/";const read=p=>JSON.parse(readFileSync(base+p,"utf8"));const content="🎠️kernel/📤️return/📦️content/";const declaration=read(content+"🧬️wire.json"),fixture=read(content+"🧪️fixture.json");const ajv=new Ajv({strict:true,allErrors:true}).addSchema(read("🎭️actor/📄️page/🧬️schema.json")).addSchema(read("🎭️actor/🚪️lifetime/🧬️schema.json")).addSchema(read("🎭️actor/🚪️lifetime/🩹️patch/🧬️schema.json"));for(const pair of [[content+"🧬️schema.json",declaration],[content+"🧪️schema.json",fixture]]){const v=ajv.compile(read(pair[0]));strictEqual(v(pair[1]),true,JSON.stringify(v.errors));}const lib=await import("@webassemblyjs/leb128/lib/leb.js");const encode=(lib.default??lib).encodeUIntBuffer;const u=v=>{const b=Buffer.alloc(8);b.writeBigUInt64LE(BigInt(v));return Buffer.from(encode(b));};const b=v=>Buffer.from([v]);const cat=a=>Buffer.concat(a);let checks=0;const m=fixture.metadata;const metadata=cat([b(declaration.records[0].statusTags[m.status]),m.nextWake===null?b(0):cat([b(1),u(m.nextWake)]),u(m.fuelUsed),u(m.effectCount),u(m.presenceCount)]);strictEqual(metadata.toString("hex"),fixture.recordVectors[0].bodyHex);checks++;const begin=fixture.uiBegin,life=begin.receipt.lifetime,surface=Buffer.from(begin.surface);const body=cat([u(life.activationGeneration),u(life.instanceId),u(life.guestLifetime),u(begin.receipt.patchSequence),u(surface.length),surface,u(begin.baseRevision),u(begin.revision),u(begin.operationCount)]);strictEqual(body.toString("hex"),fixture.recordVectors[1].bodyHex);checks++;for(const row of fixture.recordVectors){const body=Buffer.from(row.bodyHex,"hex");strictEqual(cat([b(row.tag),u(body.length),body]).toString("hex"),row.frameHex);checks++;}for(const row of fixture.scalarOperationVectors){const chunks=[b(row.opcode),u(row.node)];if(row.children)chunks.push(u(row.children.length),...row.children.map(u));strictEqual(cat(chunks).toString("hex"),row.hex);checks++;}deepStrictEqual(declaration.records.map(x=>x.tag),[0,1,2,3,4,5,6,7,8,9]);deepStrictEqual(declaration.uiOperations.map(x=>x.opcode),[0,1,2,3,4,5,6,7,8,9,10]);checks+=2;console.log("[DEBUG] return-content-declaration-oracle "+JSON.stringify({checks,schemas:2,recordFrames:fixture.recordVectors.length,scalarOperations:fixture.scalarOperationVectors.length,utf8SurfaceBytes:surface.length,presenceDeclared:declaration.records[6].name,scope:"schema/declaration/format only; no mounted parser or native encoder proof"}));})()' NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_RETURN_CONTENT_ORACLE_EVAL)'
```

