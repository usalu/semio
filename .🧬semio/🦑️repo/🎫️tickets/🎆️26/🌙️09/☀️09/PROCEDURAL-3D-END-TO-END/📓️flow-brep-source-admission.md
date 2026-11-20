# Flow BRep source admission

A concurrent extraction already placed the tessellation wire owner in its own library. After that change the original extension still contained 4,555 active lines, above Cargo source admission's 4,000-line limit.

The grant-budgeted original geometry jobs now have their own library. The existing Rust sources stay in place under the new package owner. The extension keeps its operator registration, inference bundle, and macros. The original jobs library exports the operation discriminator, admission and job type, and retains the same Session and Neural Engine dependencies. The topology output source remains unchanged.

| Unit | Files | Source bytes | Active lines |
| --- | ---: | ---: | ---: |
| Flow BRep extension | 16 | 161,427 | 2,844 |
| Original geometry jobs | 2 | 93,657 | 1,712 |
| Tessellation wire owner | 1 | 29,137 | 172 |
| Mesh extension | 5 | 132,772 | 1,721 |

The crate's workspace membership, project owner, library path, dependencies and four consuming lockfiles agree. Both edited Rust roots parse. The full native Flow suite continues through the repository Cargo ingress.

## Reproduce the admission count

```sh
repo tools exec --light bun -- -e '
import {readRustSourceClosureV1} from "./🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {readFile} from "node:fs/promises";
import {resolve} from "node:path";
const root=process.cwd(), base="✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep";
for(const directory of [base+"/📦️packages/🦀️rust",base+"/🥽️mesh",base+"/📐️tessellate",base+"/〰️analytic-curves"]){
 const entry=resolve(root,directory,"🦀️.rs");
 const source=await readRustSourceClosureV1([entry],root,["unix","target_os=macos","target_arch=aarch64"],async path=>{try{return await readFile(path)}catch(error){if(error.code==="ENOENT")return null;throw error;}});
 const bytes=source.reduce((n,x)=>n+x.bytes,0), lines=source.reduce((n,x)=>n+x.activeLines,0); console.log(directory,source.length,bytes,lines); if(bytes>262144||lines>4000)throw Error(directory);
}
'
```
