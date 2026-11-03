/** 🌉️ `change-deck-bar-spacing` wire twin: the leaf payload `ChangeDeckBarSpacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckBarSpacing {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckBarSpacing: NormWireReader<ChangeDeckBarSpacing> = normWireObject<ChangeDeckBarSpacing>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
