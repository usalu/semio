import type { Generation3dPreviewWindowTransient } from "../../🧬️schema/🟦️.ts";

/** 🚪️ Strict production parser for the language-neutral window-transient facet. */
export function parseGeneration3dPreviewWindowTransient(value: unknown): Generation3dPreviewWindowTransient {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("Generation3d preview window transient must be an object");
  const record = value as Record<string, unknown>;
  for (const key of Object.keys(record)) if (key !== "previewEvalText" && key !== "generationId") throw new Error(`Unknown Generation3d preview window transient field ${key}`);
  const previewEvalText = record.previewEvalText;
  if (previewEvalText !== undefined && previewEvalText !== null && typeof previewEvalText !== "string") throw new Error("previewEvalText must be a string or null");
  const generationId = record.generationId;
  if (generationId !== undefined && generationId !== null && typeof generationId !== "string") throw new Error("generationId must be a string or null");
  return { ...(previewEvalText === undefined ? {} : { previewEvalText }), ...(generationId === undefined ? {} : { generationId }) };
}
