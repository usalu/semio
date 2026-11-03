/** ✏️ `change-seismic-slope-governing-layer-id` wire twin: the leaf payload `ChangeSeismicSlopeGoverningLayerId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSeismicSlopeGoverningLayerId {
  mutation: "changeSeismicSlopeGoverningLayerId";
  id: string;
  newGoverningLayerId: string;
}

export const parseChangeSeismicSlopeGoverningLayerId: NormWireReader<ChangeSeismicSlopeGoverningLayerId> = normWireObject<ChangeSeismicSlopeGoverningLayerId>({ mutation: normWireRequired(normWireLiteral("changeSeismicSlopeGoverningLayerId")), id: normWireRequired(normWireString), newGoverningLayerId: normWireRequired(normWireString) });
