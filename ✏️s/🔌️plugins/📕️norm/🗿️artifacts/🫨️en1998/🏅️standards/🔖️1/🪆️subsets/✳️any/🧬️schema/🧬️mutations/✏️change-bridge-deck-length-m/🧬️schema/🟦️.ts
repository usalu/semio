/** ✏️ `change-bridge-deck-length-m` wire twin: the leaf payload `ChangeBridgeDeckLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeckLengthM {
  mutation: "changeBridgeDeckLengthM";
  id: string;
  newDeckLengthM: number;
}

export const parseChangeBridgeDeckLengthM: NormWireReader<ChangeBridgeDeckLengthM> = normWireObject<ChangeBridgeDeckLengthM>({ mutation: normWireRequired(normWireLiteral("changeBridgeDeckLengthM")), id: normWireRequired(normWireString), newDeckLengthM: normWireRequired(normWireNumber) });
