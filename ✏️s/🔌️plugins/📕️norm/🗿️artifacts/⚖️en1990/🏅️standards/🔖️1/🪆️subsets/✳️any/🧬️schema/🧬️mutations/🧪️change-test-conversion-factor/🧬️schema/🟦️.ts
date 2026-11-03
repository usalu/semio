/** 🧪 `change-test-conversion-factor` wire twin: the leaf payload `ChangeTestConversionFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTestConversionFactor {
  mutation: "changeTestConversionFactor";
  id: string;
  newConversionFactor: number;
}

export const parseChangeTestConversionFactor: NormWireReader<ChangeTestConversionFactor> = normWireObject<ChangeTestConversionFactor>({ mutation: normWireRequired(normWireLiteral("changeTestConversionFactor")), id: normWireRequired(normWireString), newConversionFactor: normWireRequired(normWireNumber) });
