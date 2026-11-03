/** 🔄 `insert-fatigue-spectrum` wire twin: the leaf payload `InsertFatigueSpectrum`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990FatigueSpectrum, parseEn1990FatigueSpectrum } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFatigueSpectrum {
  mutation: "insertFatigueSpectrum";
  index?: number;
  item: En1990FatigueSpectrum;
}

export const parseInsertFatigueSpectrum: NormWireReader<InsertFatigueSpectrum> = normWireObject<InsertFatigueSpectrum>({ mutation: normWireRequired(normWireLiteral("insertFatigueSpectrum")), index: normWireOptional(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1990FatigueSpectrum) });
