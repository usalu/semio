/** 🎟️ Stages the selected real browser session with repository-issued artifact custody. */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const workspace = process.cwd();
const framework = "🧰️framework";
const repo = `${framework}/🛍️products/🦑️repo/🔨️modules/📚️library`;
const processOwner = `${framework}/🔨️modules/🏃️process/📦️packages/🟦️typescript`;
const os = `${framework}/🛍️products/💻️os/🔨️modules`;
const entry = (path: string) => pathToFileURL(join(workspace, path)).href;
const [{createNativeBuildCallerCapabilitiesV1}, {createScriptProcessEnvelope, receiveScriptProcessInvocation, scriptInvocationEnvironment, scriptProcessEnvironment}, {issueRepositoryScriptProvidersV1}] = await Promise.all([
  import(entry(`${repo}/🏗️build/📥️caller/🟦️.ts`)),
  import(entry(`${processOwner}/🧭️routing/🟦️.ts`)),
  import(entry(`${repo}/🏃️process/🔌️providers/🟦️.ts`)),
]);
const variant = "stdio-pdf-1-7";
const phase = process.argv[2] ?? "session";
if (phase === "session" || phase === "serve") process.env.SEMIO_PLUGIN_CATALOG ??= join(workspace, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated");
const ownerEntry = {
  surface: {target: "@semio-tech/framework-surface-rs:wasm", script: `${framework}/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts`, args: ["wasm"]},
  editor: {target: "@semio-tech/framework-editor-rs:wasm", script: `${framework}/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts`, args: ["wasm"]},
  nodeGraph: {target: "@semio-tech/framework-os-node-graph-rs:wasm", script: `${os}/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust/📜️script.ts`, args: ["wasm"]},
  nativeTranspile: {target: "@semio-tech/os-plugin-native-transpile-rs:build", script: `${os}/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/📜️script.ts`, args: ["build", `--host-platform=${process.platform}`]},
} as const;
const targets = {
  catalog: "@semio-tech/s-dev-composition:generate",
  session: `@semio-tech/plugin-registry:session-${variant}`,
  descriptorEmitter: "@semio-tech/os-plugin-describe-rs:build",
  serve: `@semio-tech/framework-os-dev:serve-${variant}-react-prototype`,
  assets: `@semio-tech/framework-os-dev:serve-${variant}-react-prototype`,
};
const producer = Object.hasOwn(ownerEntry, phase) ? ownerEntry[phase as keyof typeof ownerEntry] : undefined;
const components = new Map([
  ["components", {plugins: ["stdio", "stdio-pdf"], command: "browser:pdf-components"}],
  ["components-stdio", {plugins: ["stdio"], command: "browser:stdio-component"}],
  ["components-pdf", {plugins: ["stdio-pdf"], command: "browser:pdf-component"}],
]).get(phase);
const target = producer?.target ?? (components ? "workspace:components" : targets[phase as keyof typeof targets]);
if (!target) throw Error("Expected session, components, components-stdio, components-pdf, serve, or an authored producer");
const profile = "prototype";
if (process.env.SEMIO_BUILD_MODE && process.env.SEMIO_BUILD_MODE !== profile) throw Error("The PDF browser validation requires prototype mode");
process.env.SEMIO_BUILD_MODE = profile;
const selection = ["run", target, "--exclude-dependencies", `--configuration=${profile}`];
issueRepositoryScriptProvidersV1(process.env);
const capability = createNativeBuildCallerCapabilitiesV1(workspace, selection, "nx", process.env);
const envelope = createScriptProcessEnvelope({version: 1, owner: "pdf:browser-direct-validation", maximumElapsedMilliseconds: phase === "session" ? 120_000 : 0}, capability, Date.now());
Object.assign(process.env, scriptProcessEnvironment(envelope, process.env));
await receiveScriptProcessInvocation(process.env, async invocation => {
  if (phase === "catalog") {
    const [{GenerateScript}, {specificCatalogPublicationV1}] = await Promise.all([
      import(entry(`${os}/🔌️plugin/📇️registry/📽️projection/🟦️.ts`)),
      import(entry("✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts")),
    ]);
    const root = join(workspace, "✏️s/🧑‍💻dev");
    const publication = specificCatalogPublicationV1(path => readFileSync(join(workspace, path), "utf8"));
    await new GenerateScript(root, workspace, invocation, publication).run([]);
  } else if (phase === "session") {
    const { SessionScript } = await import(entry(`${os}/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts`));
    const registry = join(workspace, os, "🔌️plugin/📇️registry");
    await new SessionScript(registry, workspace, invocation).run([variant]);
  } else if (components) {
    const [{buildComponentClosureV1}, {repositoryCargoNetworkV1}, {scriptCompilerAdmissionControlV1}, {withRepositoryCompilerRuntimeEntriesV1}] = await Promise.all([
      import(entry(`${os}/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`)),
      import(entry(`${repo}/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts`)),
      import(entry(`${processOwner}/🏗️build/🎟️capacity/🟦️.ts`)),
      import(entry(`${repo}/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts`)),
    ]);
    const control = scriptCompilerAdmissionControlV1(invocation, components.command, 0);
    await withRepositoryCompilerRuntimeEntriesV1(workspace, invocation, control, async () => {
      await buildComponentClosureV1(workspace, new Set(components.plugins), {
        facet: "browser",
        prepareCargoOwners: true,
        invocation,
        control,
        network: repositoryCargoNetworkV1(invocation, workspace),
      });
    });
  } else if (phase === "assets") {
    const [{frameworkOsViteArtifactInputsV1}, {scriptCompilerAdmissionControlV1}, {withRepositoryCacheStoreV1}, {withRepositoryArtifactV1, withRepositoryArtifactTaskV1}] = await Promise.all([
      import(entry(`${os}/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🟦️.ts`)),
      import(entry(`${processOwner}/🏗️build/🎟️capacity/🟦️.ts`)),
      import(entry(`${repo}/📦️packages/🟦️typescript/💾️storage/🟦️.ts`)),
      import(entry(`${repo}/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts`)),
    ]);
    const control = scriptCompilerAdmissionControlV1(invocation, "browser:assets", 0);
    await withRepositoryCacheStoreV1(workspace, "products", invocation, control, async (store, phaseControl) => {
      const requests = frameworkOsViteArtifactInputsV1(store.root, {version: 1, mode: "prototype", renderer: "react", command: "serve", engines: "renderer"});
      for (const request of requests) {
        if (!existsSync(join(request.path, ".semio-artifact.json"))) {
          console.log(`[DEBUG] Unpublished ${request.owner}`);
          continue;
        }
        await withRepositoryArtifactV1({workspace, invocation, storage: store.storage, control: store.control, phaseControl, request}, reader =>
          withRepositoryArtifactTaskV1(reader, invocation, async artifact => {
            await artifact.recheck();
            console.log(`[DEBUG] Verified ${request.owner} with ${artifact.files.length} files`);
          }),
        );
      }
    });
  } else if (phase === "descriptorEmitter") {
    const [{buildDescriptorEmitterV1}, {scriptCompilerAdmissionControlV1}, {withRepositoryCompilerRuntimeEntriesV1}] = await Promise.all([
      import(entry(`${os}/🔌️plugin/🖨️describe/🛠️emitter/🟦️.ts`)),
      import(entry(`${processOwner}/🏗️build/🎟️capacity/🟦️.ts`)),
      import(entry(`${repo}/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts`)),
    ]);
    const control = scriptCompilerAdmissionControlV1(invocation, "browser:descriptor-emitter", 0);
    await withRepositoryCompilerRuntimeEntriesV1(workspace, invocation, control, () => buildDescriptorEmitterV1(workspace, invocation, control));
  } else if (producer) {
    const [{runOwnedCommand}, {scriptCompilerAdmissionControlV1}, {withRepositoryCompilerRuntimeEntriesV1}] = await Promise.all([
      import(entry(`${processOwner}/🎛️execution/🟦️.ts`)),
      import(entry(`${processOwner}/🏗️build/🎟️capacity/🟦️.ts`)),
      import(entry(`${repo}/🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts`)),
    ]);
    const control = scriptCompilerAdmissionControlV1(invocation, `browser:${phase}`, 0);
    await withRepositoryCompilerRuntimeEntriesV1(workspace, invocation, control, () =>
      runOwnedCommand(process.execPath, [join(workspace, producer.script), ...producer.args], workspace, `browser:${phase}`, 0, {
        env: scriptInvocationEnvironment(invocation, process.env),
        signal: control.signal,
      }),
    );
  } else {
    const [{ActivationScript}, {ServeScript}] = await Promise.all([
      import(entry(`${os}/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts`)),
      import(entry(`${os}/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`)),
    ]);
    const root = join(workspace, os, "🧑‍💻dev/📦️packages/🟦️typescript");
    const selection = [variant, "react", profile];
    await new ActivationScript(root, workspace, invocation).run(selection);
    await new ServeScript(root, workspace, invocation).run(selection);
  }
});
