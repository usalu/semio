/** 🛡️ `change-diaphragm-length` wire twin: the leaf payload `ChangeDiaphragmLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmLength {
  diaphragmId: string;
  newValue: number;
}

export const parseChangeDiaphragmLength: NormWireReader<ChangeDiaphragmLength> = normWireObject<ChangeDiaphragmLength>({ diaphragmId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
