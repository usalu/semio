/** 🎚 `remove-factor-override` wire twin: the leaf payload `RemoveFactorOverride`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFactorOverride {
  mutation: "removeFactorOverride";
  index: number;
}

export const parseRemoveFactorOverride: NormWireReader<RemoveFactorOverride> = normWireObject<RemoveFactorOverride>({ mutation: normWireRequired(normWireLiteral("removeFactorOverride")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
