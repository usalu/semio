/** 🔸 `change-element-g-value` wire twin: the leaf payload `ChangeElementGValue`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormWireReader, normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ChangeElementGValue {
  mutation: "changeelementgvalue";
  elementId: string;
  newGValue: number;
}

export const parseChangeElementGValue: NormWireReader<ChangeElementGValue> = normWireObject<ChangeElementGValue>({ mutation: normWireRequired(normWireLiteral("changeelementgvalue")), elementId: normWireRequired(normWireString), newGValue: normWireRequired(normWireNumber) });
