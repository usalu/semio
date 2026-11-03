/** ➖ `remove-test-profile` wire twin: the leaf payload `RemoveTestProfile`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTestProfile {
  mutation: "removeTestProfile";
  pileId: string;
  index: number;
}

export const parseRemoveTestProfile: NormWireReader<RemoveTestProfile> = normWireObject<RemoveTestProfile>({ mutation: normWireRequired(normWireLiteral("removeTestProfile")), pileId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
