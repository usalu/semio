import { join } from "node:path";

const [generated] = Bun.argv.slice(2);
if (!generated) throw Error("Expected the Draw ticket's generated directory");
const source = process.cwd();
const root = "🧰️framework/🛍️products/💻️os/🔨️modules";
const launcher = join(generated, "../draw-native-dev.ts");
const dev = root + "/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts";
const deadline = Date.now() + 10_800_000;
const finishAfter = async (stage: string, success: RegExp) => {
  const logPath = join(generated, "draw-native-" + stage + ".log");
  while (Date.now() < deadline) {
    const log = Bun.file(logPath);
    if (await log.exists()) {
      const tail = await log.slice(Math.max(0, log.size - 49_152)).text();
      if (success.test(tail)) return;
      if (/repository:command failed with exit code|Error: closed|component build failed with exit code/i.test(tail)) {
        throw Error("Draw native prerequisite failed " + stage);
      }
    }
    await Bun.sleep(2000);
  }
  throw Error("Draw native prerequisite remained incomplete " + stage);
};
const run = async (stage: string, args: string[]) => {
  const task = Bun.spawn(["repo", ...args], {
    cwd: source,
    env: { ...process.env, S_OS_PORT: "16064", S_LOCAL_ONLY: "1", SEMIO_LANGUAGE: "en", SEMIO_PLUGIN_CATALOG: join(source, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated") },
    stdout: "inherit",
    stderr: "inherit",
  });
  const interrupt = () => task.kill("SIGINT");
  const terminate = () => task.kill("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", terminate);
  try {
    const code = await task.exited;
    if (code) throw Error("Draw native stage " + stage + " exited " + code);
  } finally {
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", terminate);
  }
};
const drive = (stage: string, target: string, args: string[]) =>
  run(stage, ["tools", "exec", "--light", "bun", "--", launcher, generated, stage, target, stage, ...args]);
const complete = async (stage: string, success: RegExp) => {
  const log = Bun.file(join(generated, "draw-native-" + stage + ".log"));
  return await log.exists() && success.test(await log.slice(Math.max(0, log.size - 49_152)).text());
};
const fontsReady = /"command":"fonts","stage":"complete"/;
const compiled = /\[component\] semio:draw: built, described and materialized/;
const support = finishAfter("restore-support", /"command":"support","stage":"complete"/);
const session = finishAfter("restore-session", /"command":"session","stage":"complete"/);
const fonts = finishAfter("restore-fonts", fontsReady);
const storageBlocked = async (begin = 0): Promise<number | undefined> => {
  const log = Bun.file(join(generated, "draw-native-component.log"));
  if (!await log.exists()) return;
  const output = await log.slice(Math.max(begin, log.size - 65_536)).text();
  const refusal = /error: Native build storage refused: [^\n]*needs a predicted (\d+) MiB [^\n]*plus the (\d+) MiB reserve/.exec(output);
  if (refusal) return (Number(refusal[1]) + Number(refusal[2]) + 1) * 1_048_576;
};
const awaitStorage = async (required: number) => {
  const owner = join(source, "🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage");
  const { nativeStatfsSyncV1 } = await import(join(owner, "🌐️platform/📋️contract/🟦️.ts"));
  const { nativeImageContainerFreeBytesSyncV1, nativeImageEffectiveAvailableBytesV1 } = await import(join(owner, "🍎️image/⚖️availability/🟦️.ts"));
  const store = join(source, ".cache", "b", "r", "m");
  console.log("[DEBUG] Waiting for Draw component compiler storage", Math.ceil(required / 1_048_576), "MiB");
  while (Date.now() < deadline) {
    const facts = nativeStatfsSyncV1(store);
    const bytes = nativeImageEffectiveAvailableBytesV1(facts.bavail * facts.bsize, nativeImageContainerFreeBytesSyncV1(store));
    if (bytes >= required) {
      console.log("[DEBUG] Draw component compiler storage is available", Math.floor(bytes / 1_048_576), "MiB");
      return;
    }
    await Bun.sleep(10000);
  }
  throw Error("Draw compiler storage remained below the native driver's requirement");
};
const component = (async () => {
  if (await complete("component", compiled)) return;
  await Promise.all([
    finishAfter("descriptor-emitter", /"command":"build","stage":"complete"/),
    finishAfter("restore-native-transpile", /[\\/]browser[\\/]prototype[\\/]tools[\\/]native-transpile[\\/][a-z0-9]+[\\/]semio-native-transpile(?:\.exe)?(?:\r?\n|$)/),
  ]);
  let required = await storageBlocked();
  for (let attempt = 0; ; attempt++) {
    if (required !== undefined) await awaitStorage(required);
    const start = Bun.file(join(generated, "draw-native-component.log")).size;
    try {
      await drive("component", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts", ["--manifest", "🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust/Cargo.toml"]);
      break;
    } catch (error) {
      required = await storageBlocked(start);
      if (required === undefined || attempt === 2) throw error;
    }
  }
  if (!(await complete("component", compiled))) throw Error("Draw component finished without a publication receipt");
})();
await Promise.all([support, session, fonts, component]);
const args = ["draw", "react", "prototype"];
await drive("prepare", dev, args);
await drive("activate", dev, args);
const wasmReady = /"command":"wasm","stage":"complete"/;
await Promise.all(["restore-wasm-surface", "restore-wasm-editor", "wasm-node-graph"].map(stage => finishAfter(stage, wasmReady)));
await run("serve", ["./📜️script.ts", "dev", "playground", ...args]);
