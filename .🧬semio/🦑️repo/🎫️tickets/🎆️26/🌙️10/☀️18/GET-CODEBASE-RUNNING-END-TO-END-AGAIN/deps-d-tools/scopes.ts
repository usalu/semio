import {discoverBunWorkspaces,bunRepositoryMembership} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts";
const t=performance.now();
const scopes=discoverBunWorkspaces(process.cwd());
console.log(performance.now()-t,"ms");
for(const s of scopes)console.log(JSON.stringify({directory:s.directory,manifest:s.manifest,lock:s.lock,members:s.declaration.members.length,owners:s.declaration.owners.length,m0:s.declaration.members.slice(0,2),o0:s.declaration.owners.slice(0,2)}));
