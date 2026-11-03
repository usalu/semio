/** 🌉 `change-bridge-deck-acceleration` wire twin: the leaf payload `ChangeBridgeDeckAcceleration`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeckAcceleration {
  mutation: "changeBridgeDeckAcceleration";
  id: string;
  newDeckAcceleration: number;
}

export const parseChangeBridgeDeckAcceleration: NormWireReader<ChangeBridgeDeckAcceleration> = normWireObject<ChangeBridgeDeckAcceleration>({ mutation: normWireRequired(normWireLiteral("changeBridgeDeckAcceleration")), id: normWireRequired(normWireString), newDeckAcceleration: normWireRequired(normWireNumber) });
