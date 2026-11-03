/** 🚗 `change-accidental-assumed-force` wire twin: the leaf payload `ChangeAccidentalAssumedForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccidentalAssumedForce {
  id: string;
  newAssumedForce: number;
}

export const parseChangeAccidentalAssumedForce: NormWireReader<ChangeAccidentalAssumedForce> = normWireObject<ChangeAccidentalAssumedForce>({ id: normWireRequired(normWireString), newAssumedForce: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
