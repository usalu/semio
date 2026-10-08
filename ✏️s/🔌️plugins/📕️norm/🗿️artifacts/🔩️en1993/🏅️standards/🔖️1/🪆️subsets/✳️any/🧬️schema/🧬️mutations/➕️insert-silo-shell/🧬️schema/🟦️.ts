/** ➕️ `insert-silo-shell` wire twin: the leaf payload `InsertSiloShell`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSiloShell, type SiloShell } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSiloShell {
  index?: number | null;
  siloShell: SiloShell;
}

export const parseInsertSiloShell: NormWireReader<InsertSiloShell> = normWireObject<InsertSiloShell>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), siloShell: normWireRequired(parseSiloShell) });
