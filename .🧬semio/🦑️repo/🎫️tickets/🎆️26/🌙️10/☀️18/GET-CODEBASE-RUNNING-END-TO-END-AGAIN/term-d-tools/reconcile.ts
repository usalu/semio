import {reconcileBuildOwnerRegistry} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts";
const root=process.cwd(),write=process.argv[2]==="--write";
const {registry:_,...rest}=reconcileBuildOwnerRegistry(root,{scope:"🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard",write});console.log(JSON.stringify(rest,null,1));
