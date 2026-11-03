import { readFileSync, writeFileSync } from "node:fs";
const root = process.argv[2]!;
const edit = (file: string, fn: (t: string) => string): void => writeFileSync(`${root}/${file}`, fn(readFileSync(`${root}/${file}`, "utf8")));
edit("🎨️depiction/🟦️.ts", (t) => {
  const lines = t.split("\n");
  const at = lines.findIndex((l) => l.startsWith("  const legends = [...(analytic?.legends ?? []), ...legendsOfTransport].map("));
  if (at < 0) throw new Error("legends");
  lines[at] = "  const legends = [...(analytic?.legends ?? []), ...legendsOfTransport];";
  let out = lines.join("\n");
  const sub = (from: string, to: string): void => {
    if (!out.includes(from)) throw new Error(`missing ${from.slice(0, 60)}`);
    out = out.replace(from, to);
  };
  sub("    patterns: false,", "    patterns: true,");
  sub("    if (options.annotations.legend) legends.forEach((legend, i) => (y += legendBlock(canvas, ctx, legend, `legend:${i}:${legend.quantity}`, x, y) + 1));\n", "    if (options.annotations.legend) legends.forEach((legend, i) => (y += legendBlock(canvas, ctx, legend, `legend:${i}:${legend.quantity}`, x, y) + 1));\n    if (options.annotations.labels && omitted > 0) canvas.text(\"annotations\", \"note:omitted-values\", { x, y: y + LABEL_TEXT_MM }, ctx.language === \"de\" ? `${omitted} weitere Werte im Inspektor.` : `${omitted} further values in the inspector.`, LABEL_TEXT_MM, colours.ink);\n");
  sub("resolution: resolution.kind, fidelity:", "resolution: resolution.kind, omittedValues: omitted, fidelity:");
  return out;
});
edit("🔗️contracts/🟦️.ts", (t) => {
  if (!t.includes("readonly fidelity: Fidelity;")) throw new Error("contracts");
  return t.replace("readonly fidelity: Fidelity;", "readonly fidelity: Fidelity; readonly omittedValues: number;");
});
