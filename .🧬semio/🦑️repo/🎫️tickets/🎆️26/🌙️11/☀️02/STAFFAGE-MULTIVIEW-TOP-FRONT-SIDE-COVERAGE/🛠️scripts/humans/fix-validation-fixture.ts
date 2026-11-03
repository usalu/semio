import { readFileSync, writeFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const path = `${L}/🧫️fixtures/✅️asset-validation/🔣️.json`;
const text = readFileSync(path, "utf8"); const f = JSON.parse(text);
const unavailable = { status: "unavailable", denominator: 500, cause: "below-printable-feature", reason: { en: "Test decision: the scale is declared unavailable so that its defects can be patched.", de: "Testentscheidung: Der Maßstab wird für nicht verfügbar erklärt, damit seine Mängel eingesetzt werden können." }, limitingFeature: { en: "Test feature", de: "Testmerkmal" }, limitingSizeM: 0.01, alternatives: [{ kind: "scale", denominator: 100 }] };
const prefix = { op: "replace", path: "/representations/500", value: unavailable };
let touched = 0;
for (const c of f.cases) {
  const uses = c.patch.some((p: any) => p.path.startsWith("/representations/500/")) || c.patch.some((p: any) => p.path === "/presentation/defaultScale" && p.value === 500);
  if (uses) { c.patch = [prefix, ...c.patch]; touched++; }
}
const fixRows = (n: any): void => { if (n && typeof n === "object") { for (const k of Object.keys(n)) { if (k === "500" && n[k] === "unavailable") { n[k] = "authored"; touched++; } else fixRows(n[k]); } } };
fixRows(f.manifest); for (const c of f.manifestCases ?? []) fixRows(c.patch);
writeFileSync(path, JSON.stringify(f, null, text.includes('\n  "') ? 2 : 1) + "\n"); console.log("fixture cases and rows updated:", touched);
