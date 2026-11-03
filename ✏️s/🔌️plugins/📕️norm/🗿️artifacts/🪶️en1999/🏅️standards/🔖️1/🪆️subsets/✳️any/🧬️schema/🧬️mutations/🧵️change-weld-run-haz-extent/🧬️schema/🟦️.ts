/** 🧵️ `change-weld-run-haz-extent` wire twin: the leaf payload `ChangeWeldRunHazExtent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWeldRunHazExtent {
  mutation: "changeWeldRunHazExtent";
  weldRunId: string;
  newHazExtent: number;
}

export const parseChangeWeldRunHazExtent: NormWireReader<ChangeWeldRunHazExtent> = normWireObject<ChangeWeldRunHazExtent>({ mutation: normWireRequired(normWireLiteral("changeWeldRunHazExtent")), weldRunId: normWireRequired(normWireString), newHazExtent: normWireRequired(normWireNumber) });
