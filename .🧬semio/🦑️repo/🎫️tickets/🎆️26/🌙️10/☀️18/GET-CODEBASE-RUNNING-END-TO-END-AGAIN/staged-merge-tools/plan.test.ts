import {expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {cpSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import {createHash} from "node:crypto";
import {buildPlan, drive, findRoot, loadRegistry, orderSets, validate, type Context} from "./plan.ts";

const corpus = JSON.parse(readFileSync(join(import.meta.dir, "fixtures", "merge.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(import.meta.dir, "schema.json"), "utf8"));
const sha = (text: string): string => createHash("sha256").update(text).digest("hex");

const TOY_APPLY = `
import hashlib, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")
ROOT = find_root()
ops = json.load(open(os.path.join(HERE, "ops.json"), encoding="utf-8"))
dry = "--dry-run" in sys.argv
rebase = "--rebase" in sys.argv
if rebase and not any(op.get("allowRebase") for op in ops):
    raise SystemExit("refusing unknown flag --rebase")
results = {}
for op in ops:
    path = os.path.join(ROOT, *op["path"].split("/"))
    text = open(path, encoding="utf-8").read() if os.path.exists(path) else None
    if op["mode"] == "replace":
        if text is None or (op["from"] not in text and op["to"] not in text) or text.count(op["from"]) > 1:
            raise SystemExit("refusing: anchor not found once in " + op["path"])
        results[path] = text.replace(op["from"], op["to"]) if op["to"] not in text else text
    else:
        if not rebase and text != op["text"] and hashlib.sha256((text or "").encode()).hexdigest() != op["base"]:
            raise SystemExit("refusing: " + op["path"] + " changed since staging")
        results[path] = op["text"]
for path, text in results.items():
    print(("would write " if dry else "wrote ") + path)
    if not dry:
        open(path, "w", encoding="utf-8").write(text)
`;

type Toy = {ctx: Context; base: string; sets: {id: string}[]};

function toy(): Toy {
  const base = mkdtempSync(join(tmpdir(), "sm-test-"));
  const root = join(base, "root"), ticket = join(root, "ticket"), here = join(ticket, "merge");
  mkdirSync(here, {recursive: true});
  for (const name of ["bunfig.toml", "bun.lock"]) writeFileSync(join(root, name), "");
  for (const [path, text] of Object.entries<string>(corpus.files)) {
    mkdirSync(dirname(join(root, path)), {recursive: true});
    writeFileSync(join(root, path), text);
  }
  cpSync(join(import.meta.dir, "schema.json"), join(here, "schema.json"));
  const registry: any[] = [];
  for (const set of corpus.sets) {
    registry.push({id: set.id, dir: `${set.id}-tools`, result: `result-${set.id}.md`, after: corpus.after[set.id] ?? [], verify: set.verify ? [{name: "marker", argv: set.verify}] : []});
    if (set.absent) continue;
    const dir = join(ticket, `${set.id}-tools`);
    mkdirSync(join(dir, "staged"), {recursive: true});
    writeFileSync(join(dir, "apply.py"), TOY_APPLY);
    writeFileSync(join(ticket, `result-${set.id}.md`), "done\n");
    const ops = set.ops.map((op: any) => ({...op, allowRebase: set.allowRebase, base: op.mode === "write" ? sha(op.staleBase ? "not the live text\n" : corpus.files[op.path]) : undefined}));
    writeFileSync(join(dir, "ops.json"), JSON.stringify(ops));
    if (set.leaky) writeFileSync(join(dir, "apply.py"), TOY_APPLY.replace("    if not dry:\n", "    if True:\n"));
    const targets = set.ops.map((op: any, index: number) => {
      if (op.mode === "replace") return {path: op.path, kind: "patch", ops: [{anchor: op.from, replacement: op.to}]};
      writeFileSync(join(dir, "staged", `file${index}`), op.text);
      return {path: op.path, kind: "write", staged: `staged/file${index}`, baseSha: sha(op.staleBase ? "not the live text\n" : corpus.files[op.path])};
    });
    writeFileSync(join(dir, "inventory.json"), JSON.stringify({version: 1, targets}));
  }
  writeFileSync(join(here, "sets.json"), JSON.stringify({version: 1, sets: registry}));
  return {ctx: {root, ticket, here, registry: join(here, "sets.json"), planFile: join(here, "plan.json"), scratch: join(base, "scratch"), keep: false}, base, sets: corpus.sets};
}

test("the planner derives the corpus verdicts, exclusions and order", () => {
  const {ctx, base} = toy();
  try {
    const plan = buildPlan(ctx);
    expect(validate(plan, schema.$defs.Plan, schema)).toEqual([]);
    for (const [key, verdict] of Object.entries<string>(corpus.expect.pairs)) {
      const [a, b] = key.split("|");
      expect(plan.pairs.find((row: any) => row.a === a && row.b === b)?.verdict).toBe(verdict);
    }
    for (const [key, first] of Object.entries<string>(corpus.expect.first)) {
      const [a, b] = key.split("|");
      expect(plan.pairs.find((row: any) => row.a === a && row.b === b)?.first).toBe(first);
    }
    for (const [id, reason] of Object.entries<string>(corpus.expect.excluded)) expect(plan.excluded.find((row: any) => row.id === id)?.reason).toBe(reason);
    expect(plan.order.map((step: any) => step.id)).toEqual(corpus.expect.order);
    for (const [id, needs] of Object.entries<string[]>(corpus.expect.needs)) expect(plan.order.find((step: any) => step.id === id).needs).toEqual(needs);
    expect(plan.replay.status).toBe("ok");
    expect(plan.replay.undeclared).toEqual([]);
    expect(plan.rebase).toEqual([]);
    expect(JSON.stringify(plan.pairs.flatMap((row: any) => [...row.ab, ...row.ba]).map((run: any) => run.args).flat())).toBe("[]");
    expect(plan.excluded.find((row: any) => row.id === "kappa").detail).toContain("--dry-run wrote files");
    expect(plan.excluded.find((row: any) => row.id === "lambda").detail).toContain("changed since staging");
    expect(plan.excluded.find((row: any) => row.id === "mu").detail).toContain("zeta (conflict)");
  } finally {
    rmSync(base, {recursive: true, force: true});
  }
}, 180_000);

test("the driver reproduces the planned final tree, matches the system three-way merge, refuses without --yes and when stale", () => {
  const {ctx, base} = toy();
  try {
    const plan = buildPlan(ctx);
    writeFileSync(ctx.planFile, JSON.stringify(plan));
    const lines: string[] = [];
    const options = {dryRun: false, yes: false, ignoreSync: true, includeProvisional: false, only: [], from: null, skipVerify: false, heavy: false, forceStale: false};
    expect(drive(ctx, options, (line) => lines.push(line))).toBe(6);
    expect(drive(ctx, {...options, dryRun: true}, (line) => lines.push(line))).toBe(0);
    expect(readFileSync(join(ctx.root, "lib/b.txt"), "utf8")).toBe(corpus.files["lib/b.txt"]);
    expect(lines.join("\n")).toContain("defer delta");
    expect(drive(ctx, {...options, yes: true}, (line) => lines.push(line))).toBe(0);
    for (const [path, text] of Object.entries<string>(corpus.expect.final)) expect(readFileSync(join(ctx.root, path), "utf8")).toBe(text);
    expect(lines.join("\n")).toContain("verify eta/marker: pass");
    const dir = join(base, "oracle");
    mkdirSync(dir);
    const apply = (name: string, path: string, from: string, to: string): string => {
      writeFileSync(join(dir, name), corpus.files[path].replace(from, to));
      return join(dir, name);
    };
    writeFileSync(join(dir, "base"), corpus.files["lib/a.txt"]);
    const merged = spawnSync("diff3", ["-m", apply("alpha", "lib/a.txt", "one", "ONE"), join(dir, "base"), apply("beta", "lib/a.txt", "six", "SIX")], {encoding: "utf8"});
    if (!merged.error) expect(merged.stdout).toBe(corpus.expect.final["lib/a.txt"]);
    writeFileSync(join(ctx.root, "lib/d.txt"), "drifted\n");
    expect(drive(ctx, {...options, yes: true}, (line) => lines.push(line))).toBe(4);
  } finally {
    rmSync(base, {recursive: true, force: true});
  }
}, 180_000);

test("an explicit rebase of one named set is the only way a drifted base is applied, and it is recorded in the plan", () => {
  const {ctx, base} = toy();
  try {
    const plan = buildPlan(ctx, {rebase: ["lambda"]});
    expect(plan.rebase).toEqual(["lambda"]);
    expect(plan.order.find((step: any) => step.id === "lambda").applyArgs).toEqual(corpus.expect.rebased.lambda);
    expect(plan.order.filter((step: any) => step.id !== "lambda").every((step: any) => step.applyArgs.length === 0)).toBe(true);
    writeFileSync(ctx.planFile, JSON.stringify(plan));
    const lines: string[] = [];
    const options = {dryRun: false, yes: true, ignoreSync: true, includeProvisional: false, only: [], from: null, skipVerify: true, heavy: false, forceStale: false};
    expect(drive(ctx, {...options, from: "no-such-step"}, (line) => lines.push(line))).toBe(4);
    expect(drive(ctx, {...options, only: ["delta"]}, (line) => lines.push(line))).toBe(4);
    expect(lines.join("\n")).toContain("declares after gamma");
    expect(drive(ctx, options, (line) => lines.push(line))).toBe(0);
    for (const [path, text] of Object.entries<string>(corpus.expect.finalRebased)) expect(readFileSync(join(ctx.root, path), "utf8")).toBe(text);
  } finally {
    rmSync(base, {recursive: true, force: true});
  }
}, 180_000);

test("hard edges win over hints, hints never close a cycle, a cycle is broken deterministically", () => {
  expect(orderSets(["a", "b", "c"], [["c", "a"]], [["a", "c"]]).order).toEqual(["b", "c", "a"]);
  expect(orderSets(["a", "b"], [], [["b", "a"]]).order).toEqual(["b", "a"]);
  const cyclic = orderSets(["a", "b"], [["a", "b"], ["b", "a"]], []);
  expect(cyclic.order).toEqual(["a", "b"]);
  expect(cyclic.cycle.length).toBeGreaterThan(0);
});

test("the schema rejects malformed registries and plans; bun test commands need explicit paths", () => {
  expect(validate({version: 1, sets: [{id: "Bad Id", dir: "x"}]}, schema.$defs.Registry, schema).length).toBeGreaterThan(0);
  expect(validate({version: 2, sets: []}, schema.$defs.Registry, schema).length).toBeGreaterThan(0);
  const {ctx, base} = toy();
  try {
    expect(loadRegistry(ctx).length).toBe(corpus.sets.length);
    writeFileSync(ctx.registry, JSON.stringify({version: 1, sets: [{id: "x", dir: "x", verify: [{name: "t", argv: ["bun", "test", "some/dir"]}]}]}));
    expect(() => loadRegistry(ctx)).toThrow("explicit");
  } finally {
    rmSync(base, {recursive: true, force: true});
  }
  expect(findRoot(import.meta.dir)).toContain("semio");
});
