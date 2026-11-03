/** 🪵️ `change-pile-ilock-bending-factor` wire twin: the leaf payload `ChangePileIlockBendingFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileIlockBendingFactor {
  id: string;
  newInterlockBendingFactor: number;
}

export const parseChangePileIlockBendingFactor: NormWireReader<ChangePileIlockBendingFactor> = normWireObject<ChangePileIlockBendingFactor>({ id: normWireRequired(normWireString), newInterlockBendingFactor: normWireRequired(normWireNumber) });
