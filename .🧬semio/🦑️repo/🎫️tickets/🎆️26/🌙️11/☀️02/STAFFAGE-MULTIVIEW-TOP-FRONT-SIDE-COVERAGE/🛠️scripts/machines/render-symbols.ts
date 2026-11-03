import { runPreview } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts";
import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE/🗑️generated/machines/symbols";
const sc = process.env.SCALE ?? "50";
for (const a of catalog()) { if (!["machinery","vehicles"].includes(a.category)) continue;
  for (const s of a.symbols) { try { await runPreview(["--asset", a.id, "--symbol", s.id, "--scale", sc, "--views", "--dpi", "100", "--out", OUT]); } catch (e) { console.log("ERR", a.id, s.id, (e as Error).message.slice(0,150)); } } }
