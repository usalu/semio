/** 🚗 `change-accidental-standoff` wire twin: the leaf payload `ChangeAccidentalStandoff`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccidentalStandoff {
  id: string;
  newStandoff: number;
}

export const parseChangeAccidentalStandoff: NormWireReader<ChangeAccidentalStandoff> = normWireObject<ChangeAccidentalStandoff>({ id: normWireRequired(normWireString), newStandoff: normWireRequired(normWireNumber) });
