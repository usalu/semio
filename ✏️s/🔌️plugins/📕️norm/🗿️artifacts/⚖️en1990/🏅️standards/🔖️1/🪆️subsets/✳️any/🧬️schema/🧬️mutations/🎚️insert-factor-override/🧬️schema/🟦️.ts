/** 🎚 `insert-factor-override` wire twin: the leaf payload `InsertFactorOverride`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990FactorOverride, parseEn1990FactorOverride } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFactorOverride {
  mutation: "insertFactorOverride";
  index?: number;
  item: En1990FactorOverride;
}

export const parseInsertFactorOverride: NormWireReader<InsertFactorOverride> = normWireObject<InsertFactorOverride>({ mutation: normWireRequired(normWireLiteral("insertFactorOverride")), index: normWireOptional(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1990FactorOverride) });
