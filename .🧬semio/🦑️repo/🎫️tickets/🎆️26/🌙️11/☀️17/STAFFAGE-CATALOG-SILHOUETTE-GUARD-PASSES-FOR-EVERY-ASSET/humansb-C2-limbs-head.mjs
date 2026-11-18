import { readFileSync } from "node:fs";
export default (a) => {
  const h = JSON.parse(readFileSync(process.env.HEADJSON, "utf8")).asset;
  const only = (process.env.ONLY ?? "50:front,100:front,200:front,50:plan").split(",");
  const ids = (process.env.IDS ?? "thigh-l,thigh-r,shank-l,shank-r,knee-l,knee-r,foot-l,foot-r,toe-l,toe-r").split(",");
  for (const o of only) {
    const [sc, v] = o.split(":");
    const g = a.representations[sc].geometry[v], hg = h.representations[sc].geometry[v];
    for (const id of ids) {
      const i = g.findIndex((s) => s.id === id), j = hg.findIndex((s) => s.id === id);
      if (i >= 0 && j >= 0 && g[i].solid === undefined) g[i] = JSON.parse(JSON.stringify(hg[j]));
    }
  }
};
