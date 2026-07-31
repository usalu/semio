import { readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");

function stripTsModule(src: string, name: string): string {
  const lines = src.split(/\r?\n/);
  let i = 0;
  if (lines[i]?.includes("#region") && lines[i]?.includes("Header")) {
    i++;
    while (i < lines.length && !lines[i]?.includes("#endregion")) i++;
    i++;
  }
  const out: string[] = [];
  for (; i < lines.length; i++) {
    const line = lines[i]!;
    if (/^import\s+.*from\s+["']\.\/[^"']+["'];?\s*$/.test(line.trim())) continue;
    if (/^export\s+\{[^}]+\}\s+from\s+["']\.\/[^"']+["'];?\s*$/.test(line.trim())) continue;
    out.push(line);
  }
  return `//#region 🔖️${name}\n${out.join("\n").trim()}\n//#endregion 🔖️${name}\n`;
}

function mergeTsSatellites(indexPath: string, satellites: string[]): void {
  let index = readFileSync(indexPath, "utf8");
  const dir = dirname(indexPath);
  for (const sat of satellites) {
    const satPath = join(dir, sat);
    const name = sat.replace(/\.tsx?$/, "");
    const body = stripTsModule(readFileSync(satPath, "utf8"), name);
    index = index.replace(new RegExp(`export \\* from ["']\\.\\/${name}\\.ts["'];?\\s*`, "g"), "");
    index = index.replace(new RegExp(`export \\{[^}]+\\} from ["']\\.\\/${name}\\.ts["'];?\\s*`, "g"), "");
    const importBlock = new RegExp(`import[^;]*from ["']\\.\\/${name}\\.tsx?["'];?\\s*`, "g");
    index = index.replace(importBlock, "");
    if (!index.includes(`//#region 🔖️${name}`)) index += `\n${body}`;
    unlinkSync(satPath);
  }
  writeFileSync(indexPath, index);
  console.log(`merged ${satellites.join(", ")} → ${indexPath}`);
}

function inlineRustMod(libPath: string, modNames: string[]): void {
  let lib = readFileSync(libPath, "utf8");
  const dir = dirname(libPath);
  for (const modName of modNames) {
    const modPath = join(dir, `${modName}.rs`);
    const body = readFileSync(modPath, "utf8").trimEnd();
    const inline = `pub mod ${modName} {\n// #region ${modName}\n${body}\n// #endregion ${modName}\n}\n`;
    const decl = `mod ${modName};`;
    const pubDecl = `pub mod ${modName};`;
    const pathDecl = new RegExp(`#\\[path = "${modName}\\.rs"\\]\\s*\\n(?:pub )?mod ${modName};`, "m");
    if (pathDecl.test(lib)) {
      lib = lib.replace(pathDecl, inline.trimEnd());
    } else if (lib.includes(decl)) {
      lib = lib.replace(decl, inline);
    } else if (lib.includes(pubDecl)) {
      lib = lib.replace(pubDecl, inline);
    } else {
      throw new Error(`${libPath}: missing mod ${modName}`);
    }
    unlinkSync(modPath);
  }
  writeFileSync(libPath, lib);
  console.log(`inlined ${modNames.join(", ")} in ${libPath}`);
}

function mergePySatellite(mainPath: string, satellite: string): void {
  const dir = dirname(mainPath);
  const satPath = join(dir, satellite);
  const main = readFileSync(mainPath, "utf8");
  const body = readFileSync(satPath, "utf8").trimEnd();
  const name = satellite.replace(/\.py$/, "");
  if (main.includes(body.slice(0, 80))) {
    unlinkSync(satPath);
    console.log(`skipped duplicate py merge ${satellite}`);
    return;
  }
  const merged = `${main.trimEnd()}\n\n# #region ${name}\n${body}\n# #endregion ${name}\n`;
  writeFileSync(mainPath, merged);
  unlinkSync(satPath);
  console.log(`merged ${satellite} → ${mainPath}`);
}

function mergeGoSatellite(mainPath: string, satellite: string): void {
  const dir = dirname(mainPath);
  const satPath = join(dir, satellite);
  const main = readFileSync(mainPath, "utf8");
  const body = readFileSync(satPath, "utf8").trimEnd();
  const name = satellite.replace(/\.go$/, "");
  const merged = `${main.trimEnd()}\n\n// #region ${name}\n${body}\n// #endregion ${name}\n`;
  writeFileSync(mainPath, merged);
  unlinkSync(satPath);
  console.log(`merged ${satellite} → ${mainPath}`);
}

const validateBody = stripTsModule(readFileSync(join(root, "mathematical/graph/manifest/core/validate.ts"), "utf8"), "validate");
writeFileSync(join(root, "mathematical/graph/manifest/core/index.ts"), `/** @emoji 📜️ \`@semio-tech/graph-manifest\` — compile-time graph manifest types and catalog projection. */\nexport * from "../generated/index.ts";\n${validateBody}`);
unlinkSync(join(root, "mathematical/graph/manifest/core/validate.ts"));

mergeTsSatellites(join(root, "ui/styling/js/index.ts"), ["sizing.ts", "resolve.ts", "icon-render-port.ts"]);
const uiPkg = JSON.parse(readFileSync(join(root, "ui/styling/js/package.json"), "utf8"));
delete uiPkg.exports["./resolve"];
delete uiPkg.exports["./icon-render-port"];
writeFileSync(join(root, "ui/styling/js/package.json"), `${JSON.stringify(uiPkg, null, 2)}\n`);

mergeTsSatellites(join(root, "semios/core/index.ts"), ["rust-studio.ts"]);

mergeTsSatellites(join(root, "vcs/play/index.ts"), ["demo.ts"]);

mergeTsSatellites(join(root, "framework/product/presentation/renderer/react/index.tsx"), ["json.tsx"]);

const specPath = join(root, "mit-bestand/präsentation/33.projektetage/spec.ts");
const specIndex = join(root, "mit-bestand/präsentation/33.projektetage/index.ts");
let specIndexContent = readFileSync(specIndex, "utf8");
const specBody = stripTsModule(readFileSync(specPath, "utf8"), "spec");
specIndexContent = specIndexContent.replace(/import \{ presentationMeta \} from ["']\.\/spec\.ts["'];?\s*/g, "");
specIndexContent = specIndexContent.replace(/import \{([^}]+)\} from ["']\.\/spec\.ts["'];?\s*/g, "");
if (!specIndexContent.includes("//#region 🔖️spec")) specIndexContent += `\n${specBody}`;
writeFileSync(specIndex, specIndexContent);
unlinkSync(specPath);

inlineRustMod(join(root, "infinite/cavas/rs/lib.rs"), ["theme"]);
const iconCodecPath = join(root, "infinite/cavas/rs/icon_codec.rs");
if (readFileSync(iconCodecPath, "utf8").length > 0) {
  unlinkSync(iconCodecPath);
  console.log("removed orphaned infinite/cavas/rs/icon_codec.rs (already inlined in lib.rs)");
}

inlineRustMod(join(root, "kernel/2d/rs/lib.rs"), ["booleans", "trace"]);
inlineRustMod(join(root, "kernel/2d/engine/lib.rs"), ["compute"]);
inlineRustMod(join(root, "kernel/3d/engine/lib.rs"), ["compute"]);

mergePySatellite(join(root, "compose/client/lib/py/main.py"), "store.py");
mergePySatellite(join(root, "coda/client/bin/assistant/main.py"), "reference.py");
mergeGoSatellite(join(root, "compose/client/lib/go/main.go"), "kit_graph.go");

console.log("merge-scattered-files complete");
