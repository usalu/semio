/** 🏛️ `change-footing-settlement-limit` wire twin: the leaf payload `ChangeFootingSettlementLimit`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFootingSettlementLimit {
  mutation: "changeFootingSettlementLimit";
  id: string;
  newSettlementLimit: number;
}

export const parseChangeFootingSettlementLimit: NormWireReader<ChangeFootingSettlementLimit> = normWireObject<ChangeFootingSettlementLimit>({ mutation: normWireRequired(normWireLiteral("changeFootingSettlementLimit")), id: normWireRequired(normWireString), newSettlementLimit: normWireRequired(normWireNumber) });
