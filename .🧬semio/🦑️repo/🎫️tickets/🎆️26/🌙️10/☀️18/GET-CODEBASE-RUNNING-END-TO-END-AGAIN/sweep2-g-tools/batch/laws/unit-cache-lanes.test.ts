/** 🧵️ Contract: the Cargo unit-cache lane pool and its retention are a scheduling domain: disjoint from every other domain and content island (checked by the independent Python oracle and by a second line-reader here), reachable from keyed code only through their ports, and out of every target key (an edit moves no keyed identity; the production closure of a target that really loads them names none), while the unit key, the lane layout and the entry stay keyed. */
import {expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync, readFileSync, realpathSync, statSync} from "node:fs";
import {dirname, join, normalize, resolve} from "node:path";
import {buildSourceClosureIdentityV1, type BuildSourceClosureV1} from "../../../🟦️.ts";
import {buildSchedulingDomainOfV1, buildSchedulingKeyedPathsV1, buildSchedulingPortsV1, buildSchedulingPrefixesV1, buildSchedulingV1, splitBuildSourceClosureV1} from "../../🟦️.ts";
import declared from "../../🔣️.json";

const root = (() => { let at = import.meta.dir; while (!existsSync(join(at, "nx.json"))) at = dirname(at); return at; })();
const lane = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/", ID = "unit-cache-lanes";
const domain = () => buildSchedulingV1().domains.find((row) => row.id === ID)!;
const sha = (text: string) => createHash("sha256").update(text).digest("hex");
const file = (path: string, text: string) => ({path, sha256: sha(text), base64: Buffer.from(text).toString("base64"), imports: {}});
const oracle = join(import.meta.dir, "../🐍️oracle/🐍️.py");

test("the lane domain names exactly the pool and the retention, with real directories and real port files", () => {
  expect(domain().prefixes).toEqual([lane + "🏊️pool/", lane + "📤️retention/"]);
  for (const prefix of domain().prefixes) expect(statSync(join(root, prefix)).isDirectory(), prefix).toBe(true);
  expect(domain().ports).toEqual([lane + "🏊️pool/🟦️.ts", lane + "📤️retention/🟦️.ts"]);
  for (const port of domain().ports) expect(statSync(join(root, port)).isFile() && buildSchedulingDomainOfV1(port), port).toBe(ID);
  expect(domain().contract.en.length).toBeLessThanOrEqual(400);
  expect(domain().contract.de.length).toBeLessThanOrEqual(400);
});

test("the lane prefixes overlap no other domain and no content island: the Python oracle and a plain pairwise reading agree", () => {
  const run = spawnSync("python3", ["-I", oracle, "islands"], {input: JSON.stringify(declared), encoding: "utf8"});
  expect(run.status, run.stderr).toBe(0);
  expect(JSON.parse(run.stdout)).toEqual([]);
  const others = buildSchedulingV1().domains.filter((row) => row.id !== ID).flatMap((row) => row.prefixes), islands = buildSchedulingKeyedPathsV1();
  for (const prefix of domain().prefixes) {
    for (const other of others) expect(prefix.startsWith(other) || other.startsWith(prefix), `${prefix} / ${other}`).toBe(false);
    for (const island of islands) expect(prefix.startsWith(island) || island.startsWith(prefix), `${prefix} / ${island}`).toBe(false);
  }
});

test("keyed code imports the lane scheduling modules only through their ports", () => {
  const prefixes = domain().prefixes, ports = buildSchedulingPortsV1(buildSchedulingV1(), [ID]);
  const found = spawnSync("git", ["grep", "-l", "-E", "🧵️lane/(🏊️pool|📤️retention)/", "--", "*.ts", "*.mjs", ":!.🧬semio"], {cwd: dirname(realpathSync(join(root, "package.json"))), encoding: "utf8"}).stdout.split("\n").filter(Boolean);
  const importers = found.filter((path) => !prefixes.some((prefix) => path.startsWith(prefix)) && !path.includes("/🧪️tests/") && existsSync(join(root, path)));
  expect(importers.length).toBeGreaterThan(0);
  for (const path of importers) {
    const text = readFileSync(join(root, path), "utf8");
    for (const match of text.matchAll(/(?:from\s*|import\s*\(\s*)["'](\.[^"']*)["']/gu)) {
      const target = normalize(join(dirname(path), match[1]!));
      if (prefixes.some((prefix) => target.startsWith(prefix))) expect(ports.has(target), `${path} imports ${target}`).toBe(true);
    }
  }
});

test("an edit of the pool or the retention moves no keyed identity, an edit of the lane layout, the unit key or the entry does", () => {
  const scheduling = [lane + "🏊️pool/🟦️.ts", lane + "📤️retention/🟦️.ts", lane + "🏊️pool/🧪️tests/🏊️pool-model/🟦️.ts"];
  const keyed = [lane + "🟦️.ts", lane.replace("🧵️lane/", "🔏️key/🟦️.ts"), lane.replace("🧵️lane/", "🟦️.ts")];
  const snapshot: BuildSourceClosureV1 = {version: 1, root: "/r", entry: "entry.ts", files: [file("entry.ts", "a"), ...[...scheduling, ...keyed].map((path) => file(path, "v1"))]};
  const identity = (edit?: string) => buildSourceClosureIdentityV1(splitBuildSourceClosureV1({...snapshot, files: snapshot.files.map((row) => row.path === edit ? file(row.path, "edited") : row)}, buildSchedulingV1().domains.map((row) => row.id)).keyed);
  const before = identity();
  for (const path of scheduling) { expect(buildSchedulingDomainOfV1(path), path).toBe(ID); expect(identity(path), path).toBe(before); }
  for (const path of keyed) { expect(statSync(join(root, path)).isFile(), path).toBe(true); expect(buildSchedulingDomainOfV1(path), path).toBeUndefined(); expect(identity(path), path).not.toBe(before); }
});

test("the production closure of a target that loads the lane modules names none of them, and the unfiltered walk proves it loads them", async () => {
  const library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/");
  const inference = await import(join(library, "🟨️.mjs")) as any;
  await inference.libraryBootstrap;
  const project = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust", target = JSON.parse(readFileSync(join(root, project, "📋️project.json"), "utf8")).targets["test-types-source"];
  const closure = (await inference.cacheInternals.targetScriptSources(target, root, inference.createScriptInputCache({workspaceRoot: root}), project)).closure as string[];
  expect(closure.length).toBeGreaterThan(100);
  for (const prefix of buildSchedulingPrefixesV1()) expect(closure.some((path) => path.replace("{workspaceRoot}/", "").startsWith(prefix)), prefix).toBe(false);
  const raw = (await inference.cacheInternals.relativeScriptInputs([resolve(root, project, "../../🔤️types/🧪️tests/🟦️.ts")], root, inference.createScriptInputCache({workspaceRoot: root}))) as string[];
  for (const prefix of domain().prefixes) expect(raw.some((path) => path.replace("{workspaceRoot}/", "").startsWith(prefix)), prefix).toBe(true);
}, 120000);
