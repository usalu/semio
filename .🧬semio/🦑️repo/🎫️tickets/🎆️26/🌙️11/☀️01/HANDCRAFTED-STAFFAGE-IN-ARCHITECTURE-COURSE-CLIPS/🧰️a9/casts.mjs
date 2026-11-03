import { writeFileSync } from "node:fs";

const out = process.argv[2];
const asset = process.argv[3];
const poses = process.argv[4].split(",");
const scales = process.argv[5].split(",").map(Number);
const views = process.argv[6].split(",");
const env = process.argv[7] ?? "office-neutral";
const casts = {};
for (const pose of poses)
  for (const scale of scales)
    for (const view of views)
      casts[`${asset}-${pose}-${scale}-${view}`] = { asset, pose, view, theme: "dark", viewpoint: "elevation", scale, ...(view === "thermal" ? { environment: env } : {}), language: "en" };
writeFileSync(out, JSON.stringify({ version: 1, casts }, null, 2));
