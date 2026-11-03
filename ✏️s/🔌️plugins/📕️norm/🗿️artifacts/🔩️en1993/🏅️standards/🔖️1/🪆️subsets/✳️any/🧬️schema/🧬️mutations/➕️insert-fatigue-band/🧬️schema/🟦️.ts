/** 🔁️ `insert-fatigue-band` wire twin: the leaf payload `InsertFatigueBand`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FatigueBand, parseFatigueBand } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFatigueBand {
  id: string;
  index?: number | null;
  element: FatigueBand;
}

export const parseInsertFatigueBand: NormWireReader<InsertFatigueBand> = normWireObject<InsertFatigueBand>({ id: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), element: normWireRequired(parseFatigueBand) });
