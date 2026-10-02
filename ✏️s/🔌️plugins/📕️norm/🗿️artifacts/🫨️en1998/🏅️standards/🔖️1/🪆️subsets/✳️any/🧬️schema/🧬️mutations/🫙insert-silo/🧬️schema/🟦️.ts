/** 🫙 `insert-silo` wire twin: the leaf payload `InsertSilo`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1998Silo, parseEn1998Silo } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSilo {
  mutation: "insertSilo";
  index: number;
  silo: En1998Silo;
}

export const parseInsertSilo: NormWireReader<InsertSilo> = normWireObject<InsertSilo>({ mutation: normWireRequired(normWireLiteral("insertSilo")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), silo: normWireRequired(parseEn1998Silo) });
