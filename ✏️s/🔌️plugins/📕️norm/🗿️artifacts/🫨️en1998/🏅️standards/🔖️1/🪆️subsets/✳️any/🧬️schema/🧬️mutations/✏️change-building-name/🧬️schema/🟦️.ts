/** ✏️ `change-building-name` wire twin: the leaf payload `ChangeBuildingName`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingName {
  mutation: "changeBuildingName";
  id: string;
  newName: string;
}

export const parseChangeBuildingName: NormWireReader<ChangeBuildingName> = normWireObject<ChangeBuildingName>({ mutation: normWireRequired(normWireLiteral("changeBuildingName")), id: normWireRequired(normWireString), newName: normWireRequired(normWireString) });
