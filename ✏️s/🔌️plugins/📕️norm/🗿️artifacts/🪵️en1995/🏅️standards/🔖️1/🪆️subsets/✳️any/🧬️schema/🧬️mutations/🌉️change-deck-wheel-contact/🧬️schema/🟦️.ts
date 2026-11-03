/** 🌉️ `change-deck-wheel-contact` wire twin: the leaf payload `ChangeDeckWheelContact`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckWheelContact {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckWheelContact: NormWireReader<ChangeDeckWheelContact> = normWireObject<ChangeDeckWheelContact>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
