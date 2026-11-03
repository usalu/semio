import { load, save, rep, setShape } from "./lib.ts";
const d = load("city-car");
const src = rep(d, 50).geometry;
for (const k of [100, 200]) {
  const G = rep(d, k).geometry;
  for (const v of ["elevation", "front", "plan"]) for (const s of src[v]) if (/^(seat-|steering-wheel)/.test(s.id)) setShape(G[v], structuredClone(s));
  const dec = rep(d, k).decisions;
  console.log(k, "DE omitted:", dec.omitted.de, "\n   DE retained:", dec.retained.de.slice(0, 120));
}
save("city-car", d);
