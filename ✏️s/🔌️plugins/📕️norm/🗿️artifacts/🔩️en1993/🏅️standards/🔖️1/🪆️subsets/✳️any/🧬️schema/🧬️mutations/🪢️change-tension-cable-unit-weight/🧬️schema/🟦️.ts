/** 🪢️ `change-tension-cable-unit-weight` wire twin: the leaf payload `ChangeTensionCableUnitWeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTensionCableUnitWeight {
  id: string;
  newCableUnitWeight: number;
}

export const parseChangeTensionCableUnitWeight: NormWireReader<ChangeTensionCableUnitWeight> = normWireObject<ChangeTensionCableUnitWeight>({ id: normWireRequired(normWireString), newCableUnitWeight: normWireRequired(normWireNumber) });
