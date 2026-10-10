/** 🔃️ generation3d direct `rotate-transforms` payload mirror of `RotateTransforms`. */
export interface RotateTransforms {
  targets: string[];
  ax: number;
  ay: number;
  az: number;
  angle: number;
}

/** 🔃️ Enforce the native nonzero rotation axis invariant. */
export function assertRotationAxis(value: Pick<RotateTransforms, "ax" | "ay" | "az">): void {
  if (value.ax === 0 && value.ay === 0 && value.az === 0) throw new TypeError("rotate-transforms: axis-nonzero");
}
