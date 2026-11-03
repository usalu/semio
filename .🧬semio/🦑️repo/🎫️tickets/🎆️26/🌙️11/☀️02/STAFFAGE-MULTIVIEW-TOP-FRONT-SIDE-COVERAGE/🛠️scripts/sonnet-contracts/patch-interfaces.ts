import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const cut = (from: string): void => {
  const i = t.indexOf(from);
  if (i < 0) throw new Error(from);
  const j = t.indexOf("\n", i);
  t = t.slice(0, i) + t.slice(j + 1);
};
cut("| `symbol-conventional-note`");
t += `
## 6. Update 2026-11-02 (rulings D6, D8 to D28)

Validator rules in force (\`🔨️modules/🗂️catalog\`), beyond section 4:

| Code | Rule |
|---|---|
| \`unavailable-cause-missing\` | \`cause\` required (\`below-printable-feature\` or \`no-physical-depiction\`). |
| \`unavailable-but-drawable\` (D17, D27) | Largest dimension at least 3 mm on paper at that scale; satisfied only by a \`true-extent\` symbol alternative with all three views and passing symbol extents. The symbol print-minimum part of D27 is not implemented yet. |
| \`cause-missing-symbol\` | The reason or limiting feature names a symbol id the asset does not have. |
| \`symbol-semantics-missing\` | \`semantics\` is required (\`true-extent\` \| \`true-position\` \| \`nominal\`). |
| \`alternative-asset-category\`, \`alternative-asset-chained\`, \`alternative-asset-undrawable\` (D9) | An \`asset\` alternative must be the same category, authored at that scale, and not itself unavailable. Needs \`AssetContext.assets\` as a map id to Asset. |
| \`valid-range-gap\` (D10) | Two adjacent authored presets leave denominators between their ranges. \`valid-range-overlap\` stays. |
| \`validateRanges(asset, profile)\` (D10) | Separate export. Measures print-profile suitability at both ends of every \`validRange\`; codes \`range-end-stroke\`, \`-gap\`, \`-feature\`, \`-hatch\`. Slow (minutes for the catalog), so run it explicitly. |
| \`solid-axis-not-ancestor\` (D21) | \`axisBone\` of a revolved solid is the shape's own bone or an ancestor. |
| \`lateral-ambiguous\` (D25, D28) | A non-solid shape on a spread or twisted bone whose paper error \`extent * sin(theta_max)\` exceeds the print profile minimum feature at some band. One issue per bone, listing shapes, bands, theta and error. |
| \`extent-*\`, \`symbol-extent-*\` | Tolerance is max(2 % of declared, 15 mm). Geometry only; ground-role shapes are excluded. |

Removed on ruling: \`view-twin-missing\` (D25) and \`solid-on-elevation\` (D26). Solids are allowed in every viewpoint.

Stable shared API (do not change without telling contracts):
- \`🔨️modules/🎨️depiction/🔭️views/🟦️.ts\`: \`CAMERAS\`, \`Camera\`, \`project(viewpoint, Vec3) -> {x, y}\` (paper-up metres), \`nearness(viewpoint, Vec3)\` (larger is nearer), \`ViewMissingError\`. Re-exported from the multiview module and the package barrel.
- \`DrawingScene.metadata.fidelity: "physical" | "conventional-symbol"\` is the field consumers read to show the symbol semantics to the reader; \`ResolvedDrawing.fidelity\` carries the same value. \`RepresentationResolution\` \`unavailable\` carries \`cause\`.
- \`depict()\` throws \`ViewMissingError\` for a viewpoint without shapes; \`evaluate()\` throws it when a representation has no \`front\` or \`plan\` array at all.
- \`areaWeighted\` was removed from depiction shapes (thermal owns the exposure-weighted version).
- Terminology: captions follow \`📖️terminology.md\` (viewpoint names, \`Plan section\` / \`Grundriss-Schnitt\` for a plan cut; the side elevation caption names its viewpoint too).
`;
writeFileSync(file, t);
