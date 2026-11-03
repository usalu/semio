import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
const lines = readFileSync(file, "utf8").split("\n");
const at = lines.findIndex((l) => l.startsWith("      if (paperMm >= UNAVAILABLE_MIN_PAPER_MM) add("));
if (at < 0) throw new Error("line");
lines[at] = "      if (paperMm >= UNAVAILABLE_MIN_PAPER_MM) drawable.push({ path: `${path}.status`, denominator, paperMm, offered: rep.alternatives.flatMap((alternative) => (alternative.kind === \"symbol\" ? [alternative.symbol] : [])) });";
const decl = lines.findIndex((l) => l.startsWith("  const representationPending"));
lines.splice(decl, 0, "  const drawable: { readonly path: string; readonly denominator: number; readonly paperMm: number; readonly offered: readonly Slug[] }[] = [];");
const regions = lines.findIndex((l) => l.startsWith("  asset.thermal.regions.forEach((region, index) => {"));
const block = [
  "  for (const { path, denominator, paperMm, offered } of drawable) {",
  "    const satisfied = offered.some((id) => {",
  "      const index = asset.symbols.findIndex((symbol) => symbol.id === id);",
  "      const symbol = asset.symbols[index];",
  "      const geometry = symbol?.geometry as Partial<Record<Viewpoint, readonly Shape[]>> | undefined;",
  "      return symbol?.semantics === \"true-extent\" && VIEWPOINTS.every((viewpoint) => (geometry?.[viewpoint] ?? []).length > 0) && !out.some((issue) => issue.path.startsWith(`symbols[${index}]`) && issue.code.startsWith(\"symbol-extent\"));",
  "    });",
  "    if (!satisfied) add(path, \"unavailable-but-drawable\", `The asset measures ${paperMm.toFixed(1)} mm on paper at 1:${denominator}, at least ${UNAVAILABLE_MIN_PAPER_MM} mm: draw it, or offer a true-extent symbol that has all three views and passes the extent check.`, `Das Asset misst bei 1:${denominator} ${paperMm.toFixed(1)} mm auf dem Papier, mindestens ${UNAVAILABLE_MIN_PAPER_MM} mm: zeichnen, oder ein maßstäbliches Symbol (true-extent) mit allen drei Ansichten anbieten, das die Ausdehnungsprüfung besteht.`);",
  "  }",
];
lines.splice(regions, 0, ...block);
writeFileSync(file, lines.join("\n"));
