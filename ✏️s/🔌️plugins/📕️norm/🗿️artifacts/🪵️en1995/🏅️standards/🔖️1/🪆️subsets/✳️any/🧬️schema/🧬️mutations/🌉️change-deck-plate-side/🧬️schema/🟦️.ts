/** 🌉️ `change-deck-plate-side` wire twin: the leaf payload `ChangeDeckPlateSide`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckPlateSide {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckPlateSide: NormWireReader<ChangeDeckPlateSide> = normWireObject<ChangeDeckPlateSide>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
