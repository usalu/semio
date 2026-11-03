/** 🌉️ `change-bridge-surfacing-thickness` wire twin: the leaf payload `ChangeBridgeSurfacingThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeSurfacingThickness {
  id: string;
  newSurfacingThickness: number;
}

export const parseChangeBridgeSurfacingThickness: NormWireReader<ChangeBridgeSurfacingThickness> = normWireObject<ChangeBridgeSurfacingThickness>({ id: normWireRequired(normWireString), newSurfacingThickness: normWireRequired(normWireNumber) });
