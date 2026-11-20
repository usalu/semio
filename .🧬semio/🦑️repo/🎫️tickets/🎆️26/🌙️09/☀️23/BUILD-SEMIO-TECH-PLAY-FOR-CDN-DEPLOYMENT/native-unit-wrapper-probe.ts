import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ensureUnitCacheShimV1, UNIT_CACHE_SCRIPT_V1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import { UNIT_CACHE_POLICY_V1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔏️key/🟦️.ts";
import { compilerWrapperEnvironmentV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";

const [rustc] = process.argv.slice(2);
assert(rustc);
assert.notEqual(process.platform, "win32", "The recording scheduler fixture uses POSIX sh");
const root = mkdtempSync(join(import.meta.dir, "🗑️generated", "uc-"));
const cache = join(root, "u"), workspace = join(root, "w"), temporary = join(root, "tmp");
for (const path of [cache, workspace, temporary]) mkdirSync(path);
const scheduler = join(root, "scheduler"), calls = join(root, "calls");
writeFileSync(calls, "");
writeFileSync(scheduler, `#!/bin/sh
for arg do
  case "$arg" in
    --emit=*) printf 'compile\n' >> "$SEMIO_PROBE_CALLS"; break ;;
  esac
done
exec "$@"
`, { mode: 0o755 });

const wrapper = ensureUnitCacheShimV1(cache, process.execPath, UNIT_CACHE_SCRIPT_V1);
const environment = {
  PATH: process.env.PATH,
  TMPDIR: temporary, TEMP: temporary, TMP: temporary,
  BUN_RUNTIME_TRANSPILER_CACHE_PATH: "0",
  SEMIO_UNIT_CACHE_ROOT: cache,
  SEMIO_UNIT_CACHE_WRAPPER: wrapper,
  SEMIO_UNIT_CACHE_REMOTE: "off",
  SEMIO_OFFLOAD: "never",
  SEMIO_UNIT_CACHE_SLOTS: "1",
  SEMIO_PROBE_CALLS: calls,
  RUSTC: rustc,
};
assert.deepEqual(compilerWrapperEnvironmentV1(scheduler, root, { ...environment, SEMIO_UNIT_CACHE: "off" }), {});

function invoke(lane: string, args: readonly string[]) {
  const chain = compilerWrapperEnvironmentV1(scheduler, lane, environment);
  assert.equal(chain.RUSTC_WRAPPER, wrapper);
  assert.equal(chain.RUSTC, undefined);
  assert.deepEqual(JSON.parse(chain.SEMIO_UNIT_CACHE_INNER!), [scheduler, rustc]);
  const result = spawnSync(wrapper, [rustc, ...args], {
    cwd: workspace, env: { ...environment, ...chain }, encoding: "utf8", timeout: 15_000,
  });
  assert.equal(result.status, 0, String(result.error ?? result.stderr));
  return result;
}

const version = invoke(root, ["-vV"]).stdout.trim();
assert.match(version, /^rustc .*nightly/mu);
const events = join(cache, UNIT_CACHE_POLICY_V1.layout.events, "g.jsonl");
const source = join(workspace, "lib.rs");
const original = "pub fn value() -> u32 { 41 }\n";
const edited = "pub fn value() -> u32 { 137 }\n";

const compileCount = () => readFileSync(calls, "utf8").split("\n").filter(Boolean).length;
const record = () => readFileSync(events, "utf8").trim().split("\n").map(line => JSON.parse(line) as { kind: string; key: string; crate: string });
const digest = (file: string) => createHash("sha256").update(readFileSync(file)).digest("hex");

const results = [];
for (const [targetIndex, target] of ["host", "wasm32-wasip2"].entries()) {
  const crate = "probe_" + targetIndex;
  const steps = [];
  for (const [index, kind] of ["miss", "hit", "miss", "hit"].entries()) {
    if (index === 0 || index === 2) writeFileSync(source, index === 0 ? original : edited);
    const lane = join(root, `l${targetIndex}${index}`), out = join(lane, "o");
    mkdirSync(out, { recursive: true });
    const ms = performance.now();
    const output = invoke(lane, [
      "--crate-name", crate, "lib.rs", "--edition=2024", "--crate-type", "rlib",
      "--emit=dep-info,metadata,link", "--error-format=json", "--json=artifacts", "--out-dir", out,
      "-C", "opt-level=1", "-Z", "threads=1", ...(target === "host" ? [] : ["--target", target]),
    ]);
    const eventsForUnit = record().filter(row => row.crate === crate);
    const event = eventsForUnit.filter(row => row.kind === "miss" || row.kind === "hit").at(-1);
    assert.equal(event?.kind, kind, JSON.stringify(eventsForUnit));
    assert.equal(eventsForUnit.filter(row => row.kind === "miss" || row.kind === "hit").length, index + 1);
    const outputs = [`lib${crate}.rlib`, `lib${crate}.rmeta`].map(name => digest(join(out, name)));
    assert(output.stderr.includes(out), "Compiler announcements name the current output lane");
    assert(readFileSync(join(out, `${crate}.d`), "utf8").includes(out));
    assert.equal(compileCount(), targetIndex * 2 + (index < 2 ? 1 : 2));
    steps.push({ kind, key: event!.key, outputs, milliseconds: Math.round(performance.now() - ms) });
  }
  assert.equal(steps[0]!.key, steps[1]!.key);
  assert.deepEqual(steps[0]!.outputs, steps[1]!.outputs);
  assert.notEqual(steps[0]!.key, steps[2]!.key);
  assert.notEqual(steps[0]!.outputs[0], steps[2]!.outputs[0]);
  assert.equal(steps[2]!.key, steps[3]!.key);
  assert.deepEqual(steps[2]!.outputs, steps[3]!.outputs);
  assert.equal(record().filter(row => row.crate === crate && row.kind === "publish").length, 2);
  results.push({ target, steps });
}
console.log(JSON.stringify({ version, compiles: compileCount(), results }));
