/** 🧵️ `change-weld-run-t-junction` wire twin: the leaf payload `ChangeWeldRunTJunction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireBoolean, normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldRunTJunction {
  mutation: "changeWeldRunTJunction";
  weldRunId: string;
  newTJunction: boolean;
}

export const parseChangeWeldRunTJunction: NormWireReader<ChangeWeldRunTJunction> = normWireObject<ChangeWeldRunTJunction>({ mutation: normWireRequired(normWireLiteral("changeWeldRunTJunction")), weldRunId: normWireRequired(normWireString), newTJunction: normWireRequired(normWireBoolean) });
