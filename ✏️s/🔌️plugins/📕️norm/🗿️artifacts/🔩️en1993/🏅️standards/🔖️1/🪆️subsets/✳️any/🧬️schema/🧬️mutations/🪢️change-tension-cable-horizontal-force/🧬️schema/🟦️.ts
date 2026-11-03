/** 🪢️ `change-tension-cable-horizontal-force` wire twin: the leaf payload `ChangeTensionCableHorizontalForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionCableHorizontalForce {
  id: string;
  newCableHorizontalForce: number;
}

export const parseChangeTensionCableHorizontalForce: NormWireReader<ChangeTensionCableHorizontalForce> = normWireObject<ChangeTensionCableHorizontalForce>({ id: normWireRequired(normWireString), newCableHorizontalForce: normWireRequired(normWireNumber) });
