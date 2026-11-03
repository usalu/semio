/** 🌋 `change-seismic-a-ek` wire twin: the leaf payload `ChangeSeismicAEk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSeismicAEk {
  mutation: "changeSeismicAEk";
  id: string;
  newAEk: number;
}

export const parseChangeSeismicAEk: NormWireReader<ChangeSeismicAEk> = normWireObject<ChangeSeismicAEk>({ mutation: normWireRequired(normWireLiteral("changeSeismicAEk")), id: normWireRequired(normWireString), newAEk: normWireRequired(normWireNumber) });
