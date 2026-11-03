/** 🕸️ `change-beam-transverse-as-top` wire twin: the leaf payload `ChangeBeamTransverseAsTop`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamTransverseAsTop {
  id: string;
  newTransverseAsTopM2PerM: number;
}

export const parseChangeBeamTransverseAsTop: NormWireReader<ChangeBeamTransverseAsTop> = normWireObject<ChangeBeamTransverseAsTop>({ id: normWireRequired(normWireString), newTransverseAsTopM2PerM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
