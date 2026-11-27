import {readBuildOwnerRegistryV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
import * as producer from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs";
const r=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(process.cwd())));
console.log("registry owners",r.owners.length,"gis",r.owners.filter(o=>/gis/i.test(o.name)).map(o=>o.name).join(" "));
