import { mkdirSync } from "node:fs";
import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
const out = process.argv[process.argv.findIndex((x) => x.endsWith("crops.ts")) + 1]!;
mkdirSync(out, { recursive: true });
const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances";
const jobs: [string, number, string[], string?][] = [
  ["room-thermostat", 50, ["--pose", "idle"]], ["room-thermostat", 50, ["--pose", "set-high"]], ["room-thermostat", 50, ["--pose", "calling"]],
  ["thermostatic-radiator-valve", 50, ["--pose", "open"]], ["thermostatic-radiator-valve", 50, ["--pose", "closed"]],
  ["shower-head", 50, ["--pose", "off"]], ["shower-head", 50, ["--pose", "running"]],
  ["pendant-lamp", 50, ["--pose", "off"]], ["pendant-lamp", 50, ["--pose", "on"]], ["pendant-lamp", 50, ["--pose", "off"], "thermal"], ["pendant-lamp", 50, ["--pose", "on"], "thermal"],
  ["hot-water-cylinder", 50, ["--pose", "cold"]], ["hot-water-cylinder", 50, ["--pose", "charged"]], ["hot-water-cylinder", 50, ["--pose", "cold"], "thermal"], ["hot-water-cylinder", 50, ["--pose", "charged"], "thermal"],
  ["laptop", 50, ["--pose", "open"]], ["laptop", 50, ["--pose", "closed"]], ["laptop", 100, ["--pose", "open"]],
  ["vacuum-cleaner", 100, ["--pose", "running"]], ["vacuum-cleaner", 200, ["--pose", "parked"]],
];
for (const [a, s, q, v] of jobs) {
  try { await runPreview(["--file", `${root}/${a}/🔣️.json`, "--scale", String(s), "--views", "--no-png", ...(v ? ["--view", v] : []), "--out", out, ...q]); }
  catch (e) { console.log("ERR", a, s, q.join(" "), String(e).slice(0, 200)); }
}
console.log("crops done");
