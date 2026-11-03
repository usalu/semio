/** 🔍 `change-inspection-level` wire twin: the leaf payload `ChangeInspectionLevel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990InspectionLevel, parseEn1990InspectionLevel } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeInspectionLevel {
  mutation: "changeInspectionLevel";
  newInspectionLevel: En1990InspectionLevel;
}

export const parseChangeInspectionLevel: NormWireReader<ChangeInspectionLevel> = normWireObject<ChangeInspectionLevel>({ mutation: normWireRequired(normWireLiteral("changeInspectionLevel")), newInspectionLevel: normWireRequired(parseEn1990InspectionLevel) });
