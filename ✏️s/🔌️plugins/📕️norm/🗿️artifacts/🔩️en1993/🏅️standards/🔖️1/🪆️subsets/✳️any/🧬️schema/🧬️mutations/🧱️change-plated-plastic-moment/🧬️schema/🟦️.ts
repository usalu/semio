/** 🧱️ `change-plated-plastic-moment` wire twin: the leaf payload `ChangePlatedPlasticMoment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedPlasticMoment {
  id: string;
  newPlasticMoment: number;
}

export const parseChangePlatedPlasticMoment: NormWireReader<ChangePlatedPlasticMoment> = normWireObject<ChangePlatedPlasticMoment>({ id: normWireRequired(normWireString), newPlasticMoment: normWireRequired(normWireNumber) });
