import { writeFileSync } from "node:fs";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
// Pull the discovery module's private path via a tiny re-read using the same resolver pattern.
const discUrl = new URL("./links/plugin-reg/🔎️discovery/🟦️.ts", import.meta.url);
const source = await Bun.file(discUrl).text();
// Fallback: import public API and scan process for open handles — instead resolve by walking from discovery file.
import { dirname, join } from "node:path";
import { existsSync, readFileSync, readdirSync, lstatSync } from "node:fs";
import { fileURLToPath } from "node:url";

function resolveTaxonomy(startDir: string): string {
  let current = startDir;
  for (;;) {
    for (const name of readdirSync(current)) {
      if (!name.endsWith(".json")) continue;
      const path = join(current, name);
      try {
        if (!lstatSync(path).isFile()) continue;
        const text = readFileSync(path, "utf8");
        if (text.includes('"pluginAreas"') && text.includes('"areas"')) return path;
      } catch {
        /* skip */
      }
    }
    const parent = dirname(current);
    if (parent === current) throw new Error("taxonomy not found");
    current = parent;
  }
}

const start = fileURLToPath(new URL("./links/plugin-reg/🔎️discovery/", import.meta.url));
const taxonomyPath = resolveTaxonomy(start);
const taxonomy = JSON.parse(readFileSync(taxonomyPath, "utf8"));
writeFileSync(new URL("./generated/taxonomy-path.txt", import.meta.url), taxonomyPath + "\n");
writeFileSync(new URL("./generated/plugin-area-raw.json", import.meta.url), JSON.stringify({ path: taxonomyPath, area: taxonomy.areas["✏️s/🔌️plugins"], pluginAreas: taxonomy.pluginAreas }, null, 2));
console.log(taxonomyPath);
console.log(JSON.stringify(taxonomy.areas["✏️s/🔌️plugins"]));
