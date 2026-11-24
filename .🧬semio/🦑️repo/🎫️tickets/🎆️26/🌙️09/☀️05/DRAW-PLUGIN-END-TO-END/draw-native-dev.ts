import { open } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";

const [generated, stage, target, ...args] = Bun.argv.slice(2);
if (!generated || !stage || !target || !/^[a-z]+(?:-[a-z0-9]+)*$/.test(stage)) throw Error("Expected generated folder, stage, registered script and its arguments");
if (stage === "serve") {
  const engines = ["wasm-surface", "wasm-editor", "wasm-node-graph"];
  const deadline = Date.now() + 5_100_000;
  await Promise.all(engines.map(async engine => {
    const logPath = join(generated, "draw-native-" + engine + ".log");
    while (Date.now() < deadline) {
      const log = Bun.file(logPath);
      if (await log.exists()) {
        const tail = await log.slice(Math.max(0, log.size - 65_536)).text();
        if (/"command":"wasm","stage":"complete"/.test(tail)) return;
        if (/repository:command failed with exit code|Error: closed/i.test(tail)) throw Error("Draw engine build failed " + engine);
      }
      await Bun.sleep(2000);
    }
    throw Error("Draw engine build did not complete " + engine);
  }));
}
const root = process.cwd();
const framework = join(root, "🧰️framework");
const repoLibrary = join(framework, "🛍️products/🦑️repo/🔨️modules/📚️library");
const processPackage = join(framework, "🔨️modules/🏃️process/📦️packages/🟦️typescript");
const { createScriptProcessEnvelope, scriptProcessEnvironment } = await import(join(processPackage, "🧭️routing/🟦️.ts"));
const { repositoryScriptProvidersV1 } = await import(join(repoLibrary, "🏃️process/🔌️providers/🟦️.ts"));
const { SCRIPT_PROVIDERS_ENV, writeScriptProviderMapV1 } = await import(join(processPackage, "🔌️providers/🟦️.ts"));
const { declaredNativeOwnerCommandV1 } = await import(join(repoLibrary, "🏗️build/📤️arguments/🟨️.mjs"));
const script = resolve(root, target);
const workingDirectory = dirname(script);
const commandLine = ["repo", script, ...args].map(arg => JSON.stringify(arg)).join(" ");
const mode = process.env.DRAW_NATIVE_BUILD_MODE ?? "prototype";
if (mode !== "prototype" && mode !== "release") throw Error("Expected a registered Draw build mode");
const maximumElapsedMilliseconds = stage.startsWith("serve") ? 86_400_000 : stage.startsWith("component") ? 21_600_000 : 5_400_000;
const capabilities = {
  version: 1,
  buildMode: mode,
  repositoryRoot: root,
  codeRoot: root,
  artifactDirectory: resolve(generated),
  command: declaredNativeOwnerCommandV1(target, workingDirectory, commandLine),
  transport: { maximumBytes: 134_217_728, maximumLines: 65_536 },
  child: { maximumElapsedMilliseconds, idleMilliseconds: stage.startsWith("serve") ? 86_400_000 : 600_000 },
  network: { offline: true },
};
const log = await open(join(generated, "draw-native-" + stage + ".log"), "a");
const deadline = Date.now() + Math.max(10_800_000, maximumElapsedMilliseconds);
try {
  for (let attempt = 1; ; attempt++) {
    const budget = Math.min(capabilities.child.maximumElapsedMilliseconds, deadline - Date.now());
    if (budget <= 0) throw Error("Draw native stage exhausted its retry window");
    const envelope = createScriptProcessEnvelope({ version: 1, owner: "draw:native-dev", maximumElapsedMilliseconds: budget }, { ...capabilities, child: { ...capabilities.child, maximumElapsedMilliseconds: budget } }, Date.now());
    const env = scriptProcessEnvironment(envelope, {
      ...process.env,
      [SCRIPT_PROVIDERS_ENV]: writeScriptProviderMapV1(repositoryScriptProvidersV1()),
      SEMIO_BUILD_MODE: mode,
      SEMIO_WORKSPACE_ROOT: root,
      SEMIO_LANGUAGE: "en",
      SEMIO_OFFLOAD: process.env.SEMIO_OFFLOAD ?? "distribute",
      SEMIO_PLUGIN_CATALOG: join(root, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"),
    });
    const task = Bun.spawn(["repo", script, ...args], { cwd: workingDirectory, env, stdout: log.fd, stderr: log.fd });
    const interrupt = () => task.kill("SIGINT");
    const terminate = () => task.kill("SIGTERM");
    process.once("SIGINT", interrupt);
    process.once("SIGTERM", terminate);
    const status = await task.exited.finally(() => {
      process.off("SIGINT", interrupt);
      process.off("SIGTERM", terminate);
    });
    const logFile = Bun.file(join(generated, "draw-native-" + stage + ".log"));
    const tail = await logFile.slice(Math.max(0, logFile.size - 16_000)).text();
    const tornArtifact = tail.includes("Original artifact bytes or identity changed");
    const retry = status === 75 || (status !== 0 && tornArtifact);
    if (!retry || attempt >= 6 || Date.now() >= deadline) {
      console.log("[DEBUG] Draw native", stage, status);
      process.exitCode = status;
      break;
    }
    console.log("[DEBUG] Draw native", stage, tornArtifact ? "retrying after a torn artifact read" : "recapturing source after exit 75", attempt);
    await Bun.sleep(250 * attempt);
  }
} finally {
  await log.close();
}
