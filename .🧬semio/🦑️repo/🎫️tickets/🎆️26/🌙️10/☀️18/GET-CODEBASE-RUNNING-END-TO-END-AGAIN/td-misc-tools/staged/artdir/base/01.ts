import { expect, test } from "bun:test";
import { createRequire } from "node:module";
import { readFileSync, mkdirSync, mkdtempSync, rmSync } from "node:fs";
import { resolve, join, basename } from "node:path";
import { spawnSync } from "node:child_process";
import { existsSync, realpathSync } from "node:fs";

test("test commands require explicit budgets and preserve failure and cancellation", async () => {
  const owner = resolve(import.meta.dir, ".."), source = join(owner, "🟦️.ts"), require = createRequire(import.meta.url);
  const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
  const api = await import(source);
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR!; expect(output).toBeTruthy(); mkdirSync(output, { recursive: true });
  const temporary = mkdtempSync(join(output, "test-command-")), module = join(temporary, "execution.mjs");
  try {
    const bundle = await require("esbuild").build({ entryPoints: [source], outfile: module, bundle: true, platform: "node", format: "esm", metafile: true });
    expect(Object.keys(bundle.metafile.inputs).every(path => path.startsWith("🧰️framework/🔨️modules/🏃️process/")||path.startsWith("🧰️framework/🔨️modules/⚙️runtime/")||path.startsWith("🧰️framework/🔨️modules/🧬️schema/✅️validator/")||path.startsWith("🧰️framework/📦️packages/🟦️typescript/⚙️runtime/"))).toBe(true);
    for (const row of fixture.cases) {
      if (!row.throwOnFailure) continue;
      const cancellation = new AbortController(), timer = row.abortMs === null ? undefined : setTimeout(() => cancellation.abort(), row.abortMs);
      let observed = "success";
      try { await api.runBudgetedTestCommand("node", ["-e", row.program], { cwd: temporary, budgetMs: row.budgetMs, signal: cancellation.signal, throwOnFailure: row.throwOnFailure }); }
      catch (error) { observed = String(error); }
      finally { if (timer) clearTimeout(timer); }
      expect(observed).toContain(row.expected);
    }
    for (const runtime of [process.execPath, "node"]) for (const row of fixture.cases) {
      const program = `const row=JSON.parse(process.argv[2]),cancellation=new AbortController();if(row.abortMs!==null)setTimeout(()=>cancellation.abort(),row.abortMs);import(require('node:url').pathToFileURL(process.argv[1]).href).then(async api=>{try{await api.runBudgetedTestCommand('node',['-e',row.program],{cwd:process.cwd(),budgetMs:row.budgetMs,signal:cancellation.signal,throwOnFailure:row.throwOnFailure});console.log('success')}catch(error){console.log(String(error))}});`;
      const child = spawnSync(runtime, ["-e", program, module, JSON.stringify(row)], { cwd: temporary, encoding: "utf8", timeout: 4000 });
      expect(child.status, child.stderr).toBe(row.throwOnFailure ? 0 : row.exitCode);
      if (row.throwOnFailure) expect(child.stdout).toContain(row.expected);
    }
    const oracle = spawnSync("node", ["-e", fixture.cases[1].program], { cwd: temporary, encoding: "utf8" }); expect(oracle.status).toBe(fixture.cases[1].exitCode);
    await expect(api.runBudgetedTestCommand("node", ["-e", ""], { cwd: temporary, budgetMs: -1, throwOnFailure: true })).rejects.toThrow("Invalid test command budget");
  } finally { rmSync(temporary, { recursive: true, force: true }); }
});

const system = (program: string, args: readonly string[]) => spawnSync(program, [...args], { encoding: "utf8" });
const imageRows = (name: string): string[] => system("/usr/bin/hdiutil", ["info"]).stdout.split("\n").filter(line => line.startsWith("image-path") && line.includes(name));
const attachImage = (parent: string, name: string, size = "8m"): string => {
  const image = join(parent, name + ".dmg"), mount = join(parent, name);
  expect(system("/usr/bin/hdiutil", ["create", "-size", size, "-fs", "APFS", "-volname", name, "-nospotlight", "-quiet", image]).status).toBe(0);
  mkdirSync(mount);
  expect(system("/usr/bin/hdiutil", ["attach", "-nobrowse", "-noautoopen", "-owners", "on", "-mountpoint", mount, image]).status).toBe(0);
  return mount;
};

test.skipIf(process.platform !== "darwin")("a scratch root detaches nested disk images before removal and the system tool agrees", async () => {
  const api = await import(join(resolve(import.meta.dir, ".."), "🟦️.ts"));
  const root = api.createTestScratchRootV1(process.env.SEMIO_TEST_ARTIFACT_DIR!, "scratch-dispose-"), real = realpathSync(root), name = basename(root);
  try {
    attachImage(attachImage(root, "outer", "48m"), "inner");
    expect(imageRows(name)).toHaveLength(2);
    expect(api.attachedDiskImagesV1().filter((row: { image: string }) => row.image.startsWith(real + "/")).map((row: { image: string }) => row.image)).toEqual([join(real, "outer", "inner.dmg"), join(real, "outer.dmg")]);
  } finally { api.disposeTestScratchRootV1(root); }
  expect(imageRows(name)).toEqual([]);
  expect(existsSync(root)).toBe(false);
  expect(() => api.disposeTestScratchRootV1(root)).toThrow("Scratch root created by createTestScratchRootV1 required");
}, 60000);

test.skipIf(process.platform !== "darwin")("a process that dies holding a scratch root still releases its disk images on exit", async () => {
  const source = join(resolve(import.meta.dir, ".."), "🟦️.ts"), parent = process.env.SEMIO_TEST_ARTIFACT_DIR!;
  const program = `const api=await import(process.argv[1]);const {spawnSync}=require("node:child_process");const root=api.createTestScratchRootV1(process.argv[2],"scratch-exit-");const {mkdirSync}=require("node:fs");const image=root+"/e.dmg",mount=root+"/e";spawnSync("/usr/bin/hdiutil",["create","-size","8m","-fs","APFS","-volname","e","-nospotlight","-quiet",image]);mkdirSync(mount);const attached=spawnSync("/usr/bin/hdiutil",["attach","-nobrowse","-noautoopen","-owners","on","-mountpoint",mount,image]);console.log(JSON.stringify({root,status:attached.status}));throw Error("simulated test crash");`;
  const child = spawnSync(process.execPath, ["-e", program, source, parent], { encoding: "utf8", timeout: 50000 });
  const reported = JSON.parse(child.stdout.trim().split("\n")[0]!) as { root: string; status: number };
  expect(reported.status).toBe(0);
  expect(child.status).not.toBe(0);
  expect(imageRows(basename(reported.root))).toEqual([]);
  expect(existsSync(reported.root)).toBe(false);
}, 60000);
