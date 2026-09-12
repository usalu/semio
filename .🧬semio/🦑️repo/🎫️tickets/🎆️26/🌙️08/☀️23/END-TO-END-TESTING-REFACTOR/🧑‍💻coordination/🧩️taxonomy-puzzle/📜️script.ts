#!/usr/bin/env bun

import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { pathToFileURL } from "node:url";

type Case = Readonly<{ module: string; owner: string; destination: string; tests: number }>;
type Baseline = Readonly<{ cases: readonly Readonly<{ module: string; owner: string; destination: string; bodySha256: string; bodyBytes: number; testNames: readonly string[] }>[] }>;

const ticketRoot = resolve(import.meta.dir, "../..");
const generatedRoot = join(ticketRoot, "🗑️generated/testing-taxonomy/puzzle");

function findRepoRoot(start: string): string {
  let current = start;
  while (dirname(current) !== current) {
    if (existsSync(join(current, "✏️s")) && existsSync(join(current, "🧰️framework"))) return current;
    current = dirname(current);
  }
  throw new Error(`repo root not found from ${start}`);
}

const repoRoot = findRepoRoot(ticketRoot);
const cases: readonly Case[] = [
  {
    module: "tests",
    owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs",
    destination: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs",
    tests: 6,
  },
  {
    module: "vortex_payload_laws",
    owner: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs",
    destination: "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🧪️tests/🔬️unit/🦀️.rs",
    tests: 4,
  },
];

function sha256(value: string): string {
  return createHash("sha256").update(value).digest("hex");
}

function moduleBody(source: string, name: string): string {
  const declaration = new RegExp(`\\bmod\\s+${name}\\s*\\{`, "u").exec(source);
  if (!declaration) throw new Error(`inline module ${name} not found`);
  const opening = source.indexOf("{", declaration.index);
  let depth = 1;
  for (let index = opening + 1; index < source.length; index += 1) {
    const token = source[index]!;
    if (token === "/" && source[index + 1] === "/") {
      index += 2;
      while (index < source.length && source[index] !== "\n") index += 1;
      continue;
    }
    if (token === "/" && source[index + 1] === "*") {
      let nested = 1;
      index += 2;
      while (index < source.length && nested > 0) {
        if (source[index] === "/" && source[index + 1] === "*") { nested += 1; index += 2; continue; }
        if (source[index] === "*" && source[index + 1] === "/") { nested -= 1; index += 2; continue; }
        index += 1;
      }
      index -= 1;
      continue;
    }
    if (token === "\"") {
      index += 1;
      while (index < source.length) {
        if (source[index] === "\\") { index += 2; continue; }
        if (source[index] === "\"") break;
        index += 1;
      }
      continue;
    }
    if (token === "r" || token === "b" && source[index + 1] === "r") {
      const rawStart = token === "r" ? index : index + 1;
      const raw = /^r(#{0,255})"/u.exec(source.slice(rawStart));
      if (raw) {
        const terminator = `\"${raw[1]}`;
        const end = source.indexOf(terminator, rawStart + raw[0].length);
        if (end < 0) throw new Error(`unterminated raw string in ${name}`);
        index = end + terminator.length - 1;
        continue;
      }
    }
    if (token === "{") depth += 1;
    if (token === "}") {
      depth -= 1;
      if (depth === 0) return source.slice(opening + 1, index);
    }
  }
  throw new Error(`unterminated inline module ${name}`);
}

function testNames(source: string): string[] {
  return [...source.matchAll(/#\s*\[\s*test\s*\]\s*fn\s+([A-Za-z_][A-Za-z0-9_]*)/gu)].map(match => match[1]!);
}

function rel(path: string): string {
  return relative(repoRoot, path).split(sep).join("/");
}

function baseline(): void {
  mkdirSync(generatedRoot, { recursive: true });
  const entries = cases.map(entry => {
    const source = readFileSync(join(repoRoot, entry.owner), "utf8");
    const body = moduleBody(source, entry.module);
    const names = testNames(body);
    if (names.length !== entry.tests) throw new Error(`${entry.module} expected ${entry.tests} tests, found ${names.length}`);
    if (existsSync(join(repoRoot, entry.destination))) throw new Error(`destination already exists: ${entry.destination}`);
    return { module: entry.module, owner: entry.owner, destination: entry.destination, bodySha256: sha256(body), bodyBytes: Buffer.byteLength(body), testNames: names };
  });
  const output = join(generatedRoot, "inline-module-baseline.json");
  writeFileSync(output, `${JSON.stringify({ cases: entries }, null, 2)}\n`);
  console.log(JSON.stringify({ phase: "baseline", output: rel(output), cases: entries }, null, 2));
}

async function verify(): Promise<void> {
  mkdirSync(generatedRoot, { recursive: true });
  const baseline = JSON.parse(readFileSync(join(generatedRoot, "inline-module-baseline.json"), "utf8")) as Baseline;
  const sources: { path: string; source: string }[] = [];
  const results = cases.map(entry => {
    const before = baseline.cases.find(candidate => candidate.module === entry.module);
    if (!before) throw new Error(`baseline missing ${entry.module}`);
    const owner = readFileSync(join(repoRoot, entry.owner), "utf8");
    const destination = readFileSync(join(repoRoot, entry.destination), "utf8");
    const registration = new RegExp(`#\\[cfg\\(test\\)\\]\\s*#\\[path\\s*=\\s*\"🧪️tests/🔬️unit/🦀️\\.rs\"\\]\\s*mod\\s+${entry.module}\\s*;`, "u").test(owner);
    const names = testNames(destination);
    const postSha256 = sha256(destination);
    sources.push({ path: entry.owner, source: owner }, { path: entry.destination, source: destination });
    return {
      ...before,
      registration,
      inlineBodyAbsent: !new RegExp(`\\bmod\\s+${entry.module}\\s*\\{`, "u").test(owner),
      postSha256,
      postBytes: Buffer.byteLength(destination),
      bytePreserved: postSha256 === before.bodySha256 && Buffer.byteLength(destination) === before.bodyBytes,
      testNamesPreserved: JSON.stringify(names) === JSON.stringify(before.testNames),
    };
  });
  const guardPath = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts");
  const guard = await import(pathToFileURL(guardPath).href) as typeof import("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts");
  const findings = guard.inspectTestLayoutSources(guard.testTaxonomy(repoRoot), sources, cases.map(entry => dirname(entry.destination)));
  const output = join(generatedRoot, "structural-verification.json");
  writeFileSync(output, `${JSON.stringify({ results, findings }, null, 2)}\n`);
  console.log(JSON.stringify({ phase: "verify", output: rel(output), results, findings }, null, 2));
  if (results.some(result => !result.registration || !result.inlineBodyAbsent || !result.bytePreserved || !result.testNamesPreserved) || findings.length > 0) process.exitCode = 1;
}

async function rustfmtParse(): Promise<void> {
  mkdirSync(generatedRoot, { recursive: true });
  const paths = cases.flatMap(entry => [entry.owner, entry.destination]);
  const results = [];
  for (const path of paths) {
    const child = Bun.spawn(["rustfmt", "--edition", "2021", "--emit", "stdout", join(repoRoot, path)], { cwd: repoRoot, stdout: "ignore", stderr: "pipe" });
    const stderr = await new Response(child.stderr).text();
    const status = await child.exited;
    results.push({ path, status, stderr });
  }
  const output = join(generatedRoot, "rustfmt-parse.json");
  writeFileSync(output, `${JSON.stringify({ results }, null, 2)}\n`);
  console.log(JSON.stringify({ phase: "rustfmt-parse", output: rel(output), results }, null, 2));
  if (results.some(result => result.status !== 0)) process.exitCode = 1;
}

async function cargo(mode: "check" | "test", filter?: string): Promise<void> {
  mkdirSync(join(generatedRoot, "tmp"), { recursive: true });
  const args = mode === "check"
    ? ["cargo", "check", "--tests", "--features", "component-app-assembly", "-p", "semio-s-artifact-puzzle-3d"]
    : ["cargo", "test", "--features", "component-app-assembly", "-p", "semio-s-artifact-puzzle-3d", "--lib", filter!];
  const child = Bun.spawn(args, {
    cwd: repoRoot,
    env: { ...process.env, CARGO_TARGET_DIR: join(generatedRoot, "cargo-target"), CARGO_TERM_COLOR: "never", TMPDIR: join(generatedRoot, "tmp") },
    stdin: "inherit",
    stdout: "pipe",
    stderr: "pipe",
  });
  const interrupt = (): void => child.kill("SIGINT");
  process.once("SIGINT", interrupt);
  const [stdout, stderr, status] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  process.off("SIGINT", interrupt);
  writeFileSync(join(generatedRoot, `cargo-${mode}.stdout.log`), stdout);
  writeFileSync(join(generatedRoot, `cargo-${mode}.stderr.log`), stderr);
  console.log(stdout);
  console.error(stderr);
  console.log(JSON.stringify({ phase: `cargo-${mode}`, status, filter: filter ?? null }, null, 2));
  if (status !== 0) process.exitCode = status;
}

function clean(): void {
  if (dirname(generatedRoot) !== join(ticketRoot, "🗑️generated/testing-taxonomy")) throw new Error(`refusing cleanup outside ticket scope: ${generatedRoot}`);
  rmSync(generatedRoot, { recursive: true, force: true });
  console.log(JSON.stringify({ phase: "clean", removed: rel(generatedRoot) }, null, 2));
}

switch (process.argv[2]) {
  case "baseline": baseline(); break;
  case "verify": await verify(); break;
  case "rustfmt-parse": await rustfmtParse(); break;
  case "cargo-check": await cargo("check"); break;
  case "cargo-test": await cargo("test", process.argv[3]); break;
  case "clean": clean(); break;
  default: throw new Error("usage: 📜️script.ts <baseline|verify|rustfmt-parse|cargo-check|cargo-test FILTER|clean>");
}
