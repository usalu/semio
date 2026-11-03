/** 🗼️ `change-tower-ring-mode` wire twin: the leaf payload `ChangeTowerRingMode`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerRingMode {
  id: string;
  newRingMode: number;
}

export const parseChangeTowerRingMode: NormWireReader<ChangeTowerRingMode> = normWireObject<ChangeTowerRingMode>({ id: normWireRequired(normWireString), newRingMode: normWireRequired(normWireNumber) });
