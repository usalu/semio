/** 🌉️ `change-bridge-trough-thickness` wire twin: the leaf payload `ChangeBridgeTroughThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeTroughThickness {
  id: string;
  newTroughThickness: number;
}

export const parseChangeBridgeTroughThickness: NormWireReader<ChangeBridgeTroughThickness> = normWireObject<ChangeBridgeTroughThickness>({ id: normWireRequired(normWireString), newTroughThickness: normWireRequired(normWireNumber) });
