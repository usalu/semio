import {readFileSync,existsSync} from "node:fs";
import {join} from "node:path";
const base="✏️s/🔌️plugins/📕️norm/🗿️artifacts";
for(const [family,name] of [
 ["🏛️en1992","semio-s-artifact-norm-en1992-checks-reach-and-trace"],
 ["🔩️en1993","semio-s-artifact-norm-en1993-checks-laws"],
 ["🧩️en1994","semio-s-artifact-norm-en1994-editor-laws"],
 ["🔩️en1993","semio-s-artifact-norm-en1993-editor-laws"],
 ["🫨️en1998","semio-s-artifact-norm-en1998-schema-formulas-worked-examples"],
 ["🏭️vdi3805","semio-s-artifact-norm-vdi3805-inferences-laws"],
 ["🪵️en1995","semio-s-artifact-norm-en1995-mutations-bridges"],
]){
 const root=join(base,family),workspace=Bun.TOML.parse(readFileSync(join(root,"Cargo.toml"),"utf8")).workspace;
 const path=workspace.dependencies[name]?.path;
 const manifest=path&&join(root,path,"Cargo.toml");
 const status=!path?"missing dependency":!path.endsWith("/📦️packages/🦀️rust")?"undiscoverable path":!existsSync(manifest)?"missing manifest":!workspace.members.includes(path)?"missing member":Bun.TOML.parse(readFileSync(manifest,"utf8")).package.name!==name?"name mismatch":"ok";
 console.log(name,status,path??"");
}
