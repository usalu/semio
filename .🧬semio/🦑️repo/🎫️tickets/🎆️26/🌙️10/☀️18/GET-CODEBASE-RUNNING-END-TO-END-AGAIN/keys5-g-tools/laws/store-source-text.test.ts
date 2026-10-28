/** 🏪️ Key-membership law of `@semio-tech/framework:test-source-text`: the Rust files the store source text READS (independent Python reader `store-read-set.py`) are exactly the files the target's declared store inputs select, by the repository matcher AND by fast-glob (third-party oracle).
 * Root: `KEYS5_ROOT` (a mirror root) or the live checkout. Red on a project.json without the store inputs (the test reads the store Rust module files, the key held none).
 * @see https://github.com/mrmlnc/fast-glob */
import { expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import fg from "fast-glob";
import { globMatchesV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🟦️.ts";

const root = process.env.KEYS5_ROOT ?? "/Users/ueli/Documents/semio";
const here = import.meta.dir;
const STORE = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
const project = JSON.parse(readFileSync(join(root, "🧰️framework/📦️packages/🟦️typescript/📋️project.json"), "utf8"));
const inputs: string[] = project.targets["test-source-text"].inputs.filter((input: unknown) => typeof input === "string" && input.replace(/^!/u, "").startsWith(`{workspaceRoot}/${STORE}`));
const positive = inputs.filter((input) => !input.startsWith("!")), negative = inputs.filter((input) => input.startsWith("!"));
const bare = (pattern: string) => pattern.replace(/^!?\{workspaceRoot\}\//u, "");

function listFiles(directory: string): string[] {
  const out: string[] = [];
  for (const entry of readdirSync(join(root, directory), { withFileTypes: true })) {
    if (entry.name === "node_modules") continue;
    const path = `${directory}/${entry.name}`;
    if (entry.isDirectory()) out.push(...listFiles(path));
    else if (entry.isFile()) out.push(path);
  }
  return out;
}
const bytes = (left: string, right: string) => Buffer.compare(Buffer.from(left), Buffer.from(right));

test("the python reader finds the store source files the text is built from", () => {
  const run = spawnSync("python3", ["-I", join(here, "store-read-set.py"), root], { encoding: "utf8" });
  expect(run.status).toBe(0);
  const oracle = run.stdout.split("\n").filter(Boolean);
  expect(oracle.length).toBeGreaterThan(30);
  expect(oracle[0]!.endsWith("🏪️store/⚙️engine/🎞️live/📥️lifted/🏪️store/⏳️deferred-reprojection-and-supersede-reports/🦀️.rs")).toBe(true);
});

test("the declared store inputs select exactly the files the source text reads, by the repository matcher and by fast-glob", () => {
  const oracle = spawnSync("python3", ["-I", join(here, "store-read-set.py"), root], { encoding: "utf8" }).stdout.split("\n").filter(Boolean).sort(bytes);
  const candidates = listFiles(STORE).filter((file) => file.endsWith("🦀️.rs"));
  const selected = candidates.filter((file) => positive.some((pattern) => globMatchesV1(bare(pattern), file)) && !negative.some((pattern) => globMatchesV1(bare(pattern), file))).sort(bytes);
  const third = fg.sync(positive.map(bare), { cwd: root, ignore: negative.map((pattern) => bare(pattern.slice(1))), dot: true, onlyFiles: true, unique: true }).sort(bytes);
  expect(positive.length).toBeGreaterThan(0);
  expect(oracle.filter((file) => !selected.includes(file))).toEqual([]);
  expect(selected.filter((file) => !oracle.includes(file))).toEqual([]);
  expect(third).toEqual(selected);
});
