import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
const root=process.env.SEMIO_TAXONOMY_REPO_ROOT!,ticket=dirname(dirname(import.meta.dir));
const { scanTestLayout } = await import(pathToFileURL(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts")).href);

const controller=new AbortController(),cancel=()=>controller.abort();process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
const findings = await scanTestLayout(root,{signal:controller.signal});
await Bun.write(join(ticket,"🗑️generated/taxonomy-guard/physical-census.json"), `${JSON.stringify(findings, null, 2)}\n`);
process.off("SIGINT",cancel);process.off("SIGTERM",cancel);
console.log(`[DEBUG] physical testing-layout findings=${findings.length}`);
