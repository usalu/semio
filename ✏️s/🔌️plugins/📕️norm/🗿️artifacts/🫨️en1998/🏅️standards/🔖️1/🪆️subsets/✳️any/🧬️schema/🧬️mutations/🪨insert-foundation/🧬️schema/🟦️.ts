/** 🪨 `insert-foundation` wire twin: the leaf payload `InsertFoundation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Foundation, parseEn1998Foundation } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFoundation {
  mutation: "insertFoundation";
  index?: number | null;
  foundation: En1998Foundation;
}

export const parseInsertFoundation: NormWireReader<InsertFoundation> = normWireObject<InsertFoundation>({ mutation: normWireRequired(normWireLiteral("insertFoundation")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), foundation: normWireRequired(parseEn1998Foundation) });
