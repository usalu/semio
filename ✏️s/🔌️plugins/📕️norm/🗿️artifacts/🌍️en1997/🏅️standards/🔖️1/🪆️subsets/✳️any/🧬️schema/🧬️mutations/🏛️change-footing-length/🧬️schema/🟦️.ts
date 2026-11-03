/** 🏛️ `change-footing-length` wire twin: the leaf payload `ChangeFootingLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFootingLength {
  mutation: "changeFootingLength";
  id: string;
  newLength: number;
}

export const parseChangeFootingLength: NormWireReader<ChangeFootingLength> = normWireObject<ChangeFootingLength>({ mutation: normWireRequired(normWireLiteral("changeFootingLength")), id: normWireRequired(normWireString), newLength: normWireRequired(normWireNumber) });
