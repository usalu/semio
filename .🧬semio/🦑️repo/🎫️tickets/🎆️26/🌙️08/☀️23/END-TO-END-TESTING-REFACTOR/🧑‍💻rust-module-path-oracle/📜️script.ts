import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const input = import.meta.dir;
const ticket = dirname(input);
const generated = join(ticket, "🗑️generated", "rust-inline-module-oracle");
const repo = resolve(ticket, "../../../../../../..");
const cases = [
  { name: "non-mod-imported", source: "📦️crate/lib.rs", target: "📦️crate/🏛️owner/outer/🧪️tests/🔬️imported/🦀️.rs", compile: "passed" },
  { name: "default-non-mod", source: "🏛️default-non-mod/🦀️.rs", target: "🏛️default-non-mod/outer/🧪️tests/🔬️default/🦀️.rs", compile: "passed" },
  { name: "dot-parent", source: "🏛️dot-parent/🦀️.rs", target: "🏛️dot-parent/🧪️tests/🔬️dot/🦀️.rs", compile: "passed" },
  { name: "empty-parent", source: "🏛️empty-parent/🦀️.rs", target: "🏛️empty-parent/🧪️tests/🔬️empty/🦀️.rs", compile: "passed" },
  { name: "named-parent", source: "🏛️named-parent/🦀️.rs", target: "🏛️named-parent/renamed-scope/🧪️tests/🔬️named/🦀️.rs", compile: "passed" },
  { name: "mixed-present", source: "🏛️mixed-present/🦀️.rs", target: "🏛️mixed-present/reassigned-scope/🧪️tests/🔬️mixed-present/🦀️.rs", compile: "passed" },
  { name: "mixed-absent", source: "🏛️mixed-absent/🦀️.rs", target: "🏛️mixed-absent/reassigned-scope/🧪️tests/🔬️mixed-absent/🦀️.rs", compile: "failed" },
] as const;
const run = (command: string, args: string[]) => spawnSync(command, args, { cwd: input, encoding: "utf8" });
const output = (result: ReturnType<typeof run>) => ({ status: result.status, signal: result.signal, stdout: result.stdout, stderr: result.stderr });
const maskRust = (source: string) => source.replace(/\/\/[^\n]*|\/\*[\s\S]*?\*\/|"(?:\\.|[^"\\])*"/g, value => value.replace(/[^\n]/g, " "));
const missingPredecessor = (path: string) => {
  const parts = path.split("/").filter(Boolean);
  let current = path.startsWith("/") ? "/" : "";
  for (const part of parts) {
    current = current === "/" ? `/${part}` : `${current}/${part}`;
    if (!existsSync(current)) return current;
  }
  return null;
};
const auditCurrentTraversals = () => {
  const files = spawnSync("rg", ["-l", "--glob", "*.rs", "🧪️tests", "🧰️framework", "✏️s", "🌎️hub"], { cwd: repo, encoding: "utf8", maxBuffer: 128 * 1024 * 1024 });
  if (files.status !== 0) throw new Error(files.stderr);
  const traversals: { source: string; line: number; module: string; literal: string; rawTarget: string; normalizedTarget: string; exists: boolean; missingPredecessor: string | null }[] = [];
  for (const sourcePath of files.stdout.split("\n").filter(Boolean)) {
    const source = readFileSync(join(repo, sourcePath), "utf8");
    const attributes = new Map<number, string>();
    for (const match of source.matchAll(/#\[\s*path\s*=\s*"([^"\r\n]*)"\s*\](?:\s*#\[[^\]]*\]\s*)*\s*(?:pub(?:\([^)]*\))?\s+)?(mod\s+[A-Za-z_][A-Za-z0-9_]*\s*[;{])/g)) attributes.set(match.index! + match[0].lastIndexOf(match[2]), match[1]);
    const stack: ({ base: string } | null)[] = [], masked = maskRust(source), file = join(repo, sourcePath);
    for (const match of masked.matchAll(/\bmod\s+([A-Za-z_][A-Za-z0-9_]*)\s*([;{])|[{}]/g)) {
      const token = match[0], at = match.index!;
      if (token === "}") { stack.pop(); continue; }
      const base = [...stack].reverse().find((entry): entry is { base: string } => entry !== null)?.base ?? dirname(file);
      if (token === "{") { stack.push(null); continue; }
      const literal = attributes.get(at), module = match[1]!, delimiter = match[2]!;
      if (delimiter === "{") { stack.push({ base: literal === undefined ? `${base}/${module}` : `${base}/${literal}` }); continue; }
      if (literal === undefined) continue;
      const target = `${base}/${literal}`;
      if (!target.includes("/../") || !target.includes("/🧪️tests/")) continue;
      const rawTarget = target.slice(repo.length + 1), missing = missingPredecessor(target);
      traversals.push({ source: sourcePath, line: source.slice(0, at).split("\n").length, module, literal, rawTarget, normalizedTarget: relative(repo, resolve(target)), exists: existsSync(target), missingPredecessor: missing === null ? null : relative(repo, missing) });
    }
  }
  const report = { rustFiles: files.stdout.split("\n").filter(Boolean).length, traversals };
  writeFileSync(join(generated, "current-test-traversal-audit.json"), JSON.stringify(report, null, 2) + "\n");
  console.log(JSON.stringify({ rustFiles: report.rustFiles, traversals: traversals.length, missing: traversals.filter(entry => !entry.exists).length }, null, 2));
};
mkdirSync(generated, { recursive: true });
if (process.argv[2] === "audit-current-traversals") { auditCurrentTraversals(); process.exit(0); }
const rustc = output(run("rustc", ["--version"]));
const results = cases.map((fixture) => {
  const binary = join(generated, `${fixture.name}.test`);
  const compiler = output(run("rustc", ["--edition", "2021", "--crate-name", `path_oracle_${fixture.name.replaceAll("-", "_")}`, "--test", fixture.source, "-o", binary]));
  const execution = compiler.status === 0 ? output(run(binary, ["--nocapture"])) : undefined;
  writeFileSync(join(generated, `${fixture.name}.compiler.stdout.log`), compiler.stdout);
  writeFileSync(join(generated, `${fixture.name}.compiler.stderr.log`), compiler.stderr);
  if (execution) {
    writeFileSync(join(generated, `${fixture.name}.test.stdout.log`), execution.stdout);
    writeFileSync(join(generated, `${fixture.name}.test.stderr.log`), execution.stderr);
  }
  return { ...fixture, compiler, execution, observed: compiler.status === 0 ? "passed" : "failed" };
});
const report = { rustc, cwd: relative(ticket, input), results };
writeFileSync(join(generated, "compiler-results.json"), JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report, null, 2));
if (rustc.status !== 0 || results.some((result) => result.observed !== result.compile || result.execution && result.execution.status !== 0)) process.exit(1);
