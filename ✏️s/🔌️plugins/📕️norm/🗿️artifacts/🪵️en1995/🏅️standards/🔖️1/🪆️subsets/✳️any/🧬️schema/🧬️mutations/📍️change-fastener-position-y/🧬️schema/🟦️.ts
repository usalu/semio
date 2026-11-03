/** 📍️ `change-fastener-position-y` wire twin: the leaf payload `ChangeFastenerPositionY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerPositionY {
  fastenerId: string;
  positionId: string;
  newValue: number;
}

export const parseChangeFastenerPositionY: NormWireReader<ChangeFastenerPositionY> = normWireObject<ChangeFastenerPositionY>({ fastenerId: normWireRequired(normWireString), positionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
