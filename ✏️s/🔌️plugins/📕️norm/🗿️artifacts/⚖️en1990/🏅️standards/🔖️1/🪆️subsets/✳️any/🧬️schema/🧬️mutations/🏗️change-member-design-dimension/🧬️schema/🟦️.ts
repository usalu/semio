/** 🏗️ `change-member-design-dimension` wire twin: the leaf payload `ChangeMemberDesignDimension`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMemberDesignDimension {
  mutation: "changeMemberDesignDimension";
  id: string;
  newDesignDimension: number;
}

export const parseChangeMemberDesignDimension: NormWireReader<ChangeMemberDesignDimension> = normWireObject<ChangeMemberDesignDimension>({ mutation: normWireRequired(normWireLiteral("changeMemberDesignDimension")), id: normWireRequired(normWireString), newDesignDimension: normWireRequired(normWireNumber) });
