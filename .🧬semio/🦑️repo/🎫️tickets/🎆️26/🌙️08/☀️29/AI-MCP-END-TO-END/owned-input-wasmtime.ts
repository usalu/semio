import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createReadStream, existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const ticket = import.meta.dir;
const scratch = join(ticket, "🗑️generated");
const build = join(scratch, "native-host/debug/build");
const component = join(scratch, "m-canon/browser/prototype/components/6d8a4f7ae2584b94ed7f1643/📦️component.wasm");
const transcript = join(scratch, "owned-input-wasmtime.trace");
const executable = join(scratch, process.platform === "win32" ? "owned-input-wasmtime.exe" : "owned-input-wasmtime");
const replayLimitMs = 8 * 60_000;
const [mode, fuel] = process.argv.slice(2);
assert(["--record", "--check", "--link", "--replay"].includes(mode), "Use --record, --check, --link or --replay [fuel]");

async function run(args: string[], options: { cwd?: string; env?: NodeJS.ProcessEnv } = {}, timeout = 120_000) {
  const child = Bun.spawn(args, { ...options, stdout: "inherit", stderr: "inherit" });
  const deadline = setTimeout(() => {
    console.error("[DEBUG] Wasmtime probe exceeded its wall-clock bound");
    child.kill();
  }, timeout);
  try {
    assert.equal(await child.exited, 0);
  } finally {
    clearTimeout(deadline);
  }
}

if (mode === "--record") {
  await run([process.execPath, join(ticket, "owned-input-jit.ts")], {
    env: { ...process.env, SEMIO_OWNED_INPUT_TRANSCRIPT: transcript },
  }, 70_000);
} else if (mode === "--replay") {
  assert(statSync(transcript).size <= 8 * 1024 * 1024);
  const header = readFileSync(transcript, "utf8").split("\n", 2)[1];
  const expected = header.split(" ")[3];
  const hash = createHash("sha256");
  for await (const bytes of createReadStream(component)) hash.update(bytes);
  assert.equal(hash.digest("hex"), expected, "Replay component differs from captured component");
  await run([executable, component, transcript, ...(fuel === undefined ? [] : [fuel])], {}, replayLimitMs);
} else {
  const tools = Bun.spawnSync(["repo", "tools", "list", "--json"], { stderr: "inherit" });
  assert.equal(tools.exitCode, 0);
  const rustup = JSON.parse(tools.stdout.toString()).find((entry: { id: string }) => entry.id === "rustup")?.command;
  const toolchain = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
  assert(rustup && toolchain, "Pinned Rust unavailable");
  const variants = (pkg: string) => readdirSync(join(build, pkg), { withFileTypes: true })
    .filter(entry => entry.isDirectory() && existsSync(join(build, pkg, entry.name, "out")))
    .map(entry => entry.name);
  const wasmtimePath = (variant: string) => join("wasmtime", variant, "out", `libwasmtime-${variant}`);
  const wasmtime = variants("wasmtime").filter(variant => {
    if (!existsSync(join(build, wasmtimePath(variant) + ".rlib"))) return false;
    const fingerprint = JSON.parse(readFileSync(join(build, "wasmtime", variant, "fingerprint/lib-wasmtime.json"), "utf8"));
    const features: string[] = JSON.parse(fingerprint.features);
    return features.includes("cranelift") && features.includes("parallel-compilation");
  }).sort((a, b) => statSync(join(build, wasmtimePath(b) + ".rlib")).mtimeMs - statSync(join(build, wasmtimePath(a) + ".rlib")).mtimeMs)[0];
  assert(wasmtime, "No cached Wasmtime with Cranelift and parallel-compilation controls");
  const search = readdirSync(build, { withFileTypes: true }).filter(entry => entry.isDirectory()).flatMap(({ name }) =>
    variants(name).flatMap(variant => {
      const path = join(name, variant, "out");
      return ["-L", `dependency=${path}`, ...(readdirSync(join(build, path)).some(file => /\.(a|lib)$/u.test(file)) ? ["-L", `native=${path}`] : [])];
    }));
  console.log(`[DEBUG] Wasmtime cached variant ${wasmtime}`);
  const path = wasmtimePath(wasmtime);
  const check = mode === "--check";
  await run([
    rustup, "run", toolchain, "rustc", "--edition=2024", join(ticket, "owned-input-wasmtime.rs"),
    ...(check ? ["--emit=metadata"] : ["-Cstrip=debuginfo"]),
    "-o", check ? executable + ".rmeta" : executable,
    "--extern", `wasmtime=${path}.rmeta`, "--extern", `wasmtime=${path}.rlib`, ...search,
  ], { cwd: build });
}
