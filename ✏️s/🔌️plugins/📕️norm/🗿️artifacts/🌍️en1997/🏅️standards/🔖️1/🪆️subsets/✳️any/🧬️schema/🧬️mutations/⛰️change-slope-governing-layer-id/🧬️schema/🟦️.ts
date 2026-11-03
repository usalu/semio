/** ⛰️ `change-slope-governing-layer-id` wire twin: the leaf payload `ChangeSlopeGoverningLayerId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlopeGoverningLayerId {
  mutation: "changeSlopeGoverningLayerId";
  id: string;
  newGoverningLayerId: string;
}

export const parseChangeSlopeGoverningLayerId: NormWireReader<ChangeSlopeGoverningLayerId> = normWireObject<ChangeSlopeGoverningLayerId>({ mutation: normWireRequired(normWireLiteral("changeSlopeGoverningLayerId")), id: normWireRequired(normWireString), newGoverningLayerId: normWireRequired(normWireString) });
