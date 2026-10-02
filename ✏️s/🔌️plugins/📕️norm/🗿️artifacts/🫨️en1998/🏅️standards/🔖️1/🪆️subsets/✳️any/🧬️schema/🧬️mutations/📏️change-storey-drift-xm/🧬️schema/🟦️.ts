/** 📏️ `change-storey-drift-xm` wire twin: the leaf payload `ChangeStoreyDriftXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeStoreyDriftXM {
  mutation: "changeStoreyDriftXM";
  buildingIndex: number;
  storeyIndex: number;
  newDriftXM: number;
}

export const parseChangeStoreyDriftXM: NormWireReader<ChangeStoreyDriftXM> = normWireObject<ChangeStoreyDriftXM>({ mutation: normWireRequired(normWireLiteral("changeStoreyDriftXM")), buildingIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), storeyIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newDriftXM: normWireRequired(normWireNumber) });
