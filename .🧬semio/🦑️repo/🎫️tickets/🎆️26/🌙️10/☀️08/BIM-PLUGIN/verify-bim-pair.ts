import { readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const verifier = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
const { verifySemanticDescriptorPairBytesV1 } = await import(pathToFileURL(join(process.cwd(), verifier)).href);
verifySemanticDescriptorPairBytesV1(readFileSync(join(process.argv[2], "🔣️.json")), readFileSync(join(process.argv[2], "🛂️.descriptor.semio")));
console.log("The emitted BIM descriptor pair is canonical.");
