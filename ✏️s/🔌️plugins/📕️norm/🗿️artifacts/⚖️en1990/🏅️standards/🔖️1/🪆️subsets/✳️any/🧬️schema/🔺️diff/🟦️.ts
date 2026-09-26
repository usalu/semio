/** 🔺️ En1990Diff — sparse optional fields over En1990Artifact. */
import { type En1990Artifact, parseEn1990Fields } from "../🟦️.ts";
export type En1990Diff = { [K in keyof En1990Artifact]?: En1990Artifact[K] | null };
export function parseEn1990Diff(value: unknown, at = "$"): En1990Diff {
  return parseEn1990Fields(value, true, at) as En1990Diff;
}
