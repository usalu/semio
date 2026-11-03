/** 🔗️ `change-conn-welds-length` wire twin: the leaf payload `ChangeConnWeldsLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnWeldsLength {
  mutation: "changeConnWeldsLength";
  connectionId: string;
  newWeldsLength: number;
}

export const parseChangeConnWeldsLength: NormWireReader<ChangeConnWeldsLength> = normWireObject<ChangeConnWeldsLength>({ mutation: normWireRequired(normWireLiteral("changeConnWeldsLength")), connectionId: normWireRequired(normWireString), newWeldsLength: normWireRequired(normWireNumber) });
