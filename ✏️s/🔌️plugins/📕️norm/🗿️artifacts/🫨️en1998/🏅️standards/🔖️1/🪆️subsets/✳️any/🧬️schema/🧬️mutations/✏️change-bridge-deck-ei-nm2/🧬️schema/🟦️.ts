/** ✏️ `change-bridge-deck-ei-nm2` wire twin: the leaf payload `ChangeBridgeDeckEiNm2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeDeckEiNm2 {
  mutation: "changeBridgeDeckEiNm2";
  id: string;
  newDeckEiNm2: number;
}

export const parseChangeBridgeDeckEiNm2: NormWireReader<ChangeBridgeDeckEiNm2> = normWireObject<ChangeBridgeDeckEiNm2>({ mutation: normWireRequired(normWireLiteral("changeBridgeDeckEiNm2")), id: normWireRequired(normWireString), newDeckEiNm2: normWireRequired(normWireNumber) });
