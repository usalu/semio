/** ✏️ `change-soil-layer-foundation-id` wire twin: the leaf payload `ChangeSoilLayerFoundationId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSoilLayerFoundationId {
  mutation: "changeSoilLayerFoundationId";
  id: string;
  newFoundationId: string;
}

export const parseChangeSoilLayerFoundationId: NormWireReader<ChangeSoilLayerFoundationId> = normWireObject<ChangeSoilLayerFoundationId>({ mutation: normWireRequired(normWireLiteral("changeSoilLayerFoundationId")), id: normWireRequired(normWireString), newFoundationId: normWireRequired(normWireString) });
