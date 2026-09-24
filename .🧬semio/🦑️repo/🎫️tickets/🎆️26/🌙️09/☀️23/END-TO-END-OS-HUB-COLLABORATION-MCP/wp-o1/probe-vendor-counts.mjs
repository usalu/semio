
import { readdirSync, readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { pathToFileURL } from "node:url";
const dep = await import('file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%94%8C%EF%B8%8Fplugin/%F0%9F%93%87%EF%B8%8Fregistry/%F0%9F%93%A6%EF%B8%8Fdeployment/%F0%9F%9F%A6%EF%B8%8F.ts');
const act = await import('file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%A7%91%E2%80%8D%F0%9F%92%BBdev/%E2%99%BB%EF%B8%8Factivation/%F0%9F%9F%A6%EF%B8%8F.ts');
const { moduleIdForDirectoryName, MODULE_PLUGIN_ROUTE } = dep;
const pluginOutRoot = act.pluginModulesRoot("dev");
console.log("pluginOutRoot", pluginOutRoot);
const fixture = JSON.parse(readFileSync(join(dirname(pluginOutRoot), "../../../🧑‍💻dev/�").replace("🧑‍💻dev/�",""), "utf8"));
