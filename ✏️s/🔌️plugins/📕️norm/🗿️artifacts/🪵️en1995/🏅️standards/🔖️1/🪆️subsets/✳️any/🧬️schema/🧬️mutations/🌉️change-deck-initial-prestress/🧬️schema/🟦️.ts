/** 🌉️ `change-deck-initial-prestress` wire twin: the leaf payload `ChangeDeckInitialPrestress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckInitialPrestress {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckInitialPrestress: NormWireReader<ChangeDeckInitialPrestress> = normWireObject<ChangeDeckInitialPrestress>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
