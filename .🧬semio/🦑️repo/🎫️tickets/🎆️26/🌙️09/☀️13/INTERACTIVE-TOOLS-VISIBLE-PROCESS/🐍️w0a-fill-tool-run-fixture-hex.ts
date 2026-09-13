import { readFileSync, writeFileSync } from "node:fs";
import * as M from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏯️tool-run/🟦️.ts";
const F = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏯️tool-run/🧫️fixtures/";
const tp = JSON.parse(readFileSync(F + "📼️trace-pages.json", "utf8"));
for (const row of tp.pages) row.hex = M.toolRunBytesToHex(M.encodeToolRunTracePage(M.toolRunTracePageFromJson(row.page)));
for (const row of tp.deltas) row.hex = M.toolRunBytesToHex(M.encodeToolRunTraceDelta(M.toolRunTraceDeltaFromJson(row.delta)));
const rev = "20" + tp.pages[0].page.identity.baseRevision;
const rev31 = "1f" + tp.pages[0].page.identity.baseRevision.slice(2);
const identity = "0d" + "04" + "0104" + "03" + "0204" + "2a" + "0304" + "01" + "0408" + rev;
const identity31 = "0d" + "04" + "0104" + "03" + "0204" + "2a" + "0304" + "01" + "0408" + rev31;
const key = "0100000000000000";
const pageWith = (fields: string[]) => "00" + fields.length.toString(16).padStart(2, "0") + fields.join("");
const mixed = tp.pages[0].hex;
tp.malformed = [
  { name: "truncated page", target: "page", hex: mixed.slice(0, -2) },
  { name: "trailing byte after the page record", target: "page", hex: mixed + "00" },
  { name: "verdict ordinal out of range", target: "page", hex: pageWith(["01" + identity, "020400", "03080100", "040808" + key, "05080104", "0608020000", "07080102", "0a0808" + key]) },
  { name: "key column one byte short", target: "page", hex: pageWith(["01" + identity, "020400", "03080100", "040807" + key.slice(2), "05080100", "0608020000", "07080102", "0a0808" + key]) },
  { name: "unknown page field", target: "page", hex: pageWith(["01" + identity, "020400", "0c0400"]) },
  { name: "delta without clear", target: "delta", hex: pageWith(["01" + identity, "030400"]) },
  { name: "step with zero repeat", target: "tick", hex: pageWith(["01" + identity, "020400", "040c010d05" + "010400" + "020400" + "030400" + "040400" + "060400"]) },
  { name: "base revision of 31 bytes", target: "tick", hex: pageWith(["01" + identity31, "020400"]) },
  { name: "sequence carried as bytes", target: "tick", hex: pageWith(["01" + identity, "02080100"]) },
];
writeFileSync(F + "📼️trace-pages.json", JSON.stringify(tp, null, 2) + "\n");
const ticks = JSON.parse(readFileSync(F + "🎞️ticks.json", "utf8"));
for (const row of ticks.ticks) row.hex = M.toolRunBytesToHex(M.encodeToolRunTick(M.toolRunTickFromJson(row.tick)));
writeFileSync(F + "🎞️ticks.json", JSON.stringify(ticks, null, 2) + "\n");
for (const row of tp.malformed) {
  const bytes = M.toolRunHexToBytes(row.hex);
  try { (row.target === "page" ? M.decodeToolRunTracePage : row.target === "delta" ? M.decodeToolRunTraceDelta : M.decodeToolRunTick)(bytes); console.log("[DEBUG] NOT REJECTED", row.name); } catch (error) { console.log("[DEBUG] rejected", row.name, String(error)); }
}
console.log("[DEBUG]", tp.pages.map((p: any) => p.hex.length / 2), ticks.ticks.map((t: any) => t.hex.length / 2));
