import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { dirname, join, relative, resolve } from "node:path";

const root = process.env.SEMIO_LAYOUT_REPO_ROOT;
if (!root) throw new Error("SEMIO_LAYOUT_REPO_ROOT is required");
const ticket = dirname(import.meta.dir);
const preimage = await readFile(join(ticket, "📓️spr-preservation-preimages-2026-09-09.md"), "utf8");
const data = JSON.parse(preimage.match(/```json\n([\s\S]*?)\n```/)![1]!);
const moved = (path: string) => path.replace(/\/🧪️tests\/(📔️registry|🧬️mutation-laws)\/🧬️mutations\//, "/🧫️fixtures/$1/🧬️mutations/");
let bodies = 0, tests = 0, fixtures = 0, edges = 0;
for (const [oldPath, original] of Object.entries(data.entries) as [string, string][]) {
  const currentPath = moved(oldPath);
  const current = await readFile(join(root, currentPath), "utf8");
  assert.equal(current.includes("#[test]"), false);
  for (const match of current.matchAll(/(?:#\[path\s*=\s*|include_(?:str|bytes)!\s*\(\s*)"([^"]+)"/g)) {
    await readFile(join(root, dirname(currentPath), match[1]!));
    edges++;
  }
  const start = original.match(/#\[cfg\(test\)\]\s*mod tests\s*\{/);
  if (!start) { assert.equal(current, original); continue; }
  const end = original.lastIndexOf("\n}");
  const mount = current.match(/#\[cfg\(test\)\]\s*#\[path\s*=\s*"([^"]+)"\]\s*mod tests;/)!;
  assert.ok(mount);
  const testPath = resolve(root, dirname(currentPath), mount[1]!);
  assert.match(testPath, /\/🧪️tests\/[^/]+\/🦀️\.rs$/);
  const inner = original.slice(start.index! + start[0].length, end);
  const expected = inner.replace(/(include_(?:str|bytes)!\s*\(\s*)"([^"]+)"(\s*\))/g, (_match, prefix, path, suffix) => prefix + JSON.stringify(relative(dirname(testPath), resolve(root, dirname(currentPath), path)).replaceAll("\\", "/")) + suffix).replace(/^    /gm, "").trim();
  const actual = (await readFile(testPath, "utf8")).replace(/^\/\/!.*\n/, "").trim();
  assert.equal(actual, expected, oldPath);
  assert.equal(current.slice(0, mount.index), original.slice(0, start.index));
  assert.equal(current.slice(mount.index! + mount[0].length), original.slice(end + 2));
  for (const match of actual.matchAll(/include_(?:str|bytes)!\s*\(\s*"([^"]+)"/g)) { await readFile(join(dirname(testPath), match[1]!)); edges++; }
  bodies++;
  tests += [...actual.matchAll(/#\[test\]/g)].length;
}
for (const [oldPath, original] of Object.entries(data.fixtures) as [string, string][]) {
  const path = moved(oldPath);
  const expected = original.replace(/\/🧪️tests\/(📔️registry|🧬️mutation-laws)\/🧬️mutations/g, "/🧫️fixtures/$1/🧬️mutations");
  assert.equal(await readFile(join(root, path), "utf8"), expected, oldPath);
  fixtures++;
}
assert.equal(bodies, 10);
assert.equal(tests, 13);
assert.equal(fixtures, 24);
console.log(`[DEBUG] Preserved ${tests} tests in ${bodies} canonical cases, ${fixtures} fixtures with only owner-path changes, and ${edges} literal Rust paths`);
const binary = process.argv[2];
if (binary) {
  const run = async (args: string[]) => {
    const child = Bun.spawn([binary, ...args], { cwd: root, stdout: "pipe", stderr: "pipe" });
    const [code, stdout, stderr] = await Promise.all([child.exited, new Response(child.stdout).text(), new Response(child.stderr).text()]);
    assert.equal(code, 0, stdout + stderr);
    return stdout;
  };
  const names = (await run(["--list"])).split("\n").filter(line => line.endsWith(": test") && /(?:command::mutation_laws_fixture|testkit::mutation_law_fixture)::mutations::.*::tests::/.test(line)).map(line => line.slice(0, -6));
  assert.equal(names.length, 13);
  for (const name of names) {
    assert.match(await run([name, "--exact", "--nocapture"]), /1 passed; 0 failed/);
    console.log(`[DEBUG] ${name} PASS`);
  }
  console.log(`[DEBUG] All ${names.length} extracted SPR laws passed`);
}
