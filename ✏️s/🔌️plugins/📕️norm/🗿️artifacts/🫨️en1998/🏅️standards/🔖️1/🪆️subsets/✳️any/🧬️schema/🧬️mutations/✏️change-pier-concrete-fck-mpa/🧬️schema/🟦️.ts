/** ✏️ `change-pier-concrete-fck-mpa` wire twin: the leaf payload `ChangePierConcreteFckMpa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierConcreteFckMpa {
  mutation: "changePierConcreteFckMpa";
  id: string;
  newConcreteFckMpa: number;
}

export const parseChangePierConcreteFckMpa: NormWireReader<ChangePierConcreteFckMpa> = normWireObject<ChangePierConcreteFckMpa>({ mutation: normWireRequired(normWireLiteral("changePierConcreteFckMpa")), id: normWireRequired(normWireString), newConcreteFckMpa: normWireRequired(normWireNumber) });
