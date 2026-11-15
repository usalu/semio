import { build } from "esbuild";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { printCommandSourceIdentitiesV1 } from "/Users/Shared/semio/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🛡️command-and-publication-verification/🟦️.ts";
import { productRoot, workspaceRoot } from "/Users/Shared/semio/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts";
const c = JSON.parse(readFileSync(join(productRoot, "🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️command-boundaries.json"), "utf8"));
const out: any = {};
for (const entry of c.entries) {
  const r = await build({ absWorkingDir: workspaceRoot, entryPoints: [join(productRoot, entry)], platform: "node", format: "esm", bundle: true, packages: "external", external: ["bun"], write: false, metafile: true, logLevel: "silent" });
  const external = [...new Set(Object.values(r.metafile!.outputs).flatMap((o: any) => o.imports).filter((i: any) => i.external && !/^(node|bun):/u.test(i.path) && i.path !== "bun").map((i: any) => i.path.startsWith("@") ? i.path.split("/").slice(0, 2).join("/") : i.path.split("/")[0]))].sort();
  out[entry] = { identities: printCommandSourceIdentitiesV1(r.metafile!.inputs as any), external };
}
console.log(JSON.stringify(out, null, 2));
