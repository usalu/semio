import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fixedSourceDispositionDecision } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const root = join(import.meta.dir, "../../../../../../..");
for (const path of process.argv.slice(2)) console.log(`[DEBUG] ${path} ${JSON.stringify(fixedSourceDispositionDecision("root-script", readFileSync(join(root, path), "utf8")))}`);
