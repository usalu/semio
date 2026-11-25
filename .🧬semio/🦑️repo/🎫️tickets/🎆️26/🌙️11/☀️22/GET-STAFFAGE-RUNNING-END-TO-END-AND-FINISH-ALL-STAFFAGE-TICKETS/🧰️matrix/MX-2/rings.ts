/** 🪪️ The rings of an engine, as one SHA-256 per asset and scale: `bun rings.ts <staffage root> <id>… [--step n] [--scales 50,500]` evaluates every nth subject (default 17, the default pose always) at the authored scales with the engine under the root, hashes every ring of the three views bit for bit (Float64) and prints `<id> 1:<scale> <states> <hash> <seconds>`. Run it on the live root and on a trial root and compare the two outputs.
 * @see ../MM/engine.patch */
import { createHash } from "node:crypto";
import { join } from "node:path";
import { parseArgs } from "node:util";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { step: { type: "string" }, scales: { type: "string" } } });
const [root, ...ids] = positionals as [string, ...string[]];
const { evaluate } = await import(join(root, "🔨️modules/🧮️evaluation/🟦️.ts"));
const engine = await import(join(root, "🧪️tests/🔭️multiview-coverage/🧰️.ts"));
for (const id of ids) {
  const asset = engine.loadAsset(id);
  for (const key of Object.keys(asset.representations).filter((candidate) => engine.authoredOf(asset, candidate) !== undefined && (values.scales === undefined || values.scales.split(",").includes(candidate)))) {
    const hash = createHash("sha256");
    const started = performance.now();
    let states = 0;
    engine.subjects(asset, Number(key)).forEach((subject: any, at: number) => {
      if (!subject.isDefault && at % Number(values.step ?? 17) !== 0) return;
      const state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
      for (const view of ["elevation", "front", "plan"] as const) for (const shape of state.shapes[view]) for (const ring of shape.rings) hash.update(new Uint8Array(Float64Array.from(ring.flatMap((p: any) => [p.x, p.y])).buffer)).update("|");
      states++;
    });
    console.log(`${id} 1:${key} ${states} ${hash.digest("hex").slice(0, 24)} ${((performance.now() - started) / 1000).toFixed(1)}`);
  }
}
