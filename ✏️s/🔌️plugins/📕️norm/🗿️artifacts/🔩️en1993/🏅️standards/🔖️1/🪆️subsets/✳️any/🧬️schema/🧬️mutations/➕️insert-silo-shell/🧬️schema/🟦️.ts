/** ➕️ `insert-silo-shell` wire twin: the leaf payload `InsertSiloShell`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SiloShell, parseSiloShell } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSiloShell {
  index?: number | null;
  siloShell: SiloShell;
}

export const parseInsertSiloShell: NormWireReader<InsertSiloShell> = normWireObject<InsertSiloShell>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), siloShell: normWireRequired(parseSiloShell) });
