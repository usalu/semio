import fs from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const libRs = join(here, "..", "..", "..", "..", "..", "..", "elements", "client", "lib", "board", "rs", "lib.rs");
let s = fs.readFileSync(libRs, "utf8");
for (const name of ["pointer_move_screen", "pointer_up_screen"]) {
  const re = new RegExp(`h\\.${name}\\(([^)]+)\\)`, "g");
  s = s.replace(re, (m, inner) => {
    if (/, false, false$/.test(inner.trim())) return m;
    return `h.${name}(${inner}, false, false)`;
  });
}
fs.writeFileSync(libRs, s);
