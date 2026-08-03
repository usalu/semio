from pathlib import Path
import sys

path = Path(sys.argv[1])
text = path.read_text()
start = text.find("export function runWasmPackWebBuild(opts: {")
if start < 0:
    raise SystemExit("start not found")
# Replace through the end of the else branch's wasmPackArgs assignment block,
# up to and including the non-threads runCmdStatus call.
marker = "  if (status !== 0) {\n    console.error(`[${logPrefix}] wasm build failed`);"
end = text.find(marker, start)
if end < 0:
    raise SystemExit("end marker not found")

new = r'''export function runWasmPackWebBuild(opts: {
  rsDir: string;
  skipEnvVar: string;
  logPrefix: string;
  pkg: WasmPackWebPkg;
  wasmBaseName: string;
  /** When true, build with atomics + `-Z build-std` for wasm-bindgen-rayon thread pools. */
  threads?: boolean;
  /** Optional Cargo feature flags passed to wasm-pack / cargo build. */
  cargoFeatures?: readonly string[];
  /** 🔿️ Cargo/wasm-pack profile. `release`/`dev` map to `--release`/`--dev`; any other name
   * (e.g. `wasm-release`) passes `--profile <name>`. wasm-pack maps custom profile names onto
   * `[package.metadata.wasm-pack.profile.custom]` (bulk-memory / trunc_sat enable flags). */
  profile?: string;

}): void {
  const { rsDir, skipEnvVar, logPrefix, pkg, wasmBaseName, threads = false, cargoFeatures = [], profile = "release" } = opts;
  const pkgDir = join(rsDir, "pkg");
  const wasmPath = join(pkgDir, `${wasmBaseName}_bg.wasm`);
  const packProfileArgs = profile === "release" ? (["--release"] as const) : profile === "dev" ? (["--dev"] as const) : (["--profile", profile] as const);
  const cargoProfileArgs = profile === "release" ? (["--release"] as const) : profile === "dev" ? (["--dev"] as const) : (["--profile", profile] as const);
  const cargoProfileDir = profile === "dev" ? "debug" : profile;
  if (process.env[skipEnvVar] === "1") {
    console.log(`[${logPrefix}] ${skipEnvVar}=1 → skipping wasm-pack build`);
    return;
  }
  if (!wasmPackInputsStale(rsDir, wasmPath)) {
    console.log(`[${logPrefix}] pkg/${wasmBaseName}_bg.wasm up to date → skipping wasm-pack build`);
    if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
    const snippetFiles = wasmPackSnippetFiles(pkgDir);
    const pkgJson = {
      type: "module",
      version: pkg.version ?? "0.1.0",
      sideEffects: pkg.sideEffects ?? ["./snippets/*"],
      ...pkg,
      files: [...new Set([...pkg.files, ...snippetFiles])],
    };
    writeFileSync(join(pkgDir, "package.json"), `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");
    return;
  }
  const buildLabel = threads ? "cargo build (threaded) + wasm-bindgen" : "wasm-pack build";
  console.log(`[${logPrefix}] ${buildLabel} ${packProfileArgs.join(" ")} --target web --out-dir pkg --out-name ${wasmBaseName} --no-pack`);
  const t0 = Date.now();
  let status: number;
  if (threads) {
    const repoRoot = getWorkspaceRoot();
    const crateName = readFileSync(join(rsDir, "Cargo.toml"), "utf8").match(/^name\s*=\s*"([^"]+)"/m)?.[1];
    if (!crateName) {
      console.error(`[${logPrefix}] missing package name in Cargo.toml`);
      process.exit(1);
    }
    const cargoWasm = join(repoRoot, `target/wasm32-unknown-unknown/${cargoProfileDir}`, `${crateName.replace(/-/g, "_")}.wasm`);
    const threadedCargoArgs = ["build", ...cargoProfileArgs, "--target", "wasm32-unknown-unknown", "-Z", "build-std=std,panic_abort", ...cargoFeatures.flatMap((feature) => ["--features", feature])];
    status = runCmdStatus("cargo", threadedCargoArgs, { cwd: rsDir, env: { ...process.env }, budgetMs: buildBudgetMs() });
    if (status !== 0) {
      console.error(`[${logPrefix}] cargo threaded build failed`);
      process.exit(status);
    }
    if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
    status = runCmdStatus(resolveWasmBindgenBin(), [cargoWasm, "--out-dir", "pkg", "--typescript", "--target", "web", "--out-name", wasmBaseName], { cwd: rsDir, env: { ...process.env }, budgetMs: buildBudgetMs() });
  } else {
    const wasmPackArgs = ["x", "wasm-pack", "build", ...packProfileArgs, "--target", "web", "--out-dir", "pkg", "--out-name", wasmBaseName, "--no-pack", ...cargoFeatures.flatMap((feature) => ["--", "--features", feature])];
    status = runCmdStatus(process.execPath, wasmPackArgs, { cwd: rsDir, env: { ...process.env }, budgetMs: buildBudgetMs() });
  }
'''

path.write_text(text[:start] + new + text[end:])
print(f"patched {path} ({end - start} -> {len(new)} chars)")
