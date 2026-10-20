import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { readRustSourceClosureV1 } from "@semio-tech/process/artifacts/native-build/source";
const root = process.argv[2]!, entry = resolve(root, process.argv[3]!), manifestDirectory = resolve(entry, process.argv[4] ?? "..");
const workspace = resolve(root, "🧰️framework/🔨️modules/📡️replication/../../..");
const hostCfg = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions"];
for (const tests of [false, true]) {
  const rows = await readRustSourceClosureV1([entry], workspace, tests ? [...hostCfg, "test"] : hostCfg, async (path: string) => await readFile(path).catch((error: any) => error.code === "ENOENT" ? null : Promise.reject(error)), { manifestDirectory, generated: () => undefined });
  console.log(tests ? "TESTS" : "NO-TESTS", rows.length, rows.reduce((s: number, r: any) => s + r.bytes, 0), rows.reduce((s: number, r: any) => s + r.activeLines, 0));
  if (process.argv[5] === "list") for (const r of rows.sort((a: any, b: any) => b.bytes - a.bytes)) console.log(r.bytes, r.activeLines, String(r.path ?? r.file ?? Object.values(r)[0]).replace(root, ""));
}
