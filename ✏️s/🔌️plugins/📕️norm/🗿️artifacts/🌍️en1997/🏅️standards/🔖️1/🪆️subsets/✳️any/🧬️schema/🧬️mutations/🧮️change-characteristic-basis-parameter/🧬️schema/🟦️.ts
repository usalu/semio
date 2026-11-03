/** 🧮 `change-characteristic-basis-parameter` wire twin: the leaf payload `ChangeCharacteristicBasisParameter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCharacteristicBasisParameter {
  mutation: "changeCharacteristicBasisParameter";
  id: string;
  newParameter: "undrainedStrength" | "frictionAngle";
}

export const parseChangeCharacteristicBasisParameter: NormWireReader<ChangeCharacteristicBasisParameter> = normWireObject<ChangeCharacteristicBasisParameter>({ mutation: normWireRequired(normWireLiteral("changeCharacteristicBasisParameter")), id: normWireRequired(normWireString), newParameter: normWireRequired(normWireLiteral("undrainedStrength", "frictionAngle")) });
