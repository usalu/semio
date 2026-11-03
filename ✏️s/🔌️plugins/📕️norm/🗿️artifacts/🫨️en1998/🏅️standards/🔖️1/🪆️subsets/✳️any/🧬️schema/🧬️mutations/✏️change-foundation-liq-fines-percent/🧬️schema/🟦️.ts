/** ✏️ `change-foundation-liq-fines-percent` wire twin: the leaf payload `ChangeFoundationLiqFinesPercent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationLiqFinesPercent {
  mutation: "changeFoundationLiqFinesPercent";
  id: string;
  newLiqFinesPercent: number;
}

export const parseChangeFoundationLiqFinesPercent: NormWireReader<ChangeFoundationLiqFinesPercent> = normWireObject<ChangeFoundationLiqFinesPercent>({ mutation: normWireRequired(normWireLiteral("changeFoundationLiqFinesPercent")), id: normWireRequired(normWireString), newLiqFinesPercent: normWireRequired(normWireNumber) });
