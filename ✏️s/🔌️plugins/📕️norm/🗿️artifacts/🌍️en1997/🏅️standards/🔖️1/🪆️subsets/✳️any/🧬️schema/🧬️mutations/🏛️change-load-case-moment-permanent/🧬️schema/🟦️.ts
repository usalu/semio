/** 🏛️ `change-load-case-moment-permanent` wire twin: the leaf payload `ChangeLoadCaseMomentPermanent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadCaseMomentPermanent {
  mutation: "changeLoadCaseMomentPermanent";
  footingId: string;
  id: string;
  newMomentPermanent: number;
}

export const parseChangeLoadCaseMomentPermanent: NormWireReader<ChangeLoadCaseMomentPermanent> = normWireObject<ChangeLoadCaseMomentPermanent>({ mutation: normWireRequired(normWireLiteral("changeLoadCaseMomentPermanent")), footingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newMomentPermanent: normWireRequired(normWireNumber) });
