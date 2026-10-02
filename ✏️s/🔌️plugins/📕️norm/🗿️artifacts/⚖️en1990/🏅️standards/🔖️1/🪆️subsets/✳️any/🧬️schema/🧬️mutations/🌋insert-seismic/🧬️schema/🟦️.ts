/** 🌋 `insert-seismic` wire twin: the leaf payload `InsertSeismic`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990SeismicAction, parseEn1990SeismicAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSeismic {
  mutation: "insertSeismic";
  index: number;
  item: En1990SeismicAction;
}

export const parseInsertSeismic: NormWireReader<InsertSeismic> = normWireObject<InsertSeismic>({ mutation: normWireRequired(normWireLiteral("insertSeismic")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1990SeismicAction) });
