export interface WriterEditorSelection { start: number; end: number }
export interface WriterMainWindowTransient { editorSelection: WriterEditorSelection | null; lintGeneration: number; engagementInput: string }

export function parseWriterMainWindowTransient(value: unknown): WriterMainWindowTransient {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new TypeError("WriterMainWindowTransient must be an object");
  const row = value as Record<string, unknown>;
  if (!Number.isInteger(row.lintGeneration) || (row.lintGeneration as number) < 0 || typeof row.engagementInput !== "string") throw new TypeError("WriterMainWindowTransient scalar fields are invalid");
  const selection = row.editorSelection;
  if (selection !== null && (typeof selection !== "object" || selection === null || !Number.isInteger((selection as Record<string, unknown>).start) || !Number.isInteger((selection as Record<string, unknown>).end))) throw new TypeError("WriterMainWindowTransient editorSelection is invalid");
  return value as WriterMainWindowTransient;
}
