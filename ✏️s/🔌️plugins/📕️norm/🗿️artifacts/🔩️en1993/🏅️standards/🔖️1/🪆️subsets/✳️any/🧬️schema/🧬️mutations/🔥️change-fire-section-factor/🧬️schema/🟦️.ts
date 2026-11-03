/** 🔥️ `change-fire-section-factor` wire twin: the leaf payload `ChangeFireSectionFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFireSectionFactor {
  id: string;
  newSectionFactor: number;
}

export const parseChangeFireSectionFactor: NormWireReader<ChangeFireSectionFactor> = normWireObject<ChangeFireSectionFactor>({ id: normWireRequired(normWireString), newSectionFactor: normWireRequired(normWireNumber) });
