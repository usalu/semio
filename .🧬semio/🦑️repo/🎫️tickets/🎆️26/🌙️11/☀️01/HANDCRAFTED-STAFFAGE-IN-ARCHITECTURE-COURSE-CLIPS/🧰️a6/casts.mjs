import { writeFileSync } from "node:fs";

const out = process.argv[2];
const which = process.argv[3] ?? "all";
const casts = {};
const add = (id, c) => (casts[id] = { theme: "dark", viewpoint: "elevation", language: "en", ...c });
if (which === "all" || which === "car") {
  const a = "vintage-car-1929";
  for (const scale of [50, 100, 200, 500]) {
    for (const view of ["dark-line", "dark-solid"]) add(`car-parked-${scale}-${view}`, { asset: a, pose: "parked", view, scale });
    add(`car-plan-${scale}-dark-line`, { asset: a, pose: "parked", view: "dark-line", scale, viewpoint: "plan" });
    add(`car-plan-${scale}-dark-solid`, { asset: a, pose: "parked", view: "dark-solid", scale, viewpoint: "plan" });
  }
  add("car-driving-50-thermal", { asset: a, pose: "driving", view: "thermal", scale: 50, environment: "office-neutral" });
  add("car-plan-50-thermal", { asset: a, pose: "driving", view: "thermal", scale: 50, viewpoint: "plan", environment: "office-neutral" });
  add("car-driveby-100", { asset: a, clip: "drive-by", view: "dark-line", scale: 100, fps: 6 });
}
if (which === "all" || which === "ac") {
  const a = "split-ac-indoor-unit";
  for (const scale of [50, 100, 200]) {
    for (const pose of ["off", "cooling"]) {
      for (const view of ["dark-line", "dark-solid", "thermal"]) add(`ac-${pose}-${scale}-${view}`, { asset: a, pose, view, scale, ...(view === "thermal" ? { environment: "office-neutral" } : {}) });
    }
    add(`ac-plan-${scale}-dark-line`, { asset: a, pose: "off", view: "dark-line", scale, viewpoint: "plan" });
  }
  add("ac-clip-50", { asset: a, clip: "start-cooling", view: "thermal", scale: 50, fps: 2, environment: "office-neutral" });
}
writeFileSync(out, JSON.stringify({ version: 1, casts }, null, 2));
console.log(Object.keys(casts).length, "casts");
