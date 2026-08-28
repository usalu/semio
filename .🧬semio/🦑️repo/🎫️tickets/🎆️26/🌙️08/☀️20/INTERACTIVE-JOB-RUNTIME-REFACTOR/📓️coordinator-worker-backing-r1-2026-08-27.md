# Coordinator Worker Backing Platform Oracle R1

Independent root execution through existing Bun/Nx exec completed with exit0 under Node v24.15.0. Eight cases passed. This is a platform behavior oracle, not implementation acceptance, browser behavior, memory retirement or an 8ms timing measurement.

## Observed Boundaries

Actual MessageChannel transfer preserves exact64 payload bytes (independent Node Buffer comparison), detaches sender backing, and produces a whole fixed ArrayBuffer. Source custom properties and their getter are not copied/invoked by this transfer. Crucially, the original sender object still retains its custom8193-byte descendant after its backing detaches; transferring backing does not retire arbitrary source wrapper properties.

Cross-realm genuine ArrayBuffer fails the local instanceof check but passes the intrinsic byteLength brand access. Forged prototypes, proxies, shared backing and typed-array views do not pass that intrinsic ArrayBuffer access. A resizable ArrayBuffer remains resizable after transfer, so transfer is not a substitute for explicit fixed-backing validation. A partial64-byte view over128-byte backing is not an entire admitted64-byte envelope.

No arbitrary whole-key enumeration is used. These tests check known supplied fields and platform intrinsic behavior; they do not certify the absence of all unknown descendants on an arbitrary caller object. Production provenance still requires exact captured worker/slot/request and pre-reserved response ownership.

## Response Schema Decision

Root read all three peer Actor return/response declaration/fixture files and the full design. Dag confirmed no competing transport schema. Approved one replacement of the existing canonical returned worker response: magic73727201, tag0result or1fault, positive activation ULEB64, actual dispatched transport request ULEB53, then canonical result bytes to EOF or exactly one closed transport-fault byte. Max4161 is4+1+10+8+4138. It is not a second guest ABI or a compatibility decoder.

The actual transport request on control turns differs from the return's original request origin. Result/header activation must match, and protocolFault remains correlatable without inventing native identity. Unknown/oversized/foreign raw roots remain retained/refused; a fixed fault reply cannot discharge its original guest error. Codec, valid-path ownership and quarantine mounting remain peer work.

Reviewed peer schema hash:e201461285772af46acacaad1cdd29c5d8b81ea5fb94463c2976a501c73a97c3; fixture2c86da55e2cdf522ac44b79d9436468bda2db99b5e06d9bcfca6ed25ddddc7cb; fixture schema8f9d17d6e1e1c8cb7dd8ad8af110ab20542806c811937184fd6d738f1f586d28. Root's platform oracle did not import those schema files or the production codec.

## Exact Research Evaluation

```javascript
(async()=>{const {strict:assert}=await import("node:assert");const {Buffer}=await import("node:buffer");const {runInNewContext}=await import("node:vm");const {MessageChannel}=await import("node:worker_threads");const byteLength=Object.getOwnPropertyDescriptor(ArrayBuffer.prototype,"byteLength").get;const resizable=Object.getOwnPropertyDescriptor(ArrayBuffer.prototype,"resizable").get;const owned=[];const cases=[];const pattern=Buffer.from(Array.from({length:64},(_,i)=>(i*17+3)&255));const raw=new ArrayBuffer(64);new Uint8Array(raw).set(pattern);const unknown=new Uint8Array(8193);let getterCalls=0;Object.defineProperty(raw,"unknown",{value:unknown});Object.defineProperty(raw,"poison",{enumerable:true,get(){getterCalls++;throw Error("getter");}});owned.push(raw,unknown);const moved=structuredClone(raw,{transfer:[raw]});assert.equal(byteLength.call(raw),0);assert.equal(byteLength.call(moved),64);assert.deepEqual(Buffer.from(moved),pattern);assert.equal(Object.hasOwn(moved,"unknown"),false);assert.equal(Object.hasOwn(moved,"poison"),false);assert.equal(raw.unknown,unknown);assert.equal(getterCalls,0);cases.push("clone-transfers-bytes-not-custom-descendants","sender-custom-descendant-still-owned");const foreign=runInNewContext("new ArrayBuffer(64)");assert.equal(foreign instanceof ArrayBuffer,false);assert.equal(byteLength.call(foreign),64);cases.push("cross-realm-intrinsic-brand");const fake=Object.create(ArrayBuffer.prototype);const proxied=new Proxy(new ArrayBuffer(64),{get(){throw Error("proxy-get");}});owned.push(fake,proxied);assert.throws(()=>byteLength.call(fake),TypeError);assert.throws(()=>byteLength.call(proxied),TypeError);cases.push("forged-and-proxy-not-intrinsic-backing");const shared=new SharedArrayBuffer(64);assert.throws(()=>byteLength.call(shared),TypeError);cases.push("shared-refused-by-arraybuffer-brand");const growing=new ArrayBuffer(64,{maxByteLength:128});assert.equal(resizable.call(growing),true);const growingMoved=structuredClone(growing,{transfer:[growing]});assert.equal(resizable.call(growingMoved),true);assert.equal(growingMoved.maxByteLength,128);cases.push("resizable-remains-resizable-after-transfer");const view=new Uint8Array(new ArrayBuffer(128),32,64);assert.throws(()=>byteLength.call(view),TypeError);assert.equal(view.buffer.byteLength,128);cases.push("partial-view-is-not-entire-backing");const channel=new MessageChannel();try{const sent=new ArrayBuffer(64);new Uint8Array(sent).set(pattern);Object.defineProperty(sent,"unknown",{value:unknown});owned.push(sent);const receive=new Promise((resolve,reject)=>{channel.port2.once("message",resolve);channel.port2.once("messageerror",reject);});channel.port1.postMessage(sent,[sent]);assert.equal(sent.byteLength,0);const received=await receive;assert.equal(byteLength.call(received),64);assert.equal(resizable.call(received),false);assert.deepEqual(Buffer.from(received),pattern);assert.equal(Object.hasOwn(received,"unknown"),false);assert.equal(sent.unknown,unknown);cases.push("actual-messageport-transfer");}finally{channel.port1.close();channel.port2.close();}console.log("[DEBUG] worker-backing-platform-oracle "+JSON.stringify({runtime:process.version,cases:cases.length,names:cases,unknownSenderBytes:unknown.byteLength,getterCalls,browser:false,retirement:false,timing:false}));})()
```

Invoked as existing `bun x nx exec --projects=workspace -- node --input-type=module -e 'await eval(process.env.SEMIO_WORKER_BACKING_ORACLE)'` with this exact evaluation in that task-specific environment variable; daemon, project-graph cache and plugin isolation disabled. No script file or runtime dependency was added.

## Actual Tool Output

### root-worker-backing-r1-run

exit_code:running; session_id:47188

```text

```

### root-worker-backing-r1-poll-1

exit_code:0; session_id:none

```text
[DEBUG] worker-backing-platform-oracle {"runtime":"v24.15.0","cases":8,"names":["clone-transfers-bytes-not-custom-descendants","sender-custom-descendant-still-owned","cross-realm-intrinsic-brand","forged-and-proxy-not-intrinsic-backing","shared-refused-by-arraybuffer-brand","resizable-remains-resizable-after-transfer","partial-view-is-not-entire-backing","actual-messageport-transfer"],"unknownSenderBytes":8193,"getterCalls":0,"browser":false,"retirement":false,"timing":false}

```

