/** 🪵️ `change-pile-interlock-bending-factor` wire twin: the leaf payload `ChangePileInterlockBendingFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileInterlockBendingFactor {
  id: string;
  newInterlockBendingFactor: number;
}

export const parseChangePileInterlockBendingFactor: NormWireReader<ChangePileInterlockBendingFactor> = normWireObject<ChangePileInterlockBendingFactor>({ id: normWireRequired(normWireString), newInterlockBendingFactor: normWireRequired(normWireNumber) });
