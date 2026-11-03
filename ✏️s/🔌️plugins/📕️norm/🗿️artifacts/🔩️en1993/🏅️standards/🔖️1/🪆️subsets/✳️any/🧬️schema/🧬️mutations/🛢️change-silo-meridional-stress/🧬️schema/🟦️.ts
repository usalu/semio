/** 🛢️ `change-silo-meridional-stress` wire twin: the leaf payload `ChangeSiloMeridionalStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSiloMeridionalStress {
  id: string;
  newMeridionalStress: number;
}

export const parseChangeSiloMeridionalStress: NormWireReader<ChangeSiloMeridionalStress> = normWireObject<ChangeSiloMeridionalStress>({ id: normWireRequired(normWireString), newMeridionalStress: normWireRequired(normWireNumber) });
