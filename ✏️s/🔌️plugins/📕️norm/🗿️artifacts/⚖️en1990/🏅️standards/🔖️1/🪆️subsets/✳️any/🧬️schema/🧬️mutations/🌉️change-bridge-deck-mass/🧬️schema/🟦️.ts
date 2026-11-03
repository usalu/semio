/** 🌉 `change-bridge-deck-mass` wire twin: the leaf payload `ChangeBridgeDeckMass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeckMass {
  mutation: "changeBridgeDeckMass";
  id: string;
  newDeckMass: number;
}

export const parseChangeBridgeDeckMass: NormWireReader<ChangeBridgeDeckMass> = normWireObject<ChangeBridgeDeckMass>({ mutation: normWireRequired(normWireLiteral("changeBridgeDeckMass")), id: normWireRequired(normWireString), newDeckMass: normWireRequired(normWireNumber) });
