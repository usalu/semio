/** 🧩️ Publishes the stdio hub composition (owner roots, shard roots and their manifest regions) from the artifact-owned contributions: `bun prepare-stdio-composition.ts`. */
import { resolve } from "node:path";
import { prepareStdioComposition } from "../../../../../../../🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts";
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
console.log(JSON.stringify(prepareStdioComposition(repoRoot, resolve(repoRoot, "🌎️hub/🧩️compositions/🗄️stdio"))));
