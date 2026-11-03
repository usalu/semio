/** 🗼️ `change-tower-bending-stiffness` wire twin: the leaf payload `ChangeTowerBendingStiffness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerBendingStiffness {
  id: string;
  newBendingStiffness: number;
}

export const parseChangeTowerBendingStiffness: NormWireReader<ChangeTowerBendingStiffness> = normWireObject<ChangeTowerBendingStiffness>({ id: normWireRequired(normWireString), newBendingStiffness: normWireRequired(normWireNumber) });
