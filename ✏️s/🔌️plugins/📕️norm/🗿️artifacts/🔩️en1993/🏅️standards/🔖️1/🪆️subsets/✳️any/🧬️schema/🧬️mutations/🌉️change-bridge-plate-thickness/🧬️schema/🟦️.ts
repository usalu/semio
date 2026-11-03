/** 🌉️ `change-bridge-plate-thickness` wire twin: the leaf payload `ChangeBridgePlateThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgePlateThickness {
  id: string;
  newPlateThickness: number;
}

export const parseChangeBridgePlateThickness: NormWireReader<ChangeBridgePlateThickness> = normWireObject<ChangeBridgePlateThickness>({ id: normWireRequired(normWireString), newPlateThickness: normWireRequired(normWireNumber) });
