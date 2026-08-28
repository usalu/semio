# Borrowed Return Message Record

The borrowed native Effect::SendMessage record encoder is implemented after its actual missing-API RED, and the combined native return-content gate now passes all seven tests. It is not a live return owner or retirement witness. The canonical content grammar remains record tag 5, effect tag 0, exact endpoint tag and fields, then opaque length-delimited payload bytes. The source Effect stays borrowed from its original structural owner; no cloned payload, whole PACK parse or temporary output allocation is permitted.

The permanent language-neutral packet is `🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/💌️message/{🧪️fixture.json,🧪️schema.json}`. Its five endpoint vectors cover numeric Shell/PluginInstance, multibyte Backbone/Extension and empty Topic. Its 8193-byte vector forces more than two raw output pages. `🧪️component.rs` has three tests for exact 1/64/4096 output, zero grants, original allocation preservation, partial-cursor cancellation and bounded invalid numeric endpoint rejection. At the initial oracle checkpoint these tests were unmounted; the subsequent native RED and implementation sequence is recorded below.

The independent oracle ran through Bun and the existing workspace Nx exec target, using strict Ajv, Node Buffer UTF-8 and the installed webassemblyjs LEB128 implementation. Actual process 40840 exited 0 with:

```text
[DEBUG] strict Ajv + independent NodeBuffer/webassemblyjs message endpoints5/large-prefix1 PASS; no native cursor execution
```

Exact evaluation source (environment variable `SEMIO_X_EVAL`; invocation `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_X_EVAL)'`):

```javascript
(async()=>{const{readFileSync}=await import("node:fs");const{default:Ajv}=await import("ajv");const{default:leb}=await import("@webassemblyjs/leb128/lib/leb.js");const root="🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/💌️message/";const f=JSON.parse(readFileSync(root+"🧪️fixture.json","utf8"));const schema=JSON.parse(readFileSync(root+"🧪️schema.json","utf8"));const validate=new Ajv({strict:true}).compile(schema);if(!validate(f))throw Error(JSON.stringify(validate.errors));const u=n=>{const b=Buffer.alloc(8);b.writeBigUInt64LE(BigInt(n));return Buffer.from(leb.encodeUIntBuffer(b));};const text=s=>{const b=Buffer.from(s,"utf8");return Buffer.concat([u(b.length),b]);};const payload=Buffer.from(f.payloadHex,"hex");for(const row of f.vectors){const e=row.endpoint;let endpoint;switch(e.kind){case"shell":endpoint=Buffer.concat([Buffer.from([0]),u(e.instance)]);break;case"backbone":endpoint=Buffer.concat([Buffer.from([1]),text(e.uri)]);break;case"pluginInstance":endpoint=Buffer.concat([Buffer.from([2]),u(e.id)]);break;case"extension":endpoint=Buffer.concat([Buffer.from([3]),text(e.id)]);break;case"topic":endpoint=Buffer.concat([Buffer.from([4]),text(e.name)]);break;}const body=Buffer.concat([Buffer.from([0]),endpoint,u(payload.length),payload]);const frame=Buffer.concat([Buffer.from([5]),u(body.length),body]);if(frame.toString("hex")!==row.recordHex)throw Error(e.kind+" "+frame.toString("hex"));}const n=f.largePayload.length;const prefix=Buffer.concat([Buffer.from([5]),u(1+1+1+u(n).length+n),Buffer.from([0,0,7]),u(n)]);if(prefix.toString("hex")!==f.largePayload.prefixHex)throw Error("large-prefix "+prefix.toString("hex"));console.log("[DEBUG] strict Ajv + independent NodeBuffer/webassemblyjs message endpoints5/large-prefix1 PASS; no native cursor execution");})()
```

This packet does not establish source retirement, section ordering, semantic UI ownership, elapsed callback bounds or live poll cutover. It deliberately preserves the existing Invocation AppFrame payload as opaque bytes; that shared fixture is also consumed by the native tests.

After Kernel framing/dialect R2 actually passed all four tests, the three message tests were mounted from the Kernel root under `return_content_message_tests`. At that checkpoint the production message module was absent and the canonical missing-API RED was queued with the sole compiler. This is historical preparation, not the latest source state.

## Native RED And Implementation Boundary

The next gate executed the intended E0432 `super::return_message` compile failure (one error, nineteen warnings, no tests, exit 1), recorded in `📓️kernel-return-message-red-r1-native-2026-08-27.md`. Only after reading that result and the source-hold release, `💌️message/🦀️component.rs` was implemented and mounted. Its constructor captures bounded scalar prefixes and source byte borrows; each write advances at most one field phase and at most the smaller of the caller's byte grant, destination extent and canonical 4096-byte page. The original Effect is never consumed, copied or parsed as PACK, and dropping the cursor merely ends a borrow. The three native tests now await GREEN; no success is inferred from source inspection.

## Actual Native GREEN

The subsequent canonical `@semio-tech/framework-rs:test-wire-retirement-native --args='--lib return_content_ -- --nocapture'` ran all three message laws plus the two framing and two existing dialect laws: 7 passed, 253 skipped, 0 failed, .177 seconds, exit 0. Actual output is in `📓️kernel-return-message-green-r2-native-2026-08-27.md` and `🧪️member-kernel-return-message-green-r2-native-2026-08-27.txt`. This supersedes the historical pending state above. No live guest output, retained source release, final clock or semantic UI publication is claimed by this codec gate.
