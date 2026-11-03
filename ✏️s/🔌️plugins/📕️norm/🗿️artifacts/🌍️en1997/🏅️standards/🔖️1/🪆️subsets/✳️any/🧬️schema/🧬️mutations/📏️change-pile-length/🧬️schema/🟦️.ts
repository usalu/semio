/** 📏️ `change-pile-length` wire twin: the leaf payload `ChangePileLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileLength {
  mutation: "changePileLength";
  id: string;
  newLength: number;
}

export const parseChangePileLength: NormWireReader<ChangePileLength> = normWireObject<ChangePileLength>({ mutation: normWireRequired(normWireLiteral("changePileLength")), id: normWireRequired(normWireString), newLength: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
