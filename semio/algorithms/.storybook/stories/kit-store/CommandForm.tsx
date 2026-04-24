// #region 🧲Header
// Change commands / read commands / raw execute JSON
// #endregion

import * as React from "react";

import { kitGraphqlExecuteRead, kitGraphqlExecuteStoreCommand, type KitGraphqlHandle } from "@semio/js";
import { ALL_CHANGE_KIT_ROOT_KEYS, ALL_READ_KIT_COMMAND_KEYS, CHANGE_KIT_PRESETS, READ_KIT_PRESETS } from "./commandSchema";
import type { KitStoreHandle } from "./semioWasm";

type Mode = "changeKit" | "readKit" | "execute";

export const CommandForm: React.FC<{
  handle: KitStoreHandle | null;
  mode: Mode;
  onMode: (m: Mode) => void;
  changeJson: string;
  onChangeJson: (s: string) => void;
  readJson: string;
  onReadJson: (s: string) => void;
  executeJson: string;
  onExecuteJson: (s: string) => void;
  onCommandRun: (o: { mode: Mode; forward: unknown; result?: unknown; error?: string; log: string }) => void;
}> = ({ handle, mode, onMode, changeJson, onChangeJson, readJson, onReadJson, executeJson, onExecuteJson, onCommandRun }) => {
  const area = mode === "changeKit" ? changeJson : mode === "readKit" ? readJson : executeJson;
  const setArea = mode === "changeKit" ? onChangeJson : mode === "readKit" ? onReadJson : onExecuteJson;

  return (
    <div className="text-foreground flex h-full min-h-0 flex-col gap-1 p-2 text-xs">
      <div className="flex flex-wrap items-center gap-1">
        {(["changeKit", "readKit", "execute"] as const).map((m) => (
          <button
            key={m}
            type="button"
            className={
              "rounded border px-1.5 py-0.5 text-[10px] " + (mode === m ? "border-violet-600 bg-violet-100 dark:bg-violet-950" : "border-zinc-300 dark:border-zinc-600")
            }
            onClick={() => onMode(m)}
          >
            {m}
          </button>
        ))}
        <span className="text-muted-foreground ml-auto text-[10px]">
          {ALL_CHANGE_KIT_ROOT_KEYS.length} ch · {ALL_READ_KIT_COMMAND_KEYS.length} read
        </span>
      </div>

      {mode === "changeKit" ? (
        <div className="flex flex-wrap gap-1">
          <select
            className="bg-background max-w-[14rem] rounded border border-zinc-300 px-1 py-0.5 text-[10px] dark:border-zinc-600"
            onChange={(e) => {
              const p = CHANGE_KIT_PRESETS.find((x) => x.id === e.target.value);
              if (p) onChangeJson(p.json);
              e.target.value = "";
            }}
            defaultValue=""
          >
            <option value="">preset…</option>
            {CHANGE_KIT_PRESETS.map((p) => (
              <option key={p.id} value={p.id}>
                {p.label}
              </option>
            ))}
          </select>
        </div>
      ) : null}
      {mode === "readKit" ? (
        <div className="flex flex-wrap gap-1">
          <select
            className="bg-background max-w-[14rem] rounded border border-zinc-300 px-1 py-0.5 text-[10px] dark:border-zinc-600"
            onChange={(e) => {
              const p = READ_KIT_PRESETS.find((x) => x.id === e.target.value);
              if (p) onReadJson(p.json);
              e.target.value = "";
            }}
            defaultValue=""
          >
            <option value="">read preset…</option>
            {READ_KIT_PRESETS.map((p) => (
              <option key={p.id} value={p.id}>
                {p.label}
              </option>
            ))}
          </select>
        </div>
      ) : null}

      <textarea
        className="bg-background min-h-[8rem] flex-1 resize-y rounded border border-zinc-300 p-1 font-mono text-[10px] leading-tight dark:border-zinc-600"
        spellCheck={false}
        value={area}
        onChange={(e) => setArea(e.target.value)}
      />
      <button
        type="button"
        className="rounded border border-violet-600 bg-violet-100 px-2 py-1 text-[11px] font-medium dark:bg-violet-950"
        disabled={!handle}
        onClick={() => {
          void (async () => {
            if (!handle) return;
            const gql: KitGraphqlHandle = {
              execute: (requestJson, onMessage) => handle.execute(requestJson, onMessage),
            };
            try {
              if (mode === "changeKit") {
                const parsed = JSON.parse(changeJson);
                const arr = Array.isArray(parsed) ? parsed : [parsed];
                const r = await handle.executeChangeKitCommands(arr);
                onCommandRun({ mode, forward: arr, result: r, log: `executeChangeKitCommands → ${JSON.stringify(r)}` });
              } else if (mode === "readKit") {
                const cmds = JSON.parse(readJson);
                const arr = Array.isArray(cmds) ? cmds : [cmds];
                const r = await kitGraphqlExecuteRead(gql, arr);
                onCommandRun({ mode, forward: arr, result: r, log: `readKitCommands → ${JSON.stringify(r)}` });
              } else {
                const raw = JSON.parse(executeJson);
                const r = await kitGraphqlExecuteStoreCommand(gql, raw);
                onCommandRun({ mode, forward: raw, result: r, log: `kitStoreExecute → ${JSON.stringify(r)}` });
              }
            } catch (e) {
              const err = e instanceof Error ? e.message : String(e);
              onCommandRun({ mode, forward: null, error: err, log: `ERROR: ${err}` });
            }
          })();
        }}
      >
        Run
      </button>
    </div>
  );
};
