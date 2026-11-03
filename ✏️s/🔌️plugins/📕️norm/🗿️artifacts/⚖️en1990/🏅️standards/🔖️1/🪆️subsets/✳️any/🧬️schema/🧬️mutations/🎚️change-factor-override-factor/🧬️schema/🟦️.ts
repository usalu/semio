/** 🎚 `change-factor-override-factor` wire twin: the leaf payload `ChangeFactorOverrideFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFactorOverrideFactor {
  mutation: "changeFactorOverrideFactor";
  id: string;
  newFactor: string;
}

export const parseChangeFactorOverrideFactor: NormWireReader<ChangeFactorOverrideFactor> = normWireObject<ChangeFactorOverrideFactor>({ mutation: normWireRequired(normWireLiteral("changeFactorOverrideFactor")), id: normWireRequired(normWireString), newFactor: normWireRequired(normWireString) });
