/** 🧊 Publishes or starts the real PDF native runtime under its repository leases. */
import {join} from "node:path";
import {pathToFileURL} from "node:url";

const root = process.cwd();
const framework = "🧰️framework";
const repo = framework + "/🛍️products/🦑️repo/🔨️modules/📚️library";
const os = framework + "/🛍️products/💻️os/🔨️modules";
const processOwner = framework + "/🔨️modules/🏃️process/📦️packages/🟦️typescript";
const wgpu = os + "/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu";
const variant = "stdio-pdf-1-7";
const mode = "prototype";
const phase = process.argv[2];
if (phase !== "host") process.env.SEMIO_PLUGIN_CATALOG ??= join(root, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated");
if (phase === "smoke" || phase === "desktop") {
  process.env.SEMIO_LOCKED_LOCALE ??= "en";
  process.env.SEMIO_LOCKED_TERMINOLOGY ??= "native";
  process.env.S_DATA_DIR ??= join(import.meta.dir, "🗑️generated/native-wgpu-save/desktop-data");
}
if (!["host", "modules", "smoke", "desktop"].includes(phase ?? "")) throw Error("Select host, modules, smoke, or desktop");
const entry = (path: string) => pathToFileURL(join(root,path)).href;
const target = phase === "host" ? "@semio-tech/framework-renderer-wgpu:native-build" : "@semio-tech/framework-os-dev:run-" + variant + "-native-" + mode;
const selection = ["run", target, "--exclude-dependencies", "--configuration=" + mode];
const [
  {createNativeBuildCallerCapabilitiesV1},
  {createScriptProcessEnvelope, receiveScriptProcessInvocation, scriptInvocationEnvironment, scriptProcessEnvironment},
  {issueRepositoryScriptProvidersV1},
  {runOwnedCommand},
  {scriptCompilerAdmissionControlV1},
] = await Promise.all([
  import(entry(repo + "/🏗️build/📥️caller/🟦️.ts")),
  import(entry(processOwner + "/🧭️routing/🟦️.ts")),
  import(entry(repo + "/🏃️process/🔌️providers/🟦️.ts")),
  import(entry(processOwner + "/🎛️execution/🟦️.ts")),
  import(entry(processOwner + "/🏗️build/🎟️capacity/🟦️.ts")),
]);
if (process.env.SEMIO_BUILD_MODE && process.env.SEMIO_BUILD_MODE !== mode) throw Error("Native PDF requires prototype admission");
process.env.SEMIO_BUILD_MODE = mode;
issueRepositoryScriptProvidersV1(process.env);
const capability = createNativeBuildCallerCapabilitiesV1(root, selection, "nx", process.env);
const envelope = createScriptProcessEnvelope({version: 1, owner: "pdf:native-wgpu-acceptance", maximumElapsedMilliseconds: 0}, capability, Date.now());
Object.assign(process.env, scriptProcessEnvironment(envelope, process.env));

await receiveScriptProcessInvocation(process.env, async invocation => {
    const control = scriptCompilerAdmissionControlV1(invocation, "native:pdf-" + phase, 0);
    if (phase === "host") {
        const [{buildNativeRenderer}, {repositoryCargoNetworkV1}, {withRepositoryCompilerRuntimeEntriesV1}] = await Promise.all([
            import(entry(wgpu + "/🏗️compiler/🦀️native/📜️script.ts")),
            import(entry(repo + "/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts")),
            import(entry(repo + "/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts")),
        ]);
        const rustPackage = join(root,wgpu,"📦️packages/🦀️rust");
        await withRepositoryCompilerRuntimeEntriesV1(root, invocation, control, () =>
            buildNativeRenderer(rustPackage, mode, root, control, repositoryCargoNetworkV1(invocation, root), invocation));
        return;
    }
    const path = phase === "modules" ? "⌨️native-entrypoint/📦️modules/📜️script.ts" : "⌨️native-entrypoint/📜️script.ts";
    const args = phase === "modules" ? ["publish",variant,mode] : ["run",variant,mode,...(phase === "smoke" ? ["--smoke"] : [])];
    await runOwnedCommand(process.execPath, [join(root,wgpu,path), ...args], root, "native:pdf-" + phase, 0, {
        env: scriptInvocationEnvironment(invocation, process.env),
        signal: control.signal,
    });
});
