/** 🔄️ `change-pile-concrete-class` wire twin: the leaf payload `ChangePileConcreteClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileConcreteClass {
  mutation: "changePileConcreteClass";
  id: string;
  newConcreteClass: "notGiven" | "c1215" | "c1620" | "c2025" | "c2530" | "c3037" | "c3545" | "c4050" | "c4555" | "c5060";
}

export const parseChangePileConcreteClass: NormWireReader<ChangePileConcreteClass> = normWireObject<ChangePileConcreteClass>({ mutation: normWireRequired(normWireLiteral("changePileConcreteClass")), id: normWireRequired(normWireString), newConcreteClass: normWireRequired(normWireLiteral("notGiven", "c1215", "c1620", "c2025", "c2530", "c3037", "c3545", "c4050", "c4555", "c5060")) });
