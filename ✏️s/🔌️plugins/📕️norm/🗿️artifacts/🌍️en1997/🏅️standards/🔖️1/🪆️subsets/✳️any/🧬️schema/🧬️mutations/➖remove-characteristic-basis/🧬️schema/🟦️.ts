/** ➖ `remove-characteristic-basis` wire twin: the leaf payload `RemoveCharacteristicBasis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveCharacteristicBasis {
  mutation: "removeCharacteristicBasis";
  index: number;
}

export const parseRemoveCharacteristicBasis: NormWireReader<RemoveCharacteristicBasis> = normWireObject<RemoveCharacteristicBasis>({ mutation: normWireRequired(normWireLiteral("removeCharacteristicBasis")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
