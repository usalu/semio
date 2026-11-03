/** 📍 `change-placement-y` wire twin: the leaf payload `ChangePlacementY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlacementY {
  mutation: "changePlacementY";
  id: string;
  newY: number;
}

export const parseChangePlacementY: NormWireReader<ChangePlacementY> = normWireObject<ChangePlacementY>({ mutation: normWireRequired(normWireLiteral("changePlacementY")), id: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });
