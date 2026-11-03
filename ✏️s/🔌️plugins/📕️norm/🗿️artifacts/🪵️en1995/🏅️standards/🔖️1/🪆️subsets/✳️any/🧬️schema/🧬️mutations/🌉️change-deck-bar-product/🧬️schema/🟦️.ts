/** 🌉️ `change-deck-bar-product` wire twin: the leaf payload `ChangeDeckBarProduct`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type PrestressBarProduct, parsePrestressBarProduct } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeDeckBarProduct {
  deckId: string;
  newValue: PrestressBarProduct;
}

export const parseChangeDeckBarProduct: NormWireReader<ChangeDeckBarProduct> = normWireObject<ChangeDeckBarProduct>({ deckId: normWireRequired(normWireString), newValue: normWireRequired(parsePrestressBarProduct) });
