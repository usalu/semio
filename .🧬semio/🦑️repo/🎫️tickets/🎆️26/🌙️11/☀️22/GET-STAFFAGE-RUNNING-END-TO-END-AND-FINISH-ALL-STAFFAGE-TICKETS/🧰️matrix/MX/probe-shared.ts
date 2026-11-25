/** 🩺️ The three matrix checks of the shared-code requests on their filed assets, with the functions the engine exports now: `bun probe-shared.ts` prints the paper findings of `used-harmonium` (elevation, 1:50 and 1:100), the blank verdict of the symbol `gardener-block` of `kneeling-gardener` (old rule: fills only; new rule: `blankFinding` when the engine has it) and the derived-view findings of `steel-column` with the fixture as it is and with the entry proposed in `allowlist.json`. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { resolveViews } = await import(join(S, "🔨️modules", "🔭️multiview", "🟦️.ts"));
const harmonium = engine.loadAsset("used-harmonium");
for (const denominator of [50, 100]) {
  const multi = resolveViews(harmonium, { kind: "pose", pose: harmonium.presentation.defaultPose }, { kind: "scale", denominator }, "light-line", engine.drawOptions("light-line", denominator));
  const drawn = multi.views.elevation;
  console.log(`[DEBUG] used-harmonium 1:${denominator} elevation paper: ${JSON.stringify(engine.paperFindings(harmonium, "elevation", drawn.scene, drawn.state.shapes.elevation).map((finding: any) => finding.message))}`);
  const box = engine.paperBox(drawn.scene, drawn.state.shapes.elevation);
  const k = drawn.scene.metadata.mmPerMetre;
  const h = harmonium.dimensions.heightM;
  console.log(`[DEBUG] used-harmonium 1:${denominator} old float rule: |${box.maxY - box.minY} - ${h * k}| = ${Math.abs(box.maxY - box.minY - h * k)} > ${engine.d18(h) * k} is ${Math.abs(box.maxY - box.minY - h * k) > engine.d18(h) * k}; in micrometres: drawn ${Math.round(((box.maxY - box.minY) / k) * 1e6)}, nominal ${Math.round(h * 1e6)}, tolerance ${Math.round(engine.d18(h) * 1e6)}`);
  console.log(`[DEBUG] used-harmonium 1:${denominator} dimension findings: ${JSON.stringify(engine.geometryFindings(harmonium, multi.state, true).filter((finding: any) => finding.check.startsWith("dimension-")).map((finding: any) => finding.message))}`);
}
const gardener = engine.loadAsset("kneeling-gardener");
for (const mode of ["dark-line", "light-solid", "thermal"]) {
  const multi = resolveViews(gardener, { kind: "pose", pose: gardener.presentation.defaultPose }, { kind: "symbol", symbol: "gardener-block", denominator: 200 }, mode, engine.drawOptions(mode, 200));
  for (const view of ["elevation", "front", "plan"]) {
    const drawn = multi.views[view];
    const fills = engine.sceneBox(drawn.scene, new Set(drawn.state.shapes[view].map((evaluated: any) => `shape:${evaluated.shape.id}`)));
    console.log(`[DEBUG] kneeling-gardener symbol 1:200 ${mode} ${view}: old rule ${fills === undefined ? "blank-shapes" : "drawn"}; new rule ${engine.blankFinding === undefined ? "not in the engine" : (engine.blankFinding(drawn.state, view, mode, drawn.scene)?.message ?? "drawn")}; reason ${engine.blankReason(drawn.state, view, mode)}`);
  }
}
const column = engine.loadAsset("steel-column");
const proposed = JSON.parse(readFileSync(join(import.meta.dir, "allowlist.json"), "utf8")).derivedViewAllowlist;
for (const key of ["50", "100", "200", "500"]) console.log(`[DEBUG] steel-column 1:${key} derived with the proposed entry (bands ${proposed[0].bands.join(", ")}): ${JSON.stringify(engine.derivedFindings(column, key, Number(key), undefined, { derived: proposed, bandCopy: engine.allowlists().bandCopy }).map((finding: any) => `${finding.check} ${finding.view}`))}`);
for (const key of ["50", "100", "200", "500"]) console.log(`[DEBUG] steel-column 1:${key} derived: ${JSON.stringify(engine.derivedFindings(column, key, Number(key)).map((finding: any) => `${finding.check} ${finding.view} ${finding.message}`))}`);
