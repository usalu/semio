export type LayoutDropPreview = { kind: string; x: number; y: number };
/** 🛠️ A Blueprint window's in-flight transform-tool gesture between dispatches — ephemeral tool state, never history. */
export type LayoutTransformToolState = {
  states: string[];
  verb: "translateSelection" | "rotateSelection" | "scaleSelection";
  authoringSeed: string;
  baseRevision: string;
  transaction: { id: string; tool: string };
  entries: { key: string; mutation: Record<string, unknown> }[];
};
export type LayoutWindowTransient = { dropPreview: LayoutDropPreview; engagementInput: string; transformTool?: LayoutTransformToolState | null };
export type LayoutWindowTransientMutation = { kind: "snapshot"; transient: LayoutWindowTransient };

export function parseLayoutWindowTransient(value: unknown): LayoutWindowTransient {
  if (!value || typeof value !== "object") throw new Error("layout-window-transient-object-required");
  const record = value as Record<string, unknown>;
  const preview = record.dropPreview as Record<string, unknown> | undefined;
  if (!preview || typeof preview.kind !== "string" || typeof preview.x !== "number" || typeof preview.y !== "number" || typeof record.engagementInput !== "string") {
    throw new Error("layout-window-transient-invalid");
  }
  if (record.transformTool !== undefined && record.transformTool !== null) parseLayoutTransformToolState(record.transformTool);
  return value as LayoutWindowTransient;
}

/** 🛠️ Parses a persisted transform-tool gesture: stable-id configuration, one of the three gumball verbs, the open transaction. */
export function parseLayoutTransformToolState(value: unknown): LayoutTransformToolState {
  if (!value || typeof value !== "object") throw new Error("layout-transform-tool-object-required");
  const tool = value as Record<string, unknown>;
  const transaction = tool.transaction as Record<string, unknown> | undefined;
  const strings = (items: unknown): boolean => Array.isArray(items) && items.every((item) => typeof item === "string" && item.length > 0);
  const entries = Array.isArray(tool.entries) && tool.entries.every((entry) => entry && typeof entry === "object" && typeof (entry as Record<string, unknown>).key === "string" && typeof (entry as Record<string, unknown>).mutation === "object");
  if (!strings(tool.states) || !["translateSelection", "rotateSelection", "scaleSelection"].includes(tool.verb as string) || typeof tool.authoringSeed !== "string" || typeof tool.baseRevision !== "string" || !transaction || typeof transaction.id !== "string" || typeof transaction.tool !== "string" || !entries) {
    throw new Error("layout-transform-tool-invalid");
  }
  return value as LayoutTransformToolState;
}

export function applyLayoutWindowTransientMutation(_base: LayoutWindowTransient, mutation: LayoutWindowTransientMutation): LayoutWindowTransient {
  if (mutation.kind !== "snapshot") throw new Error("layout-window-transient-mutation-invalid");
  return parseLayoutWindowTransient(mutation.transient);
}
