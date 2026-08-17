import { readFileSync, writeFileSync } from "fs";
const root = readFileSync("/Users/ueli/Documents/semio/Cargo.toml", "utf8");
const femPath = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml";
let fem = readFileSync(femPath, "utf8");
for (const m of ["# 🚧️ TEMPORARY VERIFICATION OVERLAY", "# ==== 🧪️ TEMPORARY VERIFICATION OVERLAY"]) {
  const i = fem.indexOf(m);
  if (i >= 0) fem = fem.slice(0, i).replace(/\n+$/, "\n");
}
fem = fem.replace(/^cargo-features = \["trim-paths"\]\n+/m, "");
const cargoFeatures = 'cargo-features = ["trim-paths"]';
const pkg = root.match(/^\[workspace\.package\]\n([\s\S]*?)(?=^\[)/m)?.[0]?.trimEnd();
const needed = ["thiserror", "serde", "serde_json", "wasm-bindgen"];
const depSec = root.match(/^\[workspace\.dependencies\]\n([\s\S]*?)(?=^\[)/m)?.[1] || "";
const depBlock = [];
for (const line of depSec.split("\n")) {
  for (const k of needed) {
    if (line.startsWith(k + " ") || line.startsWith(k + "=")) depBlock.push(line);
  }
}
const lintRust = root.match(/^\[workspace\.lints\.rust\]\n([\s\S]*?)(?=^\[)/m)?.[0];
const lintClippy = root.match(/^\[workspace\.lints\.clippy\]\n([\s\S]*?)(?=^\[)/m)?.[0];
const profileChunks = root.match(/^\[profile\.[^\]]+\]\n([\s\S]*?)(?=^\[)/gm) || [];
let overlay = `\n# ==== 🧪️ TEMPORARY VERIFICATION OVERLAY — delete this section + nested target/Cargo.lock at\n# registrar handoff (TEMPLATE.md §3 chicken-and-egg).\n[workspace]\nmembers = ["."]\nresolver = "2"\n\n${pkg}\n\n[workspace.dependencies]\n${depBlock.join("\n")}\n\n`;
if (lintRust) overlay += lintRust + "\n";
if (lintClippy) overlay += lintClippy + "\n";
for (const p of profileChunks) overlay += p + "\n";
overlay += "# END TEMPORARY VERIFICATION OVERLAY\n";
const out = cargoFeatures + "\n\n" + fem.replace(/\n+$/, "\n") + overlay;
writeFileSync(femPath, out);
console.log("OK");
