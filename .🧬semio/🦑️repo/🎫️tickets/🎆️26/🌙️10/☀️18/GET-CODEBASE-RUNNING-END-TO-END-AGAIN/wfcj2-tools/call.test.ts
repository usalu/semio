import { test } from "bun:test";
const T = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage";
const mods: Record<string, string[]> = {
  "📤️publication-contracts": ["retainedPublicationChecks", "ephemeralPublicationChecks", "freshTurnAndHostOnlyPublicationChecks"],
  "🛑️cancellation-close-and-output-ownership": ["typedRouteCancellationAndCloseChecks"],
  "🧾️proof-catalog-and-admission": ["admissionOrderAndBoundedPreparationChecks"],
};
const only = process.env.ONLY?.split(",");
for (const [dir, fns] of Object.entries(mods)) {
  const m = await import(`${T}/${dir}/🟦️.ts`);
  const names = fns.length ? fns : Object.keys(m).filter((k) => typeof m[k] === "function");
  for (const n of names) {
    if (only && !only.includes(n)) continue;
    test(`${dir}/${n}`, () => { m[n](); }, 180000);
  }
}
