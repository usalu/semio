// #region 🔖Header
// 💻 semio/algorithms/index.ts
// Specs: Exposes a runtime-safe Storybook shell for the algorithms bundle without depending on unstable semio/ui or semio/js package roots.
// Summary: Lightweight algorithm Storybook runtime with level helpers, window kinds, and a local AlgorithmApp shell.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import * as React from "react";

export type Level = "base" | "window" | "panel" | "overlay" | "temporary";

const LevelContext = React.createContext<Level>("base");

export const LevelProvider: React.FC<{ level: Level; children: React.ReactNode }> = ({ level, children }) =>
  React.createElement(LevelContext.Provider, { value: level }, children);

export function getLevelBgClass(level: Level): string {
  switch (level) {
    case "window":
      return "bg-card";
    case "panel":
      return "bg-muted/40";
    case "overlay":
      return "bg-accent/10";
    case "temporary":
      return "bg-warning/10";
    case "base":
    default:
      return "bg-background";
  }
}

export enum WindowKind {
  VEC_INPUT = "VEC_INPUT",
  PIECES_SELECTION_INPUT = "PIECES_SELECTION_INPUT",
  DESIGN_INPUT = "DESIGN_INPUT",
  DESIGN_DIFF_OUTPUT = "DESIGN_DIFF_OUTPUT",
  DESIGN_OUTPUT = "DESIGN_OUTPUT",
}

export interface VecValue {
  u: number;
  v: number;
}

export interface AlgorithmContextValue {
  kit: any;
  designGuid: string;
  vec?: VecValue;
  onVecChange?: (value: VecValue) => void;
  vecMin?: VecValue;
  vecMax?: VecValue;
  selectedPieceGuids: string[];
  onSelectedPieceGuidsChange?: (guids: string[]) => void;
  designDiff?: any;
  diffKit?: any;
  outputKit: any;
  outputDesignGuid: string;
  error?: string;
}

export interface AlgorithmWindowDef {
  id: string;
  kind: WindowKind;
  label?: string;
}

export interface AlgorithmAppProps {
  id: string;
  label: string;
  windows: AlgorithmWindowDef[];
  defaultLayout?: unknown;
  context: AlgorithmContextValue;
  className?: string;
}

const AlgorithmContext = React.createContext<AlgorithmContextValue | null>(null);

export function useAlgorithm(): AlgorithmContextValue {
  const value = React.useContext(AlgorithmContext);
  if (!value) throw new Error("useAlgorithm must be used within AlgorithmApp");
  return value;
}

function findDesign(kit: any, designGuid: string): any {
  return (kit?.designs ?? []).find((design: any) => design.guid === designGuid) ?? null;
}

function renderDesignSummary(kit: any, designGuid: string, designDiff?: any): React.ReactNode {
  const design = findDesign(kit, designGuid);
  return React.createElement(
    "div",
    { className: "space-y-3" },
    React.createElement(
      "div",
      { className: "rounded-md border border-border/60 bg-background/70 p-3" },
      React.createElement("div", { className: "text-sm font-semibold" }, design?.name ?? designGuid),
      React.createElement(
        "div",
        { className: "mt-2 grid grid-cols-2 gap-2 text-xs text-muted-foreground" },
        React.createElement("div", null, `pieces: ${design?.pieces?.length ?? 0}`),
        React.createElement("div", null, `connections: ${design?.connections?.length ?? 0}`),
      ),
    ),
    designDiff
      ? React.createElement(
          "div",
          { className: "rounded-md border border-border/60 bg-background/70 p-3 text-xs" },
          React.createElement("div", { className: "font-semibold text-foreground" }, "Diff"),
          React.createElement(
            "div",
            { className: "mt-2 grid grid-cols-2 gap-2 text-muted-foreground" },
            React.createElement("div", null, `pieces added: ${designDiff?.pieces?.added?.length ?? 0}`),
            React.createElement("div", null, `pieces removed: ${designDiff?.pieces?.removed?.length ?? 0}`),
            React.createElement("div", null, `pieces updated: ${designDiff?.pieces?.updated?.length ?? 0}`),
            React.createElement("div", null, `connections updated: ${designDiff?.connections?.updated?.length ?? 0}`),
          ),
        )
      : null,
  );
}

const AlgorithmVecInputWindow: React.FC = () => {
  const { vec, onVecChange, vecMin, vecMax } = useAlgorithm();
  if (!vec || !onVecChange) return React.createElement("div", { className: "text-sm text-muted-foreground" }, "No vector input.");
  return React.createElement(
    "div",
    { className: "grid gap-3" },
    React.createElement(
      "label",
      { className: "grid gap-1 text-xs" },
      React.createElement("span", { className: "text-muted-foreground" }, "u"),
      React.createElement("input", {
        className: "rounded-md border border-border bg-background px-2 py-1 text-sm",
        type: "number",
        step: "0.1",
        min: vecMin?.u ?? -10,
        max: vecMax?.u ?? 10,
        value: vec.u,
        onChange: (event: React.ChangeEvent<HTMLInputElement>) => onVecChange({ ...vec, u: Number(event.target.value) }),
      }),
    ),
    React.createElement(
      "label",
      { className: "grid gap-1 text-xs" },
      React.createElement("span", { className: "text-muted-foreground" }, "v"),
      React.createElement("input", {
        className: "rounded-md border border-border bg-background px-2 py-1 text-sm",
        type: "number",
        step: "0.1",
        min: vecMin?.v ?? -10,
        max: vecMax?.v ?? 10,
        value: vec.v,
        onChange: (event: React.ChangeEvent<HTMLInputElement>) => onVecChange({ ...vec, v: Number(event.target.value) }),
      }),
    ),
  );
};

const AlgorithmPiecesSelectionInputWindow: React.FC = () => {
  const { kit, designGuid, selectedPieceGuids, onSelectedPieceGuidsChange } = useAlgorithm();
  const design = findDesign(kit, designGuid);
  const selectedSet = new Set(selectedPieceGuids);
  return React.createElement(
    "div",
    { className: "space-y-2" },
    ...(design?.pieces ?? []).map((piece: any) =>
      React.createElement(
        "label",
        { className: "flex items-center gap-2 rounded-md border border-border/50 px-2 py-1.5 text-sm", key: piece.guid },
        React.createElement("input", {
          type: "checkbox",
          checked: selectedSet.has(piece.guid),
          onChange: (event: React.ChangeEvent<HTMLInputElement>) => {
            const next = event.target.checked ? [...selectedPieceGuids, piece.guid] : selectedPieceGuids.filter((guid) => guid !== piece.guid);
            onSelectedPieceGuidsChange?.(next);
          },
        }),
        React.createElement("span", { className: "truncate" }, piece.name ?? piece.guid),
      ),
    ),
  );
};

const AlgorithmDesignInputWindow: React.FC = () => {
  const { kit, designGuid } = useAlgorithm();
  return renderDesignSummary(kit, designGuid);
};

const AlgorithmDesignDiffOutputWindow: React.FC = () => {
  const { kit, diffKit, designGuid, designDiff, error } = useAlgorithm();
  if (error) return React.createElement("div", { className: "text-sm text-destructive" }, error);
  return renderDesignSummary(diffKit ?? kit, designGuid, designDiff);
};

const AlgorithmDesignOutputWindow: React.FC = () => {
  const { outputKit, outputDesignGuid } = useAlgorithm();
  return renderDesignSummary(outputKit, outputDesignGuid);
};

const WINDOW_COMPONENTS: Record<WindowKind, React.ComponentType> = {
  [WindowKind.VEC_INPUT]: AlgorithmVecInputWindow,
  [WindowKind.PIECES_SELECTION_INPUT]: AlgorithmPiecesSelectionInputWindow,
  [WindowKind.DESIGN_INPUT]: AlgorithmDesignInputWindow,
  [WindowKind.DESIGN_DIFF_OUTPUT]: AlgorithmDesignDiffOutputWindow,
  [WindowKind.DESIGN_OUTPUT]: AlgorithmDesignOutputWindow,
};

export const AlgorithmApp: React.FC<AlgorithmAppProps> = ({ id, label, windows, context, className }) =>
  React.createElement(
    AlgorithmContext.Provider,
    { value: context },
    React.createElement(
      LevelProvider,
      { level: "base" },
      React.createElement(
        "div",
        { className: className ?? "h-full w-full" },
        React.createElement(
          "div",
          { className: "flex h-full min-h-[640px] w-full flex-col overflow-hidden rounded-xl border border-border bg-background text-foreground shadow-sm", id },
          React.createElement(
            "header",
            { className: "border-b border-border px-4 py-3" },
            React.createElement("div", { className: "text-sm font-semibold" }, label),
          ),
          React.createElement(
            "div",
            { className: "grid min-h-0 flex-1 grid-cols-1 gap-3 p-3 lg:grid-cols-[minmax(0,1fr)_280px]" },
            React.createElement(
              "div",
              { className: "grid min-h-0 gap-3 md:grid-cols-2 xl:grid-cols-3" },
              ...windows.map((windowDef) =>
                React.createElement(
                  "section",
                  { className: "flex min-h-[220px] flex-col overflow-hidden rounded-lg border border-border bg-card", key: `${id}.${windowDef.id}` },
                  React.createElement("header", { className: "border-b border-border/60 px-3 py-2 text-xs font-semibold uppercase tracking-wide text-muted-foreground" }, windowDef.label ?? windowDef.id),
                  React.createElement("div", { className: "min-h-0 flex-1 overflow-auto p-3" }, React.createElement(WINDOW_COMPONENTS[windowDef.kind])),
                ),
              ),
            ),
            React.createElement(
              "aside",
              { className: "overflow-auto rounded-lg border border-border bg-card p-3" },
              React.createElement("div", { className: "text-xs font-semibold uppercase tracking-wide text-muted-foreground" }, "Details"),
              React.createElement("div", { className: "mt-3 text-sm" }, `${context.selectedPieceGuids.length} selected pieces`),
            ),
          ),
        ),
      ),
    ),
  );
