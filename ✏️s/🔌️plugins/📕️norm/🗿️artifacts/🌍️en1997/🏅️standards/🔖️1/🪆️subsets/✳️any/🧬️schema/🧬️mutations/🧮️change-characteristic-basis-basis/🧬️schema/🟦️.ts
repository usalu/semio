/** 🧮 `change-characteristic-basis-basis` wire twin: the leaf payload `ChangeCharacteristicBasisBasis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCharacteristicBasisBasis {
  mutation: "changeCharacteristicBasisBasis";
  id: string;
  newBasis: "declared" | "derived";
}

export const parseChangeCharacteristicBasisBasis: NormWireReader<ChangeCharacteristicBasisBasis> = normWireObject<ChangeCharacteristicBasisBasis>({ mutation: normWireRequired(normWireLiteral("changeCharacteristicBasisBasis")), id: normWireRequired(normWireString), newBasis: normWireRequired(normWireLiteral("declared", "derived")) });
