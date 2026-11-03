/** 🔺️ `change-truss-product` wire twin: the leaf payload `ChangeTrussProduct`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type BracingProduct, parseBracingProduct } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeTrussProduct {
  trussId: string;
  newValue: BracingProduct;
}

export const parseChangeTrussProduct: NormWireReader<ChangeTrussProduct> = normWireObject<ChangeTrussProduct>({ trussId: normWireRequired(normWireString), newValue: normWireRequired(parseBracingProduct) });
