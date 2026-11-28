/** 🧩 Split edit loop of one plugin component: cargo (root as rlib) → thin plugin module → cached framework image (reused or relinked) →
 * split link → component; every phase is timed into `<out>/split-<label>.json`. Usage: `bun split-loop.ts <label> [manifest] [deployment-dir]`.
 * The plugin crates keep the normal lane's codegen flags (no PIC); only the root crate is built as an rlib, which keeps cargo from LTO-only dependencies.
 * @see ./design.md */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { spawn } from "node:child_process";

const R = "/Users/ueli/Documents/semio";
const T = import.meta.dir;
const label = process.argv[2] ?? "split";
const manifest = resolve(R, process.argv[3] ?? "🌎️hub/🧩️compositions/🏭️process/🧩️component/📦️packages/🦀️rust/Cargo.toml");
const deployment = process.argv[4] ?? "🏭️process";
const owned = [join(R, "🌎️hub/🧩️compositions", deployment), join(R, "✏️s/🔌️plugins", deployment)];
const out = join(T, "🗑️generated", "split");
const cache = join(R, ".cache/b/r/m/split-images", deployment);
const toolchain = "nightly-2026-07-20-aarch64-apple-darwin";
const bin = join(process.env.HOME!, ".rustup/toolchains", toolchain, "lib/rustlib/aarch64-apple-darwin/bin");
const native = process.env.SEMIO_NATIVE_TRANSPILE_BIN ?? join(R, ".cache/b/p/m/browser/prototype/tools/native-transpile", process.platform, "semio-native-transpile");
const phases: Record<string, number> = {};
const started = Date.now();
mkdirSync(out, { recursive: true });
mkdirSync(cache, { recursive: true });

async function phase<T>(name: string, run: () => Promise<T> | T): Promise<T> {
  const at = Date.now();
  try { return await run(); } finally { phases[name] = (Date.now() - at) / 1000; console.log(`[split] ${name} ${phases[name]} s`); }
}

function run(command: string, args: string[], options: { cwd?: string; env?: Record<string, string>; log?: string } = {}): Promise<string> {
  return new Promise((ok, ko) => {
    const child = spawn(command, args, { cwd: options.cwd ?? R, env: { ...process.env, ...options.env }, stdio: ["ignore", "pipe", "pipe"] });
    const chunks: Buffer[] = [], errors: Buffer[] = [];
    child.stdout.on("data", c => chunks.push(c));
    child.stderr.on("data", c => errors.push(c));
    child.on("close", code => {
      const stdout = Buffer.concat(chunks).toString(), stderr = Buffer.concat(errors).toString();
      if (options.log) writeFileSync(options.log, stdout + "\n--- stderr ---\n" + stderr);
      code === 0 ? ok(stdout) : ko(Object.assign(new Error(`${command} ${args[0]} exited ${code}\n${stderr.slice(-4000)}`), { code, stderr }));
    });
  });
}

function packageNames(): string[] {
  const names: string[] = [];
  const walk = (dir: string) => { for (const entry of readdirSync(dir, { withFileTypes: true })) { if (entry.isDirectory() && entry.name !== "target" && !entry.name.startsWith("🗑️")) walk(join(dir, entry.name)); else if (entry.name === "Cargo.toml") { const name = /^\[package\][^[]*?^name\s*=\s*"([^"]+)"/ms.exec(readFileSync(join(dir, entry.name), "utf8"))?.[1]; if (name) names.push(name); } } };
  for (const root of owned) if (existsSync(root)) walk(root);
  return names.sort();
}

type Artifact = { name: string; pkg: string; manifest: string; rlib: string; owned: boolean };

const artifacts: Artifact[] = await phase("cargo", async () => {
  let stdout = "";
  for (let attempt = 1; ; attempt++) {
    try { stdout = await run("repo", ["cargo", "--store=shared", "--configuration=prototype", "--", "rustc", "--manifest-path", manifest, "--target", "wasm32-wasip2", "--crate-type", "rlib", "--message-format=json-render-diagnostics"], { env: { SEMIO_LANGUAGE: "en" }, log: join(out, `cargo-${label}-${attempt}.log`) }); break; }
    catch (error) {
      const log = readFileSync(join(out, `cargo-${label}-${attempt}.log`), "utf8");
      if (attempt >= Number(process.env.SPLIT_ATTEMPTS ?? 10) || !/"rendered":"error\[E|^error\[E|because --locked was passed/m.test(log)) throw error;
      console.log(`[split] cargo attempt ${attempt} broke on a peer's half-landed edit; retrying in 120 s`);
      await new Promise(r => setTimeout(r, 120_000));
    }
  }
  return stdout.split("\n").filter(line => line.startsWith('{"reason":"compiler-artifact"')).map(line => JSON.parse(line)).flatMap(a => (a.filenames as string[]).filter(f => f.endsWith(".rlib")).map(rlib => ({ name: a.target.name as string, pkg: /#(?:([^@]+)@)?/.exec(a.package_id as string)?.[1] ?? "", manifest: a.manifest_path as string, rlib, owned: owned.some(root => (a.manifest_path as string).startsWith(root)) })));
});
const ownedArtifacts = artifacts.filter(a => a.owned), framework = artifacts.filter(a => !a.owned);
const rootArtifact = artifacts.find(a => a.manifest === manifest);
if (!rootArtifact) throw new Error(`[DEBUG] cargo reported no rlib for ${manifest}`);
console.log(`[split] ${ownedArtifacts.length} plugin rlibs, ${framework.length} framework rlibs`);

/** 📏 Initial size of the single table a core module defines (section 4), read from the section headers only. */
function tableMinimum(path: string): number {
  const b = readFileSync(path);
  const leb = (at: { i: number }) => { let v = 0, s = 0, x = 0; do { x = b[at.i++]!; v += (x & 0x7f) * 2 ** s; s += 7; } while (x & 0x80); return v; };
  for (const at = { i: 8 }; at.i < b.length;) { const id = b[at.i++]!, size = leb(at), end = at.i + size; if (id === 4) { leb(at); at.i += 1; leb(at); return leb(at); } at.i = end; }
  throw new Error(`no table section in ${path}`);
}

const objects = join(out, "objects");
const pluginModule = join(out, "plugin.wasm");
await phase("extract", async () => {
  rmSync(objects, { recursive: true, force: true });
  const files: string[] = [];
  for (const [index, artifact] of ownedArtifacts.entries()) {
    const dir = join(objects, String(index));
    mkdirSync(dir, { recursive: true });
    await run(join(bin, "llvm-ar"), ["x", artifact.rlib], { cwd: dir });
    for (const file of readdirSync(dir)) if (file.endsWith(".o")) files.push(join(dir, file));
  }
  writeFileSync(join(out, "plugin.rsp"), files.join("\n"));
  const rootObjects = files.filter(f => f.startsWith(join(objects, String(ownedArtifacts.indexOf(rootArtifact))) + "/"));
  const names = (await run(join(bin, "llvm-nm"), ["--defined-only", "--extern-only", "--format=just-symbols", ...rootObjects])).split("\n").map(l => l.trim()).filter(n => n && !n.endsWith(":") && !/^(_R|_ZN|__|\.L|GOT\.)/.test(n));
  writeFileSync(join(out, "plugin-exports.rsp"), [...new Set(names)].sort().map(n => `--export=${n}`).join("\n") + "\n");
});
const linkPlugin = (tableBase: number) => run(join(bin, "rust-lld"), ["-flavor", "wasm", "--no-entry", "--import-memory", "--import-table", "--allow-undefined", "--emit-relocs", "--gc-sections", "--global-base=1024", `--table-base=${tableBase}`, "-z", "stack-size=0", "--export=__stack_pointer", "--export-if-defined=__wasm_call_ctors", `@${join(out, "plugin-exports.rsp")}`, "-o", pluginModule, `@${join(out, "plugin.rsp")}`], { log: join(out, `plugin-link-${label}.log`) });

const fingerprint = createHash("sha256").update(toolchain).update(framework.map(a => { const s = statSync(a.rlib); return `${a.rlib}|${s.size}|${s.mtimeMs}`; }).sort().join("\n")).digest("hex").slice(0, 24);
const imageDir = join(cache, fingerprint), image = join(imageDir, "image.wasm"), imageExports = join(imageDir, "exports.rsp");
const cachedTable = existsSync(image) ? tableMinimum(image) : 1;
await phase("plugin-module", () => linkPlugin(cachedTable));
const wanted = join(out, "wanted.rsp");
const wantedImports = join(out, "wanted-imports.rs");
await run(native, ["image-exports", "--plugin", pluginModule, "-o", wanted, "--imports", wantedImports]);
const importLines = (path: string) => existsSync(path) ? readFileSync(path, "utf8").split("\n").filter(l => l.startsWith("unsafe extern") || l.startsWith("#[link(")) : [];
const wantedSymbols = new Set([...readFileSync(wanted, "utf8").split("\n").filter(Boolean), ...importLines(wantedImports).filter(l => l.startsWith("#[link(")).map((l, k) => `${l} ${importLines(wantedImports).filter(x => x.startsWith("unsafe extern"))[k]!.replace(/fn __semio_image_import_\d+/, "fn")}`)]);
const previous = existsSync(join(imageDir, "contract.txt")) ? new Set(readFileSync(join(imageDir, "contract.txt"), "utf8").split("\n").filter(Boolean)) : new Set<string>();
const missing = [...wantedSymbols].filter(s => !previous.has(s));
const verdict = existsSync(image) && missing.length === 0 ? "reused" : existsSync(image) ? `relinked: ${missing.length} new plugin imports` : "linked: no image for this framework fingerprint";
await phase("framework-image", async () => {
  if (verdict === "reused") return;
  mkdirSync(imageDir, { recursive: true });
  const union = [...new Set([...previous, ...wantedSymbols])].sort();
  writeFileSync(join(imageDir, "contract.txt"), union.join("\n") + "\n");
  writeFileSync(imageExports, union.filter(l => l.startsWith("--export=")).join("\n") + "\n");
  const imports = union.filter(l => l.startsWith("#[link(")).map((l, k) => { const [link, decl] = l.split(" unsafe extern"); return `${link}\nunsafe extern${decl!.replace(/\bfn\(/, `fn __semio_image_import_${k}(`)}`; });
  const anchors = imports.map((_, k) => `__semio_image_import_${k} as usize`);
  const root = join(imageDir, "root.rs");
  const section = readFileSync(manifest, "utf8").split(/^\[dependencies\]$/m)[1]!.split(/^\[/m)[0]!;
  const direct = new Set([...section.matchAll(/^([A-Za-z0-9_-]+)\s*=/gm)].map(m => m[1]!));
  const externs = framework.concat(ownedArtifacts).filter(a => direct.has(a.pkg));
  if (externs.length !== direct.size) throw new Error(`[DEBUG] image root found ${externs.map(a => a.pkg)} of ${[...direct]}`);
  writeFileSync(root, `//! 🖼️ Framework image root of ${deployment}: links the framework closure, exports what the plugin module imports and declares its canonical intrinsic imports.\n${externs.map(a => `extern crate ${a.name};`).join("\n")}\n${imports.join("\n")}\n#[unsafe(no_mangle)]\npub extern "C" fn __semio_image_imports() -> usize { ${anchors.length ? anchors.join(" ^ ") : "0"} }\n`);
  const dirs = [...new Set(artifacts.map(a => dirname(a.rlib)))];
  const args = ["--crate-name", "semio_framework_image", "--edition=2024", root, "--crate-type", "cdylib", "--target", "wasm32-wasip2", "-C", "opt-level=s", "-C", "debuginfo=0", "-C", "codegen-units=1",
    ...dirs.flatMap(d => ["-L", `dependency=${d}`]), ...externs.flatMap(a => ["--extern", `${a.name}=${a.rlib}`]),
    "-C", `link-arg=@${imageExports}`, "-C", "link-arg=--export-table", "-C", "link-arg=--growable-table", "-C", `link-arg=--global-base=${Number(process.env.SPLIT_RESERVE ?? 64 << 20)}`, "-C", "link-arg=--skip-wit-component", "-C", "link-arg=--max-memory=536870912", "-C", "link-arg=-zstack-size=8388608", "-C", "link-arg=--threads=1", "-o", image];
  writeFileSync(join(imageDir, "rustc.rsp"), args.join("\n"));
  await run("rustc", [`+${toolchain.replace("-aarch64-apple-darwin", "")}`, `@${join(imageDir, "rustc.rsp")}`], { log: join(out, `image-${label}.log`) });
});
const tableBase = tableMinimum(image);
if (tableBase !== cachedTable) await phase("plugin-relink", () => linkPlugin(tableBase));

const component = join(out, "component.wasm");
await phase("split-link", () => run(native, ["link", "--framework", image, "--plugin", pluginModule, "-o", component, "--receipt", join(out, `link-${label}.json`)]));
const report = { label, verdict, fingerprint, plugin: { rlibs: ownedArtifacts.length, bytes: statSync(pluginModule).size }, framework: { rlibs: framework.length, imageBytes: statSync(image).size }, componentBytes: statSync(component).size, phases, totalSeconds: (Date.now() - started) / 1000 };
writeFileSync(join(out, `split-${label}.json`), JSON.stringify(report, null, 1));
console.log(JSON.stringify(report));
