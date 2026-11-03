/** ⚗️ `change-material-product-form` wire twin: the leaf payload `ChangeMaterialProductForm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ProductForm, parseProductForm } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeMaterialProductForm {
  mutation: "changeMaterialProductForm";
  materialId: string;
  newProductForm: ProductForm;
}

export const parseChangeMaterialProductForm: NormWireReader<ChangeMaterialProductForm> = normWireObject<ChangeMaterialProductForm>({ mutation: normWireRequired(normWireLiteral("changeMaterialProductForm")), materialId: normWireRequired(normWireString), newProductForm: normWireRequired(parseProductForm) });
