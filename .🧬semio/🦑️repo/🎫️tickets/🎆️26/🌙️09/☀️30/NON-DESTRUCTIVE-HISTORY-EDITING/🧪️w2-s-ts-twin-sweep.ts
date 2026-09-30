#!/usr/bin/env bun
/**
 * 🧹️ W2-S TS twin sweep: runs every exported zero-argument `test*` function of the given `🟦️.ts` files, each file in its own
 * bun process under a deadline, and prints PASS/FAIL/NONE per function. Detects twins broken by a schema rewrite (an
 * aggregate that became a `oneOf` of leaf `$ref`s, a leaf that gained its tag `const`).
 *
 *   bun 🧪️w2-s-ts-twin-sweep.ts <list-of-files.txt> [deadline-seconds]
 */
import { readFileSync } from "node:fs";

const repo = "/Users/ueli/Documents/semio";
const [list, seconds = "90"] = process.argv.slice(2);
const files = readFileSync(list!, "utf8").split("\n").filter((line) => line.trim() !== "");
const probe = (file: string): string => `
const module = await import(${JSON.stringify(`${repo}/${file}`)});
const names = Object.entries(module).filter(([name, value]) => name.startsWith("test") && typeof value === "function" && value.length === 0).map(([name]) => name);
if (names.length === 0) console.log("[w2-s] NONE");
for (const name of names) {
  try {
    await module[name]();
    console.log("[w2-s] PASS " + name);
  } catch (error) {
    console.log("[w2-s] FAIL " + name + " :: " + String(error?.message ?? error).split("\\n").slice(0, 3).join(" | ").slice(0, 400));
  }
}`;
for (const file of files) {
  const child = Bun.spawn(["bun", "-e", probe(file)], { cwd: repo, stdout: "pipe", stderr: "pipe" });
  const timer = setTimeout(() => child.kill(), Number(seconds) * 1000);
  const [stdout, stderr] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text()]);
  clearTimeout(timer);
  const lines = stdout.split("\n").filter((line) => line.startsWith("[w2-s] "));
  const verdict = lines.length > 0 ? lines.map((line) => line.slice(7)).join("; ") : `CRASH :: ${stderr.split("\n").filter((line) => /error/i.test(line)).slice(0, 2).join(" | ").slice(0, 400)}`;
  console.log(`${file}\n    ${verdict}`);
}
