import {discoverBunWorkspaces,bunWorkspacePackages} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts";
import {readJavascriptDependencySourceOwnersV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts";
const root=process.cwd();
const scopes=discoverBunWorkspaces(root).map(s=>({manifest:s.manifest.replace(root+"/",""),packages:[...bunWorkspacePackages(root,s)]}));
const model=readJavascriptDependencySourceOwnersV1({version:1,scopes});
console.log(JSON.stringify(model.scopes.map(s=>({scope:s.scope,members:s.members.length,owners:s.owners})),null,1));
