# Independent Issued Patch Fixture R1

The coordinator validated the authored patch receipt fixture before production implementation. Actual exit0: strict Ajv schema validation, four independent @webassemblyjs/leb128 encodings matching canonical hex, the full35byte maximum,100 u64 boundary samples, nine invalid lexical forms and five0/1-patch pairing rows.

The eight malformed byte vectors and feedback laws are authored fixture data only in this run; the coordinator did not run a production decoder or guest ACK/rejection implementation here. Native sequence reuse/exhaustion, clock retention and exact descendant handback remain runtime gates. The command created no script file or runtime dependency and did not modify the schema/vectors.

## Command

```sh
SEMIO_PATCH_FIXTURE_EVAL='(async () => {
const {readFileSync}=await import("node:fs");
const {default:Ajv}=await import("ajv");
const {default:leb}=await import("@webassemblyjs/leb128/lib/leb.js");
const {strict:assert}=await import("node:assert");
const base="🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/";
const read=path=>JSON.parse(readFileSync(base+path,"utf8"));
const lifetime=read("🧬️schema.json"), receipt=read("🩹️patch/🧬️schema.json"), fixtureSchema=read("🩹️patch/🧪️schema.json"), fixture=read("🩹️patch/🧪️fixture.json");
const ajv=new Ajv({strict:true});ajv.addSchema(lifetime);ajv.addSchema(receipt);
const validate=ajv.compile(fixtureSchema);assert.equal(validate(fixture),true,JSON.stringify(validate.errors));
const validateReceipt=ajv.getSchema(receipt.$id);
const encode=n=>{const bytes=Buffer.alloc(8);bytes.writeBigUInt64LE(BigInt(n));return Buffer.from(leb.encodeUIntBuffer(bytes));};
for(const row of fixture.vectors){const value=row.value;const bytes=Buffer.concat([encode(value.lifetime.activationGeneration),encode(value.lifetime.instanceId),encode(value.lifetime.guestLifetime),encode(value.patchSequence)]);assert.equal(bytes.toString("hex"),row.hex);assert.ok(bytes.length<=35);assert.equal(validateReceipt(value),true);}
const max=(1n<<64n)-1n;const samples=new Set([0n,1n,max,max-1n,max+1n,9007199254740991n,9007199254740992n]);
for(let width=0n;width<=20n;width++){const power=10n**width;for(const offset of [-1n,0n,1n]){const value=power+offset;if(value>=0n)samples.add(value);}}
const digits=max.toString();for(let i=1;i<digits.length;i++){const suffix=digits.length-i-1;const d=Number(digits[i]);if(d>0){for(const tail of ["0","9"]){samples.add(BigInt(digits.slice(0,i)+(d-1)+tail.repeat(suffix)));}}}
for(const n of samples){const value={...fixture.vectors[0].value,patchSequence:n.toString()};assert.equal(validateReceipt(value),n>0n&&n<=max,"u64 schema "+n);}
for(const text of ["","00","01","-1","+1"," 1","1 ","1.0","1e0"]){assert.equal(validateReceipt({...fixture.vectors[0].value,patchSequence:text}),false);}
for(const row of fixture.pairing)assert.equal((row.patchCount===0&&!row.hasReceipt)||(row.patchCount===1&&row.hasReceipt),row.accepted);
assert.equal(fixture.vectors.at(-1).hex.length/2,35);
console.log("[DEBUG] patch-receipt fixture=valid independent-webassemblyjs-vectors=4 maximum-bytes=35 canonical-u64-boundaries="+samples.size+" lexical-negatives=9 pairing-rows=5 malformed-vectors-staged=8");
})()' NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_PATCH_FIXTURE_EVAL)'
```

## Actual Output

```text
[DEBUG] patch-receipt fixture=valid independent-webassemblyjs-vectors=4 maximum-bytes=35 canonical-u64-boundaries=100 lexical-negatives=9 pairing-rows=5 malformed-vectors-staged=8

```

