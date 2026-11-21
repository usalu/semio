import type { ElevationSource } from "../../🗺️imported-map/🏔️elevation/🟦️.ts";

/** 🏔️ Replaces the document's authored source, or clears it when omitted. */
export interface ReplaceElevationSource {
  newElevationSource?: ElevationSource;
}

export const ReplaceElevationSourceKind = "replace-elevation-source" as const;
