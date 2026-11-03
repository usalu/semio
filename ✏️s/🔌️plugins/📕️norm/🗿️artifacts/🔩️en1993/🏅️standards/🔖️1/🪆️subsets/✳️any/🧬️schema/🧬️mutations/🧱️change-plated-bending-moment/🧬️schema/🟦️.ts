/** 🧱️ `change-plated-bending-moment` wire twin: the leaf payload `ChangePlatedBendingMoment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedBendingMoment {
  id: string;
  newBendingMoment: number;
}

export const parseChangePlatedBendingMoment: NormWireReader<ChangePlatedBendingMoment> = normWireObject<ChangePlatedBendingMoment>({ id: normWireRequired(normWireString), newBendingMoment: normWireRequired(normWireNumber) });
