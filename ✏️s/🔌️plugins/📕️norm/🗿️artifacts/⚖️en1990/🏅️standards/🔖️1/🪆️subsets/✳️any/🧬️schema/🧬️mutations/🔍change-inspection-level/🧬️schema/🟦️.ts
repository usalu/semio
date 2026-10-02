/** 🔍 `change-inspection-level` wire twin: the leaf payload `ChangeInspectionLevel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeInspectionLevel {
  mutation: "changeInspectionLevel";
  newInspectionLevel: string;
}

export const parseChangeInspectionLevel: NormWireReader<ChangeInspectionLevel> = normWireObject<ChangeInspectionLevel>({ mutation: normWireRequired(normWireLiteral("changeInspectionLevel")), newInspectionLevel: normWireRequired(normWireString) });
