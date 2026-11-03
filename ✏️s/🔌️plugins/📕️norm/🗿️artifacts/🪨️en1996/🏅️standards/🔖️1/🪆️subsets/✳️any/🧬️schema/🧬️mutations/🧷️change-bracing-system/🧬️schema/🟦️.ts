/** 🧷 `change-bracing-system` wire twin: the leaf payload `ChangeBracingSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBracingSystem {
  id: string;
  newBracingSystem: "Unreinforced" | "Confined" | "Reinforced";
}

export const parseChangeBracingSystem: NormWireReader<ChangeBracingSystem> = normWireObject<ChangeBracingSystem>({ id: normWireRequired(normWireString), newBracingSystem: normWireRequired(normWireLiteral("Unreinforced", "Confined", "Reinforced")) });
