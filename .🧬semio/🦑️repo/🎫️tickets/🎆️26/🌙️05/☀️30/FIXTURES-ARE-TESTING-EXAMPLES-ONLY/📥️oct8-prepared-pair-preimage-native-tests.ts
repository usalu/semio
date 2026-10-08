import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

/** 🏗️ Proves native preparation is an uncached, application-independent Nx leaf against Cargo and esbuild. */
export async function testNativeDependencies(workspace: string, output: string): Promise<void> {
  const require = createRequire(join(workspace, "package.json")), vectors = resolve(import.meta.dir, "../../🧫️fixtures/📦️native-dependencies");
  const fixture = JSON.parse(readFileSync(join(vectors, "🔣️.json"), "utf8"));
  const projects = JSON.parse(readFileSync(join(workspace, "📋️project.json"), "utf8"));
  const graph = await require("esbuild").build({ absWorkingDir: workspace, entryPoints: [fixture.entry,...fixture.sourceEntrypoints], outdir: join(output,"native-source-closure"), bundle: true, write: false, metafile: true, platform: "node", packages: "external", format: "esm", logLevel: "silent" });
  for (const source of Object.keys(graph.metafile.inputs)) {
    assert.notEqual(resolve(workspace, source), join(workspace, "📜️script.ts"));
    for (const forbidden of fixture.forbiddenImports) assert.ok(!source.includes(forbidden), source);
  }
  for (const value of Object.values(graph.metafile.outputs) as { imports: { path: string }[] }[]) for (const dependency of value.imports) assert.ok(dependency.path.startsWith("node:"), dependency.path);
  const { prepareDependencies, NativeDependenciesScript } = await import(pathToFileURL(join(workspace, fixture.entry)).href);
  const { runTool } = await import("../../🚀️bootstrap/📦️dependencies/📜️script.ts");
  const root = mkdtempSync(join(output, "native-sync-")), controller = new AbortController();
  const stop = (): void => controller.abort(), timer = setTimeout(() => controller.abort(new Error("Native synchronization probe exceeded 90s")), 90000);
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  let passed = false;
  const authorCargoFixture = (): void => { for (const [path, contents] of Object.entries(fixture.cargo.files)) writeFileSync(join(root, path), contents as string); };
  const inheritedBrowserPath = process.env.PLAYWRIGHT_BROWSERS_PATH;
  try {
    authorCargoFixture();
    const { cargoRepositoryPackages } = await import("../../../🗂️workspaces/🦀️cargo/🟦️.ts");
    assert.deepEqual(cargoRepositoryPackages(root), fixture.cargo.expectedMembers);
    delete process.env.PLAYWRIGHT_BROWSERS_PATH;
    writeFileSync(join(root, "Cargo.lock"), fixture.bindgenLock);
    const interpolate = (value: string): string => value.replace("{workspace}", root).replaceAll("/", process.platform === "win32" && value.startsWith("{workspace}") ? "\\" : "/");
    for (const row of fixture.environments) {
      const target = projects.targets[`deps-${row.kind}`], calls: string[][] = [];
      assert.equal(target.cache, false); assert.deepEqual(target.outputs, []);
      assert.equal(target.options.command, `bun "${fixture.entry}" sync ${row.kind}`);
      await prepareDependencies(row.kind, root, controller.signal, async (command: string, args: string[], cwd: string, signal: AbortSignal, capture: boolean, env: NodeJS.ProcessEnv) => {
        assert.equal(cwd, root); assert.equal(signal, controller.signal);
        if (capture) return "";
        calls.push([command, ...args]);
        for (const [key, value] of Object.entries(row.environment ?? {})) assert.equal(env[key], interpolate(value as string));
        return "";
      });
      assert.deepEqual(calls, row.commands.map((command: string[]) => command.map(interpolate)), row.kind);
    }
    for (const row of fixture.browserPaths) {
      if (row.input === null) delete process.env.PLAYWRIGHT_BROWSERS_PATH;
      else process.env.PLAYWRIGHT_BROWSERS_PATH = interpolate(row.input);
      let acquisitions = 0;
      await prepareDependencies("browsers", root, controller.signal, async (command: string, args: string[], cwd: string, signal: AbortSignal, capture: boolean, env: NodeJS.ProcessEnv) => {
        assert.equal(command, "bun");
        assert.deepEqual(args, [join(root, "node_modules/playwright/cli.js"), "install", "chromium"]);
        assert.equal(env.PLAYWRIGHT_BROWSERS_PATH, interpolate(row.expected));
        acquisitions += 1;
        return "";
      });
      assert.equal(acquisitions, 1);
    }
    writeFileSync(join(root, "Cargo.lock"), "version = 4\n");
    await assert.rejects(prepareDependencies("trunk", root, controller.signal, async () => { assert.fail("Invalid lock must fail before acquisition"); }), /exactly one wasm-bindgen/);
    for (const args of fixture.rejected) await assert.rejects(new NativeDependenciesScript(root, root).run(args));
    authorCargoFixture();
    writeFileSync(join(root, "package.json"), JSON.stringify({ name: "workspace", private: true }));
    writeFileSync(join(root, "nx.json"), JSON.stringify({ useDaemonProcess: false }));
    writeFileSync(join(root, "📜️script.ts"), "throw new Error('Application code must not run during native setup');\n");
    writeFileSync(join(root, "project.json"), JSON.stringify({ name: "workspace", targets: { "deps-cargo": projects.targets["deps-cargo"], "deps-cargo-lock": projects.targets["deps-cargo-lock"] } }));
    for (const source of Object.keys(graph.metafile.inputs)) {
      const destination = join(root, source); mkdirSync(dirname(destination), { recursive: true }); copyFileSync(join(workspace, source), destination);
    }
    for(const source of fixture.requiredPhysicalSources) assert.equal(existsSync(join(root,source)),true,source);
    const env: NodeJS.ProcessEnv = { ...process.env, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_ROOT: root, REPO_ROOT: root, NX_DAEMON: "false", NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache"), NODE_PATH: join(workspace, "node_modules"), CARGO_TARGET_DIR: join(root, "target"), CARGO_BUILD_BUILD_DIR: join(root, "build"), FORCE_COLOR: "0", NO_COLOR: "1" };
    delete env.NX_SKIP_NX_CACHE;
    const run = (command: string, args: string[]): Promise<string> => runTool(command, args, root, controller.signal, true, env);
    await run("cargo", ["fetch", "--locked", "--offline"]);
    const oracle = JSON.parse(await run("cargo", ["metadata", "--locked", "--offline", "--no-deps", "--format-version=1"]));
    const assertCargoOwner = (value: any): void => {
      assert.equal(value.packages.length, fixture.cargo.expectedMembers.length);
      assert.equal(value.packages[0].name, fixture.cargo.package);
      assert.equal(resolve(value.packages[0].manifest_path), join(root, "Cargo.toml"));
      assert.deepEqual(value.workspace_members, [value.packages[0].id]);
      assert.equal(resolve(value.workspace_root), root);
    };
    assertCargoOwner(oracle);
    const lock = readFileSync(join(root, "Cargo.lock"), "utf8");
    assert.deepEqual(Bun.TOML.parse(lock), require("smol-toml").parse(lock));
    mkdirSync(join(root,"owned"),{recursive:true});writeFileSync(join(root,"owned/📜️script.ts"),fixture.cargo.preparationScript);
    writeFileSync(join(root,"Cargo.toml"),fixture.cargo.files["Cargo.toml"]+fixture.cargo.preparationMetadata);
    await assert.rejects(run("cargo", ["fetch", "--locked", "--offline"]));
    assert.equal(readFileSync(join(root,"Cargo.lock"),"utf8"),lock);
    assert.equal(existsSync(join(root,"generated/Cargo.toml")),true,"Actual owner preparation must introduce the dependency before locked verification");
    let preparedLock: string | undefined;
    for (let attempt = 0; attempt < 2; attempt++) {
      const log = await run("node", [require.resolve("nx/bin/nx.js"), "run", "workspace:deps-cargo", "--output-style=stream"]);
      writeFileSync(join(root, `native-nx-${attempt}.log`), log);
      assert.ok(!log.includes("[local cache]")); assert.match(log, /Successfully ran target deps-cargo/);
      const currentLock=readFileSync(join(root,"Cargo.lock"),"utf8"), parsed=Bun.TOML.parse(currentLock);
      assert.deepEqual(parsed,require("smol-toml").parse(currentLock));
      assert.deepEqual((parsed.package as {name:string}[]).map(value=>value.name).sort(),fixture.cargo.preparedPackages);
      if(preparedLock!==undefined)assert.equal(currentLock,preparedLock);preparedLock=currentLock;
      const actual=JSON.parse(await run("cargo",["metadata","--locked","--offline","--no-deps","--format-version=1"]));
      assert.deepEqual(actual.packages.map((value:any)=>value.name).sort(),fixture.cargo.preparedPackages);
      await run("cargo",["fetch","--locked","--offline"]);
    }
    assert.equal(existsSync(join(root, "target")), false, "Dependency synchronization must not compile deliverables");
    writeFileSync(join(root, "Cargo.lock"), fixture.cargo.invalidLock);
    const refreshed = await run("node", [require.resolve("nx/bin/nx.js"), "run", "workspace:deps-cargo", "--output-style=stream"]);
    assert.ok(!refreshed.includes("[local cache]")); assert.match(refreshed, /Successfully ran target deps-cargo/);
    assert.equal(readFileSync(join(root,"Cargo.lock"),"utf8"),preparedLock);
    const explicit = await run("node", [require.resolve("nx/bin/nx.js"), "run", "workspace:deps-cargo-lock", "--output-style=stream"]);
    assert.match(explicit,/Successfully ran target deps-cargo-lock/);
    await run("cargo",["fetch","--locked","--offline"]);
    assert.equal(existsSync(join(root, "target")), false, "Lock refresh must not compile deliverables");
    passed = true;
  } finally {
    if (inheritedBrowserPath === undefined) delete process.env.PLAYWRIGHT_BROWSERS_PATH;
    else process.env.PLAYWRIGHT_BROWSERS_PATH = inheritedBrowserPath;
    controller.abort(); clearTimeout(timer); process.off("SIGINT", stop); process.off("SIGTERM", stop);
    if (passed) rmSync(root, { recursive: true, force: true });
  }
}
