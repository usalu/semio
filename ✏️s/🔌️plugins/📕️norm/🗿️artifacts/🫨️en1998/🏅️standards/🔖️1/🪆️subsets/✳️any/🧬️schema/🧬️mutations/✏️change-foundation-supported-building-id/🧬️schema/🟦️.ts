/** ✏️ `change-foundation-supported-building-id` wire twin: the leaf payload `ChangeFoundationSupportedBuildingId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationSupportedBuildingId {
  mutation: "changeFoundationSupportedBuildingId";
  id: string;
  newSupportedBuildingId: string;
}

export const parseChangeFoundationSupportedBuildingId: NormWireReader<ChangeFoundationSupportedBuildingId> = normWireObject<ChangeFoundationSupportedBuildingId>({ mutation: normWireRequired(normWireLiteral("changeFoundationSupportedBuildingId")), id: normWireRequired(normWireString), newSupportedBuildingId: normWireRequired(normWireString) });
