// #region 🧲Header
/** @emoji 🛝 Vcs app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution, UiVcsHostSurfaceNode } from "@semio-tech/framework-platform-core";
import { PlaygroundContext, Platform } from "@semio-tech/framework-playground-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { VCS_PLAY_SURFACE_ID_EDITOR, VCS_PLAY_SURFACE_ID_HISTORY, VcsPlayController, vcsPlayWindowBodies } from "@semio-tech/vcs-core";
import { HistoryTable } from "./index.tsx";

const vcsPlayControllerRef: { current: VcsPlayController | null } = { current: null };

function useVcsPlayController(runtimeOverride?: Platform): VcsPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime?.getActiveApp()?.controller as VcsPlayController | undefined;
      vcsPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime ? runtime.subscribe(listener) : () => {};
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => {
      const generation = runtime?.generation ?? 0;
      const revision = (runtime?.getActiveApp()?.controller as VcsPlayController | undefined)?.getInteractionRevision() ?? 0;
      return generation * 1_000_000 + revision;
    },
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as VcsPlayController | undefined;
  vcsPlayControllerRef.current = ctrl ?? null;
  return ctrl;
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

/** @emoji 🛝 Vcs app renderer contribution for playground and OS shells. */
export const vcsAppRenderer: AppRendererContribution = {
  windowBodies: vcsPlayWindowBodies,
  surfaceHosts: {
    [VCS_PLAY_SURFACE_ID_EDITOR]: VcsPlayEditorSurfaceHost,
    [VCS_PLAY_SURFACE_ID_HISTORY]: VcsPlayHistorySurfaceHost,
  },
};
//#endregion 🔖VcsPlayHost
