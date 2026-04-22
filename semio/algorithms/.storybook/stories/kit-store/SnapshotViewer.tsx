// #region 🧲Header
// semio-algorithms: snapshot() vs theKitDto() vs materializeAt
// #endregion

import * as React from "react";

import type { KitStoreHandle } from "./semioWasm";

type Tab = "live" | "theKit" | "mat" | "vcs";

export const SnapshotViewer: React.FC<{
  handle: KitStoreHandle | null;
  matAt: string;
  onMatAt: (s: string) => void;
}> = ({ handle, matAt, onMatAt }) => {
  const [tab, setTab] = React.useState<Tab>("live");
  const [json, setJson] = React.useState<string>("{}");

  React.useEffect(() => {
    if (!handle) {
      setJson("{}");
      return;
    }
    try {
      if (tab === "live") {
        setJson(JSON.stringify(handle.snapshot(), null, 2));
      } else if (tab === "theKit") {
        setJson(JSON.stringify(handle.theKitDto(), null, 2));
      } else if (tab === "mat") {
        const at = matAt.trim();
        setJson(JSON.stringify(handle.materializeAt(at.length ? at : null), null, 2));
      } else {
        setJson(JSON.stringify(handle.vcsState(), null, 2));
      }
    } catch (e) {
      setJson(String(e));
    }
  }, [handle, tab, matAt]);

  return (
    <div className="text-foreground flex h-full min-h-0 flex-col gap-1 p-2 text-xs">
      <div className="flex flex-wrap items-center gap-1">
        {(
          [
            ["live", "live snapshot()"],
            ["theKit", "theKitDto()"],
            ["mat", "materializeAt"],
            ["vcs", "vcsState()"],
          ] as const
        ).map(([k, lab]) => (
          <button
            key={k}
            type="button"
            className={
              "rounded border px-1.5 py-0.5 text-[10px] " + (tab === k ? "border-cyan-600 bg-cyan-100 dark:bg-cyan-950" : "border-zinc-300 dark:border-zinc-600")
            }
            onClick={() => setTab(k)}
          >
            {lab}
          </button>
        ))}
        <button
          type="button"
          className="ml-auto border border-zinc-300 px-1.5 py-0.5 text-[10px] dark:border-zinc-600"
          onClick={() => {
            if (handle) {
              if (tab === "live") setJson(JSON.stringify(handle.snapshot(), null, 2));
              else if (tab === "theKit") setJson(JSON.stringify(handle.theKitDto(), null, 2));
              else if (tab === "mat") {
                const at = matAt.trim();
                setJson(JSON.stringify(handle.materializeAt(at.length ? at : null), null, 2));
              } else setJson(JSON.stringify(handle.vcsState(), null, 2));
            }
          }}
        >
          refresh
        </button>
      </div>
      {tab === "mat" ? (
        <label className="text-muted-foreground flex items-center gap-1 text-[10px]">
          at (checkpoint id, empty=initial)
          <input
            className="bg-background flex-1 rounded border border-zinc-300 px-1 py-0.5 font-mono dark:border-zinc-600"
            value={matAt}
            onChange={(e) => onMatAt(e.target.value)}
          />
        </label>
      ) : null}
      <pre className="m-0 min-h-0 flex-1 overflow-auto rounded border border-zinc-200 bg-zinc-50 p-1 font-mono text-[9px] dark:border-zinc-800 dark:bg-zinc-950">{json}</pre>
    </div>
  );
};
