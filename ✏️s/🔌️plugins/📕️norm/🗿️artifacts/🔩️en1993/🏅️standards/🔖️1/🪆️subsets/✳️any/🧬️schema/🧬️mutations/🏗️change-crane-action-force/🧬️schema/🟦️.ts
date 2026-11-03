/** 🏗️ `change-crane-action-force` wire twin: the leaf payload `ChangeCraneActionForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneActionForce {
  id: string;
  elementId: string;
  newForce: number;
}

export const parseChangeCraneActionForce: NormWireReader<ChangeCraneActionForce> = normWireObject<ChangeCraneActionForce>({ id: normWireRequired(normWireString), elementId: normWireRequired(normWireString), newForce: normWireRequired(normWireNumber) });
