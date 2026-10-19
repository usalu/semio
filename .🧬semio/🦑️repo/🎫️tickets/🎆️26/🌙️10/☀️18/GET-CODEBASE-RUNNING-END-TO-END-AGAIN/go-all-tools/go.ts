import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { canonicalGoPlan } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🐹️go-plan/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../..");
const ticket = resolve(import.meta.dir, "..");
const logs = join(ticket, "🗑️generated/go-all-logs");
mkdirSync(logs, { recursive: true });
const args = process.argv.slice(2);
const flag = (name: string, fallback = ""): string => { const i = args.indexOf(name); return i < 0 ? fallback : args[i + 1] ?? fallback; };
const label = flag("--label", "run");
const only = flag("--only").split(",").filter(Boolean);
const jobs = Number(flag("--jobs", "2"));
const steps = flag("--steps", "build,vet,short,full").split(",");
const phase = flag("--phase", "after");
const tags = [...(flag("--tags") ? [`-tags=${flag("--tags")}`] : []), ...(flag("--run") ? ["-run", flag("--run")] : [])];

const goWork = readFileSync(join(root, "go.work"), "utf8");
const inWork = [...goWork.matchAll(/^\t\.\/(.+)$/gm)].map((m) => m[1]!).filter((p) => existsSync(join(root, p, "go.mod")));
const standalone = spawnSync("git", ["ls-files", "--cached", "--others", "--exclude-standard", "--", "*go.mod"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\n").filter((p) => p.endsWith("/go.mod") && existsSync(join(root, p)) && !p.startsWith(".🧬semio/")).map((p) => p.slice(0, -"/go.mod".length)).filter((p) => !inWork.includes(p));
const modules = [...inWork, ...standalone];
const selected = modules.filter((m) => only.length === 0 || only.some((o) => m.includes(o)));
const resultFile = (m: string): string => join(logs, `res-${label}-${createHash("sha1").update(m).digest("hex").slice(0, 10)}.json`);
const todo = flag("--skip-done") ? selected.filter((m) => !existsSync(resultFile(m))) : selected;

const git = (...a: string[]): string => spawnSync("git", a, { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).stdout;
const marker = join(logs, "marker");
if (!existsSync(marker)) writeFileSync(marker, "");
const watched = (): string => {
  const real = (l: string): boolean => /(🎫️ticket\.json|🎯️goals\/)/u.test(l) && !/\/(🗑️temp|🗑️generated)\//u.test(l) && !/\/\.🧬semio\//u.test(l.replace(/^.{3}\.🧬semio\//u, ""));
  const status = git("status", "--short", "-uno", ".🧬semio").split("\n").filter((l) => l && real(l)).sort().join("\n");
  const found = spawnSync("find", [".🧬semio/🦑️repo/🎯️goals", ".🧬semio/🦑️repo/🎫️tickets", "-type", "f", "(", "-name", "🎫️ticket.json", "-o", "-path", "*/🎯️goals/*", ")", "-newer", marker, "-not", "-path", `*/${relative(root, ticket).split("/").pop()}/*`], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\n").filter((l) => l && real(l)).sort().join("\n");
  return `${status}\n--newer--\n${found}`;
};
const baseline = watched();
let tripped = "";
const guard = (where: string): void => {
  if (tripped) return;
  const now = watched();
  if (now !== baseline) {
    tripped = where;
    const a = new Set(baseline.split("\n")), b = new Set(now.split("\n"));
    writeFileSync(join(logs, `GUARD-${label}.txt`), `tripped ${where}\nadded:\n${[...b].filter((l) => !a.has(l)).join("\n")}\nremoved:\n${[...a].filter((l) => !b.has(l)).join("\n")}\n`);
    console.error(`[GUARD] tracked .🧬semio/goal/ticket state changed ${where}; see GUARD-${label}.txt`);
  }
};

type Counts = { pass: number; fail: number; skip: number; topPass: number; topFail: number; topSkip: number; pkgOk: number; pkgFail: number; pkgNoTests: number };
type Step = { step: string; exit: number; seconds: number; counts?: Counts; log: string };

const run = (cmd: string, a: string[], cwd: string, env: NodeJS.ProcessEnv, file: string): Promise<number> =>
  new Promise((res) => {
    const out = join(logs, file);
    const child = spawn(cmd, a, { cwd, env, stdio: ["ignore", "pipe", "pipe"] });
    const chunks: Buffer[] = [];
    child.stdout.on("data", (d) => chunks.push(d));
    child.stderr.on("data", (d) => chunks.push(d));
    child.on("close", (code) => { writeFileSync(out, Buffer.concat(chunks)); res(code ?? 1); });
  });

const count = (file: string): Counts => {
  const c: Counts = { pass: 0, fail: 0, skip: 0, topPass: 0, topFail: 0, topSkip: 0, pkgOk: 0, pkgFail: 0, pkgNoTests: 0 };
  for (const line of readFileSync(join(logs, file), "utf8").split("\n")) {
    if (!line.startsWith("{")) continue;
    let e: { Action?: string; Test?: string; Package?: string; Output?: string };
    try { e = JSON.parse(line); } catch { continue; }
    if (e.Test) {
      const top = !e.Test.includes("/");
      if (e.Action === "pass") { c.pass++; if (top) c.topPass++; }
      else if (e.Action === "fail") { c.fail++; if (top) c.topFail++; }
      else if (e.Action === "skip") { c.skip++; if (top) c.topSkip++; }
    } else if (e.Package) {
      if (e.Action === "pass") c.pkgOk++;
      else if (e.Action === "fail") c.pkgFail++;
      else if (e.Action === "skip") c.pkgNoTests++;
    }
  }
  return c;
};

const slug = (m: string): string => m.split("/").filter((s) => !/^(🧰|🛍|🦑|🔨|📦)/u.test(s)).join("_").replace(/[^\w\u{1F000}-\u{1FFFF}.-]/gu, "") || "root";

const doModule = async (m: string): Promise<{ module: string; packages: number; steps: Step[]; error?: string }> => {
  const moduleRoot = join(root, m);
  const id = `${label}-${selected.indexOf(m)}`;
  const result = { module: m, packages: 0, steps: [] as Step[] } as { module: string; packages: number; steps: Step[]; error?: string };
  let plan;
  try { plan = canonicalGoPlan(moduleRoot); } catch (e) { result.error = `plan failed: ${(e as Error).message}`; return result; }
  result.packages = plan.packages.length;
  if (plan.packages.length === 0) { result.error = "no canonical packages"; return result; }
  const overlay = join(logs, `${id}-overlay.json`);
  writeFileSync(overlay, JSON.stringify({ Replace: plan.replacements }));
  const artifacts = join(tmpdir(), `ga-${createHash("sha1").update(m).digest("hex").slice(0, 6)}`);
  mkdirSync(artifacts, { recursive: true });
  const env = { ...process.env, GOWORK: inWork.includes(m) ? join(root, "go.work") : "off", SEMIO_TEST_ARTIFACT_DIR: artifacts };
  const ov = `-overlay=${overlay}`;
  const defs: Record<string, [string[], boolean]> = {
    build: [["build", ov, ...tags, "-o", "/dev/null", ...plan.packages], false],
    vet: [["vet", ov, ...tags, ...plan.packages], false],
    short: [["test", ov, ...tags, "-short", "-count=1", "-json", ...plan.packages], true],
    full: [["test", ov, ...tags, "-count=1", "-json", ...plan.packages], true],
  };
  for (const s of steps) {
    guard(`before ${m} ${s}`);
    if (tripped) { result.error = `halted by guard (${tripped})`; return result; }
    const [a, json] = defs[s]!;
    const file = `${id}-${s}.log`;
    const t0 = Date.now();
    const exit = await run("go", a, moduleRoot, env, file);
    const step: Step = { step: s, exit, seconds: Math.round((Date.now() - t0) / 100) / 10, log: file };
    if (json) step.counts = count(file);
    result.steps.push(step);
    guard(`after ${m} ${s}`);
  }
  return result;
};

const results: Awaited<ReturnType<typeof doModule>>[] = [];
const queue = [...todo];
await Promise.all(Array.from({ length: jobs }, async () => {
  for (let m = queue.shift(); m && !tripped; m = queue.shift()) {
    const r = await doModule(m);
    results.push(r);
    writeFileSync(resultFile(r.module), JSON.stringify(r));
    console.log(`[go-all] ${relative(root, join(root, m)).split("/").slice(-4).join("/")} ${r.error ?? ""} ${r.steps.map((s) => `${s.step}=${s.exit}${s.counts ? `(${s.counts.topPass}/${s.counts.topFail}/${s.counts.topSkip} ${s.counts.pkgOk}/${s.counts.pkgFail})` : ""}`).join(" ")}`);
  }
}));
const all = selected.filter((m) => existsSync(resultFile(m))).map((m) => JSON.parse(readFileSync(resultFile(m), "utf8")));
writeFileSync(join(logs, `summary-${label}-${phase}.json`), JSON.stringify({ label, phase, tripped, results: all }, null, 1));
console.log(`[go-all] done this-call=${results.length} total=${all.length}/${selected.length} tripped=${tripped || "no"}`);
process.exit(tripped ? 3 : 0);
