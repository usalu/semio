/** 🌉️ `change-deck-loss-relaxation` wire twin: the leaf payload `ChangeDeckLossRelaxation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckLossRelaxation {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckLossRelaxation: NormWireReader<ChangeDeckLossRelaxation> = normWireObject<ChangeDeckLossRelaxation>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
