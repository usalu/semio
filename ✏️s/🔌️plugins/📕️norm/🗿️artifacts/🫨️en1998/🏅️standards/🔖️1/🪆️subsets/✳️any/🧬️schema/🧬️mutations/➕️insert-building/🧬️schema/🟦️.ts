/** ➕️ `insert-building` wire twin: the leaf payload `InsertBuilding`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1998Building, parseEn1998Building } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBuilding {
  mutation: "insertBuilding";
  index?: number | null;
  building: En1998Building;
}

export const parseInsertBuilding: NormWireReader<InsertBuilding> = normWireObject<InsertBuilding>({ mutation: normWireRequired(normWireLiteral("insertBuilding")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), building: normWireRequired(parseEn1998Building) });
