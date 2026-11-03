/** 🛡️ `change-diaphragm-wall-height` wire twin: the leaf payload `ChangeDiaphragmWallHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmWallHeight {
  diaphragmId: string;
  newValue: number;
}

export const parseChangeDiaphragmWallHeight: NormWireReader<ChangeDiaphragmWallHeight> = normWireObject<ChangeDiaphragmWallHeight>({ diaphragmId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
