/** 🛡️ `change-diaphragm-shear-ed` wire twin: the leaf payload `ChangeDiaphragmShearEd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmShearEd {
  diaphragmId: string;
  newValue: number;
}

export const parseChangeDiaphragmShearEd: NormWireReader<ChangeDiaphragmShearEd> = normWireObject<ChangeDiaphragmShearEd>({ diaphragmId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
