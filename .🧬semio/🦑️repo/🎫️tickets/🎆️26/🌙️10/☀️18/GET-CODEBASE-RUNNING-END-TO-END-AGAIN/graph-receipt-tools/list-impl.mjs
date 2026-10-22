import { installVariant, WORKSPACE, LIBRARY, SCRATCH } from "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graph-parallel-tools/harness/variant.mjs";
installVariant();
import { join, relative } from "node:path";
const inference = await import(join(LIBRARY, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
for (const row of inference.loadedImplementationRowsV1) console.log(relative(LIBRARY, row.path));
process.exit(0);
