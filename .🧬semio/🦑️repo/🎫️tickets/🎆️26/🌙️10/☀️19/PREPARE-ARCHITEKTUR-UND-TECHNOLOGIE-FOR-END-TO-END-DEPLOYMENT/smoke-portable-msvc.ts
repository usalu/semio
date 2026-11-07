import { mkdtempSync, writeFileSync } from "node:fs";
import { delimiter, join } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd(), ticket = process.env.SEMIO_TICKET_DIR!;
const { runCmd, runCmdStatus } = await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts")).href);
const { withNativeTemporaryDirectoryV1, nativeTemporarySystemPortV1 } = await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/📁️temporary/🟦️.ts")).href);
const generated = join(ticket, "🗑️generated"), port = nativeTemporarySystemPortV1((args: string[]) => runCmdStatus("subst.exe", args));
withNativeTemporaryDirectoryV1({ target: generated, ticket, candidates: [join(process.env.LOCALAPPDATA!, "semio", "native-temp")] }, port, (alias: string) => {
  const compiler = join(alias, "msvc-stage-397620120", "VC", "Tools", "MSVC", "14.50.35717"), sdk = join(alias, "k-EJEv0T"), version = "10.0.26100.0";
  const output = mkdtempSync(join(alias, "smoke-"));
  const source = join(output, "smoke.c"), executable = join(output, "smoke.exe"), object = join(output, "smoke.obj");
  writeFileSync(source, "#include <stdio.h>\n#include <windows.h>\nint main(void) { printf(\"native-msvc:%lu\\n\", (unsigned long)GetCurrentProcessId()); return 0; }\n", { flag: "wx" });
  const bin = join(compiler, "bin", "Hostx64", "x64");
  const env = { ...process.env, SEMIO_TOOL_CC: join(bin, "cl.exe"), INCLUDE: [join(compiler, "include"), ...["ucrt", "shared", "um", "winrt", "cppwinrt"].map(part => join(sdk, "Include", version, part))].join(delimiter), LIB: [join(compiler, "lib", "x64"), ...["ucrt", "um"].map(part => join(sdk, "Lib", version, part, "x64"))].join(delimiter), PATH: [bin, join(sdk, "bin", version, "x64"), process.env.PATH].filter(Boolean).join(delimiter), TEMP: output, TMP: output };
  runCmd("repo", ["tools", "exec", "cc", "--", "/nologo", "/TC", "/MT", source, `/Fo${object}`, `/Fe${executable}`], { cwd: root, env });
  runCmd(executable, [], { cwd: output, env });
  const rustSource = join(output, "smoke.rs"), rustExecutable = join(output, "rust-smoke.exe");
  writeFileSync(rustSource, "fn main() { println!(\"native-rust-msvc\"); }\n", { flag: "wx" });
  runCmd("repo", ["tools", "exec", "rustc", "--", rustSource, "-o", rustExecutable], { cwd: root, env });
  runCmd(rustExecutable, [], { cwd: output, env });
});
