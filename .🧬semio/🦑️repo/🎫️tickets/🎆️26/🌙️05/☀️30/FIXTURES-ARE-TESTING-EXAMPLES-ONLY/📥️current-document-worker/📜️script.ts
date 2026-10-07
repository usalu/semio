import original from "../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts";
import {resolve} from "node:path";
import {dirname} from "node:path";
import {fileURLToPath} from "node:url";
const source=resolve(dirname(fileURLToPath(import.meta.url)),"../../../../../../../../✏️s/🧑‍💻dev/🧩️service-composition/👷️worker/🟦️.ts");
export default {...original,test:{...original.test,include:[],includeSource:[source]}};
