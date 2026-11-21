import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join, resolve } from "node:path";

const scratch = resolve(import.meta.dir, "🗑️generated");
const build = join(scratch, "native-mcp/debug/build");
const interpreter = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter");
const runtime = join(interpreter, "🏃️runtime");
const packages = readdirSync(build, { withFileTypes: true }).filter(entry => entry.isDirectory()).map(entry => entry.name);
const variants = new Map(packages.map(pkg => [pkg, readdirSync(join(build, pkg)).filter(id => existsSync(join(build, pkg, id, "out")))]));
const artifact = (pkg: string, id: string, crate: string) => join(pkg, id, "out", `lib${crate}-${id}`);
const external = (crate: string, path: string) => ["--extern", `${crate}=${path}`];
const fingerprint = (pkg: string, id: string, crate: string) => join(build, pkg, id, "fingerprint", `lib-${crate}`);
const crateName = (crate: string) => crate === "framework_hash" ? "semio_framework_hash" : crate;
const findIdentity = (pkg: string, crate: string, hash: string) => variants.get(pkg)?.find(id => {
  const marker = fingerprint(pkg, id, crate);
  return existsSync(marker) && readFileSync(marker, "utf8").trim() === hash;
});
const packageName = (alias: string) => {
  const crate = crateName(alias);
  return crate.startsWith("semio_") ? crate.replaceAll("_", "-") : crate;
};
const identityFor = (crate: string) => (variants.get(packageName(crate)) ?? []).filter(id => existsSync(join(build, artifact(packageName(crate), id, crate) + ".rlib")))
  .sort((left, right) => statSync(join(build, artifact(packageName(crate), right, crate) + ".rlib")).mtimeMs
    - statSync(join(build, artifact(packageName(crate), left, crate) + ".rlib")).mtimeMs)[0];
function deps(crate: string, id: string) {
  const file = fingerprint(packageName(crate), id, crate) + ".json";
  const source = readFileSync(file, "utf8");
  return new Map([...source.matchAll(/\[\d+,"([a-z0-9_]+)",false,(\d+)\]/gu)].map(([, name, digest]) => {
    const hash = Buffer.from(BigInt(digest).toString(16).padStart(16, "0"), "hex").reverse().toString("hex");
    const resolved = findIdentity(packageName(name), crateName(name), hash);
    if (!resolved) throw Error(`Missing ${crate} dependency: ${name} ${hash}`);
    return [name, resolved] as const;
  }));
}
function args(dependencies: Map<string, string>, replacements = new Map<string, string>()) {
  return [...dependencies].flatMap(([crate, id]) => {
    const replacement = replacements.get(crate);
    if (replacement) return external(crate, replacement);
    const path = artifact(packageName(crate), id, crateName(crate));
    return [".rmeta", ".rlib", ".dylib", ".so", ".dll"].filter(ext => existsSync(join(build, path + ext))).flatMap(ext => external(crate, path + ext));
  });
}
const rustup = Bun.which("rustup");
const toolchain = (await Bun.file("rust-toolchain.toml").text()).match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
if (!rustup || !toolchain) throw Error("Pinned Rust unavailable");
const search = [...variants].flatMap(([pkg, ids]) => ids.flatMap(id => {
  const out = join(pkg, id, "out");
  return ["-L", `dependency=${out}`, ...(readdirSync(join(build, out)).some(name => /\.(a|lib)$/.test(name)) ? ["-L", `native=${out}`] : [])];
}));
function compile(...options: string[]) {
  const result = Bun.spawnSync([rustup!, "run", toolchain!, "rustc", "--edition=2021", ...options, "-L", `dependency=${scratch}`, ...search], {
    cwd: build, env: { ...process.env, SEMIO_BUILD_MODE: "prototype" }, stdout: "inherit", stderr: "inherit",
  });
  if (result.exitCode) process.exit(result.exitCode);
}
const core = "semio_plugin_interpreter";
const imports = "semio_plugin_interpreter_imports";
const owned = "semio_plugin_interpreter_runtime";
if (process.argv.includes("--mcp-module")) {
  const workspace = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace");
  const module = join(workspace, "🧊️compiled-components", "🦀️.rs");
  const wrapper = join(scratch, "mcp-compiled-components-check.rs");
  const crate = "semio_os_mcp_plugin_runtime";
  const dependencies = deps(crate, identityFor(crate));
  const needed = new Set(["semio_plugin_host", "semio_os_mcp_foundation", "semio_os_mcp_port", "semio_framework_job", "semio_framework_value", "semio_framework_async", "semio_framework_async_sync", "framework_hash"]);
  const replacements = new Map<string, string>();
  const cacheTest = process.argv.includes("--cache-test");
  const execute = process.argv.includes("--execute");
  if (cacheTest) {
    dependencies.set("serde_json", deps(owned, identityFor(owned)).get("serde_json")!);
    needed.add("serde_json");
  }
  if (process.argv.includes("--current-host")) {
    const host = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host");
    const facade = "semio_plugin_host";
    const guest = "semio_plugin_host_guest_runtime";
    const relay = "semio_plugin_host_post_turn_relay";
    const shard = "semio_plugin_host_shard";
    const executor = "semio_plugin_host_shard_executor";
    const facadeDeps = deps(facade, dependencies.get(facade)!);
    replacements.set(owned, join(scratch, `lib${owned}-prepared_custody.rlib`));
    const metadata = (crate: string, source: string, compileDeps: Map<string, string>) => {
      const output = join(scratch, `lib${crate}-prepared_mcp.${execute ? "rlib" : "rmeta"}`);
      if (!process.argv.includes("--reuse-built")) {
        compile("--crate-name", crate, "--crate-type", "rlib", ...(execute ? ["-Copt-level=1"] : ["--emit", "metadata"]), "-Cmetadata=prepared_mcp",
          source, "-o", output, ...args(compileDeps, replacements));
      }
      replacements.set(crate, output);
    };
    metadata(guest, join(host, "🎭️guest-runtime", "🦀️.rs"), deps(guest, facadeDeps.get(guest)!));
    metadata(shard, join(host, "🧵️shard", "🦀️.rs"), deps(shard, facadeDeps.get(shard)!));
    metadata(relay, join(host, "🔀️post-turn-relay", "🦀️.rs"), deps(relay, facadeDeps.get(relay)!));
    metadata(executor, join(host, "🧵️shard", "🧵️executor", "🦀️.rs"), deps(executor, facadeDeps.get(executor)!));
    metadata(facade, join(host, "📦️packages", "🦀️rust", "🦀️.rs"), facadeDeps);
  }
  await Bun.write(wrapper, `#[path = ${JSON.stringify(module)}]\npub mod compiled_components;\n`);
  const output = join(scratch, execute ? (process.platform === "win32" ? "mcp-compiled-components-check.exe" : "mcp-compiled-components-check") : "mcp-compiled-components-check.rmeta");
  compile("--crate-name", "mcp_compiled_components_check", ...(cacheTest ? ["--test"] : ["--crate-type", "rlib"]), ...(execute ? ["-Cdebuginfo=0", "-Cstrip=symbols"] : ["--emit", "metadata"]), wrapper, "-o", output,
    ...args(new Map([...dependencies].filter(([name]) => needed.has(name))), replacements));
  if (execute) process.exit(Bun.spawnSync([output, "--nocapture"], { stdout: "inherit", stderr: "inherit" }).exitCode ?? 1);
  process.exit(0);
}

const direct=deps(owned,identityFor(owned));
const fixture=resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🧪️tests/🧫️fixtures/🌱️channel/🦀️.rs");
const wrapper=join(scratch,"mcp-channel-admission-law.rs"),output=join(scratch,process.platform==="win32"?"mcp-channel-admission-law.exe":"mcp-channel-admission-law");
await Bun.write(wrapper,`#[path=${JSON.stringify(fixture)}] mod fixture;
use semio_plugin_interpreter_runtime::OwnedRuntime;
use semio_plugin_guest_contract::PluginHostError;
fn main(){let runtime=OwnedRuntime::new(semio_framework_job::default_now_us);let host=semio_os_kernel::CHANNEL_VERSION;for guest in [host-1,host,host+1]{let compiled=runtime.compile_component(&fixture::owned_channel_guest(guest)).unwrap();let result=runtime.instantiate_actor(&compiled);if guest==host{assert!(result.is_ok());}else{match result{Err(PluginHostError::Refused(fault))=>{assert_eq!(fault.code.0,semio_os_kernel::CHANNEL_MISMATCH_CODE);assert_eq!(fault.param("guest"),Some(guest.to_string().as_str()));assert_eq!(fault.param("host"),Some(host.to_string().as_str()));},other=>panic!("unexpected admission {:?}",other.err())}}}}
`);
const replacements=new Map([[core,join(scratch,`lib${core}-prepared_custody.rlib`)],[imports,join(scratch,`lib${imports}-prepared_custody.rlib`)]]);
compile(wrapper,"-o",output,...args(direct,replacements),...external(owned,join(scratch,`lib${owned}-prepared_custody.rlib`)));
const result=Bun.spawnSync([output],{stdout:"inherit",stderr:"inherit"});if(result.exitCode)process.exit(result.exitCode);console.log("owned runtime channel admission and mismatch refusal passed");
