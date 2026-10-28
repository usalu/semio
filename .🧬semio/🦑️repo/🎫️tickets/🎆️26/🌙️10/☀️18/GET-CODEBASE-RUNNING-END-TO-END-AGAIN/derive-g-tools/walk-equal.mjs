// [derive-g] on the REAL tree: the roster of the test-domain command walk read directly equals the roster of resolveCommandWalkV1 (graph-free). usage: bun walk-equal.mjs <scratch-gen-dir-with-derived-row>
import{readFileSync}from"node:fs";import{join}from"node:path";
const ROOT="/Users/ueli/Documents/semio",B=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const g=await import(B+"🕸️graph/🕰️generations/🟨️.mjs"),D=process.argv[2];
const p=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),m=JSON.parse(readFileSync(join(D,"m",p.manifest+".json"),"utf8"));
const index=g.unpackFactIndexV1(readFileSync(join(D,"x",m.factIndex+".facts.br")),g.GENERATION_POLICY_V1.maximumIndexBytes);
const rows=index.rows.filter(r=>r.kind==="derived");const q=rows.map(r=>JSON.parse(r.path)).filter(q=>q.walk===1&&q.roots.some(x=>x.includes("test")));
console.log("derived rows",rows.length,"test walk questions",q.length);const question=q[0];
const inference=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(B+"👁️observation/🟨️.mjs");const impl=await inference.loadCurrentImplementationV1();
const roots=question.roots;
const direct=await obs.observeInferenceSnapshotV1(()=>impl.cacheInternals.relativeScriptInputs?impl.cacheInternals.relativeScriptInputs(roots,ROOT,impl.createScriptInputCache({workspaceRoot:ROOT})):null);
const viaWalk=await obs.observeInferenceSnapshotV1(()=>impl.resolveCommandWalkV1(ROOT,roots,impl.createScriptInputCache({workspaceRoot:ROOT,lateClosures:true})));
const a=[...direct],b=[...viaWalk.value];console.log("direct",a.length,"walk",b.length,"equal",JSON.stringify(a)===JSON.stringify(b),"derived",viaWalk.derived.length,"supportRows",viaWalk.derived[0]?.support.length,"rows",viaWalk.rows.length);
if(JSON.stringify(a)!==JSON.stringify(b)){const sa=new Set(a),sb=new Set(b);console.log("only direct",a.filter(x=>!sb.has(x)).slice(0,5),"only walk",b.filter(x=>!sa.has(x)).slice(0,5));}
