/** ➕️ `insert-forming-gain` wire twin: the leaf payload `InsertFormingGain`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FormingGain, parseFoundationBlock } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFormingGain {
  index?: number | null;
  formingGain: FormingGain;
}

export const parseInsertFormingGain: NormWireReader<InsertFormingGain> = normWireObject<InsertFormingGain>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), formingGain: normWireRequired(parseFoundationBlock) });
