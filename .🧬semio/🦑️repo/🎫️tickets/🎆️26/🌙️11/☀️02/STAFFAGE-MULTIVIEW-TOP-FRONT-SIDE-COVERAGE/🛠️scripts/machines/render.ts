import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { catalog, authoredScales } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE/🗑️generated/machines";
const ids = (process.env.IDS ?? "").split(",").filter(Boolean);
const poses = (process.env.POSES ?? "").split(",").filter(Boolean);
const scales = (process.env.SCALES ?? "").split(",").filter(Boolean).map(Number);
const dpi = process.env.DPI ?? "110";
const extra = (process.env.EXTRA ?? "").split(" ").filter(Boolean);
for (const a of catalog()) {
  if (!["machinery","vehicles"].includes(a.category)) continue;
  if (ids.length && !ids.includes(a.id)) continue;
  for (const s of (scales.length ? scales : [...authoredScales(a)])) {
    if (!authoredScales(a).includes(s)) continue;
    for (const p of (poses.length ? poses : [a.presentation.defaultPose])) {
      try { await runPreview(["--asset", a.id, "--scale", String(s), "--pose", p, "--views", "--dpi", dpi, "--out", `${OUT}/${a.id}`, ...extra]); } catch (e) { console.log("ERR", a.id, s, p, (e as Error).message); }
    }
  }
}
