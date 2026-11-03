/** 🌉️ `change-deck-span` wire twin: the leaf payload `ChangeDeckSpan`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckSpan {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckSpan: NormWireReader<ChangeDeckSpan> = normWireObject<ChangeDeckSpan>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
