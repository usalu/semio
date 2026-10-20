import {readFileSync} from "node:fs";
const REPO = "/Users/ueli/Documents/semio", C = `${REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`;
const files = readFileSync(`${C}/tax-stage-tools/index/grep-files.txt`, "utf8").split("\n").filter(Boolean);
const pats = ["🛂️/🟦️.ts", "🌿️/🟦️.ts", "🦀️/🟦️.ts", "/🛂️\"", "/🌿️\"", "/🦀️\"", "🛂️.ts", "🌿️.ts", "🦀️.ts"];
for (const f of files) {
  let text: string; try { text = readFileSync(`${REPO}/${f}`, "utf8"); } catch { continue; }
  text.split("\n").forEach((line, i) => { for (const p of pats) if (line.includes(p)) { console.log(`${f.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "L")}:${i + 1}: [${p}] ${line.trim().slice(0, 170)}`); break; } });
}
