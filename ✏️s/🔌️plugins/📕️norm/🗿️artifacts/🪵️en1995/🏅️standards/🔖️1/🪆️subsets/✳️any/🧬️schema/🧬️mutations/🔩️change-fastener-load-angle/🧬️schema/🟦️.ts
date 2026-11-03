/** 🔩️ `change-fastener-load-angle` wire twin: the leaf payload `ChangeFastenerLoadAngle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerLoadAngle {
  fastenerId: string;
  newValue: number;
}

export const parseChangeFastenerLoadAngle: NormWireReader<ChangeFastenerLoadAngle> = normWireObject<ChangeFastenerLoadAngle>({ fastenerId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
