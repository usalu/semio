import { readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { dirname, join } from "node:path";

function inlineMods(libPath: string, modNames: string[]): void {
  let lib = readFileSync(libPath, "utf8");
  const dir = dirname(libPath);
  for (const modName of modNames) {
    const modPath = join(dir, `${modName}.rs`);
    const body = readFileSync(modPath, "utf8").trimEnd();
    const inline = `pub mod ${modName} {\n// #region ${modName}\n${body}\n// #endregion ${modName}\n}\n`;
    const decl = `pub mod ${modName};`;
    if (!lib.includes(decl)) throw new Error(`${libPath}: missing ${decl}`);
    lib = lib.replace(decl, inline);
    unlinkSync(modPath);
  }
  writeFileSync(libPath, lib);
  console.log(`inlined ${modNames.join(", ")} in ${libPath}`);
}

const root = join(import.meta.dirname, "../../../../../../");
inlineMods(join(root, "mathematical/graph/lib.rs"), ["geometry", "scene_json"]);
inlineMods(join(root, "mathematical/graph/port/directed/lib.rs"), ["scene_json", "types"]);
inlineMods(join(root, "mathematical/graph/port/directed/normal/lib.rs"), ["board_host"]);
inlineMods(join(root, "mathematical/graph/normal/undirected/lib.rs"), ["fixture_layout"]);
inlineMods(join(root, "infinite/canvas/vello/lib.rs"), ["icon_codec", "theme"]);
