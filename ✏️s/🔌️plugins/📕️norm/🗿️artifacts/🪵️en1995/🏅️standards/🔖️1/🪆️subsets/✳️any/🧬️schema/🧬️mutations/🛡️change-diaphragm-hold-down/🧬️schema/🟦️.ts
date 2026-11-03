/** 🛡️ `change-diaphragm-hold-down` wire twin: the leaf payload `ChangeDiaphragmHoldDown`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmHoldDown {
  diaphragmId: string;
  newValue: number;
}

export const parseChangeDiaphragmHoldDown: NormWireReader<ChangeDiaphragmHoldDown> = normWireObject<ChangeDiaphragmHoldDown>({ diaphragmId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
