import { writeFileSync } from "node:fs";

const out = process.argv[2];
const plan = {
  laptop: { poses: ["open", "closed"], scales: [50, 100, 200] },
  "desktop-monitor": { poses: ["on", "off"], scales: [50, 100, 200] },
  "pendant-lamp": { poses: ["on", "off"], scales: [50, 100, 200] },
};
const views = ["dark-line", "dark-solid", "thermal", "forces", "moments"];
const casts = {};
for (const [asset, { poses, scales }] of Object.entries(plan)) {
  for (const pose of poses) {
    for (const scale of scales) {
      for (const view of views) {
        if (scale !== 50 && !["dark-line", "thermal"].includes(view)) continue;
        casts[`${asset}-${pose}-${scale}-${view}`] = { asset, pose, view, theme: "dark", viewpoint: "elevation", scale, ...(view === "thermal" ? { environment: "office-neutral" } : {}), language: "en" };
      }
    }
  }
}
writeFileSync(out, JSON.stringify({ version: 1, casts }, null, 2));
console.log(Object.keys(casts).length, "casts");
