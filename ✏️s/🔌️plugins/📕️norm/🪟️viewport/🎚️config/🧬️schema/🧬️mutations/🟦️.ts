import type { ChangeViewportView } from "./🎥️change-viewport-view/🟦️.ts";
import type { NormViewportWindowConfig } from "../🟦️.ts";

/** 🎚️ The closed wire vocabulary for a norm viewport window's local configuration. */
export type NormViewportWindowConfigMutation = { readonly ChangeViewportView: ChangeViewportView };

export function applyNormViewportWindowConfigMutation(_base: NormViewportWindowConfig, mutation: NormViewportWindowConfigMutation): NormViewportWindowConfig {
  const view = mutation.ChangeViewportView;
  return {
    cameraPreset: view.cameraPreset ?? undefined,
    hiddenLayers: [...(view.hiddenLayers ?? [])],
    colourMap: view.colourMap ?? undefined,
    unitSystem: view.unitSystem ?? undefined,
  };
}
