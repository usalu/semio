import { readFileSync, writeFileSync } from "node:fs";

const [matrixDir, testLog, outFile] = process.argv.slice(2) as [string, string, string];
const m = JSON.parse(readFileSync(`${matrixDir}/matrix.json`, "utf8")) as { assets: number; cells: { asset: string; category: string; scale: string; scaleKind: string; view: string; mode: string; status: string }[]; summary: { total: Record<string, number> } };
const summary = readFileSync(`${matrixDir}/summary.md`, "utf8");
const tests = readFileSync(testLog, "utf8").trim();
const cats = [...new Set(m.cells.map((c) => c.category))];
const row = (category: string | null): string => {
  const cells = m.cells.filter((c) => category === null || c.category === category);
  const count = (s: string): number => cells.filter((c) => c.status === s).length;
  const bands = new Set(cells.filter((c) => c.scaleKind !== "alternative").map((c) => `${c.asset}|${c.scale}`)).size;
  const assets = new Set(cells.map((c) => c.asset)).size;
  return `| ${category ?? "**total**"} | ${assets} | ${bands} | 3 | 7 | ${count("ok")} | ${count("unavailable")} | ${count("range-end")} | ${count("group-drift")} | ${count("fail")} |`;
};
const text = `# 📝️ sonnet-validation: coverage matrix, reference cases, proof sheets

Library tests: \`🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage\` (engine \`🧰️.ts\`, derived-view detection \`🪞️derived.ts\`, landmarks \`⚓️landmarks.ts\`, cases \`🟦️.ts\`, scenarios \`🥒️.feature\`, oracle \`🐍️.py\`, matrix \`📊️matrix.ts\`, proof sheets \`🖨️proofs.ts\`), vectors \`🧫️fixtures/🔭️multiview-coverage/🔣️.json\`.
Scripts: \`T/🛠️scripts/validation/run.sh\` (matrix, summary and proofs in one run), \`summary.ts\`, \`report.ts\`. Matrix: \`T/🗑️generated/validation/matrix.json\` (copy of the run below).

## Coverage matrix (v3)

| Category | Assets | Bands (asset x scale entry) | Views | Modes | OK | Declared unavailable | Range-end | Group-drift | FAIL |
|---|---|---|---|---|---|---|---|---|---|
${[...cats.map(row), row(null)].join("\n")}

Cells are asset x scale entry (four presets, declared keys, both validRange ends, every alternative of an unavailable scale) x viewpoint x mode. A cell is ok, declared unavailable (complete reason, cause, limiting feature, alternatives, and the resolver answers scale-unavailable in all three views), range-end, group-drift or FAIL; a missing or unchecked cell is FAIL (\`cell-missing\`, \`cell-unchecked\`).

${summary}

## Reference cases and tests (\`🧪️tests/🔭️multiview-coverage/🟦️.ts\`)

${tests}

Also changed: section-views, front-projection and caption-fit expectations follow the approved terminology (T1 to T10); seven-modes, view-synchrony and transport-view run all three viewpoints; \`🧪️tests/📏️print-size/🧰️d3-format.d.ts\` declares the oracle's d3-format types and the package tsconfig includes it and the coverage scripts.
`;
writeFileSync(outFile, text);
