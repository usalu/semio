import {mkdirSync} from "node:fs";
import {testArtifactPublication} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🧪️tests/🟦️.ts";
const output=process.argv[2]!;mkdirSync(output,{recursive:true});
await testArtifactPublication(output);console.log("publication suite ok");
