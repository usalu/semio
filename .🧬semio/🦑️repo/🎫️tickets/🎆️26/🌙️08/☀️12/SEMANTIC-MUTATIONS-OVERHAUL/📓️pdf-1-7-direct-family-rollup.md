# PDF 1.7 Direct Mutation Family Rollup

## Exact Root Counts

This census reads only the seven explicit PDF 1.7 schema mutation roots. It does not inventory any other standard, artifact, or `compose/**`.

| Subset | Direct Owners | Mutation-Root Files | Required Surfaces | Nested Mutation Owners |
| --- | ---: | ---: | ---: | ---: |
| any | 16 | 147 | 112 | 0 |
| a | 14 | 29 | 14 | 0 |
| e | 12 | 103 | 84 | 0 |
| h | 10 | 87 | 70 | 0 |
| ua | 11 | 95 | 77 | 0 |
| vt | 18 | 151 | 126 | 0 |
| x | 14 | 119 | 98 | 0 |
| Total | 95 | 731 | 581 | 0 |

The exact family total is **95 concrete direct mutation owners**. All seven roots have no nested `🦠️mutation` implementation owner. PDF/A intentionally declares Rust only and null wire identities; the other six declare seven language/wire surfaces.

## Acceptance Boundary

- Coordinator independent scoped policy is zero across all 17 classes for all seven roots: Any, A, E, H, UA, VT, and X. X's final transcript is `🧪️pdf-x-independent-policy.log`.
- UA, VT, and X exact path/command evidence is in their sibling `📓️pdf-1-7-*-direct-cutover.md`, `🔣️pdf-1-7-*-cutover-files.json`, and `📓️pdf-1-7-*-validation-commands.md` files.
- E/H canonical root codec mounts were corrected and their four existing targets plus both root components independently parsed.
- No new Cargo/Nx runtime was launched in this subset wave. Runtime behavior remains deferred to the coordinator's serialized shared STDIO validation; prior shared compilation is not represented as validation of later uncompiled changes.

## Executed Census Command

```sh
bun -e 'const fs=require("fs");const roots=[{"subset":"any","root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🧬️mutations"},{"subset":"a","root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/🧬️schema/🧬️mutations"},{"subset":"e","root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️e/🧬️schema/🧬️mutations"},{"subset":"h","root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️h/🧬️schema/🧬️mutations"},{"subset":"ua","root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/🧬️schema/🧬️mutations"},{"subset":"vt","root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🧬️mutations"},{"subset":"x","root":"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/🧬️schema/🧬️mutations"}];const walk=d=>fs.readdirSync(d,{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(d+"/"+e.name):[d+"/"+e.name]);const rows=roots.map(({subset,root})=>{const files=walk(root);const leaves=fs.readdirSync(root,{withFileTypes:true}).filter(e=>e.isDirectory()&&fs.existsSync(root+"/"+e.name+"/🦀️component.rs")&&fs.existsSync(root+"/"+e.name+"/🔣️component.json")).map(e=>JSON.parse(fs.readFileSync(root+"/"+e.name+"/🔣️component.json","utf8"))).filter(d=>d.semanticKind);return {subset,root,directOwners:leaves.length,files:files.length,requiredSurfaces:leaves.reduce((n,d)=>n+d.requiredLanguageSurfaces.length,0),nestedOwners:files.filter(f=>f.includes("/🦠️mutation/")).length};});console.log(JSON.stringify({rows,totalOwners:rows.reduce((n,r)=>n+r.directOwners,0),totalFiles:rows.reduce((n,r)=>n+r.files,0)},null,2));'
```
