/** 🦴️ `change-conn-hole-stagger` wire twin: the leaf payload `ChangeConnHoleStagger`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnHoleStagger {
  mutation: "changeConnHoleStagger";
  aluminiumConnectionId: string;
  newHoleStagger: number;
}

export const parseChangeConnHoleStagger: NormWireReader<ChangeConnHoleStagger> = normWireObject<ChangeConnHoleStagger>({ mutation: normWireRequired(normWireLiteral("changeConnHoleStagger")), aluminiumConnectionId: normWireRequired(normWireString), newHoleStagger: normWireRequired(normWireNumber) });
