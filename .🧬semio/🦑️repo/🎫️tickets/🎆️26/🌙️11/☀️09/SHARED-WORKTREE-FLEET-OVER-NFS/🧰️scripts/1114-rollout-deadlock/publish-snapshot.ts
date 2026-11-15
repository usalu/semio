import {copyFile,mkdir,rm} from "node:fs/promises";
import {dirname,join,relative,resolve} from "node:path";
const L=process.env.PS_L??"/Users/Shared/semio",M=`${L}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree`,STORE=process.env.PS_STORE??"/Users/Shared/semio-store";
const {codeDigestV1}=await import(`${M}/🩺️attest/🟦️.ts`);
const {moduleSourcesV1,importClosureV1,sealCodeV1,publishCodeSnapshotV1,verifyCodeSnapshotV1,storeCodeDirectoryV1}=await import(`${M}/🛡️resilience/🟦️.ts`);
const scratch=process.argv[2]!,root=join(scratch,"seal");
if(process.argv[3]!=="go"){console.log("dry: would publish",await codeDigestV1(M));process.exit(0);}
await rm(root,{recursive:true,force:true});
const digest=await codeDigestV1(M),sources=await moduleSourcesV1(M),all=[...sources,...await importClosureV1(M,sources)];
for(const path of all){const from=resolve(M,...path.split("/")),to=join(root,relative(L,from));await mkdir(dirname(to),{recursive:true});await copyFile(from,to);}
const moduleRoot=join(root,relative(L,M));
if(await codeDigestV1(moduleRoot)!==digest||await codeDigestV1(M)!==digest){console.log("code changed while copying");process.exit(2);}
const sealed=await sealCodeV1({root,moduleRoot,digest,digestOf:codeDigestV1});
console.log("sealed",JSON.stringify(sealed),digest);
if(sealed.reason!==null)process.exit(3);
const published=await publishCodeSnapshotV1({root,store:STORE,digest,originId:"g1-server"});
console.log("published",JSON.stringify(published),"verify",await verifyCodeSnapshotV1(join(storeCodeDirectoryV1(STORE),digest)));
process.exit(0);
