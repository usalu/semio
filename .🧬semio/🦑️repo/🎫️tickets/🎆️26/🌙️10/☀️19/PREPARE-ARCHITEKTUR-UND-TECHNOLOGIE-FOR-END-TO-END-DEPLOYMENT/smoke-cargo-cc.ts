import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { basename, delimiter, join } from "node:path";
import { pathToFileURL } from "node:url";

const root = process.cwd(), ticket = process.env.SEMIO_TICKET_DIR!;
const { runCmd, runCmdStatus } = await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts")).href);
const { withNativeTemporaryDirectoryV1, nativeTemporarySystemPortV1 } = await import(pathToFileURL(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/📁️temporary/🟦️.ts")).href);
const port = nativeTemporarySystemPortV1((args: string[]) => runCmdStatus("subst.exe", args));
withNativeTemporaryDirectoryV1({ target: join(ticket, "🗑️generated"), ticket, candidates: [join(process.env.LOCALAPPDATA!, "semio", "native-temp")] }, port, (alias: string) => {
  const compiler = join(alias, "msvc-stage-397620120", "VC", "Tools", "MSVC", "14.50.35717"), sdk = join(alias, "k-EJEv0T"), version = "10.0.26100.0", bin = join(compiler, "bin", "Hostx64", "x64"), dir = mkdtempSync(join(alias, "cargo-"));
  mkdirSync(join(dir, "src"));
  writeFileSync(join(dir, "Cargo.toml"), 'cargo-features = ["trim-paths"]\n[package]\nname = "native-msvc-smoke"\nversion = "0.1.0"\nedition = "2021"\n[workspace]\n[build-dependencies]\ncc = "=1.8.0"\n');
  writeFileSync(join(dir, "build.rs"), 'fn main() { cc::Build::new().file("src/native.c").compile("native_smoke"); }\n');
  writeFileSync(join(dir, "src", "native.c"), "#include <windows.h>\nint native_smoke(void) { return GetCurrentProcessId() != 0; }\n");
  writeFileSync(join(dir, "src", "main.rs"), 'extern "C" { fn native_smoke() -> i32; }\nfn main() { assert_eq!(unsafe { native_smoke() }, 1); println!("cargo-cc-msvc"); }\n');
  const env = { ...process.env, SEMIO_TOOL_CC: join(bin, "cl.exe"), CC: join(bin, "cl.exe"), CXX: join(bin, "cl.exe"), AR: join(bin, "lib.exe"), INCLUDE: [join(compiler, "include"), ...["ucrt", "shared", "um", "winrt", "cppwinrt"].map(part => join(sdk, "Include", version, part))].join(delimiter), LIB: [join(compiler, "lib", "x64"), ...["ucrt", "um"].map(part => join(sdk, "Lib", version, part, "x64"))].join(delimiter), PATH: [bin, join(sdk, "bin", version, "x64"), process.env.PATH].filter(Boolean).join(delimiter), VCToolsInstallDir: compiler + "\\", WindowsSdkDir: sdk + "\\", WindowsSDKVersion: version + "\\", VSCMD_ARG_HOST_ARCH: "x64", VSCMD_ARG_TGT_ARCH: "x64", TEMP: dir, TMP: dir };
  runCmd("repo", ["cargo", "--store=isolated", "--configuration=prototype", "--parallelism=1", "--", "run", "--manifest-path", join(ticket, "🗑️generated", basename(dir), "Cargo.toml")], { cwd: root, env });
});
