/** ⚡️ The catalog invariants of `🧪️tests/⚡️asset-power` that loop over every asset inside one test, restricted to people: the total power is finite and positive for a living body, the elements are sorted largest first, every rating has a basis (0 W exactly when passive), an asset with operating heat states a nameplate, and the manifest entry carries the evaluated power.
 *  bun power.ts <slug>… */
import { POWER_TOLERANCE_W, powerOf, totalPowerW } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { catalogFile, S } from "./people.ts";

const manifest = await Bun.file(`${S}/🗂️catalog/🔣️.json`).json();
let failing = 0;
for (const slug of process.argv.slice(2)) {
  const asset = (await Bun.file(catalogFile(slug)).json()).asset;
  const power = powerOf(asset);
  const watts = power.elements.map((element: { powerW: number }) => element.powerW);
  const problems: string[] = [];
  if (!Number.isFinite(power.totalW) || power.totalW < 0) problems.push(`total ${power.totalW}`);
  if (power.totalW > 0 !== (asset.thermal.body !== undefined || asset.thermal.passengers.length > 0 || asset.thermal.regions.some((region: { ratedPowerW?: number }) => (region.ratedPowerW ?? 0) > 0))) problems.push("a positive total without a source, or none with one");
  if (JSON.stringify(watts) !== JSON.stringify([...watts].sort((a, b) => b - a))) problems.push("elements not sorted");
  for (const region of asset.thermal.regions) if (region.ratedPowerW !== undefined && (region.ratedPowerBasis === undefined || (region.ratedPowerBasis === "passive") !== (region.ratedPowerW === 0))) problems.push(`rating basis of ${region.id}`);
  const heated = asset.thermal.regions.some((region: { operating?: { heatW: number }; hydronic?: unknown }) => (region.operating !== undefined && region.operating.heatW !== 0) || region.hydronic !== undefined);
  if (heated && !asset.thermal.regions.some((region: { ratedPowerW?: number }) => region.ratedPowerW !== undefined)) problems.push("operating heat without a nameplate");
  const entry = manifest.entries.find((candidate: { id: string }) => candidate.id === slug);
  const inManifest = entry === undefined ? "no manifest entry" : Math.abs(entry.powerW - totalPowerW(asset)) <= Math.max(POWER_TOLERANCE_W, 0.05) ? "manifest in step" : `manifest says ${entry.powerW} W`;
  if (problems.length > 0) failing++;
  console.log(`${slug}: ${totalPowerW(asset).toFixed(1)} W, ${problems.length === 0 ? "invariants hold" : problems.join("; ")}; ${inManifest}`);
}
console.log(`failing ${failing}`);
