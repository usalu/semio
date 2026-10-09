import {resolve,join} from "node:path";
const root=resolve(import.meta.dir,"../../../../../../../..");
const command=process.argv[2]??"source";
const sqlite=join(root,"✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts");
const tests=command==="source"?[join(import.meta.dir,"🟦️.ts")]:command==="runtime"?[join(import.meta.dir,"🟦️.ts"),sqlite]:command==="witness"?[join(import.meta.dir,"🧪️witness.ts")]:[];
if(tests.length===0)throw Error("Unknown Draw facet command");
const child=Bun.spawn(["bun","test",...tests],{cwd:root,stdout:"inherit",stderr:"inherit"});const result=await child.exited;if(result!==0)throw Error("Draw facets failed "+result);
