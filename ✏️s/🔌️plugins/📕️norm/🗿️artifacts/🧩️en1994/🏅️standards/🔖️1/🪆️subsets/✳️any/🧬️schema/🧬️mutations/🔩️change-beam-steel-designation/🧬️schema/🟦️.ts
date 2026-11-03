/** 🔩️ `change-beam-steel-designation` wire twin: the leaf payload `ChangeBeamSteelDesignation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSteelDesignation {
  id: string;
  newDesignation: string;
}

export const parseChangeBeamSteelDesignation: NormWireReader<ChangeBeamSteelDesignation> = normWireObject<ChangeBeamSteelDesignation>({ id: normWireRequired(normWireString), newDesignation: normWireRequired(normWireString) });
