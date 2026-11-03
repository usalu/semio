/** 🔩️ `change-row-pitch-to-previous` wire twin: the leaf payload `ChangeRowPitchToPrevious`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowPitchToPrevious {
  id: string;
  newPitchToPrevious: number;
}

export const parseChangeRowPitchToPrevious: NormWireReader<ChangeRowPitchToPrevious> = normWireObject<ChangeRowPitchToPrevious>({ id: normWireRequired(normWireString), newPitchToPrevious: normWireRequired(normWireNumber) });
