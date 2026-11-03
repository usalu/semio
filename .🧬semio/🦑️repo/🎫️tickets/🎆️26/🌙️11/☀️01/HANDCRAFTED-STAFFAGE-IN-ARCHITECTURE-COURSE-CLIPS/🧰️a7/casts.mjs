import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

const out = process.argv[2];
const casts = {};
for (const pose of ["idle", "full-load"]) {
  for (const scale of [50, 100, 200]) {
    for (const view of ["dark-line", "dark-solid", "thermal"]) {
      casts[`rack-${pose}-${scale}-${view}`] = { asset: "server-rack", pose, view, theme: "dark", viewpoint: "elevation", scale, ...(view === "thermal" ? { environment: "office-neutral" } : {}), language: "en" };
    }
  }
}
mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, JSON.stringify({ version: 1, casts }, null, 2));
console.log(Object.keys(casts).length, "casts");
