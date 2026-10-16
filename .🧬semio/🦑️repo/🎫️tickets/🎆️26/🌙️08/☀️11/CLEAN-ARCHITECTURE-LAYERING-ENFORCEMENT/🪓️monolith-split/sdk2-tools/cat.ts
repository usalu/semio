/** 🧭️ Buckets the violating plugin imports of a scan2 output into the SDK-BOUNDARY-2 categories. Read-only. */
import { readFileSync } from "node:fs";
const recs = JSON.parse(readFileSync(process.argv[2], "utf8"));
const bucket = (r: any): string => {
  const t: string = r.resolved.replace("🧰️framework/", "");
  if (r.file.startsWith("✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/")) return "L";
  if (t.includes("/🧑‍💻dev/🧪️tests/🧪️time-travel/")) return "B4";
  if (t.startsWith("🛍️products/🎤️presentation/")) return "B7";
  if (t.startsWith("🛍️products/🦑️repo/")) return "B1/B5";
  if (t.includes("/🏪️store/🪆️child/") || t.includes("/🏪️store/🔗️link/") || t.includes("/🏪️store/📦️blob/")) return "B2";
  if (t.includes("/📺️renderer/") || t.includes("/♾️infinite/") ) return "B3";
  if (t.startsWith("🛍️products/")) return "B6";
  return "other:" + t;
};
const by = new Map<string, any[]>();
for (const r of recs) { const b = bucket(r); (by.get(b) ?? by.set(b, []).get(b)!).push(r); }
const mode = process.argv[3];
for (const [b, rs] of [...by].sort()) {
  console.log(`${b}: edges=${rs.length} files=${new Set(rs.map((r: any) => r.file)).size}`);
  if (mode === b || (mode === "mine" && ["L","B4","B6","B7"].includes(b))) for (const r of rs) console.log(`   ${r.file.replace("✏️s/🔌️plugins/", "P/")}:${r.line} [${r.kind}] ${r.spec.slice(-90)} -> ${r.resolved.replace("🧰️framework/", "F/")}`);
}
