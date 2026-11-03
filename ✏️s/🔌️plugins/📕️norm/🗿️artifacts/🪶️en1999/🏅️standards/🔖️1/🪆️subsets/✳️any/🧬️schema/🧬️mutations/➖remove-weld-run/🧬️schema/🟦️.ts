/** 🗂️ `remove-weld-run` wire twin: the leaf payload `RemoveWeldRun`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveWeldRun {
  mutation: "removeWeldRun";
  id: string;
}

export const parseRemoveWeldRun: NormWireReader<RemoveWeldRun> = normWireObject<RemoveWeldRun>({ mutation: normWireRequired(normWireLiteral("removeWeldRun")), id: normWireRequired(normWireString) });
