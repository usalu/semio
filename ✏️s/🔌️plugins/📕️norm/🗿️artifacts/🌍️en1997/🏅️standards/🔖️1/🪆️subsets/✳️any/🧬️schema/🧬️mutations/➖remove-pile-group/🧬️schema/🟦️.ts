/** ➖ `remove-pile-group` wire twin: the leaf payload `RemovePileGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePileGroup {
  mutation: "removePileGroup";
  index: number;
}

export const parseRemovePileGroup: NormWireReader<RemovePileGroup> = normWireObject<RemovePileGroup>({ mutation: normWireRequired(normWireLiteral("removePileGroup")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
