export type SequenceScriptWindowTransient = { lastRunJson: string };
export type SequenceScriptWindowTransientMutation = { kind: "snapshot"; transient: SequenceScriptWindowTransient };

export function parseSequenceScriptWindowTransient(value: unknown): SequenceScriptWindowTransient {
  if (!value || typeof value !== "object" || typeof (value as Record<string, unknown>).lastRunJson !== "string") {
    throw new Error("sequence-script-window-transient-invalid");
  }
  return value as SequenceScriptWindowTransient;
}

export function applySequenceScriptWindowTransientMutation(_base: SequenceScriptWindowTransient, mutation: SequenceScriptWindowTransientMutation): SequenceScriptWindowTransient {
  if (mutation.kind !== "snapshot") throw new Error("sequence-script-window-transient-mutation-invalid");
  return parseSequenceScriptWindowTransient(mutation.transient);
}
