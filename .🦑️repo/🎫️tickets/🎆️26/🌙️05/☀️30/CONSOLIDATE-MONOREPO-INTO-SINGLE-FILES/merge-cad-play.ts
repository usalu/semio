import { readFileSync, writeFileSync, unlinkSync } from "node:fs";

const dir = "c:/git/compose/cad/js/renderer/play";
const index = readFileSync(`${dir}/index.ts`, "utf8");
let main = readFileSync(`${dir}/main.tsx`, "utf8");

const importBlock = /import\s*\{[^}]*\}\s*from\s*["']\.\/index\.ts["'];?\s*\n/s;
if (!importBlock.test(main)) {
  throw new Error("main.tsx missing ./index.ts import block");
}
main = main.replace(importBlock, "");

const merged = `${index.replace(/\/\/ 💻️ cad\/js\/renderer-r3f\/play\/index\.ts[^\n]*\n/, "// 💻️ cad/js/renderer/play/main.tsx — Spatial play shell (headless + React chrome + Vite entry).\n")}${main.replace(
  /^\/\*\* @emoji 🎮️ Vite entry:[^\n]*\n/,
  "",
)}`;

writeFileSync(`${dir}/main.tsx`, merged);
unlinkSync(`${dir}/index.ts`);
console.log("merged cad play index.ts into main.tsx");
