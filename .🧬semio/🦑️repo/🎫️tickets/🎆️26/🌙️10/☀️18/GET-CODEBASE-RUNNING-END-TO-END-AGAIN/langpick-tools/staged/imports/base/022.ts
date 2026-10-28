// 📜️ Standalone entry (SEMIO_UNITS_TRACE=<file> appends the id of every executed unit, for tests that count executions) of `repo units` for the actions of a build tool: `bun 📜️script.ts exec --spec <file> --out <file> …`. The process has no surrounding repository invocation, so progress goes to stderr and a signal aborts the unit.
import {spawn} from "node:child_process";
import {appendFileSync, readFileSync} from "node:fs";
import {runUnitsCommandV1, unitsArgumentsV1, type UnitsCommandPortsV1} from "./🟦️.ts";

const controller = new AbortController();
for (const signal of ["SIGINT", "SIGTERM"] as const) process.on(signal, () => controller.abort());
const args = process.argv.slice(2);
const requested = unitsArgumentsV1(args.slice(args[0] === "units" ? 1 : 0)).language;
const language = await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🟦️.ts");
const ports: UnitsCommandPortsV1 = {
  write: line => console.log(line),
  progress: async stage => {console.error(`[units] ${stage}`);},
  check: () => {if (controller.signal.aborted) throw Error("cancelled");},
  signal: controller.signal,
  language: asked => asked ?? requested ?? language.resolveLanguageV1(["en", "de"], [], process.env).language as "en" | "de",
  run: (argv, options) => new Promise(accept => {
    const chunks: Buffer[] = [], child = spawn(argv[0]!, argv.slice(1), {cwd: options.cwd, env: {...process.env, ...options.env}, stdio: ["ignore", "pipe", "pipe"], windowsHide: true, signal: options.signal});
    child.stdout.on("data", (chunk: Buffer) => chunks.push(chunk));
    child.stderr.on("data", (chunk: Buffer) => chunks.push(chunk));
    child.on("error", error => accept({code: 1, output: String(error.message)}));
    child.on("close", code => accept({code: code ?? 1, output: Buffer.concat(chunks).toString("utf8")}));
  }),
  files: () => Promise.reject(Error("units exec lists no files")),
};
const trace = process.env.SEMIO_UNITS_TRACE;
const spec = ((): string | undefined => {const at = args.findIndex(argument => argument === "--spec" || argument.startsWith("--spec=")); return at < 0 ? undefined : args[at]!.includes("=") ? args[at]!.slice(args[at]!.indexOf("=") + 1) : args[at + 1];})();
if (trace && spec) try {appendFileSync(trace, `${(JSON.parse(readFileSync(spec, "utf8")) as {id: string}).id}\n`);} catch {/* tracing is optional */}
try {await runUnitsCommandV1(process.cwd(), args, ports);}
catch (error) {console.error(error instanceof Error ? error.message : String(error)); process.exitCode = 1;}
