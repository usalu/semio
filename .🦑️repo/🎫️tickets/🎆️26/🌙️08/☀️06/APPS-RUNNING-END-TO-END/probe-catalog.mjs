import { readdirSync, readFileSync, existsSync, statSync, writeFileSync } from "fs";
import { join } from "path";

const root = process.cwd();
const out = [];
function log(...a){ out.push(a.map(String).join(" ")); console.log(...a); }

const fw = readdirSync(root).find((n) => n.includes("framework"));
log("fw", JSON.stringify(fw));
const products = readdirSync(join(root, fw)).find((n) => n.includes("products"));
const osn = readdirSync(join(root, fw, products)).find((n) => n.endsWith("os"));
const modules = readdirSync(join(root, fw, products, osn)).find((n) => n.includes("modules"));
const plugin = readdirSync(join(root, fw, products, osn, modules)).find((n) => n.includes("plugin"));
const packages = readdirSync(join(root, fw, products, osn, modules, plugin)).find((n) => n.includes("packages"));
const ts = readdirSync(join(root, fw, products, osn, modules, plugin, packages)).find((n) => n.includes("typescript"));
const registry = readdirSync(join(root, fw, products, osn, modules, plugin, packages, ts)).find((n) => n.includes("registry"));
const reg = join(root, fw, products, osn, modules, plugin, packages, ts, registry);
log("registry", reg);
log("ents", readdirSync(reg).join(", "));
const gen = readdirSync(reg).find((n) => n.includes("generated"));
log("generated", JSON.stringify(gen));
let fem = [];
let rows = 0;
if (gen) {
  const gp = join(reg, gen);
  log("gen ents", readdirSync(gp).join(", "));
  for (const f of readdirSync(gp)) {
    if (f.includes("playgrounds") && f.endsWith(".json")) {
      const j = JSON.parse(readFileSync(join(gp, f), "utf8"));
      rows = j.length;
      fem = j.filter((r) => String(r.variant || "").includes("fem") || (r.aliases || []).some((a) => String(a).includes("fem")));
      log("rows", rows);
      log("fem", JSON.stringify(fem, null, 2));
    }
  }
} else {
  log("MISSING GENERATED");
}

// verify catalogPath from repo-lib index
const repo = readdirSync(join(root, fw, products)).find((n) => n.includes("repo"));
const rmod = readdirSync(join(root, fw, products, repo)).find((n) => n.includes("modules"));
const lib = readdirSync(join(root, fw, products, repo, rmod)).find((n) => n.includes("lib"));
const lpkg = readdirSync(join(root, fw, products, repo, rmod, lib)).find((n) => n.includes("packages"));
const lts = readdirSync(join(root, fw, products, repo, rmod, lib, lpkg)).find((n) => n.includes("typescript"));
const idxdir = join(root, fw, products, repo, rmod, lib, lpkg, lts);
const idx = readdirSync(idxdir).find((n) => n.endsWith("index.ts") && !n.includes("test"));
const src = readFileSync(join(idxdir, idx), "utf8");
const m = src.match(/catalogPath = join\([^,]+,\s*"([^"]+)"/);
log("literal", m?.[1]);
if (m) {
  const full = join(root, m[1].replace(/^\.\//, ""));
  log("full exists?", existsSync(full), full);
}

// also try importing the load function
const { pathToFileURL } = await import("url");
const mod = await import(pathToFileURL(join(idxdir, idx)).href);
const catalog = mod.loadFrameworkOsPlaygroundCatalog();
log("loaded catalog length", catalog.length);
const resolved = mod.resolveFrameworkOsPlaygroundPlugin(catalog, ["fem", "3d"]);
log("resolve fem 3d", JSON.stringify(resolved));

writeFileSync(join(process.env.TICKET || ".", "probe-catalog-result.json"), JSON.stringify({ rows, fem, resolved, catalogLen: catalog.length, generated: !!gen }, null, 2));
