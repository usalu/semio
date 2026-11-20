import { run } from "./central-final-graph-check.mjs";
import { resolve } from "node:path";

const root = resolve("✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪡️apply/🗃️page-cache");
const { testPageCache } = await import(resolve(root, "🧪️tests/🟦️.ts"));
console.log(JSON.stringify({ graph: await run(process.cwd()), pageCache: testPageCache() }));
