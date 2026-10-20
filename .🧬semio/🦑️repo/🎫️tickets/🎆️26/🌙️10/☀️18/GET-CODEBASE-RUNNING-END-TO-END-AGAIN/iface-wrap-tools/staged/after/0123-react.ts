/** 🔧️ The plugin module url, rebuilt-at, in-flight and install progress state of the backbone worker hook.
 *
 * 🧱️ Child hook module of {@link ../🟦️.ts | useBackboneWorker}: runs the original hook statements in order. */
import { useCallback, useEffect, useRef, useState } from "../../../⚛️react-port/🟦️.ts";
import { type PluginModuleAcquisitionProgress } from "@semio-tech/framework";

/** 🧰️ Holds the plugin module url, rebuilt-at, in-flight and install progress state. */
export function usePluginInstallState() {
  /** 🔌️ The exact (possibly cache-busted `?v=`) module URL each currently-loaded plugin was acquired
   * at — `LoadedProgramState`/`PluginWasmHandle` carry no URL of their own. 🧬️ H1-react: its old
   * reader, `evictPluginModule` (the deleted refcounted module-URL lease pool, packet H2's "must not
   * exist" list), is gone — `reloadPlugin`/`uninstallPlugin` now free resources entirely through
   * `handle.dispose()` (`ShardClient.dispose` per activated actor), so this map is unread bookkeeping
   * kept for a future consumer rather than removed mid-packet. */
  const pluginModuleUrlByIdRef = useRef<Map<string, string>>(new Map());
  /** 🔁️ The `rebuiltAt` each currently-loaded plugin's artifact was acquired at, so the `PluginSource`
   * pump can tell a genuine rebuild from a replayed availability snapshot
   * ({@link pluginAvailabilityRouteV1}). Absent means "acquired unbusted", i.e. the first load, which
   * names no build. */
  const pluginArtifactRebuiltAtRef = useRef<Map<string, number>>(new Map());
  /** 🧾️ Records what an acquisition was built from, forgetting it again when the acquisition named no
   * build — so the map never claims a build the loaded artifact does not have. */
  const recordPluginArtifactRebuiltAt = useCallback((pluginId: string, rebuiltAt: number | undefined) => {
    if (rebuiltAt === undefined) pluginArtifactRebuiltAtRef.current.delete(pluginId);
    else pluginArtifactRebuiltAtRef.current.set(pluginId, rebuiltAt);
  }, []);
  /** 🗝️ Per-pluginId mutual exclusion across `installPlugin`/`reloadPlugin`/`uninstallPlugin` — the
   * boot effect and the `PluginSource` subscription effect can both request the same pluginId around
   * mount (e.g. the host plugin already appears in the connect-time `snapshot`), and without this guard
   * both calls would independently acquire a module lease, race their `UPSERT_LOADED_PLUGIN` dispatches,
   * and leak whichever lease lost the race (nothing left holding a reference to release it). */
  const pluginOpInFlightRef = useRef<Set<string>>(new Set());
  /** 🛑️ The abort of every install currently in flight, by pluginId — what the install band's cancel
   * control fires. A ref, not reducer state: nothing renders from it (the band renders from
   * `pluginStatusById`'s own `"installing"`), and an abort must reach the exact load that is running
   * rather than whatever a render pass last captured. */
  const pluginInstallAbortsRef = useRef<Map<string, AbortController>>(new Map());
  /** 📈️ Bytes every downloading install has verified so far — what the install band shows beside its
   * cancel. Chunk reports land in a ref and reach state at most every 120 ms, so a 60 MB module does
   * not re-render the shell once per 64 KiB chunk; a finished install clears its row at once. */
  const [pluginInstallProgressById, setPluginInstallProgressById] = useState<Readonly<Record<string, PluginModuleAcquisitionProgress>>>({});
  const pluginInstallProgressRef = useRef<Map<string, PluginModuleAcquisitionProgress>>(new Map());
  const pluginInstallProgressFlushRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const notePluginInstallProgress = useCallback((pluginId: string, progress: PluginModuleAcquisitionProgress | null) => {
    if (progress) pluginInstallProgressRef.current.set(pluginId, progress);
    else if (!pluginInstallProgressRef.current.delete(pluginId)) return;
    if (progress && pluginInstallProgressFlushRef.current !== null) return;
    if (pluginInstallProgressFlushRef.current !== null) clearTimeout(pluginInstallProgressFlushRef.current);
    pluginInstallProgressFlushRef.current = setTimeout(
      () => {
        pluginInstallProgressFlushRef.current = null;
        setPluginInstallProgressById(Object.fromEntries(pluginInstallProgressRef.current));
      },
      progress ? 120 : 0,
    );
  }, []);
  useEffect(
    () => () => {
      if (pluginInstallProgressFlushRef.current !== null) clearTimeout(pluginInstallProgressFlushRef.current);
    },
    [],
  );
  return { pluginModuleUrlByIdRef, pluginArtifactRebuiltAtRef, recordPluginArtifactRebuiltAt, pluginOpInFlightRef, pluginInstallAbortsRef, pluginInstallProgressById, notePluginInstallProgress };
}
