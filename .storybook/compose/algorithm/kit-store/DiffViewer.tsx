// #region 🧲Header
// compose-algorithms: last executeChangeKitCommands result (kind + inverse)
// #endregion

import ReactJson from "@microlink/react-json-view";
import * as React from "react";

import { useRjvTheme } from "./useKitStore";

export const DiffViewer: React.FC<{
  last: { forward: unknown; result: unknown; error?: string } | null;
}> = ({ last }) => {
  const theme = useRjvTheme();
  return (
    <div className="text-foreground flex h-full min-h-0 flex-col gap-1 border-t border-zinc-200 p-2 text-xs dark:border-zinc-800">
      <div className="text-muted-foreground font-medium">Last change / inverse</div>
      {!last ? (
        <div className="text-muted-foreground">(no commands run yet)</div>
      ) : (
        <div className="min-h-0 flex-1 space-y-2 overflow-auto font-mono text-[10px]">
          {last.error ? (
            <pre className="text-destructive m-0 wrap-break-word whitespace-pre-wrap">{last.error}</pre>
          ) : null}
          <div>
            <div className="text-muted-foreground">forward</div>
            <div className="bg-muted/30 m-0 max-h-40 overflow-auto rounded p-1">
              <RjvValue value={last.forward} theme={theme} />
            </div>
          </div>
          <div>
            <div className="text-muted-foreground">result (kind + inverse)</div>
            <div className="bg-muted/30 m-0 max-h-40 overflow-auto rounded p-1">
              <RjvValue value={last.result} theme={theme} />
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

// #region 🔖 RjvValue
// Renders any JSON-serialisable value with @microlink/react-json-view.
// Wraps primitives into `{ value: ... }` because rjv requires an `object` root.
const RjvValue: React.FC<{ value: unknown; theme: "rjv-default" | "monokai" }> = ({ value, theme }) => {
  const src = value && typeof value === "object" ? (value as object) : { value };
  const name = value && typeof value === "object" ? false : "value";
  return (
    <ReactJson
      src={src}
      name={name as false | string}
      theme={theme}
      iconStyle="triangle"
      indentWidth={2}
      collapsed={2}
      displayDataTypes={false}
      displayObjectSize={false}
      enableClipboard={false}
      style={{ background: "transparent", fontSize: "10px" }}
    />
  );
};
// #endregion 🔖 RjvValue
