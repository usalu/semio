import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ensureUnitCacheShimV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import { compilerWrapperEnvironmentV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts";

/** [DEBUG] N1 live probe: one tiny crate through the driver's lane-g compiler chain on the main checkout; prints the lookup rows. Usage: bun N1-live-probe.ts <scratch-dir> */
const scratch = process.argv[2]!, root = process.env.SEMIO_WORKSPACE_ROOT ?? "/Users/ueli/Documents/semio";
const base = mkdtempSync(join(scratch, "n1-")), cache = join(base, "u"), work = join(base, "w"), out = join(base, "o");
for (const path of [cache, work, out]) mkdirSync(path, { recursive: true });
const wrapper = ensureUnitCacheShimV1(cache);
const environment = { ...process.env, SEMIO_WORKSPACE_ROOT: root, SEMIO_UNIT_CACHE: "on", SEMIO_UNIT_CACHE_ROOT: cache, SEMIO_UNIT_CACHE_WRAPPER: wrapper, SEMIO_REMOTE_CACHE_UPLOAD: "false", SEMIO_OFFLOAD: "never", SEMIO_UNIT_CACHE_SLOTS: "1" };
const scheduler = join(base, "scheduler");
writeFileSync(scheduler, "#!/bin/sh\nexec \"$@\"\n", { mode: 0o755 });
const chain = compilerWrapperEnvironmentV1(scheduler, join(base, "l"), { ...environment, RUSTC: "rustc" });
console.log("chain keys", Object.keys(chain).sort().join(" "));
writeFileSync(join(work, "lib.rs"), "pub fn n1() -> u32 { " + (Date.now() % 100000) + " }\n");
const result = spawnSync(wrapper, ["rustc", "--crate-name", "n1probe", "lib.rs", "--edition=2024", "--crate-type", "rlib", "--emit=dep-info,metadata,link", "--error-format=json", "--json=artifacts", "--out-dir", out, "-C", "opt-level=1"], { cwd: work, env: { ...environment, ...chain }, encoding: "utf8", timeout: 120_000 });
console.log("status", result.status, result.stderr.slice(0, 300));
for (const directory of ["x", "y"]) for (const name of readdirSync(join(cache, directory)).filter(file => file.endsWith(".jsonl"))) for (const line of readFileSync(join(cache, directory, name), "utf8").split("\n").filter(Boolean)) console.log(directory + "/" + name, line.slice(0, 600));
