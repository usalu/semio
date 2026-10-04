import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";

const root = resolve(import.meta.dir, "../../../../../../../.."), ticket = dirname(import.meta.dir), output = join(ticket, "🗑️generated/primary-generator-entry");
const owner = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry", launch = `${owner}/🚀️launch`;
const hash = (text: string) => createHash("sha256").update(text).digest("hex");
const rows: { path: string; before: string | null; beforeHash: string | null; authored: string; authoredHash: string }[] = [];
const propose = (path: string, authored: string) => { const before = existsSync(join(root, path)) ? readFileSync(join(root, path), "utf8") : null; rows.push({ path, before, beforeHash: before === null ? null : hash(before), authored, authoredHash: hash(authored) }); };
mkdirSync(output, { recursive: true });
const fixture = { taxonomy: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", generatorContract: "plugin-registry", target: "@semio-tech/plugin-registry:generate", implementation: "bun ./📜️script.ts generate", seed: ".vscode/🧩️launch.seed.jsonc", output: ".vscode/launch.json", name: "🏭️generate", group: "3_dev", order: -49.8 };
if (process.argv[2] === "prepare") {
  const string = { type: "string", minLength: 1 }, schema = { $schema: "http://json-schema.org/draft-07/schema#", $id: "semio.framework.launch.primary-generator.v1", type: "object", additionalProperties: false, required: Object.keys(fixture), properties: Object.fromEntries(Object.keys(fixture).map(key => [key, key === "order" ? { type: "number" } : string])) };
  propose(`${launch}/🧬️schema/🏭️generate/🔣️.json`, JSON.stringify(schema, null, 2) + "\n");
  propose(`${launch}/🧫️fixtures/🏭️generate/🔣️.json`, JSON.stringify(fixture, null, 2) + "\n");
  propose(`${launch}/🧪️tests/🏭️generate/🟦️.ts`, readFileSync(join(import.meta.dir, "🟦️.ts"), "utf8"));
  const entry = `${launch}/🧪️tests/🧱️placement/🟦️.ts`;
  propose(entry, readFileSync(join(root, entry), "utf8") + '\nimport "../🏭️generate/🟦️.ts";\n');
  const lifecycle = `${owner}/🧪️tests/🚀️launch/🟦️.ts`, before = readFileSync(join(root, lifecycle), "utf8");
  const marker = '      expect([...commands].some((row) => row === `bun nx run workspace:${command}` || row.startsWith(`bun nx run workspace:${command} `)), command).toBe(true);';
  if (!before.includes(marker)) throw Error("Canonical lifecycle law marker missing");
  const commands = '    const commands = new Set(launch.configurations.map((entry) => String(entry.command ?? "")));';
  if (!before.includes(commands)) throw Error("Canonical lifecycle source map missing");
  propose(lifecycle, before.replace(commands, commands + '\n    const generator = JSON.parse(readFileSync(new URL("../../🚀️launch/🧫️fixtures/🏭️generate/🔣️.json", import.meta.url), "utf8"));').replace(marker, '      const route = command === "generate" ? generator.target : `workspace:${command}`;\n      expect([...commands].some((row) => row === `bun nx run ${route}` || row.startsWith(`bun nx run ${route} `)), command).toBe(true);'));
  writeFileSync(join(output, "test-only-before-authored-1.json"), JSON.stringify({ root, rows }, null, 2));
  const seed = readFileSync(join(root, fixture.seed), "utf8"), require = createRequire(join(root, "package.json")), jsonc = require("jsonc-parser"), document = jsonc.parse(seed), index = document.configurations.findIndex(row => row && typeof row === "object" && row.name === fixture.name);
  if (index < 0 || document.configurations.filter(row => row && typeof row === "object" && row.name === fixture.name).length !== 1) throw Error("Primary launcher identity is missing or ambiguous");
  const authored = jsonc.applyEdits(seed, jsonc.modify(seed, ["configurations", index, "command"], `bun nx run ${fixture.target}`, { formattingOptions: { insertSpaces: true, tabSize: 2, eol: "\n" } }));
  writeFileSync(join(output, "seed-before-authored-1.json"), JSON.stringify({ rows: [{ path: fixture.seed, before: seed, beforeHash: hash(seed), authored, authoredHash: hash(authored) }] }, null, 2));
  console.log(`[DEBUG] five test-only rows and one exact seed command proposal captured with full before/inverses; all unmounted`);
} else if (process.argv[2] === "amend-tests") {
  const capture = JSON.parse(readFileSync(join(output, "test-only-before-authored-1.json"), "utf8")), row = capture.rows.find(row => row.path === `${launch}/🧪️tests/🏭️generate/🟦️.ts`), path = join(root, row.path), before = readFileSync(path, "utf8"), authored = readFileSync(join(import.meta.dir, "🟦️.ts"), "utf8");
  if (before !== row.authored) throw Error("Mounted test-only source guard changed");
  writeFileSync(join(output, "test-only-amend-before-authored-2.json"), JSON.stringify({ rows: [{ path: row.path, before, beforeHash: hash(before), authored, authoredHash: hash(authored) }] }, null, 2));
  if (readFileSync(path, "utf8") !== before) throw Error("Immediate test amendment guard changed");
  writeFileSync(path, authored);
  console.log("[DEBUG] amended test-only renderer setup under full byte guards; primary seed unchanged");
} else if (process.argv[2] === "mount-tests" || process.argv[2] === "mount-seed") {
  const name = process.argv[2] === "mount-tests" ? "test-only-before-authored-1.json" : "seed-before-authored-1.json", capture = JSON.parse(readFileSync(join(output, name), "utf8"));
  for (const row of capture.rows) if ((existsSync(join(root, row.path)) ? readFileSync(join(root, row.path), "utf8") : null) !== row.before) throw Error(`Source guard changed: ${row.path}`);
  for (const row of capture.rows) { const path = join(root, row.path); if ((existsSync(path) ? readFileSync(path, "utf8") : null) !== row.before) throw Error(`Immediate source guard changed: ${row.path}`); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, row.authored); }
  console.log(`[DEBUG] mounted ${capture.rows.length} ${process.argv[2]} rows under full source guards; generated outputs untouched`);
} else if (process.argv[2] === "capture-publication-before") {
  const { renderCatalogFiles } = await import(join(root, owner, "📽️projection/🟦️.ts")), { declaredProjectTargets, generateLaunchJson } = await import(join(root, launch, "🟦️.ts"));
  const catalog = renderCatalogFiles(root), files = Object.entries(catalog.files).map(([name, authored]) => { const path = `${owner}/🤖️generated/${name}`, before = existsSync(join(root, path)) ? readFileSync(join(root, path), "utf8") : null; return { path, before, beforeHash: before === null ? null : hash(before), authored, authoredHash: hash(authored) }; });
  const before = readFileSync(join(root, fixture.output), "utf8"), authored = generateLaunchJson(root, catalog.playgrounds, declaredProjectTargets(root));
  files.push({ path: fixture.output, before, beforeHash: hash(before), authored, authoredHash: hash(authored) });
  writeFileSync(join(output, "publication-before-authored-1.json"), JSON.stringify({ files, rust: files.filter(row => row.path.endsWith(".rs")).map(row => ({ path: row.path, beforeHash: row.beforeHash, authoredHash: row.authoredHash })) }, null, 2));
  console.log(`[DEBUG] canonical publication prepared read-only: ${files.length} outputs and ${files.filter(row => row.path.endsWith(".rs")).length} exact Rust owners; no output written`);
} else if (process.argv[2] === "capture-publication-terminal") {
  const capture = JSON.parse(readFileSync(join(output, "publication-before-authored-1.json"), "utf8"));
  const files = capture.files.map(row => { const current = readFileSync(join(root, row.path), "utf8"); return { ...row, current, currentHash: hash(current), gap: current !== row.authored }; });
  const amendment = JSON.parse(readFileSync(join(output, "test-only-amend-before-authored-2.json"), "utf8")).rows;
  const source = [...JSON.parse(readFileSync(join(output, "test-only-before-authored-1.json"), "utf8")).rows.map(row => amendment.find(amended => amended.path === row.path) ?? row), ...JSON.parse(readFileSync(join(output, "seed-before-authored-1.json"), "utf8")).rows].map(row => { const current = readFileSync(join(root, row.path), "utf8"); return { ...row, current, currentHash: hash(current), gap: current !== row.authored }; });
  const rust = files.filter(row => row.path.endsWith(".rs")).map(row => ({ path: row.path, beforeHash: row.beforeHash, currentHash: row.currentHash, changed: row.beforeHash !== row.currentHash }));
  const graph = JSON.parse(readFileSync(join(root, ".nx/workspace-data/project-graph.json"), "utf8")), split = fixture.target.lastIndexOf(":"), inferred = graph.nodes[fixture.target.slice(0, split)]?.data.targets[fixture.target.slice(split + 1)];
  const produced = Bun.JSONC.parse(readFileSync(join(root, fixture.output), "utf8")), selected = produced.configurations.filter(row => row && typeof row === "object" && row.name === fixture.name);
  const result = { files, source, rust, outputGaps: files.filter(row => row.gap).length, sourceGaps: source.filter(row => row.gap).length, rustChanges: rust.filter(row => row.changed).length, declaredTarget: fixture.target, inferred, selected };
  writeFileSync(join(output, "publication-terminal-1.json"), JSON.stringify(result, null, 2));
  if (result.outputGaps || result.sourceGaps || result.rustChanges || !inferred || selected.length !== 1 || selected[0].command !== `bun nx run ${fixture.target}`) throw Error(`Publication/source admission gap: ${result.outputGaps}/${result.sourceGaps}/${result.rustChanges}`);
  console.log(`[DEBUG] primary generator publication current: ${files.length} outputs/${source.length} source rows zero gaps; ${rust.length} Rust owners unchanged; canonical target exists and primary row preserved`);
} else throw Error("Unknown primary generator proposal command");
