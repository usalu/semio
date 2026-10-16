// 🧾️ TOML/JSON validity of every manifest and registry this slice touched.
import {readFileSync, existsSync} from "node:fs";
const ROOT = "/Users/ueli/Documents/semio";
const T = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/otcp-work/touched.txt";
const paths = [...new Set(readFileSync(T, "utf8").split("\n").filter(Boolean).map(l => l.split("\t")[1]!.trim()))];
let bad = 0, n = 0;
for (const p of [...paths, "Cargo.toml", "Cargo.lock", "🧰️framework/Cargo.toml", "🧰️framework/Cargo.lock", "📋️build-owners.json", "🔒️dependencies.json", ".vscode/launch.json"]) {
  const f = ROOT + "/" + p;
  if (!existsSync(f)) continue;
  const text = readFileSync(f, "utf8");
  try {
    if (p.endsWith(".toml") || p.endsWith(".lock")) { const v: any = Bun.TOML.parse(text); n++; if (p.endsWith("Cargo.toml") && v.package) { const bf = v.package.metadata?.semio?.["rust-source-boundary"]; if (v.package.metadata?.semio && bf && JSON.stringify(Object.keys(bf).sort()) !== JSON.stringify(["maxActiveLines", "maxSourceBytes", "maxSourceFiles"])) { console.log("BOUNDARY KEYS", p); bad++; } } }
    else if (p.endsWith(".json")) { JSON.parse(text); n++; }
  } catch (e) { console.log("INVALID", p, String(e).slice(0, 160)); bad++; }
}
console.log(`checked ${n} files, ${bad} problems`);
