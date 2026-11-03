import { load, save } from "./lib.ts";
for (const id of process.argv.slice(2)) {
  const d = load(id);
  let n = 0;
  const walk = (v: any): void => {
    if (Array.isArray(v)) return v.forEach(walk);
    if (v && typeof v === "object") {
      if (v.solid?.kind === "extruded" && v.solid.sweep === undefined) { v.solid.sweep = "w"; n++; }
      Object.values(v).forEach(walk);
    }
  };
  walk(d.asset);
  if (n) save(id, d);
  console.log(id, n);
}
