// #region 🧲Header
// semio-algorithms: last executeChangeKitCommands result (kind + inverse)
// #endregion

import * as React from "react";

export const DiffViewer: React.FC<{
  last: { forward: unknown; result: unknown; error?: string } | null;
}> = ({ last }) => {
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
            <pre className="bg-muted/30 m-0 max-h-40 overflow-auto rounded p-1 wrap-break-word whitespace-pre-wrap">{safeStr(last.forward)}</pre>
          </div>
          <div>
            <div className="text-muted-foreground">result (kind + inverse)</div>
            <pre className="bg-muted/30 m-0 max-h-40 overflow-auto rounded p-1 wrap-break-word whitespace-pre-wrap">{safeStr(last.result)}</pre>
          </div>
        </div>
      )}
    </div>
  );
};

function safeStr(v: unknown): string {
  try {
    return JSON.stringify(v, null, 2);
  } catch {
    return String(v);
  }
}
