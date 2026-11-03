/** ➕️ `insert-system` wire twin: the leaf payload `InsertSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998System, parseEn1998System } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSystem {
  mutation: "insertSystem";
  buildingId: string;
  index?: number | null;
  system: En1998System;
}

export const parseInsertSystem: NormWireReader<InsertSystem> = normWireObject<InsertSystem>({ mutation: normWireRequired(normWireLiteral("insertSystem")), buildingId: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), system: normWireRequired(parseEn1998System) });
