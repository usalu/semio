import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🕸️wasm/📦️delivery/🧬️schema/🔣️.json" with {type:"json"};
import {frameworkOsEngineDeliveriesV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts";
const d=frameworkOsEngineDeliveriesV1("/Users/ueli/Documents/semio/.cache/b/p/m",{version:1,mode:"prototype",facet:"play"} as any);
console.log(JSON.stringify(d).slice(0,1500));console.log(validateJsonSchemaSubset(schema as any,d));
