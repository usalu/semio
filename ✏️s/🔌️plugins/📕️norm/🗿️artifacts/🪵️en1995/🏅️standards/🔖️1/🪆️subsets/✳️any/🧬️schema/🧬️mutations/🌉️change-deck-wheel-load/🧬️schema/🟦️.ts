/** 🌉️ `change-deck-wheel-load` wire twin: the leaf payload `ChangeDeckWheelLoad`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckWheelLoad {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckWheelLoad: NormWireReader<ChangeDeckWheelLoad> = normWireObject<ChangeDeckWheelLoad>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
