import { createRequire } from "node:module";
import { realpathSync } from "node:fs";
import { join } from "node:path";
const root = new URL("../../../../../../../../../", import.meta.url).pathname.replace(/\/$/, "");
const from = {
  demonstrator: join(root, "♻️mit-bestand/🧺️demonstrator/x.js"),
  uiReact: join(root, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/x.js"),
  hubAdmin: join(root, "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/x.js"),
  plugins: join(root, "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/x.js"),
};
const rows = [];
for (const name of ["react", "react-dom", "@types/react", "@types/react-dom"]) for (const [who, file] of Object.entries(from)) {
  try { const p = createRequire(file).resolve(name + (name.startsWith("@types") ? "/package.json" : "/package.json")); rows.push(`${name}\t${who}\t${realpathSync(p)}`); } catch (e) { rows.push(`${name}\t${who}\tUNRESOLVED ${e.code}`); }
}
console.log(rows.join("\n"));
const byName = new Map();
for (const r of rows) { const [n, , p] = r.split("\t"); (byName.get(n) ?? byName.set(n, new Set()).get(n)).add(p); }
for (const [n, s] of byName) console.log(`${n}: ${s.size} physical copy/copies`);
