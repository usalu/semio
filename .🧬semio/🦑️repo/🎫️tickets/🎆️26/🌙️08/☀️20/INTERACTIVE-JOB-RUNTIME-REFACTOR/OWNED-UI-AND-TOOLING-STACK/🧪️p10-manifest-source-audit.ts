//#region 🧪️P10ManifestSourceAudit
import { readdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join, relative } from "node:path";

const root = process.cwd();
const ticket = join(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/OWNED-UI-AND-TOOLING-STACK");
const ignored = new Set(["compose", "node_modules", ".git", ".nx", "target", ".🧬semio", "dist"]);
const sourceExtensions = new Set([".ts", ".tsx", ".mts", ".cts", ".js", ".jsx", ".mjs", ".cjs", ".json", ".css", ".mdx"]);

type Evidence = { kind: "import" | "config" | "script"; file: string; line: number; text: string };
type Row = { dependency: string; section: string; version: string; evidence: Evidence[]; status: "evidenced" | "no-owned-scope-evidence" };
type Package = { manifest: string; name: string | null; ownershipScope: string; rows: Row[] };

async function walk(directory: string, files: string[] = []): Promise<string[]> {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    if (ignored.has(entry.name)) continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) await walk(path, files);
    else files.push(path);
  }
  return files;
}

function scopeOf(manifest: string): string {
  return join(root, dirname(manifest));
}

function expression(dependency: string): RegExp {
  const escaped = dependency.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  return new RegExp(`(?:from\\s*|import\\s*\\(|require\\s*\\()?["']${escaped}(?:["'/])`, "g");
}

function evidenceFor(dependency: string, file: string, contents: string, scope: string): Evidence[] {
  const result: Evidence[] = [];
  const importExpression = expression(dependency);
  const isConfig = /(?:vite|vitest|eslint|postcss|tailwind|storybook|playwright|project|tsconfig|nx).*\.(?:[cm]?[jt]s|json)$/u.test(file);
  for (const [index, text] of contents.split(/\r?\n/u).entries()) {
    if (importExpression.test(text)) result.push({ kind: "import", file: relative(root, file), line: index + 1, text: text.trim().slice(0, 180) });
    importExpression.lastIndex = 0;
    if (isConfig && text.includes(dependency)) result.push({ kind: "config", file: relative(root, file), line: index + 1, text: text.trim().slice(0, 180) });
  }
  return result.slice(0, 4);
}

const listed = await new Response(Bun.spawn(["rg", "--files", "-g", "!compose/**", "-g", "!**/node_modules/**", "-g", "!**/.🧬semio/**", "-g", "!**/target/**", "-g", "!**/dist/**"], { cwd: root }).stdout).text();
const files = listed.trim().split("\n").filter(Boolean);
const manifests = files.filter(path => path.endsWith("package.json")).map(path => join(root, path));
const packages: Package[] = [];
const sourceCache = new Map<string, Promise<readonly (readonly [string, string])[]>>();
for (const manifestPath of manifests.sort()) {
  const manifest = JSON.parse(await readFile(manifestPath, "utf8"));
  const sections = ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"] as const;
  const scope = scopeOf(relative(root, manifestPath));
  const contents = await (sourceCache.get(scope) ?? (() => {
    const relativeScope = relative(root, scope);
    const value = Promise.all(files.filter(path => !path.endsWith("package.json") && (!relativeScope || path.startsWith(`${relativeScope}/`)) && sourceExtensions.has(path.slice(path.lastIndexOf(".")))).map(async path => [join(root, path), await readFile(join(root, path), "utf8").catch(() => "")] as const));
    sourceCache.set(scope, value);
    return value;
  })());
  const rows: Row[] = [];
  for (const section of sections) for (const [dependency, version] of Object.entries(manifest[section] ?? {})) {
    const evidence = contents.flatMap(([file, text]) => evidenceFor(dependency, file, text, scope));
    for (const [name, script] of Object.entries(manifest.scripts ?? {})) if (String(script).includes(dependency)) evidence.push({ kind: "script", file: relative(root, manifestPath), line: 0, text: `${name}: ${script}`.slice(0, 180) });
    rows.push({ dependency, section, version: String(version), evidence: evidence.slice(0, 4), status: evidence.length ? "evidenced" : "no-owned-scope-evidence" });
  }
  if (rows.length) packages.push({ manifest: relative(root, manifestPath), name: manifest.name ?? null, ownershipScope: relative(root, scope), rows });
}
const externalRows = packages.flatMap(pkg => pkg.rows.filter(row => !row.version.startsWith("workspace:")).map(row => ({ ...row, manifest: pkg.manifest, package: pkg.name, ownershipScope: pkg.ownershipScope })));
const unused = externalRows.filter(row => row.status === "no-owned-scope-evidence");
const result = { generatedAt: new Date().toISOString(), scope: { excludes: ["compose/**", ".🧬semio/**", "node_modules/**", "target/**", "dist/**"], evidence: ["static import/require", "recognized config reference", "package script reference"], limitation: "Static evidence only; dynamic imports and generated-at-runtime package loading require a package-local allowlist." }, totals: { manifests: packages.length, directRows: packages.reduce((count, pkg) => count + pkg.rows.length, 0), externalRows: externalRows.length, noOwnedScopeEvidence: unused.length }, packages, highConfidenceUnusedCandidates: unused };
const markdown = `# P10 Manifest–Source Dependency Parity Audit\n\nGenerated from each manifest directory's static source/config/script evidence; \`compose/\` is excluded. Full per-row evidence is in \`📊️p10-manifest-source-parity.json\`.\n\n## Totals\n\n| Manifests | Direct rows | External rows | No package-scope evidence |\n| ---: | ---: | ---: | ---: |\n| ${result.totals.manifests} | ${result.totals.directRows} | ${result.totals.externalRows} | ${result.totals.noOwnedScopeEvidence} |\n\n## High-Confidence Candidate Rule\n\nA row is a candidate only when its manifest directory has no static import/require, recognized config reference, or package-script reference. Dynamic loading and code outside the declared package directory need a package-local allowlist before deletion.\n\n## Proposed Gates\n\n1. \`bun ./📜️script.ts verify dependencies parity js --format json\` regenerates this data and fails on an undeclared external import/config command.\n2. \`bun ./📜️script.ts verify dependencies parity js --no-unowned-rows\` fails any row with no evidence unless \`dependency-audit.allow.json\` at that package root names it with a reason and expiry.\n3. \`bun nx run-many -t test --projects=<affected-projects> --skip-nx-cache\` validates each manifest deletion; CI then runs \`bun ./📜️script.ts verify dependencies list js\` as the freeze ratchet.\n\n## Largest No-Evidence External Groups\n\n${Object.entries(unused.reduce<Record<string, number>>((counts, row) => ({ ...counts, [row.dependency]: (counts[row.dependency] ?? 0) + 1 }), {})).sort((a, b) => b[1] - a[1]).slice(0, 30).map(([dependency, count]) => `- \`${dependency}\`: ${count} rows`).join("\n")}\n`;
await writeFile(join(ticket, "📊️p10-manifest-source-parity.json"), `${JSON.stringify(result, null, 2)}\n`);
await writeFile(join(ticket, "📓️p10-manifest-source-parity.md"), markdown);
console.log(JSON.stringify(result.totals));
//#endregion 🧪️P10ManifestSourceAudit
