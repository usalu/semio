import { readFileSync } from "node:fs";
import { unionPolygons } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
const d = JSON.parse(readFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/🗑️generated/final/union-inputs4.json", "utf8"));
for (const x of d) { try { const u = unionPolygons(x.input); console.log(x.asset, "ok", u.length); } catch (e) { console.log(x.asset, "ERR", (e as Error).message.slice(0, 80)); } }
