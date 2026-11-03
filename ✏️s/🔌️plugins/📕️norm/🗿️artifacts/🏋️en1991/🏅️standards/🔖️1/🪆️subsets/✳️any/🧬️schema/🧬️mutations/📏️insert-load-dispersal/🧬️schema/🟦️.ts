/** 📏 `insert-load-dispersal` wire twin: the leaf payload `InsertLoadDispersal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type LoadDispersal, parseLoadDispersal } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLoadDispersal {
  index: number;
  item: LoadDispersal;
}

export const parseInsertLoadDispersal: NormWireReader<InsertLoadDispersal> = normWireObject<InsertLoadDispersal>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseLoadDispersal) });
