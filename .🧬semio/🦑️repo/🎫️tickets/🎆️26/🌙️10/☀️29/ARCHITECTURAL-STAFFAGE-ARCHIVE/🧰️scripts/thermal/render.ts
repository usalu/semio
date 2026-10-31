import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { catalogAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🌡️thermal/🟦️.ts";
import environments from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌤️environments/🔣️.json" with { type: "json" };
import { defaultDepictionOptions, depict } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { UNCHECKED_PROFILE } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🧱️synthetic-figure/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermal } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";
import { writeSvg } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🖼️svg/🟦️.ts";

const out = process.argv[2]!;
mkdirSync(out, { recursive: true });
const env = (id: string) => environments.presets.find((p) => p.id === id)!.environment as any;
const jobs: [string, string, string, number, string, string, string?][] = [
  ["walking-commuter", "standing", "winter-overcast", 50, "surface-temperature", "shared"],
  ["walking-commuter", "standing", "winter-overcast", 50, "skin-temperature", "shared"],
  ["walking-commuter", "standing", "winter-overcast", 50, "net-radiative-flux", "shared"],
  ["walking-commuter", "standing", "winter-overcast", 50, "sensible-heat-flux", "auto"],
  ["walking-commuter", "standing", "summer-hot-sun", 100, "latent-heat-flux", "shared"],
  ["walking-commuter", "standing", "summer-hot-sun", 500, "surface-temperature", "shared"],
  ["english-oak", "calm", "summer-hot-sun", 200, "latent-heat-flux", "shared"],
  ["english-oak", "calm", "summer-hot-sun", 200, "moisture-flux", "shared"],
  ["city-car", "driving", "spring-mild-sun", 100, "surface-temperature", "shared"],
  ["city-car", "driving", "spring-mild-sun", 100, "emitted-radiant-flux", "shared"],
  ["labrador-dog", "standing-alert", "summer-hot-sun", 50, "surface-temperature", "shared"],
];
for (const [id, pose, preset, denominator, quantity, mode] of jobs) try {
  const asset = catalogAsset(id);
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator } });
  const field = solveThermal(state, env(preset));
  const o = { ...defaultDepictionOptions("thermal", "en", denominator), thermalQuantity: quantity as any, range: { mode } as any, patterns: true, profile: UNCHECKED_PROFILE, environment: env(preset) };
  const scene = depict(state, o, { thermal: field });
  const name = `${id}-${pose}-${preset}-1-${denominator}-${quantity}-${mode}`;
  writeFileSync(`${out}/${name}.svg`, writeSvg(scene));
  execFileSync("rsvg-convert", ["-d", "200", "-p", "200", "-f", "png", "-o", `${out}/${name}.png`, `${out}/${name}.svg`]);
  console.log("[DEBUG]", name);
} catch (e) {
  console.log("[DEBUG] FAILED", id, denominator, (e as Error).message.slice(0, 200));
}
