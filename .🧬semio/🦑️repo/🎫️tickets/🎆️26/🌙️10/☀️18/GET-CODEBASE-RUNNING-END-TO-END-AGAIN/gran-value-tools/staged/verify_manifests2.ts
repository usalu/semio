/** 🧾 Third-party (Bun TOML) differential of every changed Cargo.toml between two trees against the hygiene ledger and the facade-tail owner set.
 *  usage: bun verify_manifests2.ts <pre-root> <post-root> */
import { readFileSync, readdirSync, statSync, existsSync } from "node:fs";
import { join, relative } from "node:path";

const [pre, post] = process.argv.slice(2);
const ledger: { crate: string; dir: string; dependency: string; verdict: string }[] = JSON.parse(readFileSync(`${import.meta.dir}/../data/hygiene-edges.json`, "utf8"));
const byDir = new Map<string, Map<string, string>>();
for (const e of ledger) (byDir.get(e.dir) ?? byDir.set(e.dir, new Map()).get(e.dir)!).set(e.dependency, e.verdict);
const OWNERS = new Set(["semio-framework-value-ordered", "semio-framework-value-snapshot-projection", "semio-framework-value-paged-text", "semio-framework-value-retirement-queue", "semio-framework-value-original-aliases"]);
const CARVE = new Set(["semio-framework-value-intrinsic", "semio-framework-value-derive-resources", "semio-framework-hash", "semio-framework-native-build-compiler-resources", "semio-framework-value-clone-source", "semio-framework-value-native-decoding", "semio-framework-value-paged-list", "semio-framework-value-allocation-return", "semio-framework-value-edit", "semio-framework-value-schema", "semio-framework-value-registry", "semio-framework-schema-registry", "semio-framework-value-native-encoding", "semio-framework-value-original-aliases", "semio-framework-value-ordered"]);
function* walk(dir: string): Generator<string> {
  for (const n of readdirSync(dir)) {
    const p = join(dir, n);
    if (statSync(p).isDirectory()) yield* walk(p);
    else if (n === "Cargo.toml") yield p;
  }
}
const keys = (t: any, k: string) => Object.entries<any>(t[k] ?? {}).map(([a, v]) => (v && typeof v === "object" && v.package) || a);
let changed = 0, removedNormal = 0, demoted = 0, addedNormal = 0, bad = 0, parseFail = 0;
const fail = (m: string) => { bad++; console.log("FAIL", m); };
for (const p of walk(post)) {
  const rel = relative(post, p);
  const a = join(pre, rel);
  if (!existsSync(a)) continue;
  const ta = readFileSync(a, "utf8"), tb = readFileSync(p, "utf8");
  if (ta === tb) continue;
  changed++;
  let A: any, B: any;
  try { A = Bun.TOML.parse(ta); B = Bun.TOML.parse(tb); } catch { parseFail++; fail(`${rel} does not parse`); continue; }
  const dir = rel.replace(/\/Cargo\.toml$/, "");
  const led = byDir.get(dir) ?? new Map<string, string>();
  const an = new Set(keys(A, "dependencies")), bn = new Set(keys(B, "dependencies")), ad = new Set(keys(A, "dev-dependencies")), bd = new Set(keys(B, "dev-dependencies"));
  for (const k of an) if (!bn.has(k)) {
    removedNormal++;
    const v = led.get(k);
    if (v === "unused") { if (bd.has(k) && !ad.has(k)) fail(`${rel}: unused ${k} reappeared in dev`); }
    else if (v === "test-only") { if (bd.has(k)) demoted++; else fail(`${rel}: test-only ${k} vanished`); }
    else if (!CARVE.has(k) && !(OWNERS.has(k) && rel.includes("🌱️value/📦️packages/🦀️rust"))) fail(`${rel}: unexpected removal of ${k}`);
  }
  for (const k of bn) if (!an.has(k)) { addedNormal++; if (!OWNERS.has(k) && !CARVE.has(k) && !["syn", "quote", "proc-macro2"].includes(k)) fail(`${rel}: unexpected addition of ${k}`); }
  for (const k of bd) if (!ad.has(k) && !(an.has(k) && !bn.has(k)) && k !== "semio-framework-value-derive-resources") fail(`${rel}: dev addition ${k} is not a demotion`);
}
console.log(JSON.stringify({ changedManifests: changed, removedFromDependencies: removedNormal, demotedToDev: demoted, addedToDependencies: addedNormal, parseFailures: parseFail, failures: bad }));
process.exit(bad ? 1 : 0);
