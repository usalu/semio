
test("no other module picks a language: the protocol is the one source every runtime reads", () => {
  const roots = ["🧰️framework/🔨️modules/⚙️runtime", "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/📁️filesystem", "🧰️framework/🛍️products/🦑️repo/🔨️modules"];
  const readers = ["🧰️framework/🔨️modules/⚙️runtime/🗣️language/", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/🗣️language/🐹️.go", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⚙️preferences/🌐️language/🦀️.rs", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/"];
  const idioms = [/split\(\s*\/\[[-_.:@\\]{3,7}\]\/u?\s*\)/u, /\.slice\(0,\s*2\)\.toLowerCase\(\)|toLowerCase\(\)\.slice\(0,\s*2\)/u, /re\.split\(r"\[[-_.:@]{3,5}\]"/u, /split\(\['\.',\s*'@'\]\)/u, /MustCompile\(`\[\._@-\]`\)/u, /\bSEMIO_DASHBOARD_LOCALE\b|\bSEMIO_LOCALE\b/u];
  const variables = new Set([protocol.explicitVariable, protocol.preferenceListVariable, ...protocol.environmentOrder]);
  const findings: string[] = [];
  const orderCopies = (value: unknown, path: string): void => {
    if (Array.isArray(value)) { if (value.length >= 3 && value.every((item) => typeof item === "string" && (variables.has(item) || /^SEMIO_[A-Z_]*LOCALE$/u.test(item))) && value.filter((item) => protocol.environmentOrder.includes(item)).length >= 2) findings.push(`${path}: a copy of the variable order`); value.forEach((item, index) => orderCopies(item, `${path}[${index}]`)); }
    else if (value !== null && typeof value === "object") for (const [key, item] of Object.entries(value)) orderCopies(item, `${path}/${key}`);
  };
  const walk = (directory: string): void => {
    for (const entry of readdirSync(join(root, directory), { withFileTypes: true })) {
      const rel = `${directory}/${entry.name}`;
      if (entry.name.startsWith(".") || ["node_modules", "dist", "target", "🤖️generated"].includes(entry.name) || readers.some((reader) => rel.startsWith(reader))) continue;
      if (entry.isDirectory()) { walk(rel); continue; }
      if (!/\.(ts|mts|mjs|cjs|js|rs|go|py|json)$/u.test(entry.name) || statSync(join(root, rel)).size > 4 << 20) continue;
      const text = readFileSync(join(root, rel), "utf8");
      for (const idiom of idioms) if (idiom.test(text)) findings.push(`${rel}: ${idiom.source}`);
      if (entry.name.endsWith(".json")) { try { orderCopies(JSON.parse(text), rel); } catch {} }
    }
  };
  for (const directory of roots) walk(directory);
  expect(findings).toEqual([]);
});
