import { resolve } from "node:path";
import { acquireCargoBuildLeaseV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";

type CargoInvocation = { argv: string[]; cwd: string; env: Record<string, string> };

const invocation: CargoInvocation = await Bun.file(process.argv[2]).json();
const env: Record<string, string | undefined> = {
  ...invocation.env,
  HOME: process.env.HOME,
  USERPROFILE: process.env.USERPROFILE,
  XDG_DATA_HOME: process.env.XDG_DATA_HOME,
  RUSTC_WRAPPER: "",
};
const args = [...invocation.argv, "--offline", "--jobs=1"];
const stop = new AbortController();
process.once("SIGINT", () => stop.abort());
process.once("SIGTERM", () => stop.abort());
const lease = await acquireCargoBuildLeaseV1({
  directory: resolve(".cache/b/l/m/data"),
  buildDirectory: invocation.env.CARGO_BUILD_BUILD_DIR,
  targetDirectory: invocation.env.CARGO_TARGET_DIR,
  args: args.slice(1),
  signal: stop.signal,
});
try {
  const cargo = Bun.spawn(args, { cwd: invocation.cwd, env, stdout: "inherit", stderr: "inherit", signal: stop.signal });
  process.exitCode = await cargo.exited;
} finally {
  lease.release();
}
