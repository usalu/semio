/** 🏗️ `change-member-mat-design-strength` wire twin: the leaf payload `ChangeMemberMatDesignStrength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberMatDesignStrength {
  mutation: "changeMemberMatDesignStrength";
  id: string;
  newMaterialDesignStrength: number;
}

export const parseChangeMemberMatDesignStrength: NormWireReader<ChangeMemberMatDesignStrength> = normWireObject<ChangeMemberMatDesignStrength>({ mutation: normWireRequired(normWireLiteral("changeMemberMatDesignStrength")), id: normWireRequired(normWireString), newMaterialDesignStrength: normWireRequired(normWireNumber) });
