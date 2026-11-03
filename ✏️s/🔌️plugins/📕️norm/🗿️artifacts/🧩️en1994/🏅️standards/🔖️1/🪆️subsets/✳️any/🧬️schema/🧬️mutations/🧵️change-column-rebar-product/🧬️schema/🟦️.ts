/** 📉️ `change-column-rebar-product` wire twin: the leaf payload `ChangeColumnRebarProduct`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ReinforcementProduct, parseReinforcementProduct } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeColumnRebarProduct {
  id: string;
  newReinforcementProduct: ReinforcementProduct;
}

export const parseChangeColumnRebarProduct: NormWireReader<ChangeColumnRebarProduct> = normWireObject<ChangeColumnRebarProduct>({ id: normWireRequired(normWireString), newReinforcementProduct: normWireRequired(parseReinforcementProduct) });
