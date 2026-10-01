import {mergeConfig} from "vitest/config";
import {resolve} from "node:path";
import base from "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🎚️config/🟦️.ts";
export default mergeConfig(base,{test:{include:[resolve(import.meta.dir,"../🔬️process/🟦️.ts")],includeSource:[]}});
