/** 🧩️ The host contract of the element-host registry: what a lane implementation registers and the Interpreter resolves, spelled with no renderer-library type.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}. A host is a function of its props; how the lane mounts one (the React lane mounts it as a component) stays inside the lane.
 */

import type { ComponentSceneHostProps } from "@semio-tech/framework";

//#region ExtensionHost
/** 📨️ What a `Component::Extension` host receives: the authored props and the identity of the node that carries them. */
export type ExtensionHostPropsV1 = { readonly value: unknown; readonly nodeId: string | number; readonly nodeKey: string };

/** 🎛️ The host of one extension id: a function of {@link ExtensionHostPropsV1} whose rendered result only the lane that mounts it reads. */
export type ExtensionHostV1 = (props: ExtensionHostPropsV1) => unknown;

/** 🔎️ Answers the host for an extension id at render time, or `undefined` for an id it does not own; reading the id and the host only then keeps a registration safe while the host modules are still evaluating. */
export type ExtensionHostResolverV1 = (extensionId: string) => ExtensionHostV1 | undefined;
//#endregion ExtensionHost

//#region ComponentSceneHost
/** 🎬️ The host of one `ComponentKind` scene: a function of the authored scene props whose rendered result only the lane that mounts it reads. */
export type ComponentSceneHostV1 = (props: ComponentSceneHostProps) => unknown;

/** 🧭️ Resolves a scene host at render time: modules form a cycle with the hosts' own helpers, so a captured host can still be `undefined` at registration and only a live read is ready by first paint. */
export type ComponentSceneHostResolverV1 = () => ComponentSceneHostV1 | undefined;
//#endregion ComponentSceneHost
