/** 🧰️ QA9 driver, one process: check + sweep + consistency + view sheets (every pose and 3 instants per clip at every authored scale) + contact montages.
 * usage: bun all.ts <ids comma> [--dpi 150] [--scales 50,100] [--only-pose id] [--no-render] [--probe pose|clip@t] */
import { mkdirSync, readFileSync, writeFileSync, appendFileSync } from "node:fs";
import sharp from "sharp";
import { type Asset, type AssetDocument } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { checkAssetDetailed } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";
import { runPreview } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { consistency, sweep } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

const ROOT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/QA9";
const [idsArg, ...rest] = process.argv.slice(2);
const flag = (name: string): string | undefined => { const i = rest.indexOf(`--${name}`); return i < 0 ? undefined : rest[i + 1]; };
const has = (name: string): boolean => rest.includes(`--${name}`);
mkdirSync(OUT, { recursive: true });
const LOG = `${OUT}/all.txt`;
writeFileSync(LOG, "");
const log = (s: string): void => { appendFileSync(LOG, s + "\n"); };
const f = (v: number): string => v.toFixed(3);

for (const id of idsArg!.split(",")) {
  const file = `${ROOT}/${id}/🔣️.json`;
  const asset = (JSON.parse(readFileSync(file, "utf8")) as AssetDocument).asset as Asset;
  const authored = Object.values(asset.representations).filter((r: any) => r.status === "authored").map((r: any) => r.denominator as number);
  const scales = flag("scales") ? flag("scales")!.split(",").map(Number) : authored;
  const report = checkAssetDetailed(asset, { samples: 5, physics: false });
  log(`${report.problems.length ? "FAIL" : "ok"} check ${id} (${report.lines.length} lines)`);
  for (const p of report.problems) log("  PROBLEM " + p);
  for (const l of report.lines.filter((x) => /error|fail|warn|problem|✗/i.test(x))) log("  " + l);
  for (const sc of authored) {
    const findings = [...sweep(asset, sc), ...consistency(asset, sc)];
    log(`${findings.length ? "FAIL" : "ok"} three-views ${id} 1:${sc}`);
    for (const fi of findings) log(`  [${fi.check}] ${fi.message}`);
  }
  const probe = flag("probe");
  if (probe) {
    const q: any = probe.includes("@") ? { kind: "clip", clip: probe.split("@")[0], timeS: Number(probe.split("@")[1]) } : { kind: "pose", pose: probe };
    for (const sc of scales.slice(0, 1)) try {
      const state = evaluate(asset, q, { kind: "scale", scale: { denominator: sc } });
      for (const vp of ["plan"] as const) for (const s of state.shapes[vp]) {
        const pts = s.rings.flat();
        if (pts.length === 0) { log(`probe 1:${sc} ${vp} ${s.shape.id} EMPTY`); continue; }
        log(`probe 1:${sc} ${vp} ${s.shape.id} [${f(Math.min(...pts.map((p) => p.x)))},${f(Math.max(...pts.map((p) => p.x)))}]x[${f(Math.min(...pts.map((p) => p.y)))},${f(Math.max(...pts.map((p) => p.y)))}]`);
      }
    } catch { /* probe pose absent for this asset */ }
  }
  if (has("no-render")) continue;
  mkdirSync(`${OUT}/${id}`, { recursive: true });
  const specs: { name: string; args: string[] }[] = [];
  const onlyPose = flag("only-pose");
  for (const p of asset.poses) if (!onlyPose || onlyPose === p.id) specs.push({ name: `pose-${p.id}`, args: ["--pose", p.id] });
  if (!onlyPose) for (const c of asset.clips) for (const fr of [0, 0.5, 1]) { const t = +(c.durationS * (fr === 1 ? 0.999 : fr)).toFixed(3); specs.push({ name: `clip-${c.id}@${t}`, args: ["--clip", c.id, "--time", String(t)] }); }
  for (const sc of scales) {
    const tiles: { name: string; buf: Buffer; w: number; h: number }[] = [];
    for (const spec of specs) {
      const written = await runPreview(["--file", file, "--scale", String(sc), "--views", "--no-png", "--out", `${OUT}/${id}`, ...spec.args]);
      for (const svg of written) {
        const png = svg.replace(/\.svg$/, ".png");
        const info = await sharp(svg, { density: Number(flag("dpi") ?? 150) }).flatten({ background: "#ffffff" }).png().toFile(png);
        log(`png ${png}`);
        tiles.push({ name: spec.name, buf: readFileSync(png), w: info.width, h: info.height });
      }
    }
    if (tiles.length > 1) {
      const colW = 900, cols = 2;
      const resized = await Promise.all(tiles.map(async (t) => { const b = await sharp(t.buf).resize({ width: colW }).png().toBuffer(); const m = await sharp(b).metadata(); return { b, h: m.height! }; }));
      const rows = Math.ceil(resized.length / cols);
      const rowH = Array.from({ length: rows }, (_, r) => Math.max(...resized.slice(r * cols, r * cols + cols).map((x) => x.h)));
      const total = rowH.reduce((a, b) => a + b, 0);
      let y = 0;
      const comps = resized.map((x, i) => { const r = Math.floor(i / cols); const top = rowH.slice(0, r).reduce((a, b) => a + b, 0); return { input: x.b, left: (i % cols) * colW, top }; });
      void y;
      const montage = `${OUT}/${id}/montage-1-${sc}.png`;
      await sharp({ create: { width: colW * cols, height: total, channels: 3, background: "#ffffff" } }).composite(comps).png().toFile(montage);
      log(`montage ${montage} order: ${tiles.map((t) => t.name).join(" | ")}`);
    }
  }
}
log("done");
