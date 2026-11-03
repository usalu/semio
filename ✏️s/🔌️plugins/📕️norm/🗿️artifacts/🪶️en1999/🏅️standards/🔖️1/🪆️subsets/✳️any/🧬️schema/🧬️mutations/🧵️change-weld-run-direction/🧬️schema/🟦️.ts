/** 🧵️ `change-weld-run-direction` wire twin: the leaf payload `ChangeWeldRunDirection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WeldDirection, parseWeldDirection } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeWeldRunDirection {
  mutation: "changeWeldRunDirection";
  weldRunId: string;
  newDirection: WeldDirection;
}

export const parseChangeWeldRunDirection: NormWireReader<ChangeWeldRunDirection> = normWireObject<ChangeWeldRunDirection>({ mutation: normWireRequired(normWireLiteral("changeWeldRunDirection")), weldRunId: normWireRequired(normWireString), newDirection: normWireRequired(parseWeldDirection) });
