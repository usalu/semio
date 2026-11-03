/** 🌉️ `change-deck-bar-resistance` wire twin: the leaf payload `ChangeDeckBarResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckBarResistance {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckBarResistance: NormWireReader<ChangeDeckBarResistance> = normWireObject<ChangeDeckBarResistance>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
