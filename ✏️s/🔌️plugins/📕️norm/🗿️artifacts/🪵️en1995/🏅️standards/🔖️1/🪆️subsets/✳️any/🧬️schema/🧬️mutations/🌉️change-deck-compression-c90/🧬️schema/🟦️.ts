/** 🌉️ `change-deck-compression-c90` wire twin: the leaf payload `ChangeDeckCompressionC90`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckCompressionC90 {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckCompressionC90: NormWireReader<ChangeDeckCompressionC90> = normWireObject<ChangeDeckCompressionC90>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
