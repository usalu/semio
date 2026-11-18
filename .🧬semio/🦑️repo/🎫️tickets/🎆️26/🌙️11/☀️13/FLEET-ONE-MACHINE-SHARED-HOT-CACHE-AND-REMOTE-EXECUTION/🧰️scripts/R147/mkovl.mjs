import{writeFileSync}from"node:fs";
const S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R147/";
const L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",B=L+"/🏗️build",PN=B+"/🕸️graph/📥️inference/📦️project-nodes",OP=PN+"/🌐️owner-pool",PROC="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity";
const rows=[[OP+"/🟦️.ts","pool.ts"],[OP+"/🧬️schema/🔣️.json","schema.json"],[PN+"/🟨️.mjs","pn.mjs"],[B+"/🧠️memory/🟨️.mjs","memory.mjs"],[B+"/🧠️memory/🟨️.d.mts","memory.d.mts"],[B+"/🌐️remote-execution/🌡️load/🟦️.ts","load.ts"],[B+"/🌐️remote-execution/🛰️sidecar/🏊️pool/🟦️.ts","sidecar.ts"],[PROC+"/🙋️asks/🟦️.ts","../mirror/🙋️asks/🟦️.ts"],[PROC+"/🙋️asks/🧬️schema/🔣️.json","../mirror/🙋️asks/🧬️schema/🔣️.json"]];
writeFileSync(S+"over.resolved",rows.map(([live,s])=>live+"="+S+"stage/"+s).join("\n")+"\n");
