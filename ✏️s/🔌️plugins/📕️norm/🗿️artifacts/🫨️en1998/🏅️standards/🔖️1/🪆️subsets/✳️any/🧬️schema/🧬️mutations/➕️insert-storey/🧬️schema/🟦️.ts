/** ➕️ `insert-storey` wire twin: the leaf payload `InsertStorey`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Storey, parseEn1998Storey } from "../../../📸️snapshot/🟦️.ts";

export interface InsertStorey {
  mutation: "insertStorey";
  buildingId: string;
  index?: number | null;
  storey: En1998Storey;
}

export const parseInsertStorey: NormWireReader<InsertStorey> = normWireObject<InsertStorey>({ mutation: normWireRequired(normWireLiteral("insertStorey")), buildingId: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), storey: normWireRequired(parseEn1998Storey) });
