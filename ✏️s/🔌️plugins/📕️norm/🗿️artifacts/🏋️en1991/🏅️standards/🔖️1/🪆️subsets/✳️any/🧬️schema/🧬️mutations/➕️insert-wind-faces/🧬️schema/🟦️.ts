/** ➕️ `insert-wind-faces` wire twin: the leaf payload `InsertWindFaces`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseWindFace, type WindFace } from "../../../📸️snapshot/🟦️.ts";

export interface InsertWindFaces {
  index: number;
  item: WindFace;
}

export const parseInsertWindFaces: NormWireReader<InsertWindFaces> = normWireObject<InsertWindFaces>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseWindFace) });
