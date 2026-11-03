/** 🎲 `insert-reliability-analysis` wire twin: the leaf payload `InsertReliabilityAnalysis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990ReliabilityAnalysis, parseEn1990ReliabilityAnalysis } from "../../../📸️snapshot/🟦️.ts";

export interface InsertReliabilityAnalysis {
  mutation: "insertReliabilityAnalysis";
  index?: number;
  item: En1990ReliabilityAnalysis;
}

export const parseInsertReliabilityAnalysis: NormWireReader<InsertReliabilityAnalysis> = normWireObject<InsertReliabilityAnalysis>({ mutation: normWireRequired(normWireLiteral("insertReliabilityAnalysis")), index: normWireOptional(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1990ReliabilityAnalysis) });
