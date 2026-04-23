// #region 🧲Header
// semio-algorithms: Kit / Store story — event log from KitStoreHandle.subscribe
// #endregion

import ReactJson from "@microlink/react-json-view";
import * as React from "react";

import { useRjvTheme } from "./useKitStore";

export interface LoggedEvent {
  readonly id: string;
  readonly t: number;
  readonly payload: unknown;
}

const fmtTime = (t: number) => new Date(t).toISOString().split("T")[1]!.slice(0, 12);

export const EventsFeed: React.FC<{
  events: readonly LoggedEvent[];
  onClear: () => void;
  filter: string;
  onFilterChange: (v: string) => void;
}> = ({ events, onClear, filter, onFilterChange }) => {
  const f = filter.trim().toLowerCase();
  const rows = f ? events.filter((e) => JSON.stringify(e.payload).toLowerCase().includes(f)) : events;
  const theme = useRjvTheme();

  return (
    <div className="text-foreground flex h-full min-h-0 flex-col gap-2 border-t border-zinc-200 p-2 text-xs dark:border-zinc-800">
      <div className="text-muted-foreground flex shrink-0 items-center justify-between gap-2">
        <span className="font-medium">Events ({rows.length})</span>
        <div className="flex items-center gap-1">
          <input
            className="bg-background w-32 rounded border border-zinc-300 px-1 py-0.5 text-[10px] dark:border-zinc-600"
            placeholder="filter…"
            value={filter}
            onChange={(e) => onFilterChange(e.target.value)}
          />
          <button type="button" className="rounded border border-zinc-300 px-1.5 py-0.5 dark:border-zinc-600" onClick={onClear}>
            clear
          </button>
        </div>
      </div>
      <ul className="min-h-0 flex-1 list-none space-y-1 overflow-auto p-0 font-mono text-[10px]">
        {rows.map((e) => (
          <li
            key={e.id}
            className="border-b border-zinc-100 py-0.5 dark:border-zinc-900"
            style={{ color: isErr(e.payload) ? "var(--destructive, #b91c1c)" : undefined }}
          >
            <div className="text-muted-foreground">
              {fmtTime(e.t)} {eventKind(e.payload)}
            </div>
            <div className="m-0 max-h-24 overflow-auto">
              <RjvPayload payload={e.payload} theme={theme} />
            </div>
          </li>
        ))}
        {rows.length === 0 ? <li className="text-muted-foreground">(no events)</li> : null}
      </ul>
    </div>
  );
};

// #region 🔖 RjvPayload
// Renders a single event payload with @microlink/react-json-view.
// Non-object payloads are wrapped into `{ value: ... }` because rjv requires an object root.
const RjvPayload: React.FC<{ payload: unknown; theme: "rjv-default" | "monokai" }> = ({ payload, theme }) => {
  const src = payload && typeof payload === "object" ? (payload as object) : { value: payload };
  const name = payload && typeof payload === "object" ? false : "value";
  return (
    <ReactJson
      src={src}
      name={name as false | string}
      theme={theme}
      iconStyle="triangle"
      indentWidth={2}
      collapsed={1}
      displayDataTypes={false}
      displayObjectSize={false}
      enableClipboard={false}
      style={{ background: "transparent", fontSize: "10px" }}
    />
  );
};
// #endregion 🔖 RjvPayload

function isErr(p: unknown): boolean {
  if (p == null) return false;
  const s = JSON.stringify(p);
  return s.includes("InvalidOperation") || s.includes("not yet") || s.includes("error");
}

function eventKind(p: unknown): string {
  if (p && typeof p === "object" && "log" in p) return "log";
  if (p && typeof p === "object" && "field" in p) return "field";
  if (p && typeof p === "object" && "SetRejected" in (p as object)) return "reject";
  return "event";
}
