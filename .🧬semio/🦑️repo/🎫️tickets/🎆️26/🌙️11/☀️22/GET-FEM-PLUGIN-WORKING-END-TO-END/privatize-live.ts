import {patchedBunCacheEntriesV1,patchedDependencyKeysOfWorkspaceV1,privatizeCasEntriesV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🧲️adopt/🔒️private/🟦️.ts";
const workspace="C:/semio",keys=await patchedDependencyKeysOfWorkspaceV1(workspace,workspace+"/.cache/b/d/m/j");
const entries=await patchedBunCacheEntriesV1(workspace+"/.cache/b/d/m/bun-cache",keys);
console.log([...keys],entries.length);
console.log("replaced",await privatizeCasEntriesV1(entries));
