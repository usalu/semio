/** 🪵️ `change-pile-interlock-stiff-factor` wire twin: the leaf payload `ChangePileInterlockStiffFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileInterlockStiffFactor {
  id: string;
  newInterlockStiffnessFactor: number;
}

export const parseChangePileInterlockStiffFactor: NormWireReader<ChangePileInterlockStiffFactor> = normWireObject<ChangePileInterlockStiffFactor>({ id: normWireRequired(normWireString), newInterlockStiffnessFactor: normWireRequired(normWireNumber) });
