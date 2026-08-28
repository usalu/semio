# Independent Kernel Owner Fixture R1

Actual exit0. Strict Ajv accepts the updated neutral fixture and rejects29missing/modified/additional-field cases. Independent TextEncoder/Node Buffer output agrees for surface13bytes and payload16bytes across grants1/64/4096, six byte rows. This is schema/encoding proof only; the real held-lock runtime laws are separate native gates.

Captured hashes were stable before/after. No generated publication, cleanup or native compiler was run by this coordinator check.

## Command

```sh
SEMIO_KERNEL_FIXTURE_EVAL='(async()=>{const {readFileSync}=await import("node:fs");const {Buffer}=await import("node:buffer");const {default:Ajv}=await import("ajv");const {deepStrictEqual,strictEqual}=await import("node:assert");const f=JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🧪️fixtures/🚪️turn-patch-owner.json","utf8"));const s=JSON.parse(readFileSync("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🧪️fixtures/🚪️turn-patch-owner.schema.json","utf8"));const v=new Ajv({strict:true,allErrors:true}).compile(s);strictEqual(v(f),true,JSON.stringify(v.errors));let n=0;for(const k of s.required){const x={...f};delete x[k];strictEqual(v(x),false);n++;const value=f[k];const changed=Array.isArray(value)?[...value,7]:typeof value==="boolean"?!value:typeof value==="number"?value+1:value+"x";strictEqual(v({...f,[k]:changed}),false);n++;}strictEqual(v({...f,unowned:true}),false);n++;const rows=[];for(const k of ["surface","payload"]){const a=new TextEncoder().encode(f[k]);const b=Buffer.from(f[k],"utf8");deepStrictEqual(Array.from(a),Array.from(b));for(const grant of f.byteGrants){const parts=[];for(let p=0;p<a.length;p+=grant)parts.push(Buffer.from(a.subarray(p,p+grant)));deepStrictEqual(Buffer.concat(parts),b);rows.push({field:k,grant,bytes:a.length,hex:b.toString("hex")});}}console.log("[DEBUG] kernel-turn-patch-oracle "+JSON.stringify({valid:1,negatives:n,byteRows:rows,scope:"schema-and-UTF8-only; native contention is separate"}));})()' NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false bun x nx exec --projects=workspace -- bun -e 'await eval(process.env.SEMIO_KERNEL_FIXTURE_EVAL)'
```

## Hashes Before

```text
3d3fa36c0997386bfe0e8ec7c2e4b16580855cd43a17364dc95d8d59beed0ad2  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🧪️fixtures/🚪️turn-patch-owner.json
2495610066de87689bc69a93d996661e19aada55054757de17fba46afd688b1c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🧪️fixtures/🚪️turn-patch-owner.schema.json
```

## Hashes After

```text
3d3fa36c0997386bfe0e8ec7c2e4b16580855cd43a17364dc95d8d59beed0ad2  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🧪️fixtures/🚪️turn-patch-owner.json
2495610066de87689bc69a93d996661e19aada55054757de17fba46afd688b1c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🧪️fixtures/🚪️turn-patch-owner.schema.json
```

## Actual Output

```text
[DEBUG] kernel-turn-patch-oracle {"valid":1,"negatives":29,"byteRows":[{"field":"surface","grant":1,"bytes":13,"hex":"373ae4bf9de795992cf09f9982"},{"field":"surface","grant":64,"bytes":13,"hex":"373ae4bf9de795992cf09f9982"},{"field":"surface","grant":4096,"bytes":13,"hex":"373ae4bf9de795992cf09f9982"},{"field":"payload","grant":1,"bytes":16,"hex":"ceb1ceb22cf09f99822fe4bf9de79599"},{"field":"payload","grant":64,"bytes":16,"hex":"ceb1ceb22cf09f99822fe4bf9de79599"},{"field":"payload","grant":4096,"bytes":16,"hex":"ceb1ceb22cf09f99822fe4bf9de79599"}],"scope":"schema-and-UTF8-only; native contention is separate"}

```

