/** ✏️ `change-building-ns-q` wire twin: the leaf payload `ChangeBuildingNsQ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingNsQ {
  mutation: "changeBuildingNsQ";
  id: string;
  newNonstructuralQ: number;
}

export const parseChangeBuildingNsQ: NormWireReader<ChangeBuildingNsQ> = normWireObject<ChangeBuildingNsQ>({ mutation: normWireRequired(normWireLiteral("changeBuildingNsQ")), id: normWireRequired(normWireString), newNonstructuralQ: normWireRequired(normWireNumber) });
