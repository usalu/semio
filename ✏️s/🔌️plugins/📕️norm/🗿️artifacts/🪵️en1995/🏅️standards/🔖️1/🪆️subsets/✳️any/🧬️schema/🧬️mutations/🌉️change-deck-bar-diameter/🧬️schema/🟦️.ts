/** 🌉️ `change-deck-bar-diameter` wire twin: the leaf payload `ChangeDeckBarDiameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckBarDiameter {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckBarDiameter: NormWireReader<ChangeDeckBarDiameter> = normWireObject<ChangeDeckBarDiameter>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
