import {readFileSync} from "node:fs";
import {encodeFaultPack} from "@semio-tech/framework/packs";
const law = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
console.log(Buffer.from(encodeFaultPack(law.fault)).toString("hex"));
