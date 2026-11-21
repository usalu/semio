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
const capabilities = {
  version: 1,
  buildMode: mode,
  repositoryRoot: root,
  codeRoot: root,
  artifactDirectory: resolve(generated),
  command: declaredNativeOwnerCommandV1(target, workingDirectory, commandLine),
  transport: { maximumBytes: 134_217_728, maximumLines: 65_536 },
  child: { maximumElapsedMilliseconds: 5_400_000, idleMilliseconds: 600_000 },
  network: { offline: true },
};
const envelope = createScriptProcessEnvelope({ version: 1, owner: "draw:native-dev", maximumElapsedMilliseconds: 5_400_000 }, capabilities, Date.now());
const env = scriptProcessEnvironment(envelope, {
  ...process.env,
  [SCRIPT_PROVIDERS_ENV]: writeScriptProviderMapV1(repositoryScriptProvidersV1()),
  SEMIO_BUILD_MODE: mode,
  SEMIO_LANGUAGE: "en",
  SEMIO_OFFLOAD: process.env.SEMIO_OFFLOAD ?? "auto",
  SEMIO_PLUGIN_CATALOG: join(root, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated"),
});
const log = await open(join(generated, "draw-native-" + stage + ".log"), "a");
const deadline = Date.now() + capabilities.child.maximumElapsedMilliseconds;
try {
  for (let attempt = 1; ; attempt++) {
    const task = Bun.spawn(["repo", script, ...args], { cwd: workingDirectory, env, stdout: log.fd, stderr: log.fd });
    const interrupt = () => task.kill("SIGINT");
    const terminate = () => task.kill("SIGTERM");
    process.once("SIGINT", interrupt);
    process.once("SIGTERM", terminate);
    const status = await task.exited.finally(() => {
      process.off("SIGINT", interrupt);
      process.off("SIGTERM", terminate);
    });
    if (status !== 75 || attempt >= 6 || Date.now() >= deadline) {
      console.log("[DEBUG] Draw native", stage, status);
      process.exitCode = status;
      break;
    }
    console.log("[DEBUG] Draw native", stage, "recapturing source after exit 75", attempt);
    await Bun.sleep(250 * attempt);
  }
} finally {
  await log.close();
}
