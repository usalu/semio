# Canonical Return Fixture Oracle R1

Actual canonical Bun/Nx diagnostic exit 0 after strict schema corrections. This validates fixture shape and six fixed drive encodings only; no native codec, state-machine or semantic content execution is credited.

```text
[DEBUG] strict AJV PASS, webassemblyjs 6 vectors PASS, maximum drive 43
```

Earlier attempts are not semantic passes: Nx argument quoting failed; selecting an obsolete Actor project ran no command; eval initially lacked require. The subsequent actual strict Ajv compilation found missing explicit string type in the u64 refinement and integer type in the non-final-page length condition. Both were corrected in the schema without relaxing strict validation. The final run independently encoded every unsigned field with `@webassemblyjs/leb128` from an exact Node Buffer u64 value and matched all six authored hex vectors, including 43-byte maximum.

## Exact Evaluation

The following source was supplied through `SEMIO_RETURN_FIXTURE_EVAL`, with `NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_RETURN_FIXTURE_EVAL)'`.

```javascript
(async()=>{const fs=await import('node:fs');const {default:Ajv}=await import('ajv');const {default:leb}=await import('@webassemblyjs/leb128/lib/leb.js');const dir='/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/';const read=p=>JSON.parse(fs.readFileSync(dir+p,'utf8'));const ajv=new Ajv({strict:true});for(const p of ['🚪️lifetime/🧬️schema.json','📄️page/🧬️schema.json','📤️return/🧬️schema.json'])ajv.addSchema(read(p));const f=read('📤️return/🧪️fixture.json');const validate=ajv.compile(read('📤️return/🧪️schema.json'));if(!validate(f))throw Error(JSON.stringify(validate.errors));const num=n=>{const b=Buffer.alloc(8);b.writeBigUInt64LE(BigInt(n));return Array.from(leb.encodeUIntBuffer(b));};const origin=o=>[...num(o.activationGeneration),...num(o.requestSequence)];const id=i=>[...origin(i.origin),...num(i.returnSequence)];const ctl=c=>[f.controlTags[c.kind],...(c.kind==='inputAck'?[...id(c.receipt.identity),...num(c.receipt.pageSequence),...num(c.receipt.length),Number(c.receipt.final)]:id(c.identity))];for(const v of f.wireVectors){const got=Buffer.from(v.value.kind==='execute'?[0,...origin(v.value.origin)]:[1,...ctl(v.value.control)]).toString('hex');if(got!==v.hex)throw Error(JSON.stringify({got,want:v.hex}));}console.log('[DEBUG] strict AJV PASS, webassemblyjs '+f.wireVectors.length+' vectors PASS, maximum drive '+f.wireVectors.at(-1).hex.length/2);})()
```
