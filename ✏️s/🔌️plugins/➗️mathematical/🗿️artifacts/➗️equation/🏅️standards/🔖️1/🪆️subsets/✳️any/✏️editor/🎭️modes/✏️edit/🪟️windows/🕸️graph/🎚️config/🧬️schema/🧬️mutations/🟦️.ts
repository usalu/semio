import type { EquationCamera, EquationGraphWindowConfig } from "../🟦️";

export type EquationGraphWindowConfigMutation = { kind: "set-camera"; camera: EquationCamera };

export function applyEquationGraphWindowConfigMutation(
  state: EquationGraphWindowConfig,
  mutation: EquationGraphWindowConfigMutation,
): EquationGraphWindowConfig {
  return { ...state, camera: mutation.camera };
}
