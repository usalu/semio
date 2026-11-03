/** 🌉 `change-bridge-deck-twist-limit` wire twin: the leaf payload `ChangeBridgeDeckTwistLimit`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeckTwistLimit {
  mutation: "changeBridgeDeckTwistLimit";
  id: string;
  newDeckTwistLimit: number;
}

export const parseChangeBridgeDeckTwistLimit: NormWireReader<ChangeBridgeDeckTwistLimit> = normWireObject<ChangeBridgeDeckTwistLimit>({ mutation: normWireRequired(normWireLiteral("changeBridgeDeckTwistLimit")), id: normWireRequired(normWireString), newDeckTwistLimit: normWireRequired(normWireNumber) });
