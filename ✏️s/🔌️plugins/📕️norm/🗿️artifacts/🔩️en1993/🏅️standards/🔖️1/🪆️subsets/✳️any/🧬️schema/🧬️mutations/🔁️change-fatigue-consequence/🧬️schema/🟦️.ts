/** 🔁️ `change-fatigue-consequence` wire twin: the leaf payload `ChangeFatigueConsequence`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FatigueConsequence, parseFatigueConsequence } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeFatigueConsequence {
  id: string;
  newConsequence: FatigueConsequence;
}

export const parseChangeFatigueConsequence: NormWireReader<ChangeFatigueConsequence> = normWireObject<ChangeFatigueConsequence>({ id: normWireRequired(normWireString), newConsequence: normWireRequired(parseFatigueConsequence) });
