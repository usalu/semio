import { readdirSync, readFileSync } from "node:fs";
const base = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/⚡️implementations/🦀️rust";
for (const e of readdirSync(base)) {
  console.log("entry:", JSON.stringify(e), "hex:", Buffer.from(e).toString("hex"));
  console.log("  NFC==raw:", e.normalize("NFC") === e, " NFD==raw:", e.normalize("NFD") === e);
}
const p = base + "/" + readdirSync(base)[0] + "/📝️text/📜️script.ts";
for (const [label, cand] of [["raw", p], ["NFC", p.normalize("NFC")], ["NFD", p.normalize("NFD")]]) {
  try { readFileSync(cand); console.log(label, "OK"); } catch (err) { console.log(label, "FAIL", err.code); }
}
