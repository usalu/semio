import{readFileSync,readdirSync,statSync,writeFileSync,existsSync}from"node:fs";import{createHash}from"node:crypto";import{join}from"node:path";
export const R="/Users/ueli/Documents/semio/",LIVE=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R113/",STAGE=SC+"s/a/b/owner-pool/",BK=SC+"backup/";
export const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
export const walk=(dir,rel="")=>readdirSync(dir).flatMap(name=>{const p=join(dir,name),r=rel+name;return statSync(p).isDirectory()?walk(p,r+"/"):[r];});
export const changed=()=>walk(STAGE).filter(r=>!existsSync(LIVE+r)||sha(LIVE+r)!==sha(STAGE+r));
