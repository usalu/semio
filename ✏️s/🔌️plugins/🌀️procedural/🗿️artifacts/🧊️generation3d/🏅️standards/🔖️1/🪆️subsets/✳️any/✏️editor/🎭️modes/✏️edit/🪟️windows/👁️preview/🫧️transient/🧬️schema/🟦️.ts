/** 🫧️ Computed evaluation owned by one exact `procedural-preview` window. */
export interface Generation3dPreviewWindowTransient {
  previewEvalText?: string | null;
}

/** 🚪️ Strict production parser for the language-neutral window-transient facet. */
export function parseGeneration3dPreviewWindowTransient(value: unknown): Generation3dPreviewWindowTransient {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("Generation3d preview window transient must be an object");
  const record = value as Record<string, unknown>;
  for (const key of Object.keys(record)) if (key !== "previewEvalText") throw new Error(`Unknown Generation3d preview window transient field ${key}`);
  const previewEvalText = record.previewEvalText;
  if (previewEvalText !== undefined && previewEvalText !== null && typeof previewEvalText !== "string") throw new Error("previewEvalText must be a string or null");
  return previewEvalText === undefined ? {} : { previewEvalText };
}
