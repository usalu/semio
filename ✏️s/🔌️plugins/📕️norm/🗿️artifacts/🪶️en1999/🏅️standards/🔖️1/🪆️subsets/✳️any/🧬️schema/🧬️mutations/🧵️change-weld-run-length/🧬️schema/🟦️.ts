/** 🧵️ `change-weld-run-length` wire twin: the leaf payload `ChangeWeldRunLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldRunLength {
  mutation: "changeWeldRunLength";
  weldRunId: string;
  newLength: number;
}

export const parseChangeWeldRunLength: NormWireReader<ChangeWeldRunLength> = normWireObject<ChangeWeldRunLength>({ mutation: normWireRequired(normWireLiteral("changeWeldRunLength")), weldRunId: normWireRequired(normWireString), newLength: normWireRequired(normWireNumber) });
