/** 🏛️ `change-load-case-horiz-permanent` wire twin: the leaf payload `ChangeLoadCaseHorizPermanent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeLoadCaseHorizPermanent {
  mutation: "changeLoadCaseHorizPermanent";
  footingId: string;
  id: string;
  newHorizontalPermanent: number;
}

export const parseChangeLoadCaseHorizPermanent: NormWireReader<ChangeLoadCaseHorizPermanent> = normWireObject<ChangeLoadCaseHorizPermanent>({ mutation: normWireRequired(normWireLiteral("changeLoadCaseHorizPermanent")), footingId: normWireRequired(normWireString), id: normWireRequired(normWireString), newHorizontalPermanent: normWireRequired(normWireNumber) });
