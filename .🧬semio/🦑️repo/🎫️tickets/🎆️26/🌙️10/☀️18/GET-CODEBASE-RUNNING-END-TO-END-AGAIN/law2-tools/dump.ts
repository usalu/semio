import { readModuleTreeSource } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { writeFileSync } from "node:fs";
writeFileSync(process.argv[3]!, readModuleTreeSource(process.argv[2]!));
