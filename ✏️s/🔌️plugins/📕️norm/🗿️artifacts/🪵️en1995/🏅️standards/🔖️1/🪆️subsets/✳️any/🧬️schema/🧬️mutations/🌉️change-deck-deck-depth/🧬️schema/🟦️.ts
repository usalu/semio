/** 🌉️ `change-deck-deck-depth` wire twin: the leaf payload `ChangeDeckDeckDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckDeckDepth {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckDeckDepth: NormWireReader<ChangeDeckDeckDepth> = normWireObject<ChangeDeckDeckDepth>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
