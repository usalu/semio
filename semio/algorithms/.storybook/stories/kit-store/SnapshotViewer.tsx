// #region 🧲Header
// semio-algorithms: snapshot() vs theKitDto() vs materializeAt
// #endregion

import ReactJson from "@microlink/react-json-view";
import * as React from "react";

import type { KitStoreHandle } from "./semioWasm";
import { useRjvTheme } from "./useKitStore";

type Tab = "live" | "theKit" | "mat" | "vcs";

export const SnapshotViewer: React.FC<{
  handle: KitStoreHandle | null;
  matAt: string;
  onMatAt: (s: string) => void;
}> = ({ handle, matAt, onMatAt }) => {
  const [tab, setTab] = React.useState<Tab>("live");
  const [value, setValue] = React.useState<unknown>({});
  const [errorText, setErrorText] = React.useState<string | null>(null);
  const theme = useRjvTheme();

  const load = React.useCallback(() => {
    if (!handle) {
      setValue({});
      setErrorText(null);
      return;
    }
    try {
      if (tab === "live") setValue(handle.snapshot());
      else if (tab === "theKit") setValue(handle.theKitDto());
      else if (tab === "mat") {
        const at = matAt.trim();
        setValue(handle.materializeAt(at.length ? at : null));
      } else setValue(handle.vcsState());
      setErrorText(null);
    } catch (e) {
      setErrorText(e instanceof Error ? e.message : String(e));
      setValue({});
    }
  }, [handle, tab, matAt]);

  React.useEffect(() => {
    load();
  }, [load]);

  const srcObject = value && typeof value === "object" ? (value as object) : { value };
  const rootName = value && typeof value === "object" ? false : "value";

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
          onClick={load}
        >
          refresh
        </button>
      </div>
      {tab === "mat" ? (
        <div className="space-y-1">
          <p className="text-muted-foreground m-0 text-[10px] leading-snug">
            Read-only: materialized <code className="bg-muted-foreground/10 rounded px-0.5">KitFullDto</code> at the checkpoint (or initial when empty). Does not change the live store.
            Use <span className="text-foreground font-medium">VCS → Preview @ cp</span> to jump here from a selected checkpoint.
          </p>
          <label className="text-muted-foreground flex items-center gap-1 text-[10px]">
            at (checkpoint id, empty = initial only)
            <input
              className="bg-background flex-1 rounded border border-zinc-300 px-1 py-0.5 font-mono dark:border-zinc-600"
              value={matAt}
              onChange={(e) => onMatAt(e.target.value)}
            />
          </label>
        </div>
      ) : null}
      {errorText ? (
        <pre className="text-destructive m-0 max-h-24 overflow-auto font-mono text-[10px] wrap-break-word whitespace-pre-wrap">{errorText}</pre>
      ) : null}
      <div className="m-0 min-h-0 flex-1 overflow-auto rounded border border-zinc-200 bg-zinc-50 p-1 dark:border-zinc-800 dark:bg-zinc-950">
        <ReactJson
          src={srcObject}
          name={rootName as false | string}
          theme={theme}
          iconStyle="triangle"
          indentWidth={2}
          collapsed={2}
          displayDataTypes={false}
          displayObjectSize={false}
          enableClipboard
          style={{ background: "transparent", fontSize: "9px" }}
        />
      </div>
    </div>
  );
};
