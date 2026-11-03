/** 📏️ `change-fatigue-stress-relief` wire twin: the leaf payload `ChangeFatigueStressRelief`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StressRelief, parseStressRelief } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFatigueStressRelief {
  id: string;
  newRelief: StressRelief;
}

export const parseChangeFatigueStressRelief: NormWireReader<ChangeFatigueStressRelief> = normWireObject<ChangeFatigueStressRelief>({ id: normWireRequired(normWireString), newRelief: normWireRequired(parseStressRelief) });
