import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { createReadStream, existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join, resolve, sep } from "node:path";

const ticket = import.meta.dir;
const scratch = join(ticket, "🗑️generated");
const build = join(scratch, "native-host/debug/build");
const wfc = process.argv.includes("--wfc");
const liveNote = process.argv.includes("--live-note");
assert(!(wfc && liveNote));
const model = wfc ? "f0caf2db1142c1ba93f50ac6" : "6d8a4f7ae2584b94ed7f1643";
function generatedArgument(flag: string, fallback: string): string {
  const supplied = process.argv.filter(row => row.startsWith(flag + "="));
  assert(supplied.length <= 1);
  const selected = supplied.length ? resolve(supplied[0].slice(flag.length + 1)) : fallback;
  assert(selected.startsWith(scratch + sep));
  return selected;
}
const component = generatedArgument("--component", join(scratch, "m-canon/browser/prototype/components", model, "📦️component.wasm"));
const transcript = generatedArgument("--transcript", join(scratch,
  wfc ? "owned-input-canonical-wfc.trace" : liveNote ? "owned-input-note-live.trace" : "owned-input-wasmtime.trace"));
const sample = process.argv.includes("--sample");
const root = "semio_plugin_interpreter";
const relink = process.argv.includes("--relink");
const suffix = sample ? "-sample" : "";
const library = generatedArgument("--library", join(scratch, `libsemio_plugin_interpreter-owned-input${suffix}.rlib`));
const executable = join(scratch, `owned-input-core-replay${suffix}${process.platform === "win32" ? ".exe" : ""}`);
const [fuel] = process.argv.slice(2).filter(arg => !["--replay", "--relink", "--sample", "--wfc", "--live-note"].includes(arg)
  && !["--component=", "--transcript=", "--library="].some(prefix => arg.startsWith(prefix)));
assert(relink || !process.argv.some(row => row.startsWith("--library=")));
const config = sample ? ["--cfg", 'feature="testkit"'] : [];

async function run(args: string[], cwd: string, timeout: number) {
  const child = Bun.spawn(args, { cwd, stdout: "inherit", stderr: "inherit" });
  const deadline = setTimeout(() => {
    console.error("[DEBUG] Scalar replay exceeded its wall-clock bound");
    child.kill();
  }, timeout);
  try {
    assert.equal(await child.exited, 0);
  } finally {
    clearTimeout(deadline);
  }
}

if (!process.argv.includes("--replay")) {
  const tools = Bun.spawnSync(["repo", "tools", "list", "--json"], { stderr: "inherit" });
  assert.equal(tools.exitCode, 0);
  const rustup = JSON.parse(tools.stdout.toString()).find((entry: { id: string }) => entry.id === "rustup")?.command;
  const toolchain = readFileSync("rust-toolchain.toml", "utf8").match(/^channel\s*=\s*"([^"]+)"/mu)?.[1];
  assert(rustup && toolchain, "Pinned Rust unavailable");

  const packages = readdirSync(build, { withFileTypes: true }).filter(entry => entry.isDirectory()).map(entry => entry.name);
  const variants = new Map(packages.map(pkg => [pkg, readdirSync(join(build, pkg), { withFileTypes: true })
    .filter(entry => entry.isDirectory() && existsSync(join(build, pkg, entry.name, "out")))
    .map(entry => entry.name)]));
  const search = packages.flatMap(pkg => variants.get(pkg)!.flatMap(variant => {
    const path = join(pkg, variant, "out");
    return ["-L", `dependency=${path}`, ...(readdirSync(join(build, path)).some(file => /\.(a|lib)$/u.test(file)) ? ["-L", `native=${path}`] : [])];
  }));
  const rustc = [rustup, "run", toolchain, "rustc"];
  if (!relink) {
    const fingerprint = (pkg: string, id: string, crate: string) => join(build, pkg, id, "fingerprint", "lib-" + crate);
    const artifact = (pkg: string, id: string, crate: string) => join(pkg, id, "out", `lib${crate}-${id}.rlib`);
    const rootPackage = "semio-plugin-interpreter";
    const artifactCandidates = (pkg: string, crate: string) => (variants.get(pkg) ?? [])
      .filter(id => existsSync(join(build, artifact(pkg, id, crate))) && existsSync(fingerprint(pkg, id, crate) + ".json"))
      .sort((a, b) => statSync(join(build, artifact(pkg, b, crate))).mtimeMs - statSync(join(build, artifact(pkg, a, crate))).mtimeMs);
    const cached = artifactCandidates(rootPackage, root).find(id =>
      JSON.parse(readFileSync(fingerprint(rootPackage, id, root) + ".json", "utf8")).features === "[]");
    assert(cached, "Missing cached production interpreter fingerprint");
    const dependencyHashes = (json: string) => new Map([...json.matchAll(/\[\d+,"([a-z0-9_]+)",false,(\d+)\]/gu)]
      .map(([, crate, hash]) => [crate, hash]));
    const rootHashes = dependencyHashes(readFileSync(fingerprint(rootPackage, cached, root) + ".json", "utf8"));
    const actor = "semio_framework_actor";
    const actorPkg = "semio-framework-actor";
    const actorVariant = process.env.SEMIO_CORE_REPLAY_ACTOR ?? artifactCandidates(actorPkg, actor)[0];
    assert(actorVariant, "Missing cached actor dependency");
    const actorHashes = dependencyHashes(readFileSync(fingerprint(actorPkg, actorVariant, actor) + ".json", "utf8"));
    const externs = [...rootHashes].flatMap(([crate, rootDigest]) => {
      const pkg = crate.replaceAll("_", "-");
      const digest = actorHashes.get(crate) ?? rootDigest;
      const hash = Buffer.from(BigInt(digest).toString(16).padStart(16, "0"), "hex").reverse().toString("hex");
      const id = crate === actor ? actorVariant : artifactCandidates(pkg, crate).find(id => {
        const path = fingerprint(pkg, id, crate);
        return existsSync(path) && readFileSync(path, "utf8").trim() === hash;
      });
      assert(id, `Missing exact cached dependency ${crate}`);
      console.log(`[DEBUG] linking ${crate} variant ${id}`);
      const rlib = artifact(pkg, id, crate);
      return ["--extern", `${crate}=${rlib.replace(/\.rlib$/u, ".rmeta")}`, "--extern", `${crate}=${rlib}`];
    });
    assert(externs.length === 16);
    const source = resolve("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs");
    console.log(`[DEBUG] compiling current scalar interpreter at opt-level 3 using cached dependency identities from ${cached}`);
    await run([...rustc, "--edition=2021", "--crate-name", root, "--crate-type", "rlib", "-Copt-level=3", "-Zthreads=1",
      ...config, source, "-o", library, ...externs, ...search], build, 120_000);
  }
  const metadata = library.replace(/\.rlib$/u, ".rmeta");
  const interpreter = [
    ...(metadata !== library && existsSync(metadata) ? ["--extern", `${root}=${metadata}`] : []),
    "--extern", `${root}=${library}`,
  ];
  await run([...rustc, "--edition=2024", "-Copt-level=3", "-Cstrip=debuginfo", ...config, join(ticket, "owned-input-core-replay.rs"),
    "-o", executable, ...interpreter, ...search], build, 120_000);
}

assert(statSync(transcript).size <= 8 * 1024 * 1024);
const header = readFileSync(transcript, "utf8").split("\n", 2)[1];
const expected = header.split(" ")[3];
const hash = createHash("sha256");
for await (const bytes of createReadStream(component)) hash.update(bytes);
assert.equal(hash.digest("hex"), expected, "Replay component differs from captured component");
await run([executable, component, transcript, ...(fuel === undefined ? [] : [fuel])], ticket, 8 * 60_000);
