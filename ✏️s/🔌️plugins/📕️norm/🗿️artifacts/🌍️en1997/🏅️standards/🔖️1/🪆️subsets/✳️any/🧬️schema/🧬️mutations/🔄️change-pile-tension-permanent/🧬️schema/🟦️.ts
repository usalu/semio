/** 🔄️ `change-pile-tension-permanent` wire twin: the leaf payload `ChangePileTensionPermanent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileTensionPermanent {
  mutation: "changePileTensionPermanent";
  id: string;
  newTensionPermanent: number;
}

export const parseChangePileTensionPermanent: NormWireReader<ChangePileTensionPermanent> = normWireObject<ChangePileTensionPermanent>({ mutation: normWireRequired(normWireLiteral("changePileTensionPermanent")), id: normWireRequired(normWireString), newTensionPermanent: normWireRequired(normWireNumber) });
