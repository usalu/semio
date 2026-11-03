/** 🌉️ `change-deck-loss-moisture` wire twin: the leaf payload `ChangeDeckLossMoisture`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckLossMoisture {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckLossMoisture: NormWireReader<ChangeDeckLossMoisture> = normWireObject<ChangeDeckLossMoisture>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
