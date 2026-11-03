/** 📐️ `change-taper-apex-depth` wire twin: the leaf payload `ChangeTaperApexDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTaperApexDepth {
  taperId: string;
  newValue: number;
}

export const parseChangeTaperApexDepth: NormWireReader<ChangeTaperApexDepth> = normWireObject<ChangeTaperApexDepth>({ taperId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
