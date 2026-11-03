/** ➖ `remove-permanent` wire twin: the leaf payload `RemovePermanent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemovePermanent {
  mutation: "removePermanent";
  index: number;
}

export const parseRemovePermanent: NormWireReader<RemovePermanent> = normWireObject<RemovePermanent>({ mutation: normWireRequired(normWireLiteral("removePermanent")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
