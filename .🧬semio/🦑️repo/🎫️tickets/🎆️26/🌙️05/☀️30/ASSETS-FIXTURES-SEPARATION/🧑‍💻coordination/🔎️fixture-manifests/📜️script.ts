import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { inspectTestLayoutSources, isExcludedTestPath, testTaxonomy } from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const root = process.env.SEMIO_FIXTURE_REPO_ROOT!, ticket = dirname(dirname(import.meta.dir));
const ignored = ["node_modules", ".git", "target", "dist", "📤️dist", "build", "out", "storybook-static", ".venv", "__pycache__", "obj", "bin", ".🧬semio"];
const listed = spawnSync("rg", ["--files", "--hidden", "--no-ignore", ...ignored.flatMap(name => ["-g", `!**/${name}/**`])], { cwd: root, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });
if (listed.status !== 0) throw new Error(listed.stderr);
const paths = listed.stdout.trim().split("\n").filter(path => !isExcludedTestPath(root, path));
const taxonomy = testTaxonomy(root);
const legacy = new Map<string, number>();
for (const path of paths) {
  const parts = path.split("/");
  for (let index = 0; index < parts.length - 1; index++) {
    const part = parts[index]!;
    if (part === taxonomy.testFixturesDirName) break;
    if (["fixture", "fixtures", "testfixture", "testfixtures", "testdata"].includes(part.replace(/[^\p{L}\p{N}]/gu, "").toLowerCase())) {
      const owner = parts.slice(0, index + 1).join("/");
      legacy.set(owner, (legacy.get(owner) ?? 0) + 1);
      break;
    }
  }
}
writeFileSync(join(ticket, "📓️legacy-fixture-directory-census-2026-09-09.md"), "# Legacy Fixture Directory Census\n\nThe first fixture-like directory in each eligible authored path is counted; interiors of canonical opaque fixture examples are excluded.\n\n```json\n" + JSON.stringify([...legacy].sort(), null, 2) + "\n```\n");
console.log(`[DEBUG] ${legacy.size} noncanonical fixture-like roots`);

let manifests = 0;
const sources = paths.map(path => {
  const manifest = ["Cargo.toml", "package.json"].includes(path.split("/").at(-1)!) && !path.split("/").includes(taxonomy.testFixturesDirName);
  if (manifest) manifests++;
  return { path, source: manifest ? readFileSync(join(root, path), "utf8") : "" };
});
const findings = inspectTestLayoutSources(taxonomy, sources).filter(row => row.code === "production-fixture-dependency");
writeFileSync(join(ticket, "🗑️generated/coordinator/fixture-manifest-dependencies.json"), JSON.stringify(findings, null, 2) + "\n");
writeFileSync(join(ticket, "📓️fixture-manifest-dependencies-2026-09-09.md"), "# Package Manifest Fixture Dependencies\n\n" + JSON.stringify({ files: paths.length, manifests, findings: findings.length }) + "\n\n```json\n" + JSON.stringify(findings, null, 2) + "\n```\n");
console.log(`[DEBUG] ${paths.length} paths, ${manifests} manifests, ${findings.length} fixture dependencies`);
process.exitCode = findings.length ? 1 : 0;
