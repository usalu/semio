/** ➕ `insert-load-case` wire twin: the leaf payload `InsertLoadCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997FoundationLoadCase, parseEn1997FoundationLoadCase } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLoadCase {
  mutation: "insertLoadCase";
  footingId: string;
  index: number;
  loadCase: En1997FoundationLoadCase;
}

export const parseInsertLoadCase: NormWireReader<InsertLoadCase> = normWireObject<InsertLoadCase>({ mutation: normWireRequired(normWireLiteral("insertLoadCase")), footingId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), loadCase: normWireRequired(parseEn1997FoundationLoadCase) });
