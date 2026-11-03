/** 🧵️ `change-weld-run-throat` wire twin: the leaf payload `ChangeWeldRunThroat`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldRunThroat {
  mutation: "changeWeldRunThroat";
  weldRunId: string;
  newThroat: number;
}

export const parseChangeWeldRunThroat: NormWireReader<ChangeWeldRunThroat> = normWireObject<ChangeWeldRunThroat>({ mutation: normWireRequired(normWireLiteral("changeWeldRunThroat")), weldRunId: normWireRequired(normWireString), newThroat: normWireRequired(normWireNumber) });
