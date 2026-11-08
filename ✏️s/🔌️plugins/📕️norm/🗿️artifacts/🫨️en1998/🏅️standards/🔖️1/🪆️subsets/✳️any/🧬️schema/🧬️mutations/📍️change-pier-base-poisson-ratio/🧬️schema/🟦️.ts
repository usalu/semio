/** 📍️ `change-pier-base-poisson-ratio` wire twin: the leaf payload `ChangePierBasePoissonRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierBasePoissonRatio {
  mutation: "changePierBasePoissonRatio";
  id: string;
  newPoissonRatio: number;
}

export const parseChangePierBasePoissonRatio: NormWireReader<ChangePierBasePoissonRatio> = normWireObject<ChangePierBasePoissonRatio>({ mutation: normWireRequired(normWireLiteral("changePierBasePoissonRatio")), id: normWireRequired(normWireString), newPoissonRatio: normWireRequired(normWireNumber) });
