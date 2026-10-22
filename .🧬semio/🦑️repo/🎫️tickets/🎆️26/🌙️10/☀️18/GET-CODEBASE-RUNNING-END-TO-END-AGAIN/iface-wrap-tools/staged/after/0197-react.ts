/** 🗺️ The element-host registry: the one contract the generic React renderer reads to find the concrete element hosts it mounts.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}. The Interpreter knows only this contract; the composition root (`🪢️ElementHosts`) registers the concrete scene hosts, the extension hosts and the document-selection port, so no host is imported by the Interpreter.
 */

import type { ComponentKind, ComponentSceneHostProps } from "@semio-tech/framework";
import type { ComponentType } from "../../⚛️react-port/🟦️.ts";
import type { ComponentSceneHostResolverV1, ExtensionHostPropsV1, ExtensionHostResolverV1 } from "../🧩️host-contract/🟦️.ts";

//#region ComponentSceneHostRegistry
const componentSceneHosts = new Map<ComponentKind, ComponentSceneHostResolverV1>();

/** 🧭️ Registers the host that renders one `ComponentKind`; the returned function retires exactly this registration. */
export function registerComponentSceneHostV1(kind: ComponentKind, resolve: ComponentSceneHostResolverV1): () => void {
  componentSceneHosts.set(kind, resolve);
  return () => {
    if (componentSceneHosts.get(kind) === resolve) componentSceneHosts.delete(kind);
  };
}

/** 🧭️ The scene host registered for `kind` as the component this lane mounts, or `undefined` when the composition root supplied none; the one place a registered host becomes a React component. */
export function resolveComponentSceneHost(kind: ComponentKind): ComponentType<ComponentSceneHostProps> | undefined {
  return componentSceneHosts.get(kind)?.() as ComponentType<ComponentSceneHostProps> | undefined;
}
//#endregion ComponentSceneHostRegistry

//#region ExtensionHostRegistry
const extensionHostResolvers = new Set<ExtensionHostResolverV1>();

/** 🧩️ Registers a resolver of extension hosts; the returned function retires exactly this registration. */
export function registerExtensionHostV1(resolve: ExtensionHostResolverV1): () => void {
  extensionHostResolvers.add(resolve);
  return () => {
    extensionHostResolvers.delete(resolve);
  };
}

/** 🧩️ The extension host the first registered resolver owns for `extensionId` as the component this lane mounts, or `undefined` when none does; the one place a registered host becomes a React component. */
export function resolveExtensionHost(extensionId: string): ComponentType<ExtensionHostPropsV1> | undefined {
  for (const resolve of extensionHostResolvers) {
    const host = resolve(extensionId);
    if (host !== undefined) return host as ComponentType<ExtensionHostPropsV1>;
  }
  return undefined;
}
//#endregion ExtensionHostRegistry

//#region LeftoverSelectionPort
/** 🌲️ The document-scoped selection a guest's leftover publication leaves behind, as far as a retained tree reads it. */
export type LeftoverSelectionOverlayV1 = { readonly ids: readonly string[] };

/** 🌲️ The document selection a tree row reads: its subscription, its current overlay and its row predicate. */
export type LeftoverSelectionPortV1 = {
  readonly subscribe: (listener: () => void) => () => void;
  readonly overlay: () => LeftoverSelectionOverlayV1 | null;
  readonly itemSelected: (itemId: string, leftoverIds: readonly string[] | undefined) => boolean;
};

const NO_LEFTOVER_SELECTION: LeftoverSelectionPortV1 = { subscribe: () => () => {}, overlay: () => null, itemSelected: () => false };

let leftoverSelectionPort: LeftoverSelectionPortV1 = NO_LEFTOVER_SELECTION;

/** 🌲️ Supplies the document selection every retained tree reads; the returned function retires exactly this port. */
export function registerLeftoverSelectionPortV1(port: LeftoverSelectionPortV1): () => void {
  leftoverSelectionPort = port;
  return () => {
    if (leftoverSelectionPort === port) leftoverSelectionPort = NO_LEFTOVER_SELECTION;
  };
}

/** 🌲️ Subscribes to the registered document selection; a stable function, so a store hook never resubscribes. */
export function subscribeLeftoverSelectionV1(listener: () => void): () => void {
  return leftoverSelectionPort.subscribe(listener);
}

/** 🌲️ The registered document selection's current overlay, `null` while none is published or registered. */
export function leftoverSelectionOverlayV1(): LeftoverSelectionOverlayV1 | null {
  return leftoverSelectionPort.overlay();
}

/** 🌲️ Whether the registered document selection marks the tree row `itemId` selected. */
export function leftoverItemSelectedV1(itemId: string, leftoverIds: readonly string[] | undefined): boolean {
  return leftoverSelectionPort.itemSelected(itemId, leftoverIds);
}
//#endregion LeftoverSelectionPort
