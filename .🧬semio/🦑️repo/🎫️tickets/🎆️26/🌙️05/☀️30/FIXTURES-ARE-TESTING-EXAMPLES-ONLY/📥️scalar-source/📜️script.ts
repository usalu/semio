import {dirname,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import original from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";

const directory=dirname(fileURLToPath(import.meta.url));
export default {...original,cacheDir:resolve(directory,"../🗑️generated/scalar-vite-cache"),test:{...original.test,include:[],includeSource:[resolve(directory,"../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx")],testNamePattern:/OwnedResidentScalar/}};
