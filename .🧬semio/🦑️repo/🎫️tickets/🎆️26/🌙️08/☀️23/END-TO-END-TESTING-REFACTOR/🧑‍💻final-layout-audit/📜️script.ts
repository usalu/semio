import { readFileSync } from "node:fs";
import { relative } from "node:path";
import { inspectTestLayoutSources, isExcludedTestPath, testImplementationFilenames, testTaxonomy, type TestLayoutSource } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = process.cwd();
const exclusions = ["--glob", "!node_modules/**", "--glob", "!.git/**", "--glob", "!.🧬semio/**"];
const list = Bun.spawnSync({ cmd: ["rg", "--files", "--no-ignore", "--hidden", ...exclusions], cwd: repoRoot, stdout: "pipe" });
if (list.exitCode !== 0) throw new Error("rg --files failed");

const taxonomy = testTaxonomy(repoRoot);
const extensions = /\.(?:ts|tsx|mts|cts|js|jsx|mjs|cjs|py|go)$/u;
const javascript = /\.(?:ts|tsx|mts|cts|js|jsx|mjs|cjs)$/u;
const python = /\.py$/u;
const go = /\.go$/u;
const files = new TextDecoder().decode(list.stdout).split("\n").filter(Boolean).map(path => path.split("\\").join("/")).filter(path => extensions.test(path) && !isExcludedTestPath(repoRoot, path));
const matchingPaths = (glob: string, pattern: string): readonly string[] => {
  const match = Bun.spawnSync({ cmd: ["rg", "-l", "--no-ignore", "--hidden", ...exclusions, "--glob", glob, "-e", pattern], cwd: repoRoot, stdout: "pipe" });
  if (match.exitCode !== 0 && match.exitCode !== 1) throw new Error(`rg candidate query failed for ${glob}`);
  return new TextDecoder().decode(match.stdout).split("\n").filter(Boolean).map(path => path.split("\\").join("/"));
};
const isCaseSource = (path: string): boolean => path.includes(`/${taxonomy.testsDirName}/`);
const isCanonicalImplementation = (path: string): boolean => {
  const segments = path.split("/");
  const index = segments.lastIndexOf(taxonomy.testsDirName);
  return index >= 0 && index === segments.length - 3 && testImplementationFilenames(taxonomy).includes(segments.at(-1)!);
};
const executableCandidate = (path: string, text: string): boolean =>
  javascript.test(path)
    ? /import\.meta\.vitest|SelfTests?|\b(?:async\s+)?function\s+test[A-Z]|\b(?:const|let|var)\s+test[A-Z]|\bplay\s*[:=]|(?:from|require\()["'](?:@jest\/globals|@playwright\/test|@storybook\/test|ava|bun:test|jest|mocha|node:test|vitest|node:assert(?:\/strict)?)["']/u.test(text)
    : python.test(path)
      ? /^\s*(?:async\s+)?def\s+test[A-Za-z0-9_]*\s*\(|^class\s+Test[A-Za-z0-9_]*\b|\b(?:unittest|pytest)\b/mu.test(text)
      : go.test(path)
        ? /^func\s+(?:Test|Benchmark|Fuzz)[A-Z0-9_][A-Za-z0-9_]*\s*\(/mu.test(text)
        : false;
const candidatePaths = new Set([
  ...matchingPaths("*.{ts,tsx,mts,cts,js,jsx,mjs,cjs}", "import\\.meta\\.vitest|SelfTests?|\\b(?:async\\s+)?function\\s+test[A-Z]|\\b(?:const|let|var)\\s+test[A-Z]|\\bplay\\s*[:=]|(?:from|require\\()[\\\"'](?:@jest/globals|@playwright/test|@storybook/test|ava|bun:test|jest|mocha|node:test|vitest|node:assert(?:/strict)?)[\\\"']"),
  ...matchingPaths("*.py", "^\\s*(?:async\\s+)?def\\s+test[A-Za-z0-9_]*\\s*\\(|^class\\s+Test[A-Za-z0-9_]*\\b|\\b(?:unittest|pytest)\\b"),
  ...matchingPaths("*.go", "^func\\s+(?:Test|Benchmark|Fuzz)[A-Z0-9_][A-Za-z0-9_]*\\s*\\(")
]);
const selectedPaths = files.filter(path => isCaseSource(path) || isCanonicalImplementation(path) || candidatePaths.has(path));
const source = new Map(selectedPaths.map(path => [path, readFileSync(path, "utf8")]));
const inspected = [...source].filter(([path, text]) => isCaseSource(path) || isCanonicalImplementation(path) || executableCandidate(path, text)).map(([path, source]): TestLayoutSource => ({ path, source })).sort((a, b) => a.path.localeCompare(b.path));
const findings = inspectTestLayoutSources(taxonomy, inspected).filter(finding => extensions.test(finding.path));
console.log(JSON.stringify({
  inspected: inspected.length,
  canonicalImplementations: inspected.filter(entry => isCanonicalImplementation(entry.path)).length,
  executableCandidates: inspected.filter(entry => executableCandidate(entry.path, entry.source)).length,
  findings
}, null, 2));
