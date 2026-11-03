/** 🌉️ `change-deck-lamination-width` wire twin: the leaf payload `ChangeDeckLaminationWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckLaminationWidth {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckLaminationWidth: NormWireReader<ChangeDeckLaminationWidth> = normWireObject<ChangeDeckLaminationWidth>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
