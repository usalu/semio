export type LayoutDropPreview = { kind: string; x: number; y: number };
export type LayoutWindowTransient = { dropPreview: LayoutDropPreview; engagementInput: string };
export type LayoutWindowTransientMutation = { kind: "snapshot"; transient: LayoutWindowTransient };

export function parseLayoutWindowTransient(value: unknown): LayoutWindowTransient {
  if (!value || typeof value !== "object") throw new Error("layout-window-transient-object-required");
  const record = value as Record<string, unknown>;
  const preview = record.dropPreview as Record<string, unknown> | undefined;
  if (!preview || typeof preview.kind !== "string" || typeof preview.x !== "number" || typeof preview.y !== "number" || typeof record.engagementInput !== "string") {
    throw new Error("layout-window-transient-invalid");
  }
  return value as LayoutWindowTransient;
}

export function applyLayoutWindowTransientMutation(_base: LayoutWindowTransient, mutation: LayoutWindowTransientMutation): LayoutWindowTransient {
  if (mutation.kind !== "snapshot") throw new Error("layout-window-transient-mutation-invalid");
  return parseLayoutWindowTransient(mutation.transient);
}
