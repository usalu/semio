/** 🦴️ `change-transverse-sides` wire twin: the leaf payload `ChangeTransverseSides`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StiffenerSides, parseStiffenerSides } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeTransverseSides {
  mutation: "changeTransverseSides";
  transverseStiffenerId: string;
  newSides: StiffenerSides;
}

export const parseChangeTransverseSides: NormWireReader<ChangeTransverseSides> = normWireObject<ChangeTransverseSides>({ mutation: normWireRequired(normWireLiteral("changeTransverseSides")), transverseStiffenerId: normWireRequired(normWireString), newSides: normWireRequired(parseStiffenerSides) });
