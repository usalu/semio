/** 🎚 `change-factor-override-value` wire twin: the leaf payload `ChangeFactorOverrideValue`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFactorOverrideValue {
  mutation: "changeFactorOverrideValue";
  id: string;
  newValue: number;
}

export const parseChangeFactorOverrideValue: NormWireReader<ChangeFactorOverrideValue> = normWireObject<ChangeFactorOverrideValue>({ mutation: normWireRequired(normWireLiteral("changeFactorOverrideValue")), id: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
