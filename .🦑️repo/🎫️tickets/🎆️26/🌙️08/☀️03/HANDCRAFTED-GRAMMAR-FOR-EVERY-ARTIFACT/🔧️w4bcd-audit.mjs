import { existsSync, readdirSync, readFileSync, statSync } from "fs";
import { join } from "path";

const pluginsRoot = "/Users/ueli/Documents/semio/✏️s/🔌️plugins";
const keys = ["norm","raster","animate","lowpoly","draw","layout","shooting","remodel","note","fem","architect","process","playbook","gis","forms","imperative","space","sourcing","writer"];
const owned2 = readdirSync(pluginsRoot).filter(p => {
  const ascii = p.replace(/[^\x00-\x7f]/g,"");
  return keys.some(k => ascii.includes(k));
});
console.log("matched plugins", owned2.join(", "));

function listDirs(d){ if(!existsSync(d)) return []; return readdirSync(d).filter(n=>statSync(join(d,n)).isDirectory()); }

const genericMarkers = [
  'field = IDENT "=" (TEXT | fence',
  'document = field*',
  'operation = field*',
  'diff = field*',
  'layer = IDENT "@" FLOAT',
  'document = TEXT*',
  'stock = "stock" slash-path',
  'feature = point | polygon',
  'step = IDENT ":" IDENT "("',
  'document = (header | clause | table | assign)*',
  'document = statement*',
  'operation = section*\nsection = ("',
];

const weak = [];
const all = [];
for (const plugin of owned2) {
  for (const art of listDirs(join(pluginsRoot, plugin, "🗿️artifacts"))) {
    for (const facet of ["🗣️dsl","🔧️op","🔺️diff","🎒️pack","📡️spr"]) {
      const dir = join(pluginsRoot, plugin, "🗿️artifacts", art, facet);
      if (!existsSync(dir)) continue;
      const isProto = facet === "🎒️pack" || facet === "📡️spr";
      const name = isProto ? "📡️component.protocol.semio" : "📖️component.grammar.semio";
      const path = join(dir, name);
      if (!existsSync(path)) continue;
      const body = readFileSync(path, "utf8");
      all.push({plugin, art, facet, path, isProto, body});
      if (!isProto) {
        const hit = genericMarkers.find(m => body.includes(m));
        if (hit) weak.push({path: path.replace(pluginsRoot+"/",""), hit});
      } else {
        if (!body.includes("schema ")) weak.push({path: path.replace(pluginsRoot+"/",""), hit: "protocol missing schema"});
      }
    }
  }
}
console.log("total facet specs", all.length);
console.log("weak", weak.length);
for (const w of weak) console.log(w.hit, "=>", w.path);

// also check op component.rs for writer
