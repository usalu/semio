/** 📍️ `change-fastener-position-x` wire twin: the leaf payload `ChangeFastenerPositionX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerPositionX {
  fastenerId: string;
  positionId: string;
  newValue: number;
}

export const parseChangeFastenerPositionX: NormWireReader<ChangeFastenerPositionX> = normWireObject<ChangeFastenerPositionX>({ fastenerId: normWireRequired(normWireString), positionId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
