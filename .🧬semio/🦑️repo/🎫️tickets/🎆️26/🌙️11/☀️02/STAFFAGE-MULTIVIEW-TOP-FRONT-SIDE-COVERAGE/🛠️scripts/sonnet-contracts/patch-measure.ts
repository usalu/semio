import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const sub = (from: string, to: string): void => {
  if (!t.includes(from)) throw new Error(`missing: ${from.slice(0, 70)}`);
  t = t.replace(from, to);
};
sub("function measureOnce(asset: Asset, rep: AuthoredRepresentation, requested: number, profile: PrintProfile): Suitability {", "/** 🖨️ What suitability measures: the pens, hatches and three-view geometry of an authored representation or of a symbol. */\ntype Drawn = { readonly pens: AuthoredRepresentation[\"pens\"]; readonly hatches: AuthoredRepresentation[\"hatches\"]; readonly geometry: AuthoredRepresentation[\"geometry\"] };\n\nfunction measureOnce(asset: Asset, rep: Drawn, selector: RepresentationSelector, requested: number, profile: PrintProfile): Suitability {");
sub("{ kind: \"pose\", pose: pose.id }, { kind: \"scale\", scale: { denominator: requested } });", "{ kind: \"pose\", pose: pose.id }, selector);");
sub("measureOnce(asset, representation, requested, profile)", "measureOnce(asset, representation, { kind: \"scale\", scale: { denominator: requested } }, requested, profile)");
sub("import type { DrawingScale, RepresentationResolution,", "import type { DrawingScale, RepresentationResolution, RepresentationSelector,");
t += `
const SYMBOL_CACHE = new WeakMap<SymbolRepresentation, Map<string, Suitability>>();

/** ✅️ The suitability of a symbol drawn at \`requested\` under a print profile: its pens, hatch spacing, gaps and feature widths measured on the evaluated paper geometry of all three viewpoints, the same measurements as an authored representation. A \`true-extent\` symbol is physically scaled and must pass; paper-millimetre symbols measure at their fixed paper size. Cached per symbol, scale and thresholds. */
export function measureSymbolSuitability(asset: Asset, symbol: SymbolRepresentation, requested: number, profile: PrintProfile): Suitability {
  let byKey = SYMBOL_CACHE.get(symbol);
  if (byKey === undefined) SYMBOL_CACHE.set(symbol, (byKey = new Map()));
  const key = \`\${thresholdKey(requested, profile)}|\${fingerprint(JSON.stringify([symbol, asset.rig, asset.poses]))}\`;
  let result = byKey.get(key);
  if (result === undefined) byKey.set(key, (result = measureOnce(asset, symbol, { kind: "symbol", symbol: symbol.id }, requested, profile)));
  return result;
}
`;
writeFileSync(file, t);
