/** ⚙ `change-fatigue-model-assumed-udl` wire twin: the leaf payload `ChangeFatigueModelAssumedUdl`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFatigueModelAssumedUdl {
  id: string;
  newAssumedUdl: number;
}

export const parseChangeFatigueModelAssumedUdl: NormWireReader<ChangeFatigueModelAssumedUdl> = normWireObject<ChangeFatigueModelAssumedUdl>({ id: normWireRequired(normWireString), newAssumedUdl: normWireRequired(normWireNumber) });
