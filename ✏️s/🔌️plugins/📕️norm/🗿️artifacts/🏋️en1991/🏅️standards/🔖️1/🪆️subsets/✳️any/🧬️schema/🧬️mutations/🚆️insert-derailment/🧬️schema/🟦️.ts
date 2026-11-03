/** 🚆 `insert-derailment` wire twin: the leaf payload `InsertDerailment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type DerailmentCase, parseDerailmentCase } from "../../../📸️snapshot/🟦️.ts";

export interface InsertDerailment {
  index: number;
  item: DerailmentCase;
}

export const parseInsertDerailment: NormWireReader<InsertDerailment> = normWireObject<InsertDerailment>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseDerailmentCase) });
