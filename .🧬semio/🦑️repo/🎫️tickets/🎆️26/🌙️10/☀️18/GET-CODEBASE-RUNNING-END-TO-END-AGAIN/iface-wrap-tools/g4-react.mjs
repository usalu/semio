import fs from "node:fs";
import { fileURLToPath } from "node:url";
import path from "node:path";
import { create, edit, editAll, load, R, rebase, set, sha } from "./lib.mjs";
import { analyse, ELEMENTS, PORT_ABS, renderPort, rewriteText } from "./react-port.mjs";
import { src } from "./paths.mjs";

const rel = (absolute) => path.relative(R, absolute).split(path.sep).join("/");
const E = rel(ELEMENTS);
const ENGINE = E.replace(/\/🧱️elements$/, "");
const HERE = fileURLToPath(new URL(".", import.meta.url));

function specifierFrom(file) {
  const value = path.relative(path.dirname(R + file), PORT_ABS).split(path.sep).join("/");
  return value.startsWith(".") ? value : "./" + value;
}

function agentsFixAfter(file) {
  const manifestPath = path.join(HERE, "..", "agents-fix-tools", "staged", "manifest.json");
  if (!fs.existsSync(manifestPath)) return null;
  const row = JSON.parse(fs.readFileSync(manifestPath, "utf8")).files.find((entry) => entry.path === file);
  if (!row) return null;
  const live = fs.readFileSync(R + file, "utf8");
  return sha(live) === row.beforeSha256 ? fs.readFileSync(path.join(HERE, "..", "agents-fix-tools", "staged", row.after), "utf8") : null;
}

export function build() {
  const analysis = analyse();
  const host = `${E}/🎬️MediaTransportHost/🟦️.tsx`;
  load(host);
  const rebased = agentsFixAfter(host);
  if (rebased !== null) rebase(host, rebased);
  create(rel(PORT_ABS), renderPort(analysis));
  for (const file of analysis.files) {
    const relative = rel(file);
    set(relative, rewriteText(file, load(relative), analysis.exportsOf));
  }

  const lifecycle = `${ENGINE}/🎬️media/🚚️lifecycle/🟦️.ts`;
  edit(lifecycle, `export type MediaTransportOperation = {`, `/** 🪟️ The owner a media host renders for: the app instance and controller that own the window, the window itself and the port its exports travel through. */\nexport type MediaTransportOwnerV1 = {\n  readonly instanceId: number;\n  readonly controllerId: string;\n  readonly windowId: string;\n  readonly port: MediaTransportPort;\n};\n\nexport type MediaTransportOperation = {`);

  const scope = `${E}/🎬️MediaTransportHost/🔗️owner-scope/🟦️.tsx`;
  create(scope, src("react/owner-scope.tsx").replace("__PORT__", specifierFrom(scope)));

  const P = specifierFrom(host);
  const referenceLine = `/// <reference types="vitest/importMeta" />\n`;
  if (load(host).startsWith(referenceLine)) set(host, load(host).slice(referenceLine.length));
  edit(host, `import { createContext, useContext, useEffect, useMemo, useRef, useState, type ReactElement } from "${P}";`, `import { useEffect, useMemo, useRef, useState, type ReactElement } from "${P}";`);
  edit(host, `import { collectMediaTransportBytes, type MediaTransportPort } from "@semio-tech/framework-os/modules/renderer/engine/media/lifecycle";\n`, `import { collectMediaTransportBytes, type MediaTransportPort } from "@semio-tech/framework-os/modules/renderer/engine/media/lifecycle";\nimport { useMediaTransportOwner } from "./🔗️owner-scope/🟦️.tsx";\n`);
  edit(host, `export type MediaTransportOwner = {\n  readonly instanceId: number;\n  readonly controllerId: string;\n  readonly windowId: string;\n  readonly port: MediaTransportPort;\n};\n\nexport const MediaTransportOwnerContext = createContext<MediaTransportOwner | null>(null);\n\n`, ``);
  edit(host, `const owner = useContext(MediaTransportOwnerContext);`, `const owner = useMediaTransportOwner();`);

  const contract = `${E}/🗣️Interpreter/🧩️host-contract/🟦️.ts`;
  create(contract, src("react/host-contract.ts"));
  const registry = `${E}/🗣️Interpreter/🗺️element-host-registry/🟦️.ts`;
  const RP = specifierFrom(registry);
  edit(registry, `import type { ComponentType } from "${RP}";\nimport type { ComponentKind, ComponentSceneHostProps } from "@semio-tech/framework";\n`, `import type { ComponentKind, ComponentSceneHostProps } from "@semio-tech/framework";\nimport type { ComponentType } from "${RP}";\nimport type { ComponentSceneHostResolverV1, ExtensionHostPropsV1, ExtensionHostResolverV1 } from "../🧩️host-contract/🟦️.ts";\n`);
  edit(registry, `/** 🧭️ Resolves a scene host at render time: modules form a cycle with the hosts' own helpers, so a captured component can still be \`undefined\` at registration and only a live read is ready by first paint. */\nexport type ComponentSceneHostResolverV1 = () => ComponentType<ComponentSceneHostProps> | undefined;\n\n`, ``);
  edit(registry, `/** 🧭️ The scene host registered for \`kind\`, or \`undefined\` when the composition root supplied none. */\nexport function resolveComponentSceneHost(kind: ComponentKind): ComponentType<ComponentSceneHostProps> | undefined {\n  return componentSceneHosts.get(kind)?.();\n}`, `/** 🧭️ The scene host registered for \`kind\` as the component this lane mounts, or \`undefined\` when the composition root supplied none; the one place a registered host becomes a React component. */\nexport function resolveComponentSceneHost(kind: ComponentKind): ComponentType<ComponentSceneHostProps> | undefined {\n  return componentSceneHosts.get(kind)?.() as ComponentType<ComponentSceneHostProps> | undefined;\n}`);
  edit(registry, `/** 🧩️ What a \`Component::Extension\` host receives: the authored props and the identity of the node that carries them. */\nexport type ExtensionHostPropsV1 = { readonly value: unknown; readonly nodeId: string | number; readonly nodeKey: string };\n\n/** 🧩️ Answers the host for an extension id at render time, or \`undefined\` for an id it does not own; reading the id and the host only then keeps a registration safe while the host modules are still evaluating. */\nexport type ExtensionHostResolverV1 = (extensionId: string) => ComponentType<ExtensionHostPropsV1> | undefined;\n\n`, ``);
  edit(registry, `/** 🧩️ The extension host the first registered resolver owns for \`extensionId\`, or \`undefined\` when none does. */\nexport function resolveExtensionHost(extensionId: string): ComponentType<ExtensionHostPropsV1> | undefined {\n  for (const resolve of extensionHostResolvers) {\n    const host = resolve(extensionId);\n    if (host !== undefined) return host;\n  }`, `/** 🧩️ The extension host the first registered resolver owns for \`extensionId\` as the component this lane mounts, or \`undefined\` when none does; the one place a registered host becomes a React component. */\nexport function resolveExtensionHost(extensionId: string): ComponentType<ExtensionHostPropsV1> | undefined {\n  for (const resolve of extensionHostResolvers) {\n    const host = resolve(extensionId);\n    if (host !== undefined) return host as ComponentType<ExtensionHostPropsV1>;\n  }`);

  const hosts = `${E}/🪢️ElementHosts/🟦️.ts`;
  edit(hosts, `import { registerComponentSceneHostV1, registerExtensionHostV1, registerLeftoverSelectionPortV1 } from "../🗣️Interpreter/🗺️element-host-registry/🟦️.ts";\n`, `import { registerComponentSceneHostV1, registerExtensionHostV1, registerLeftoverSelectionPortV1 } from "../🗣️Interpreter/🗺️element-host-registry/🟦️.ts";\nimport type { ExtensionHostV1 } from "../🗣️Interpreter/🧩️host-contract/🟦️.ts";\n`);
  edit(hosts, `function mediaTransportExtensionHost(extensionId: string): typeof MediaTransportHost | undefined {`, `function mediaTransportExtensionHost(extensionId: string): ExtensionHostV1 | undefined {`);

  const modeWindows = `${E}/🏛️ShellHost/📋️panel-hosting/🟠️mode-windows/🟦️.tsx`;
  edit(modeWindows, `import { MediaTransportOwnerContext, type MediaTransportOwner } from "../../../🎬️MediaTransportHost/🟦️.tsx";\n`, `import { MediaTransportOwnerScope } from "../../../🎬️MediaTransportHost/🔗️owner-scope/🟦️.tsx";\nimport type { MediaTransportOwnerV1 } from "@semio-tech/framework-os/modules/renderer/engine/media/lifecycle";\n`);
  editAll(modeWindows, `): MediaTransportOwner | null =>`, `): MediaTransportOwnerV1 | null =>`, 2);
  edit(modeWindows, `<MediaTransportOwnerContext.Provider value={spawnedMediaOwner(windowId)}>`, `<MediaTransportOwnerScope owner={spawnedMediaOwner(windowId)}>`);
  edit(modeWindows, `<MediaTransportOwnerContext.Provider value={browserActorStore === undefined ? primaryMediaOwner(kind.id) : null}>`, `<MediaTransportOwnerScope owner={browserActorStore === undefined ? primaryMediaOwner(kind.id) : null}>`);
  edit(modeWindows, `<MediaTransportOwnerContext.Provider value={primaryMediaOwner(instance.id)}>`, `<MediaTransportOwnerScope owner={primaryMediaOwner(instance.id)}>`);
  editAll(modeWindows, `</MediaTransportOwnerContext.Provider>`, `</MediaTransportOwnerScope>`, 3);

  const lifecycleTest = `${E}/🎬️MediaTransportHost/🧪️tests/♻️lifecycle/🟦️.tsx`;
  edit(lifecycleTest, `const { MediaTransportHost, MediaTransportOwnerContext } = dependencies;`, `const { MediaTransportHost, MediaTransportOwnerScope } = dependencies;`);
  editAll(lifecycleTest, `createElement(MediaTransportOwnerContext.Provider, { value: {`, `createElement(MediaTransportOwnerScope, { owner: {`, 2);
  edit(lifecycleTest, `const dependencies = await import("../../🟦️.tsx");`, `const dependencies = { ...(await import("../../🟦️.tsx")), ...(await import("../../🔗️owner-scope/🟦️.tsx")) };`);
  const browserTest = `${E}/🎬️MediaTransportHost/🧪️tests/♻️lifecycle/🌐️browser/🟦️.tsx`;
  edit(browserTest, `import { MediaTransportHost, MediaTransportOwnerContext, type MediaTransportPort } from "../../../🟦️.tsx";`, `import { MediaTransportHost, type MediaTransportPort } from "../../../🟦️.tsx";\nimport { MediaTransportOwnerScope } from "../../../🔗️owner-scope/🟦️.tsx";`);
  edit(browserTest, `createElement(MediaTransportOwnerContext.Provider, { value: {`, `createElement(MediaTransportOwnerScope, { owner: {`);
  return { rewritten: analysis.files.length, exportedNames: analysis.used.react.size + analysis.used.dom.size, bare: analysis.bare, rebasedOnAgentsFix: rebased !== null };
}
