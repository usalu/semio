// #region 🧲Header
/** @emoji 🛝 Imperative app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution } from "@semio-tech/framework-platform-core";
import { PlaygroundContext, Platform } from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import {
  IMPERATIVE_PLAY_APP_ID,
  IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON,
  IMPERATIVE_PLAY_SURFACE_ID,
  ImperativePlayController,
  imperativePlayWindowBodies,
} from "@semio-tech/imperative-core";
import { ImperativeEditor } from "./index.tsx";

import type { UiImperativeHostSurfaceNode } from "@semio-tech/framework-platform-core";
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

/** @emoji 🛝 Imperative app renderer for playground and OS shells. */
export const imperativeAppRenderer: AppRendererContribution = {
  windowBodies: imperativePlayWindowBodies,
  surfaceHosts: {
    [IMPERATIVE_PLAY_SURFACE_ID]: ImperativePlayPaneSurfaceHost,
  },
};
//#endregion 🔖ImperativePlayHost