/** ➕️ `insert-load-case` wire twin: the leaf payload `InsertLoadCase`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WallLoadCase, parseWallLoadCase } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLoadCase {
  wallId: string;
  index: number;
  loadCase: WallLoadCase;
}

export const parseInsertLoadCase: NormWireReader<InsertLoadCase> = normWireObject<InsertLoadCase>({ wallId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), loadCase: normWireRequired(parseWallLoadCase) });
