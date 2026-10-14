import {dirname,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import original from "../../../../../../../../🧰️framework/🔨️modules/🎭️actor/🧪️tests/🎚️config/🟦️.ts";

const directory=dirname(fileURLToPath(import.meta.url));
export default {...original,cacheDir:resolve(directory,"../🗑️generated/actor-vite-cache"),test:{...original.test,includeSource:["../../📃️page/🟦️.ts","../../📤️return/🟦️.ts"]}};
