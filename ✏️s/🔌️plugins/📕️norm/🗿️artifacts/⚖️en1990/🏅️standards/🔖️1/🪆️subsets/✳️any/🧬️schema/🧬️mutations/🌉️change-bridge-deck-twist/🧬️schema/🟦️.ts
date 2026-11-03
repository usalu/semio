/** 🌉 `change-bridge-deck-twist` wire twin: the leaf payload `ChangeBridgeDeckTwist`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeckTwist {
  mutation: "changeBridgeDeckTwist";
  id: string;
  newDeckTwist: number;
}

export const parseChangeBridgeDeckTwist: NormWireReader<ChangeBridgeDeckTwist> = normWireObject<ChangeBridgeDeckTwist>({ mutation: normWireRequired(normWireLiteral("changeBridgeDeckTwist")), id: normWireRequired(normWireString), newDeckTwist: normWireRequired(normWireNumber) });
