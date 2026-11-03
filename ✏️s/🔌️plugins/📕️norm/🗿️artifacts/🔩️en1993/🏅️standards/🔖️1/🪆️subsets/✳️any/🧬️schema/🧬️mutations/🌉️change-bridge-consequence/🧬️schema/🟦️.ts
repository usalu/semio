/** 🌉️ `change-bridge-consequence` wire twin: the leaf payload `ChangeBridgeConsequence`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FatigueConsequence, parseFatigueConsequence } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeBridgeConsequence {
  id: string;
  newConsequence: FatigueConsequence;
}

export const parseChangeBridgeConsequence: NormWireReader<ChangeBridgeConsequence> = normWireObject<ChangeBridgeConsequence>({ id: normWireRequired(normWireString), newConsequence: normWireRequired(parseFatigueConsequence) });
