/** 🌉️ `change-deck-lamination-product` wire twin: the leaf payload `ChangeDeckLaminationProduct`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type LaminationProduct, parseLaminationProduct } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeDeckLaminationProduct {
  deckId: string;
  newValue: LaminationProduct;
}

export const parseChangeDeckLaminationProduct: NormWireReader<ChangeDeckLaminationProduct> = normWireObject<ChangeDeckLaminationProduct>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(parseLaminationProduct) });
