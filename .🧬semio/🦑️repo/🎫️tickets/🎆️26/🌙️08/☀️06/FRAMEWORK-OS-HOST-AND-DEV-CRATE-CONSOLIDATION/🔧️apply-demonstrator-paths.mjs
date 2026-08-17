
import fs from "node:fs";
import path from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const TICK = process.argv[2];
const targets = JSON.parse(fs.readFileSync(path.join(TICK, "🧪demonstrator-path-targets.json"), "utf8"));
const dem = path.join(ROOT, "♻️mit-bestand/🧺️demonstrator");

function toDemRel(repoRel) {
  const abs = path.join(ROOT, repoRel.replace(/^\.\//, ""));
  return path.relative(dem, abs).split(path.sep).join("/");
}

const paths = {
  playgrounds: toDemRel(targets.playgrounds),
  osDevScript: toDemRel(targets.osDevScript),
  pluginModulesFromPlay: path.relative(dem, path.join(ROOT, targets.pluginModules.replace(/^\.\//, ""))).split(path.sep).join("/"),
  canvas: targets.canvasEntry.replace(/^\.\//, "./"),
  world: targets.worldEntry.replace(/^\.\//, "./"),
  fwCore: targets.fwCore.replace(/^\.\//, "./"),
  osCore: targets.osCore.replace(/^\.\//, "./"),
};

console.log(JSON.stringify(paths, null, 2));

{
  let t = fs.readFileSync(path.join(dem, "📜️script.ts"), "utf8");
  const before = t;
  t = t.replace(
    /import \{ buildEngineWasm, buildPlugins, ensurePluginRegistry \} from "[^"]+";/,
    `import { buildEngineWasm, buildPlugins, ensurePluginRegistry } from "${paths.osDevScript}";`,
  );
  if (t === before) throw new Error("script.ts import not replaced");
  fs.writeFileSync(path.join(dem, "📜️script.ts"), t);
  console.log("fixed script.ts");
}

{
  let t = fs.readFileSync(path.join(dem, "⚙️vite.config.ts"), "utf8");
  t = t.replace(
    /import \{ PLAYGROUND_BUILD_TARGETS \} from "[^"]+";/,
    `import { PLAYGROUND_BUILD_TARGETS } from "${paths.playgrounds}";`,
  );
  t = t.replace(
    /import \{ semioBackboneVitePlugin, semioBlobVitePlugin \} from "[^"]+";/,
    `import { semioBackboneVitePlugin, semioBlobVitePlugin } from "${paths.osDevScript}";`,
  );
  t = t.replace(
    /const pluginModulesDir = path\.join\(playDir, "[^"]+"\);/,
    `const pluginModulesDir = path.join(playDir, "${paths.pluginModulesFromPlay}");`,
  );

  function setAlias(pkg, replacement) {
    const re = new RegExp(
      `(\\{ find: "${pkg.replace(/[.*+?^${}()|[\\]\\\\]/g, "\\$&")}", replacement: path\\.resolve\\(repoRoot, ")[^"]+("\\) \\})`,
    );
    if (!re.test(t)) {
      console.warn("alias not found:", pkg);
      return;
    }
    t = t.replace(re, `$1${replacement}$2`);
  }

  setAlias("@semio-tech/infinite-canvas-react-renderer", paths.canvas);
  setAlias("@semio-tech/infinite-world-r3f", paths.world);
  setAlias("@semio-tech/framework-core", paths.fwCore);
  setAlias("@semio-tech/framework-os-core", paths.osCore);

  const still = t.split("\n").filter((l) => l.includes("⚡️implementations") && (l.includes("import ") || l.includes("find:") || l.includes("pluginModulesDir") || l.includes("replacement:")));
  fs.writeFileSync(path.join(dem, "⚙️vite.config.ts"), t);
  console.log("fixed vite.config.ts; remaining implementations refs:", still.length);
  if (still.length) console.log(still.join("\n"));
}

{
  const FW = fs.readdirSync(ROOT).find((n) => n.includes("framework") && fs.statSync(path.join(ROOT, n)).isDirectory());
  const modules = path.join(ROOT, FW, "🛍️products/💻️os/🔨️modules");
  const DEV = fs.readdirSync(modules).map((n) => path.join(modules, n)).find((p) => path.basename(p).includes("dev"));
  const vite = path.join(DEV, "📦️packages/🟦️typescript/⚙️vite.config.ts");
  let t = fs.readFileSync(vite, "utf8");
  let changed = false;
  if (t.includes('@semio-tech/framework-core') && /framework-core", replacement: path\.resolve\(repoRoot, "[^"]*📦️index\.ts"/.test(t)) {
    t = t.replace(/(\{ find: "@semio-tech\/framework-core", replacement: path\.resolve\(repoRoot, ")[^"]+/, `$1${targets.fwCore}`);
    changed = true;
  }
  if (t.includes('@semio-tech/framework-os-core') && /framework-os-core", replacement: path\.resolve\(repoRoot, "[^"]*📦️index\.ts"/.test(t)) {
    t = t.replace(/(\{ find: "@semio-tech\/framework-os-core", replacement: path\.resolve\(repoRoot, ")[^"]+/, `$1${targets.osCore}`);
    changed = true;
  }
  if (changed) {
    fs.writeFileSync(vite, t);
    console.log("fixed os-dev vite core aliases");
  } else {
    console.log("os-dev vite core aliases ok or already glue");
  }
}
