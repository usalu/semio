import { readFileSync } from "node:fs";
import process from "node:process";

const [runner, instrumentedGuest, ...args] = process.argv.slice(2);
const compile = WebAssembly.compile;
const instantiate = WebAssembly.instantiate;
const exit = process.exit;
let stage;
WebAssembly.compile = bytes => compile.call(WebAssembly,
  bytes.byteLength > 170_000_000 ? readFileSync(instrumentedGuest) : bytes);
WebAssembly.instantiate = async (...inputs) => {
  const result = await instantiate.call(WebAssembly, ...inputs);
  stage ??= (result.instance ?? result).exports.member_stage;
  return result;
};
process.exit = status => {
  console.error("[DEBUG] Last instrumented member stage", stage?.value);
  exit.call(process, status);
};
process.argv = [process.argv[0], runner, ...args];
await import(runner);
