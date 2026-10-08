/** ➕️ `insert-crane-runway` wire twin: the leaf payload `InsertCraneRunway`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CraneRunway, parseCraneRunway } from "../../../📸️snapshot/🟦️.ts";

export interface InsertCraneRunway {
  index?: number | null;
  craneRunway: CraneRunway;
}

export const parseInsertCraneRunway: NormWireReader<InsertCraneRunway> = normWireObject<InsertCraneRunway>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), craneRunway: normWireRequired(parseCraneRunway) });
