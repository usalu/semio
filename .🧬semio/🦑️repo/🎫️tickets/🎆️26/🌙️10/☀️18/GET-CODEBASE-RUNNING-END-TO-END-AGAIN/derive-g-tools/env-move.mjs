// [derive-g] simulates ONE environment move on a scratch copy of a sealed generation: the @env owner holds a stale digest row of the sub-workspace manifest (so only @env is dirty) and the manifest records another environment digest (`environment !== previousEnv`). usage: bun env-move.mjs <scratch-gen-dir> [label]
import{readFileSync,writeFileSync}from"node:fs";import{join}from"node:path";import{createHash}from"node:crypto";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const{canonicalJsonV1}=await import(L+"🧮️fact-codec/🟨️.mjs");
const D=process.argv[2],label=process.argv[3]??"moved",sha=b=>createHash("sha256").update(b).digest("hex");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
manifest.env=sha("derive-g:"+label+":"+manifest.env);
const{unpackFactIndexV1,packFactIndexV1,GENERATION_POLICY_V1}=await import(L+"🟨️.mjs");
const index=unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),GENERATION_POLICY_V1.maximumIndexBytes);
const row=index.rows.find(r=>r.kind==="digest"&&r.path==="✏️s/Cargo.toml");if(!row)throw Error("digest row missing");
index.rows[index.rows.indexOf(row)]={...row,fact:sha("derive-g:stale:"+label)};
const packed=packFactIndexV1(index,GENERATION_POLICY_V1.maximumIndexBytes),indexDigest=sha(packed.bytes);writeFileSync(join(D,"x",indexDigest+".facts.br"),packed.bytes);manifest.factIndex=indexDigest;
const bytes=Buffer.from(canonicalJsonV1(manifest)),digest=sha(bytes);
writeFileSync(join(D,"m",digest+".json"),bytes);writeFileSync(join(D,"current.json"),JSON.stringify({...pointer,manifest:digest}));
console.log(JSON.stringify({manifest:digest,env:manifest.env}));
