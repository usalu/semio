// #region 🧲Header
/** @emoji 🛝 Playground play host for Vcs — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, registerUiVcsSurfaceHost, Platform } from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import type { UiVcsHostSurfaceNode } from "@semio-tech/framework-platform-core";

import {
  VCS_PLAY_APP_ID,
  VCS_PLAY_CONTROLLER_ID,
  VCS_PLAY_SURFACE_ID_EDITOR,
  VCS_PLAY_SURFACE_ID_HISTORY,
  VcsPlayController,
  registerVcsPlayDeclarativeBodies,
} from "@semio-tech/vcs-core";

let vcsPlayChromeRegistered = false;
const vcsPlayControllerRef: { current: VcsPlayController | null } = { current: null };

function useVcsPlayController(runtimeOverride?: Platform): VcsPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribe(listener) : () => {}),
    () => runtime?.generation ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as VcsPlayController | undefined;
  vcsPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useVcsPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as VcsPlayController | undefined;
      vcsPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as VcsPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function VcsPlayEditorSurfaceHost({ node: _node }: { readonly node: UiVcsHostSurfaceNode }): ReactElement {
  const ctrl = useVcsPlayController();
  const projection = ctrl?.projection();
  if (!ctrl || !projection) {
    return <div className="p-double text-sm text-muted-foreground">No VCS document</div>;
  }
  return (
    <div className="flex h-full min-h-0 flex-col gap-double p-double">
      <div className="flex flex-wrap items-center gap-single">
        <button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => ctrl.run("incrementCounter")}>
          + Counter ({projection.counter})
        </button>
        <button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => ctrl.run("commitCheckpoint")}>
          Commit checkpoint
        </button>
        <button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => ctrl.run("undo")}>
          Undo
        </button>
        <button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => ctrl.run("redo")}>
          Redo
        </button>
        <button type="button" className="rounded border px-2 py-1 text-xs" onClick={() => ctrl.run("createAlternative")}>
          New alternative
        </button>
      </div>
      <section className="rounded border p-double text-sm">
        <div>
          <strong>{projection.title}</strong> · counter {projection.counter}
        </div>
        <div className="text-muted-foreground">{projection.notes || "—"}</div>
      </section>
    </div>
  );
}

function VcsPlayHistorySurfaceHost({ node: _node }: { readonly node: UiVcsHostSurfaceNode }): ReactElement {
  const ctrl = useVcsPlayController();
  const columns = ctrl?.historyColumns() ?? [];
  return (
    <div className="h-full min-h-0 overflow-auto p-single">
      <HistoryTable columns={columns} />
    </div>
  );
}

function VcsPlayInner({ playground }: { readonly playground: Playground }): ReactElement {
  useVcsPlayController(playground.runtime);
  useVcsPlayInteractionRevision(playground.runtime);
  return <PlaygroundView runtime={playground.runtime} defaultAppId={VCS_PLAY_APP_ID} />;
}

export function registerVcsPlaySurfaceHosts(): void {
  if (vcsPlayChromeRegistered) return;
  vcsPlayChromeRegistered = true;
  registerUiVcsSurfaceHost(VCS_PLAY_SURFACE_ID_EDITOR, VcsPlayEditorSurfaceHost);
  registerUiVcsSurfaceHost(VCS_PLAY_SURFACE_ID_HISTORY, VcsPlayHistorySurfaceHost);
  registerVcsPlayDeclarativeBodies();
}

function VcsPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
  return <VcsPlayInner playground={playground} />;
}

export function mountVcsPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<VcsPlayChrome playground={playground} />, rootId);
}

const vcsPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerVcsPlaySurfaceHosts,
  mount: mountVcsPlayChrome,
};

export function bootVcsPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, vcsPlayChromeBoot, rootId);
}
//#endregion 🔖VcsPlayHost