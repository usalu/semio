#!/usr/bin/env python3
"""oracle2-g: rewrites the staged read-oracle test file for the exception cover (one-off authoring tool; anchored)."""
import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def cut(start, end):
    a = s.index(start); b = s.index(end, a)
    return a, b
def replace_block(start, end, new):
    global s
    a, b = cut(start, end)
    s = s[:a] + new + s[b:]
def rep(old, new):
    global s
    assert s.count(old) == 1, (old[:70], s.count(old))
    s = s.replace(old, new)

rep('import {mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync} from "node:fs";', 'import {mkdirSync, mkdtempSync, readdirSync, realpathSync, rmSync, writeFileSync} from "node:fs";')
rep('import {coverV1, namedFilesV1, planGrantsV1, planV1} from "../🪢️policy/🟦️.ts";', 'import {captureV1} from "../🏃️run/🟦️.ts";\nimport {namedFilesV1, planGrantsV1, planV1, splitPlanV1, type ReadPlanV1} from "../🪢️policy/🟦️.ts";')

replace_block('const tree = corpus.tree as Record<string, string>', 'const python = () => {', '''const tree = corpus.tree as Record<string, string>, files = Object.keys(tree).sort();
const directoriesOf = (paths: readonly string[]) => new Set(paths.flatMap((path) => path.split("/").slice(0, -1).map((_, index, parts) => parts.slice(0, index + 1).join("/"))));
const listerOf = (paths: readonly string[]) => {
  const directories = directoriesOf(paths);
  return (directory: string) => {
    if (directory !== "" && !directories.has(directory)) return undefined;
    const prefix = directory ? `${directory}/` : "";
    return [...new Set(paths.filter((path) => path.startsWith(prefix)).map((path) => path.slice(prefix.length).split("/")[0]!))];
  };
};
const directories = directoriesOf(files), lister = listerOf(files);
const ROOT = "/w/ws", here = import.meta.dirname, oracle = join(here, "🐍️oracle/🐍️.py");

type Definition = {allowed: string[]; ambient: string[]; names: string[]};
const planOf = (row: Definition & {tree?: string[]; refusals?: boolean}) => planV1(ROOT, row, listerOf(row.tree ?? files), {refusals: row.refusals ?? true});
const linuxFixture = (row: (typeof corpus.linuxGrants)[number]) => ({list: (absolute: string) => (row.tree as Record<string, string[]>)[absolute], isDirectory: (absolute: string) => row.directories.includes(absolute), real: (absolute: string) => absolute});

let seed = 20261026;
const random = () => ((seed = (seed * 1664525 + 1013904223) % 4294967296) / 4294967296);
const pick = <T>(values: readonly T[]): T => values[Math.floor(random() * values.length)]!;
const WORDS = ["🧪", "📦", "a", "b c", 'q"x', "p.json", "back\\\\slash"];

/** The random workspaces of the laws: a real directory per case, so the Python evaluator and the macOS kernel see the same files as the cover. */
const base = realpathSync(mkdtempSync(join(tmpdir(), "ro-law-")));
const laws = Array.from({length: 40}, (_, index) => {
  const root = join(base, `case-${index}`), made: string[] = [];
  const grow = (directory: string, depth: number) => {
    const width = 2 + Math.floor(random() * 5);
    for (let at = 0; at < width; at++) {
      const name = `${pick(WORDS)}${at}`, path = directory ? `${directory}/${name}` : name;
      if (depth < 3 && random() < 0.45) grow(path, depth + 1);
      else { const file = name.startsWith("p.json") ? `${directory ? `${directory}/` : ""}p.json` : path; if (!made.includes(file)) { mkdirSync(join(root, dirname(file)), {recursive: true}); writeFileSync(join(root, file), file); made.push(file); } }
    }
  };
  mkdirSync(root, {recursive: true});
  grow("", 0);
  const share = [0.05, 0.5, 0.9, 1][index % 4]!, top = [...new Set(made.map((path) => path.split("/")[0]!))];
  const allowed = made.filter(() => random() < share).sort(), ambient = index % 5 === 0 ? [pick(top)] : [], names = index % 3 === 0 ? ["p.json"] : [];
  return {root, files: made.slice().sort(), allowed: [...allowed, "gone/none.txt"], ambient, names};
});
const expectedOf = (row: {files: string[]; allowed: string[]; ambient: string[]; names: string[]}) => {
  const trees = ["node_modules", ".cache", ".git", "target", ".🧬semio/🦑️repo/⚡️cache", ...row.ambient];
  return row.files.filter((path) => row.allowed.includes(path) || trees.some((t) => path === t || path.startsWith(`${t}/`)) || row.names.includes(path.slice(path.lastIndexOf("/") + 1))).sort();
};
const realLister = (root: string) => (directory: string) => { try { return readdirSync(directory ? join(root, directory) : root); } catch { return undefined; } };
const lawPlans = laws.map((row) => planV1(row.root, row, realLister(row.root), {refusals: true, alternatives: true}));
const capacityOf = (plan: ReadPlanV1) => Math.max(3, Math.ceil(plan.rules.length / 3));
const lawRuns = lawPlans.map((plan) => splitPlanV1(plan, capacityOf(plan)));

const tiny = Array.from({length: 60}, (_, index) => {
  let paths: string[] = [];
  for (let attempt = 0; attempt < 50; attempt++) {
    paths = [];
    const top = 1 + Math.floor(random() * 3);
    for (let at = 0; at < top; at++) {
      const name = `n${at}`;
      if (random() < 0.5) paths.push(name);
      else for (let leaf = 0; leaf < 1 + Math.floor(random() * 2); leaf++) paths.push(`${name}/${random() < 0.2 ? `d${leaf}/` : ""}f${leaf}`);
    }
    if (new Set([...directoriesOf(paths), ...paths]).size + 1 <= 8) break;
  }
  return {files: paths.sort(), allowed: paths.filter(() => random() < 0.5), ambient: [] as string[], names: random() < 0.2 ? ["f0"] : [], refusals: index % 4 !== 0};
});

const probe = {
  root: ROOT,
  policy: Object.fromEntries(corpus.policy.map((row) => [row.id, {profile: seatbeltProfileV1(planOf(row)), rules: planOf(row).rules}])),
  linuxGrants: Object.fromEntries(corpus.linuxGrants.map((row) => [row.id, landlockGrantsV1({...planV1(row.root, row, (directory) => (row.tree as Record<string, string[]>)[directory ? join(row.root, directory) : row.root], {refusals: false}), ambient: row.ambient.concat(["node_modules"]), names: row.names}, linuxFixture(row))])),
  random: laws.map((row, index) => ({root: row.root, files: row.files, allowed: row.allowed, ambient: row.ambient, names: row.names, profile: seatbeltProfileV1(lawPlans[index]!), runProfiles: (lawRuns[index] ?? []).map(seatbeltProfileV1)})),
  brute: tiny.map((row) => ({files: row.files, allowed: row.allowed, ambient: row.ambient, names: row.names, refusals: row.refusals})),
};
''')
rep('''const python = () => {
  const scratch''', '''let memo: ReturnType<typeof JSON.parse> | undefined;
const python = () => {
  if (memo) return memo;
  const scratch''')
rep('''    expect(done.status, done.stderr).toBe(0);
    return JSON.parse(done.stdout);''', '''    expect(done.status, done.stderr).toBe(0);
    return (memo = JSON.parse(done.stdout));''')
open(p, "w", encoding="utf-8").write(s)
