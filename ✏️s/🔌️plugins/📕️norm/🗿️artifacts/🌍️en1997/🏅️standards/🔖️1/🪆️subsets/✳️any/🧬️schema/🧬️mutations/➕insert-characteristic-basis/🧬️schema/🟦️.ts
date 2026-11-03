/** ➕ `insert-characteristic-basis` wire twin: the leaf payload `InsertCharacteristicBasis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997CharacteristicBasis, parseEn1997CharacteristicBasis } from "../../../📸️snapshot/🟦️.ts";

export interface InsertCharacteristicBasis {
  mutation: "insertCharacteristicBasis";
  index: number;
  characteristicBasis: En1997CharacteristicBasis;
}

export const parseInsertCharacteristicBasis: NormWireReader<InsertCharacteristicBasis> = normWireObject<InsertCharacteristicBasis>({ mutation: normWireRequired(normWireLiteral("insertCharacteristicBasis")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), characteristicBasis: normWireRequired(parseEn1997CharacteristicBasis) });
