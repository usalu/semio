/** 🗼 `insert-tower` wire twin: the leaf payload `InsertTower`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1998Tower, parseEn1998Tower } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTower {
  mutation: "insertTower";
  index: number;
  tower: En1998Tower;
}

export const parseInsertTower: NormWireReader<InsertTower> = normWireObject<InsertTower>({ mutation: normWireRequired(normWireLiteral("insertTower")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), tower: normWireRequired(parseEn1998Tower) });
