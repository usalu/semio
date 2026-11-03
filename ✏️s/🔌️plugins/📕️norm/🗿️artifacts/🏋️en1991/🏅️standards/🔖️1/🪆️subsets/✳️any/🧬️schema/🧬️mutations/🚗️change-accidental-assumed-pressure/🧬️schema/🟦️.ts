/** 🚗 `change-accidental-assumed-pressure` wire twin: the leaf payload `ChangeAccidentalAssumedPressure`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccidentalAssumedPressure {
  id: string;
  newAssumedPressure: number;
}

export const parseChangeAccidentalAssumedPressure: NormWireReader<ChangeAccidentalAssumedPressure> = normWireObject<ChangeAccidentalAssumedPressure>({ id: normWireRequired(normWireString), newAssumedPressure: normWireRequired(normWireNumber) });
