/** 🪵️ `change-pile-interlock-stiffness-factor` wire twin: the leaf payload `ChangePileInterlockStiffnessFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileInterlockStiffnessFactor {
  id: string;
  newInterlockStiffnessFactor: number;
}

export const parseChangePileInterlockStiffnessFactor: NormWireReader<ChangePileInterlockStiffnessFactor> = normWireObject<ChangePileInterlockStiffnessFactor>({ id: normWireRequired(normWireString), newInterlockStiffnessFactor: normWireRequired(normWireNumber) });
