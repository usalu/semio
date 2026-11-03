/** ➕️ `insert-weld-group` wire twin: the leaf payload `InsertWeldGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WeldGroup, parseWeldGroup } from "../../../📸️snapshot/🟦️.ts";

export interface InsertWeldGroup {
  index?: number | null;
  weldGroup: WeldGroup;
}

export const parseInsertWeldGroup: NormWireReader<InsertWeldGroup> = normWireObject<InsertWeldGroup>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), weldGroup: normWireRequired(parseWeldGroup) });
