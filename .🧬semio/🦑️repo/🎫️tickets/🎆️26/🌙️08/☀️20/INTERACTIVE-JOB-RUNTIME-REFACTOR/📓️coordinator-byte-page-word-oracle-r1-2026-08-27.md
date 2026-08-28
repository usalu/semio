# Coordinator Byte Page Word Oracle R1 — 2026-08-27

Actual independent read-only Bun/Nx evaluation exited 0. Strict Ajv's canonical decimal-u64 schema agreed with an independent BigInt range/absolute-end lexical oracle for **1394 distinct strings**, including overflow, altered decimal prefixes, signs, leading zeros, fractions, exponent syntax, spaces and line terminators. Node Buffer independently round-tripped 512 deterministic unsigned 64-bit values.

```text
[DEBUG] actor-byte-page-word-oracle {"checked":1394,"bufferCases":512,"mismatches":[],"scope":"Ajv canonical decimal-u64 versus BigInt and Node Buffer; no runtime timing"}
```

This verifies the word schema, not whole page authority, native layout, timing or unknown-root retirement. Schema SHA-256 was stable before/after: `08732c8b215162a04e546d4c935f842814aeeba07bc2ad664fb64f9e5c894611`.

The command below ran inline without creating a script file; it read only the canonical byte-page schema.

```sh
SEMIO_BYTE_PAGE_ORACLE_EVAL='(async()=>{const {readFileSync}=await import("node:fs");const {Buffer}=await import("node:buffer");const {default:Ajv}=await import("ajv");const {strictEqual}=await import("node:assert");const base="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📄️page/";const schema=JSON.parse(readFileSync(base+"🧬️schema.json","utf8"));const validate=new Ajv({strict:true}).compile(schema.definitions.word);const max=18446744073709551615n;let seed=1n;const values=new Set(["0","1",max.toString(),(max+1n).toString(),"-1","+1","00","01","1.0","1e0","0\n","1\r","1\r\n","1\u2028","1\u2029","1 "," 1","\n1","", "184467440737095516150"]);for(let i=0;i<512;i++){seed=(seed*6364136223846793005n+1442695040888963407n)&max;const bytes=Buffer.alloc(8);bytes.writeBigUInt64LE(seed);strictEqual(bytes.readBigUInt64LE().toString(),seed.toString());values.add(seed.toString());values.add((seed+max+1n).toString());}const top=max.toString();for(let i=0;i<top.length;i++)for(let d=0;d<10;d++){values.add(top.slice(0,i)+d+"0".repeat(top.length-i-1));values.add(top.slice(0,i)+d+"9".repeat(top.length-i-1));}const mismatches=[];let checked=0;for(const word of values){const canonical=/^(?:0|[1-9][0-9]*)(?![\s\S])/.test(word);const expected=canonical&&BigInt(word)<=max;const actual=validate(word);checked++;if(actual!==expected)mismatches.push({word,expected,actual});}console.log("[DEBUG] actor-byte-page-word-oracle "+JSON.stringify({checked,bufferCases:512,mismatches,scope:"Ajv canonical decimal-u64 versus BigInt and Node Buffer; no runtime timing"}));strictEqual(mismatches.length,0,"byte-page decimal-u64 schema must reject all noncanonical words");})()' NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_BYTE_PAGE_ORACLE_EVAL)'
```

