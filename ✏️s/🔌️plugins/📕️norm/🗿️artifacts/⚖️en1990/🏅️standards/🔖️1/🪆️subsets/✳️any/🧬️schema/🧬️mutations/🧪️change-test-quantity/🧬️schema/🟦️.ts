/** 🧪 `change-test-quantity` wire twin: the leaf payload `ChangeTestQuantity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTestQuantity {
  mutation: "changeTestQuantity";
  id: string;
  newQuantity: string;
}

export const parseChangeTestQuantity: NormWireReader<ChangeTestQuantity> = normWireObject<ChangeTestQuantity>({ mutation: normWireRequired(normWireLiteral("changeTestQuantity")), id: normWireRequired(normWireString), newQuantity: normWireRequired(normWireString) });
