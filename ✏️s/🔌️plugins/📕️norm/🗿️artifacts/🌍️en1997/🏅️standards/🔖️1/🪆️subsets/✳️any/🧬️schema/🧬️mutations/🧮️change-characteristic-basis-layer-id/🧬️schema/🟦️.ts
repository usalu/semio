/** 🧮 `change-characteristic-basis-layer-id` wire twin: the leaf payload `ChangeCharacteristicBasisLayerId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCharacteristicBasisLayerId {
  mutation: "changeCharacteristicBasisLayerId";
  id: string;
  newLayerId: string;
}

export const parseChangeCharacteristicBasisLayerId: NormWireReader<ChangeCharacteristicBasisLayerId> = normWireObject<ChangeCharacteristicBasisLayerId>({ mutation: normWireRequired(normWireLiteral("changeCharacteristicBasisLayerId")), id: normWireRequired(normWireString), newLayerId: normWireRequired(normWireString) });
