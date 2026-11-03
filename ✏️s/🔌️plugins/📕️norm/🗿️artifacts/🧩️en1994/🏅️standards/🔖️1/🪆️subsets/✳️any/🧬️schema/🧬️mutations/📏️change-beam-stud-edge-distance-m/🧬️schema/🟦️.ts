/** 📏️ `change-beam-stud-edge-distance-m` wire twin: the leaf payload `ChangeBeamStudEdgeDistanceM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamStudEdgeDistanceM {
  id: string;
  newStudEdgeDistanceM: number;
}

export const parseChangeBeamStudEdgeDistanceM: NormWireReader<ChangeBeamStudEdgeDistanceM> = normWireObject<ChangeBeamStudEdgeDistanceM>({ id: normWireRequired(normWireString), newStudEdgeDistanceM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
