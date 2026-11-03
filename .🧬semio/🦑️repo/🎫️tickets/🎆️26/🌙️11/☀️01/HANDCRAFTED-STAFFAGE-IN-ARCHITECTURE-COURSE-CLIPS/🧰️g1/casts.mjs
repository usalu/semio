import { writeFileSync } from "node:fs";

const [out, asset, kind, names, scales, views, env] = process.argv.slice(2);
const casts = {};
for (const name of names.split(","))
  for (const scale of scales.split(",").map(Number))
    for (const view of views.split(",")) {
      const entry = { asset, [kind]: name, view, theme: "dark", viewpoint: "elevation", scale, language: "en" };
      if (view === "thermal") entry.environment = env ?? "office-neutral";
      if (kind === "clip") entry.fps = 15;
      casts[`${asset}-${name}-${scale}-${view}`] = entry;
    }
writeFileSync(out, JSON.stringify({ version: 1, casts }, null, 2));
