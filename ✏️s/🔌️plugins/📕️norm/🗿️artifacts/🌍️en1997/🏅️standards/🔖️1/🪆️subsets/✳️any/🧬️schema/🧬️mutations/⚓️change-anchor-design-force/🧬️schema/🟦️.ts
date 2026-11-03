/** ⚓️ `change-anchor-design-force` wire twin: the leaf payload `ChangeAnchorDesignForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAnchorDesignForce {
  mutation: "changeAnchorDesignForce";
  id: string;
  newDesignForce: number;
}

export const parseChangeAnchorDesignForce: NormWireReader<ChangeAnchorDesignForce> = normWireObject<ChangeAnchorDesignForce>({ mutation: normWireRequired(normWireLiteral("changeAnchorDesignForce")), id: normWireRequired(normWireString), newDesignForce: normWireRequired(normWireNumber) });
