/** 🕸️ `change-beam-hogging-rebar-m2-per-m` wire twin: the leaf payload `ChangeBeamHoggingRebarM2PerM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamHoggingRebarM2PerM {
  id: string;
  newAsHoggingM2PerM: number;
}

export const parseChangeBeamHoggingRebarM2PerM: NormWireReader<ChangeBeamHoggingRebarM2PerM> = normWireObject<ChangeBeamHoggingRebarM2PerM>({ id: normWireRequired(normWireString), newAsHoggingM2PerM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
