// Law of the hybrid cover: for random workspace layouts and random declared sets the rendered Seatbelt profile grants EXACTLY the declared files, as (1) the TypeScript grant function, (2) an independent Python evaluator of the rendered text and (3) the macOS kernel itself (sandbox-exec + cat) say; and it is never larger than one literal per declared file.
// run: bun test ./<this file>   (always with ./ : a bare path is a filter that scans the whole disk)
import {expect, test} from "bun:test";
import {mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync, readdirSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {hybridCoverV1, hybridGrantsV1} from "../cover.mjs";
import {hybridProfileV1} from "../oracle.mjs";

let seed = 20261026;
const random = () => ((seed = (seed * 1664525 + 1013904223) % 4294967296) / 4294967296);
const root = realpathSync(mkdtempSync(join(tmpdir(), "hybrid-law-")));
const files: string[] = [];
const make = (directory: string, depth: number) => {
  const width = 2 + Math.floor(random() * 5);
  for (let index = 0; index < width; index++) {
    const name = `${["🧪", "📦", "a", "b c", 'q"x'][Math.floor(random() * 5)]}${index}`;
    if (depth < 3 && random() < 0.45) { mkdirSync(join(root, directory, name), {recursive: true}); make(directory ? `${directory}/${name}` : name, depth + 1); }
    else { const path = directory ? `${directory}/${name}.txt` : `${name}.txt`; mkdirSync(join(root, directory), {recursive: true}); writeFileSync(join(root, path), path); files.push(path); }
  }
};
make("", 0);
files.sort();
const list = (directory: string) => { try { return readdirSync(directory ? join(root, directory) : root); } catch { return undefined; } };
const cases = Array.from({length: 24}, (_, index) => { const share = [0.05, 0.5, 0.9, 1][index % 4]!; return files.filter(() => random() < share).sort(); });
cases.push([], files.slice());
const covers = cases.map((allowed) => hybridCoverV1(allowed, list));
const profiles = Object.fromEntries(covers.map((cover, index) => [String(index), hybridProfileV1(root, cover, [])]));

test("the TypeScript grant function equals the declared set and the cover is never larger than one literal per file", () => {
  covers.forEach((cover, index) => {
    expect(files.filter((path) => hybridGrantsV1(cover, path)), `case ${index}`).toEqual(cases[index]!);
    expect(cover.rules.length + cover.denials.length, `case ${index} size`).toBeLessThanOrEqual(Math.max(1, cases[index]!.length));
  });
});

test("the hybrid wins where a directory is mostly declared", () => {
  const all = files.slice(), almost = files.slice(1);
  expect(hybridCoverV1(all, list).rules.length).toBeLessThan(files.length);
  const cover = hybridCoverV1(almost, list);
  expect(cover.rules.length + cover.denials.length).toBeLessThan(almost.length);
  expect(files.filter((path) => hybridGrantsV1(cover, path))).toEqual(almost.slice().sort());
});

test("an independent Python evaluator of the rendered profile text grants exactly the declared set", () => {
  const probe = join(root, "probe.json"), here = import.meta.dirname;
  writeFileSync(probe, JSON.stringify({root, files, profiles}));
  const run = spawnSync("python3", ["-I", join(here, "evaluate.py"), probe], {encoding: "utf8", maxBuffer: 64 * 1024 * 1024});
  expect(run.status, run.stderr).toBe(0);
  const evaluated = JSON.parse(run.stdout) as Record<string, string[]>;
  cases.forEach((allowed, index) => expect(evaluated[String(index)], `case ${index}`).toEqual(allowed.slice().sort()));
});

test.skipIf(process.platform !== "darwin")("the macOS kernel enforces the rendered profile: declared files read, every other file is refused", () => {
  const picks = [3, 7, 11, 25];
  for (const index of picks) {
    const profile = join(root, `kernel-${index}.sb`);
    writeFileSync(profile, profiles[String(index)]!);
    for (const path of files) {
      const read = spawnSync("sandbox-exec", ["-f", profile, "/bin/cat", join(root, path)], {encoding: "utf8"});
      expect(read.status === 0, `case ${index} ${path}`).toBe(cases[index]!.includes(path));
    }
  }
});

test("cleanup", () => { rmSync(root, {recursive: true, force: true}); expect(true).toBe(true); });
