/** 🔩️ `change-fastener-offset-x` wire twin: the leaf payload `ChangeFastenerOffsetX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenerOffsetX {
  fastenerId: string;
  newValue: number;
}

export const parseChangeFastenerOffsetX: NormWireReader<ChangeFastenerOffsetX> = normWireObject<ChangeFastenerOffsetX>({ fastenerId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
