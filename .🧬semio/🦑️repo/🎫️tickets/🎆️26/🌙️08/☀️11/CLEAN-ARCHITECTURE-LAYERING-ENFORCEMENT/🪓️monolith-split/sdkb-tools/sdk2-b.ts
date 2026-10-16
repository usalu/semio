/** 🔎️ Prints the violating records of one SDK-BOUNDARY-2 bucket with imported names. Read-only. */
import { readFileSync } from "node:fs";
const [scan, want] = process.argv.slice(2);
const recs = JSON.parse(readFileSync(scan, "utf8"));
const bucket = (r: any): string => {
  const t: string = r.resolved.replace("🧰️framework/", "");
  if (r.file.startsWith("✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/")) return "L";
  if (t.includes("/🧑‍💻dev/🧪️tests/🧪️time-travel/")) return "B4";
  if (t.startsWith("🛍️products/🎤️presentation/")) return "B7";
  if (t.startsWith("🛍️products/🦑️repo/")) return "B1/B5";
  if (t.includes("/🏪️store/🪆️child/") || t.includes("/🏪️store/🔗️link/") || t.includes("/🏪️store/📦️blob/")) return "B2";
  if (t.includes("/📺️renderer/") || t.includes("/♾️infinite/")) return "B3";
  if (t.startsWith("🛍️products/")) return "B6";
  return "other";
};
for (const r of recs) if (bucket(r) === want) {
  const names = [...r.named.map((n: any) => (n.typeOnly || r.typeOnly ? "t:" : "") + n.imported), ...(r.defaultName ? ["default " + r.defaultName] : []), ...(r.namespaceName ? ["* as " + r.namespaceName + " {" + (r.members ?? []).join(",") + "}"] : []), ...(r.members && !r.namespaceName ? ["members:" + r.members.join(",")] : [])];
  console.log(`${r.file.replace("✏️s/🔌️plugins/", "P/").replace("🏅️standards/🔖️1/🪆️subsets/✳️any/", "…/")}:${r.line} [${r.kind}${r.typeOnly ? " type" : ""}] ${r.spec.replace(/^(\.\.?\/)+/, "").slice(-60)}\n      -> ${r.resolved.replace("🧰️framework/", "F/")}\n      {${names.join(", ")}}`);
}
