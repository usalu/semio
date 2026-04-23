// #region 🧲Header
// Storybook: KitStoreHandle, subscribe buffer, VCS id fields, last change result
// #endregion

import * as React from "react";

import { ensureSemioWasm, KitStoreHandle } from "./semioWasm";

import type { LoggedEvent } from "./EventsFeed";

// #region 🎨 RJV Theme
// Tracks the `dark` class on <html> and maps it to a base-16 theme name
// consumed by `@microlink/react-json-view` so JSON viewers track Storybook theme toggles.
export type RjvThemeName = "rjv-default" | "monokai";

export function useRjvTheme(): RjvThemeName {
  const [isDark, setIsDark] = React.useState<boolean>(() => {
    if (typeof document === "undefined") return false;
    return document.documentElement.classList.contains("dark");
  });
  React.useEffect(() => {
    if (typeof document === "undefined") return;
    const el = document.documentElement;
    const update = () => setIsDark(el.classList.contains("dark"));
    update();
    const observer = new MutationObserver(update);
    observer.observe(el, { attributes: true, attributeFilter: ["class"] });
    return () => observer.disconnect();
  }, []);
  return isDark ? "monokai" : "rjv-default";
}
// #endregion 🎨 RJV Theme

let evSeq = 0;
function nextEvId() {
  evSeq += 1;
  return `ev-${evSeq}`;
}

export interface LastChangeResult {
  readonly forward: unknown;
  readonly result: unknown;
  readonly error?: string;
  readonly mode: "changeKit" | "readKit" | "execute" | "log";
}

export function useKitStore(seedKit: unknown) {
  const [ready, setReady] = React.useState(false);
  const [initErr, setInitErr] = React.useState<string | null>(null);
  const [handle, setHandle] = React.useState<KitStoreHandle | null>(null);
  const [events, setEvents] = React.useState<readonly LoggedEvent[]>([]);
  const [filter, setFilter] = React.useState("");
  const [last, setLast] = React.useState<LastChangeResult | null>(null);
  const [matAt, setMatAt] = React.useState("");

  const [sessionId, setSessionId] = React.useState("");
  const [draftId, setDraftId] = React.useState("");
  const [txId, setTxId] = React.useState("");
  const [cpId, setCpId] = React.useState("");
  const [altId, setAltId] = React.useState("");
  const [msg, setMsg] = React.useState("checkpoint (story)");

  const [cmdMode, setCmdMode] = React.useState<"changeKit" | "readKit" | "execute">("changeKit");
  const [changeJson, setChangeJson] = React.useState(`{ "name": "Kit (story edit)" }`);
  const [readJson, setReadJson] = React.useState(`{ "name": null }`);
  const [executeJson, setExecuteJson] = React.useState(`{ "readKitCommands": { "commands": [ { "name": null } ] } }`);

  const pushEvent = React.useCallback((payload: unknown) => {
    setEvents((prev) => [...prev, { id: nextEvId(), t: Date.now(), payload }]);
  }, []);

  const log = React.useCallback(
    (line: string) => {
      pushEvent({ log: line });
      setLast({ forward: null, result: { log: line }, mode: "log" });
    },
    [pushEvent],
  );

  React.useEffect(() => {
    let cancelled = false;
    setInitErr(null);
    setReady(false);
    void (async () => {
      try {
        await ensureSemioWasm();
        const h = KitStoreHandle.create(seedKit);
        if (cancelled) {
          h.free();
          return;
        }
        setHandle(h);
        setReady(true);
      } catch (e) {
        if (!cancelled) setInitErr(e instanceof Error ? e.message : String(e));
      }
    })();
    return () => {
      cancelled = true;
      setHandle((prev: KitStoreHandle | null) => {
        try {
          prev?.free();
        } catch {
          /* ignore */
        }
        return null;
      });
    };
  }, [seedKit]);

  React.useEffect(() => {
    if (!handle) return;
    const cb = (payload: unknown) => {
      setEvents((prev) => [...prev, { id: nextEvId(), t: Date.now(), payload }]);
    };
    handle.subscribe(cb);
  }, [handle]);

  const onCommandRun = React.useCallback(
    (o: { mode: "changeKit" | "readKit" | "execute"; forward: unknown; result?: unknown; error?: string; log: string }) => {
      pushEvent({ command: o.mode, log: o.log, forward: o.forward, result: o.result, error: o.error });
      setLast(
        o.error
          ? { forward: o.forward, result: o.result, error: o.error, mode: o.mode }
          : { forward: o.forward, result: o.result, mode: o.mode },
      );
    },
    [pushEvent],
  );

  return {
    ready,
    initErr,
    handle,
    events,
    filter,
    setFilter,
    onClear: () => setEvents([]),
    last,
    matAt,
    setMatAt,
    sessionId,
    setSessionId,
    draftId,
    setDraftId,
    txId,
    setTxId,
    cpId,
    setCpId,
    altId,
    setAltId,
    msg,
    setMsg,
    cmdMode,
    setCmdMode,
    changeJson,
    setChangeJson,
    readJson,
    setReadJson,
    executeJson,
    setExecuteJson,
    log,
    onCommandRun,
  };
}
