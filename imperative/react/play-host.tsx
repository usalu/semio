// #region 🧲Header
/** @emoji 🛝 Playground play host for Imperative — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, registerUiImperativeSurfaceHost, Platform } from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import {
  IMPERATIVE_PLAY_APP_ID,
  IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON,
  IMPERATIVE_PLAY_SURFACE_ID,
  ImperativePlayController,
  registerImperativePlayDeclarativeBodies,
} from "@semio-tech/imperative-core";

import type { UiImperativeHostSurfaceNode } from "@semio-tech/framework-platform-core";

let imperativePlayChromeRegistered = false;
const imperativePlayControllerRef: { current: ImperativePlayController | null } = { current: null };

function useImperativePlayController(runtimeOverride?: Platform): ImperativePlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as ImperativePlayController | undefined;
  imperativePlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function ImperativePlayPaneSurfaceHost(_props: { readonly node: UiImperativeHostSurfaceNode }): ReactElement {
  const ctrl = useImperativePlayController();
  const onDocumentChange = reactHostPort.useCallback(
    (json: string) => {
      ctrl?.run("setDocumentJson", { json });
    },
    [ctrl],
  );
  return (
    <ImperativeEditor
      className="h-full min-h-0"
      documentJson={ctrl?.getDocumentJson() ?? IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON}
      onDocumentChange={onDocumentChange}
    />
  );
}

export function registerImperativePlaySurfaceHosts(): void {
  if (imperativePlayChromeRegistered) return;
  imperativePlayChromeRegistered = true;
  registerUiImperativeSurfaceHost(IMPERATIVE_PLAY_SURFACE_ID, ImperativePlayPaneSurfaceHost);
  registerImperativePlayDeclarativeBodies();
}

function ImperativePlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useImperativePlayController(runtime);
  return <PlaygroundView runtime={runtime} defaultAppId={IMPERATIVE_PLAY_APP_ID} />;
}

function ImperativePlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <ImperativePlayInner runtime={runtime} />;
}

export function mountImperativePlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<ImperativePlayChrome runtime={playground.runtime} />, rootId);
}

const imperativePlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerImperativePlaySurfaceHosts,
  mount: mountImperativePlayChrome,
};

export function bootImperativePlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, imperativePlayChromeBoot, rootId);
}
//#endregion 🔖ImperativePlayHost