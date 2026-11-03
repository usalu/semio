/** 📍 `change-placement-z` wire twin: the leaf payload `ChangePlacementZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlacementZ {
  mutation: "changePlacementZ";
  id: string;
  newZ: number;
}

export const parseChangePlacementZ: NormWireReader<ChangePlacementZ> = normWireObject<ChangePlacementZ>({ mutation: normWireRequired(normWireLiteral("changePlacementZ")), id: normWireRequired(normWireString), newZ: normWireRequired(normWireNumber) });
