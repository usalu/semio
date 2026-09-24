import { readFileSync } from "node:fs";
import { discoverTestContributions } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const root = "/Users/ueli/Documents/semio";
const tally = (m: Map<string, Set<string>>, k: string, v: unknown, owner: string) => { const key = `${k}=${JSON.stringify(v)}`; if (!m.has(key)) m.set(key, new Set()); m.get(key)!.add(owner.split("/").slice(2, 5).join("/")); };
const m = new Map<string, Set<string>>();
for (const c of discoverTestContributions(root)) {
  const raw = JSON.parse(readFileSync(`${root}/${c.manifestPath}`, "utf8"));
  for (const f of raw.fixtureManifests ?? []) {
    if (f.generator && !/^(linux|darwin|win32)-(x64|arm64)$/.test(f.generator.platform)) tally(m, "platform", f.generator.platform, c.owner);
    if (!["generated", "authored", "downloaded", "vendored"].includes(f.provenance?.source)) tally(m, "source", f.provenance?.source, c.owner);
    if (!["metre","millimetre","centimetre","inch","foot","unitless"].includes(f.units?.length)) tally(m, "length", f.units?.length, c.owner);
    if (f.units?.angle && !["radian","degree"].includes(f.units.angle)) tally(m, "angle", f.units.angle, c.owner);
    if (f.units?.up && !["y","z"].includes(f.units.up)) tally(m, "up", f.units.up, c.owner);
    if (f.units?.handedness && !["right","left"].includes(f.units.handedness)) tally(m, "handedness", f.units.handedness, c.owner);
    if (f.family && !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(f.family)) tally(m, "family", f.family, c.owner);
  }
  for (const mm of raw.mutationManifests ?? []) for (const mu of mm.mutations ?? []) for (const o of mu.outcomes ?? []) if (!["applied","no-op","empty","disjoint","rejected"].includes(o)) tally(m, "outcome", o, c.owner);
  for (const p of raw.comparisonProfiles ?? []) if (p.text && !["none","utf8"].includes(p.text)) tally(m, "text", p.text, c.owner);
  for (const d of raw.noOracleDecisions ?? []) for (const s of d.substitutes ?? []) if (!["specification-vectors","metamorphic-laws","independent-implementations","second-parser"].includes(s)) tally(m, "substitute", s, c.owner);
}
for (const [k, v] of m) console.log(k, [...v].join(" "));
