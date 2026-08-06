import { readFileSync, existsSync, readdirSync, statSync } from "fs";
import { join } from "path";
const pluginsRoot = join(import.meta.dir, "../../../../../..", "✏️s/🔌️plugins");
const owned = [
  ["🖨️raster", "🖨️raster", "scene"],
  ["🎞️animate", "🎬️present", "scene"],
  ["💠️lowpoly", "💠️lowpoly", "scene"],
  ["🖍️draw", "🖍️draw", "scene"],
  ["📏️layout", "📏️layout", "scene"],
  ["🎥️shooting", "🎥️shooting", "scene"],
  ["📸️remodel", "📸️remodel", "scene"],
  ["🗒️note", "🗒️note", "embed"],
];
const facets = [
  ["🗣️dsl", "📖️component.grammar.semio"],
  ["🔧️op", "📖️component.grammar.semio"],
  ["🔺️diff", "📖️component.grammar.semio"],
  ["🎒️pack", "📡️component.protocol.semio"],
  ["📡️spr", "📡️component.protocol.semio"],
];
let semio = 0, ts = 0, barrels = 0;
const issues = [];
for (const [plugin, artifact, family] of owned) {
  const base = join(pluginsRoot, plugin, "🗿️artifacts", artifact);
  for (const [facet, name] of facets) {
    const g = join(base, facet, name);
    const t = join(base, facet, "🟦️component.ts");
    if (!existsSync(g)) issues.push("missing " + g);
    else {
      semio++;
      const body = readFileSync(g, "utf8");
      if (!body.includes(`use family-${family}`) && name.includes("grammar")) issues.push("bad family " + g);
      if (body.includes("NULL")) issues.push("NULL " + g);
      if (body.includes('layer = IDENT "@"')) issues.push("generic stub " + g);
      if (body.includes(`_projection bytes`) || body.includes(`_op_payload`)) issues.push("bad protocol " + g);
      if (!body.startsWith("dialect ")) issues.push("no dialect " + g);
    }
    if (!existsSync(t)) issues.push("missing " + t);
    else ts++;
  }
  const idx = join(pluginsRoot, plugin, "📦️packages/🟦️typescript/📦️index.ts");
  if (existsSync(idx)) {
    const b = readFileSync(idx, "utf8");
    if (plugin !== "🎞️animate" && b.includes("parse") && b.includes("encode")) barrels++;
  }
}
console.log(JSON.stringify({ semio, ts, barrels, total: semio + ts + barrels, issues }, null, 2));
// spot check note family and raster pixel
console.log("--- note header ---");
console.log(readFileSync(join(pluginsRoot, "🗒️note/🗿️artifacts/🗒️note/🗣️dsl/📖️component.grammar.semio"), "utf8").split("\n").slice(0, 8).join("\n"));
console.log("--- present pack ---");
console.log(readFileSync(join(pluginsRoot, "🎞️animate/🗿️artifacts/🎬️present/🎒️pack/📡️component.protocol.semio"), "utf8"));
