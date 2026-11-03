import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
const t = readFileSync(file, "utf8");
const start = t.indexOf("  const out: Issue[] = [];\n  for (const denominator of SCALE_DENOMINATORS) {\n    const rep = asset.representations[`${denominator}`];\n    if (rep?.status !== \"authored\") continue;\n    const ends");
if (start < 0) throw new Error("anchor");
const end = t.indexOf("  return out;\n}", start) + "  return out;\n}".length;
const body = `  for (const denominator of SCALE_DENOMINATORS) {
    const rep = asset.representations[\`\${denominator}\`];
    if (rep?.status !== "authored") continue;
    measureAt(denominator, rep, denominator, "nominal");
    const ends = [...new Set([rep.validRange.minDenominator, rep.validRange.maxDenominator])].filter((end) => end !== denominator && Number.isFinite(end) && end > 0);
    for (const end of ends) measureAt(denominator, rep, end, "range-end");
  }
  return out;
}

/** 🔣️ The print-profile findings of the \`true-extent\` symbols of an asset at every scale they declare: a physically scaled symbol is measured like an authored representation (pens, hatch spacing, gaps and feature widths over its three viewpoints) and each violation is one \`symbol-print-<check>\` issue. */
export function validateSymbols(asset: Asset, profile: PrintProfile): readonly Issue[] {
  const out: Issue[] = [];
  asset.symbols.forEach((symbol, index) => {
    if (symbol.semantics !== "true-extent") return;
    for (const denominator of symbol.denominators) {
      for (const violation of measureSymbolSuitability(asset, symbol, denominator, profile).violations) out.push({ path: \`symbols[\${index}]\`, code: \`symbol-print-\${violation.check}\`, message: text(\`The \${violation.check} of \${violation.shapes.join("+")} in the true-extent symbol "\${symbol.id}" is \${violation.measuredMm.toFixed(3)} mm at 1:\${denominator}, below the \${profile.id} minimum \${violation.requiredMm} mm.\`, \`Das Merkmal \${violation.check} von \${violation.shapes.join("+")} im maßstäblichen Symbol "\${symbol.id}" beträgt \${violation.measuredMm.toFixed(3)} mm bei 1:\${denominator}, unter dem Minimum \${violation.requiredMm} mm von \${profile.id}.\`), data: { denominator, check: violation.check, shapes: violation.shapes, measuredMm: Number(violation.measuredMm.toFixed(3)), requiredMm: violation.requiredMm } });
    }
  });
  return out;
}`;
writeFileSync(file, t.slice(0, start) + body + t.slice(end));
