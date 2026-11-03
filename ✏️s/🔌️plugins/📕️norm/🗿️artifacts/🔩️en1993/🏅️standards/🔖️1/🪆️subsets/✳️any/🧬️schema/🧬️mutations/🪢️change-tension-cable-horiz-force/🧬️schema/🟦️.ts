/** 🪢️ `change-tension-cable-horiz-force` wire twin: the leaf payload `ChangeTensionCableHorizForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionCableHorizForce {
  id: string;
  newCableHorizontalForce: number;
}

export const parseChangeTensionCableHorizForce: NormWireReader<ChangeTensionCableHorizForce> = normWireObject<ChangeTensionCableHorizForce>({ id: normWireRequired(normWireString), newCableHorizontalForce: normWireRequired(normWireNumber) });
