//#region 🧬️Configuration
/** 🧬️ Gis2dConfig */
export interface Gis2dConfig {
  /** @state config */
  layerVisibility: Record<string, boolean>;
  /** @state config */
  cameraJson: string;
  /** @state config */
  renderMode: string;
  /** @state config */
  vectorStyle: string;
  /** @state config */
  lodMode: string;
  /** @state config */
  layerStrokeScale: Record<string, number>;
  /** @state config */
  locale: string;
}
//#endregion 🧬️Configuration
