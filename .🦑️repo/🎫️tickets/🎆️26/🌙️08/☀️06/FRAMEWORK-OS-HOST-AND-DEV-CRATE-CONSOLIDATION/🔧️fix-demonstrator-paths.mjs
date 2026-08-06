import fs from "node:fs";
import path from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const FW = fs.readdirSync(ROOT).find((n) => n.includes("framework") && fs.statSync(path.join(ROOT, n)).isDirectory());

function findDir(parent, pred) {
  return fs.readdirSync(parent).map((n) => path.join(parent, n)).find((p) => pred(path.basename(p), p));
}

function existsRel(rel) {
  const p = path.join(ROOT, rel.replace(/^\.\//, ""));
  return { ok: fs.existsSync(p), p };
}

const modules = path.join(ROOT, FW, "🛍️products/💻️os/🔨️modules");
const DEV = findDir(modules, (b) => b.includes("dev"));
const plugin = findDir(modules, (b) => b.includes("plugin"));
const inf = findDir(modules, (b) => b.includes("infinite"));
const canvas = findDir(inf, (b) => b.includes("canvas"));
const world = findDir(inf, (b) => b.includes("world"));
const r3f = findDir(world, (b) => b.includes("r3f"));
const ui = findDir(path.join(ROOT, FW, "🔨️modules"), (b) => b.includes("ui"));
const assets = findDir(path.join(ROOT, FW, "🔨️modules"), (b) => b.includes("assets"));

const targets = {
  registryPlaygrounds: path.join(plugin, "📦️packages/🟦️typescript", findDir(path.join(plugin, "📦️packages/🟦️typescript"), (b) => b.includes("registry")) && "", "…"),
};

const regTs = path.join(plugin, "📦️packages/🟦️typescript");
const reg = findDir(regTs, (b) => b.includes("registry"));
const gen = findDir(reg, (b) => b.includes("generated"));
const playgrounds = path.join(gen, fs.readdirSync(gen).find((n) => n.includes("playgrounds") && n.endsWith(".ts")));
const osDevScript = path.join(DEV, "📦️packages/🟦️typescript/📜️script.ts");
const pluginModules = path.join(DEV, "🔌️plugin-modules");

const canvasReact = (() => {
  const rr = findDir(canvas, (b) => b.includes("react-renderer"));
  const pkg = path.join(rr, "📦️packages/🟦️typescript");
  const entry = fs.existsSync(path.join(pkg, "📦️index.tsx"))
    ? path.join(pkg, "📦️index.tsx")
    : fs.existsSync(path.join(pkg, "🟦️glue.ts"))
      ? path.join(pkg, "🟦️glue.ts")
      : fs.existsSync(path.join(pkg, "🟦️glue.tsx"))
        ? path.join(pkg, "🟦️glue.tsx")
        : pkg;
  return entry;
})();

const worldR3f = (() => {
  const pkg = path.join(r3f, "📦️packages/🟦️typescript");
  for (const name of ["📦️index.tsx", "🟦️glue.tsx", "🟦️glue.ts", "📦️index.ts"]) {
    const p = path.join(pkg, name);
    if (fs.existsSync(p)) return p;
  }
  return pkg;
})();

const fwCore = (() => {
  const pkg = path.join(ROOT, FW, "📦️packages/🟦️typescript");
  for (const name of ["🟦️glue.ts", "📦️index.ts", "index.ts"]) {
    const p = path.join(pkg, name);
    if (fs.existsSync(p)) return p;
  }
  return pkg;
})();

const osCore = (() => {
  const pkg = path.join(ROOT, FW, "🛍️products/💻️os/📦️packages/🟦️typescript");
  for (const name of ["🟦️glue.ts", "📦️index.ts", "index.ts"]) {
    const p = path.join(pkg, name);
    if (fs.existsSync(p)) return p;
  }
  return pkg;
})();

const uiReact = (() => {
  // prefer os-dev's path style
  const a = path.join(ui, "⚛️react/📦️packages/🟦️typescript/📦️index.tsx");
  const b = path.join(ui, "📦️packages/