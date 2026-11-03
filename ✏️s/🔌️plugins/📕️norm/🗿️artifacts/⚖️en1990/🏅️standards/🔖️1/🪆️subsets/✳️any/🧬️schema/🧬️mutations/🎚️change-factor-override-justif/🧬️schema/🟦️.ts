/** 🎚 `change-factor-override-justif` wire twin: the leaf payload `ChangeFactorOverrideJustif`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFactorOverrideJustif {
  mutation: "changeFactorOverrideJustif";
  id: string;
  newJustification: string;
}

export const parseChangeFactorOverrideJustif: NormWireReader<ChangeFactorOverrideJustif> = normWireObject<ChangeFactorOverrideJustif>({ mutation: normWireRequired(normWireLiteral("changeFactorOverrideJustif")), id: normWireRequired(normWireString), newJustification: normWireRequired(normWireString) });
