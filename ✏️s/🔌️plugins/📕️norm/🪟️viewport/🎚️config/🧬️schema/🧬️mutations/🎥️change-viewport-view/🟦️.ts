/** 🎥️ Replaces the viewport view; missing members clear. */
export interface ChangeViewportView {
  readonly cameraPreset?: string | null;
  readonly hiddenLayers?: readonly string[];
  readonly colourMap?: string | null;
  readonly unitSystem?: string | null;
}
