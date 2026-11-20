# B-Rep native registry source admission

The existing spline construction split already had consistent workspace membership, crate dependencies, project ownership and lock records. The repository source-closure reader still counted 4,296 lines in the native registry against its 4,000-line limit.

The solid topology transaction now has its own first-party crate. It owns bounded arena insertion, label construction, unpublished rollback and solid provenance. The native registry still owns handle encoding, live-index publication, session identity and the unpublished-topology counter. Existing native callers keep the same public transaction type and method signatures.

The repository source-closure reader reports 3,844 active lines across 24 registry files, 503 lines for the solid transaction, and 883 lines for spline construction. The crate manifests, workspace paths, owner project metadata and lock dependencies agree for both extracted crates. The edited Rust sources parse. Native Flow verification continues through the repository's Cargo ingress.

## Reproduce the active source count

```sh
repo tools exec --light bun -- -e '
import {readRustSourceClosureV1} from "./🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
const root=process.cwd();
const native="🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧠️native-registry";
for(const owned of [native,native+"/🧮️spline-construction",native+"/🏗️solid-transaction"]){
 const inputs=await readRustSourceClosureV1([resolve(root,owned,"🦀️.rs")],root,["unix","target_os=macos","target_arch=aarch64"],async path=>{try{return await readFile(path)}catch(error){if(error.code==="ENOENT")return null;throw error;}});
 const lines=inputs.reduce((n,x)=>n+x.activeLines,0);
 console.log(owned.split("/").at(-1),inputs.length,lines,inputs.reduce((n,x)=>n+x.bytes,0));
 if(lines>4000)throw Error("active lines exceed 4000");
}
'
```
