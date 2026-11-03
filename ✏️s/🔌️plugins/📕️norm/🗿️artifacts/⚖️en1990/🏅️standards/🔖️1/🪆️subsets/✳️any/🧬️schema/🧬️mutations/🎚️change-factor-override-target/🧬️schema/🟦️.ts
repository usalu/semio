/** 🎚 `change-factor-override-target` wire twin: the leaf payload `ChangeFactorOverrideTarget`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFactorOverrideTarget {
  mutation: "changeFactorOverrideTarget";
  id: string;
  newTarget: string;
}

export const parseChangeFactorOverrideTarget: NormWireReader<ChangeFactorOverrideTarget> = normWireObject<ChangeFactorOverrideTarget>({ mutation: normWireRequired(normWireLiteral("changeFactorOverrideTarget")), id: normWireRequired(normWireString), newTarget: normWireRequired(normWireString) });
