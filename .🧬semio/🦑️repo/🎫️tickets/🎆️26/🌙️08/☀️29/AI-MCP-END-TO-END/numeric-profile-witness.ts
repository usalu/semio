import assert from "node:assert/strict";
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import workload from "./numeric-profile-witness.json";
import schema from "./numeric-profile-witness.schema.json";

assert(new Ajv({ strict: true }).compile(schema)(workload));
const output = join(import.meta.dir, "🗑️generated", "numeric-profile");
mkdirSync(output, { recursive: true });
function invoke(command: string[], capture = false) {
  const child = Bun.spawnSync(command, { stdout: capture ? "pipe" : "inherit", stderr: "inherit" });
  assert.equal(child.exitCode, 0, command.join(" "));
  return child.stdout?.toString() ?? "";
}
const tools = JSON.parse(invoke(["repo", "tools", "list", "--json"], true));
const rustup = tools.find((entry: { id: string }) => entry.id === "rustup")?.command;
const channel = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
assert(rustup && channel);
const rustc = [rustup, "run", channel, "rustc", "--edition=2021", "-Cdebuginfo=0", "-Cdebug-assertions=on", "-Coverflow-checks=on", "-Zthreads=1"];
const crate = "semio_plugin_interpreter_numeric";
const source = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🔢️numeric-semantics/🦀️.rs";
const executables = new Map<number, string>();
for (const level of [0, 3]) {
  const library = join(output, `lib${crate}_o${level}.rlib`);
  const exe = join(output, `witness-o${level}${process.platform === "win32" ? ".exe" : ""}`);
  invoke([...rustc, `-Copt-level=${level}`, "--crate-type=rlib", "--crate-name", crate, source, "-o", library]);
  invoke([...rustc, "-Copt-level=3", "--extern", `${crate}=${library}`, join(import.meta.dir, "numeric-profile-witness.rs"), "-o", exe]);
  executables.set(level, exe);
}

function wasmOperation<T extends number | bigint>(type: number, opcode: number): (a: T, b: T) => T {
  const module = new WebAssembly.Module(Uint8Array.of(
    0, 97, 115, 109, 1, 0, 0, 0,
    1, 7, 1, 0x60, 2, type, type, 1, type,
    3, 2, 1, 0,
    7, 5, 1, 1, 102, 0, 0,
    10, 9, 1, 7, 0, 0x20, 0, 0x20, 1, opcode, 0x0b,
  ));
  return new WebAssembly.Instance(module).exports.f as (a: T, b: T) => T;
}
const narrowOps = workload.i32.map(opcode => wasmOperation<number>(0x7f, opcode));
const wideOps = workload.i64.map(opcode => wasmOperation<bigint>(0x7e, opcode));
let narrow = workload.seed, wide = BigInt(workload.seed);
for (let index = 0; index < workload.iterations; index++) {
  const parameter = Math.imul(workload.seed + index, 3);
  for (const op of narrowOps) narrow = op(narrow, parameter);
  const wideParameter = BigInt(parameter) * 3_754_819n;
  for (const op of wideOps) wide = op(wide, wideParameter);
}
const expected = { i32: narrow, i64: wide.toString() };
const args = [String(workload.iterations), String(workload.seed), workload.i32.join(","), workload.i64.join(",")];
const timings: Record<number, number[]> = { 0: [], 3: [] };
for (let repeat = 0; repeat < workload.repeats; repeat++) {
  for (const level of repeat % 2 === 0 ? [0, 3] : [3, 0]) {
    const { nanoseconds, ...result } = JSON.parse(invoke([executables.get(level)!, ...args], true));
    assert.deepEqual(result, expected);
    timings[level]!.push(nanoseconds / 1e6);
  }
}
const medians = Object.fromEntries(Object.entries(timings).map(([level, values]) => [level, values.toSorted((a, b) => a - b)[values.length >> 1]]));
console.log("[DEBUG]", JSON.stringify({ operations: workload.iterations * (workload.i32.length + workload.i64.length), expected, timings, medians }));
