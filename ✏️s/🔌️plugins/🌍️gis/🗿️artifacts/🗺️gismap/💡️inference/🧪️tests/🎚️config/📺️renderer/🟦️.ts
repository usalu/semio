import base from "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";
import {fileURLToPath} from "node:url";
import {defineConfig} from "vitest/config";
export default defineConfig({...base,test:{...base.test,name:"gis-gismap-presentation",include:[fileURLToPath(new URL("../../📺️presentation/🟦️.tsx",import.meta.url)),fileURLToPath(new URL("../../🔬️probe/🟦️.ts",import.meta.url))],includeSource:[],passWithNoTests:false}});
