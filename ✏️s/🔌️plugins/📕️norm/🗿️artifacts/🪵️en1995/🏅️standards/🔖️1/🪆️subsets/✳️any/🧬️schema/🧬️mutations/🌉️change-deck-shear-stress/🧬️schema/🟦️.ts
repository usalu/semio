/** 🌉️ `change-deck-shear-stress` wire twin: the leaf payload `ChangeDeckShearStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDeckShearStress {
  deckId: string;
  newValue: number;
}

export const parseChangeDeckShearStress: NormWireReader<ChangeDeckShearStress> = normWireObject<ChangeDeckShearStress>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
