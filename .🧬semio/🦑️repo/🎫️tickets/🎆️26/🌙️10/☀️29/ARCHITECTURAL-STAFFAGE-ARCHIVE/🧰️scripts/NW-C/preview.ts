import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const GEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-C";
const pv = await import(`${S}/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { defaultDepictionOptions, depict } = await import(`${S}/🔨️modules/🎨️depiction/🟦️.ts`);
const { writeSvg } = await import(`${S}/🔨️modules/📤️export/🖼️svg/🟦️.ts`);
const { solveStructure } = await import(`${S}/🔨️modules/🏗️structure/🟦️.ts`);
const { solveThermalAt } = await import(`${S}/🔨️modules/🌡️thermal/🟦️.ts`);
const { DEFAULT_PRINT_PROFILE } = await import(`${S}/🔨️modules/🖨️representation/🟦️.ts`);
// usage: preview2.ts <asset> <scales> <modes: print,zoom,views> [pose] [clip@time]
const [id, scales, mode, pose, clipAt] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(`${S}/🗂️catalog/🧱️components/${id}/🔣️.json`, "utf8")).asset;
const out = `${GEN}/${id}`;
mkdirSync(out, { recursive: true });
const views = ["dark-line", "dark-solid", "light-line", "light-solid", "thermal", "forces", "moments"];
const query = clipAt ? { kind: "clip", clip: clipAt.split("@")[0], timeS: Number(clipAt.split("@")[1]) } : { kind: "pose", pose: pose && pose !== "-" ? pose : asset.presentation.defaultPose };
const tag = query.kind === "clip" ? `${query.clip}@${query.timeS}` : query.pose;
for (const sc of scales.split(",")) {
  const den = Number(sc);
  if (asset.representations[sc].status !== "authored") continue;
  const selector = { kind: "scale", scale: { denominator: den } };
  const state = evaluate(asset, query, selector);
  const analysis = { thermal: solveThermalAt(state.asset, query), structure: solveStructure(state) };
  const list = mode.includes("views") ? views : ["dark-line"];
  for (const v of list) {
    const o = defaultDepictionOptions(v, "en", den);
    const scene = depict(state, { ...o, profile: DEFAULT_PRINT_PROFILE, representation: selector, annotations: { ...o.annotations, legend: v !== "dark-line" } }, analysis);
    const emit = (name: string, svg: string, dpi: number) => { const p = `${out}/${name}.svg`; writeFileSync(p, svg); execFileSync("rsvg-convert", ["-d", String(dpi), "-p", String(dpi), "-f", "png", "-o", `${out}/${name}.png`, p]); };
    if (v === "dark-line" && mode.includes("print")) emit(`${tag}-${v}-1-${den}-print`, pv.printSized(scene), 600);
    if (mode.includes("zoom") || mode.includes("views")) emit(`${tag}-${v}-1-${den}`, writeSvg(scene), v === "dark-line" ? 300 : 150);
  }
}
