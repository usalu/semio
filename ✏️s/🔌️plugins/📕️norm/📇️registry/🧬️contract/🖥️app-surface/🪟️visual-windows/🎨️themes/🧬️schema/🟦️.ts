/** 🎨️ `ThemeBundle` wire twin, mirrors `./🦀️.rs` (canonical JSON: camelCase keys, unit enums as camelCase strings). LocalizedCopy and Rgb come from the structural scene schema, LabelDensity from the viewport config. */
import type { LocalizedCopy, Rgb } from "../../../../../../🎬️structural-scene/🧬️schema/🟦️.ts";
import type { LabelDensity } from "../../🧊️viewport3d/🎚️config/🧬️schema/🟦️.ts";

export type ThemeVariant = "light" | "dark" | "highContrast";
export type EdgeStyle = "none" | "thin" | "bold";
export type MotionPolicy = "system" | "reduced" | "full";

export interface ResultRamps {
  stress: string;
  temperature: string;
  pressure: string;
  settlement: string;
}

export interface RoleColour {
  role: string;
  rgb: Rgb;
}

export interface ThemeBundle {
  id: string;
  name: LocalizedCopy;
  variant: ThemeVariant;
  utilisationRamp: string;
  resultRamps: ResultRamps;
  domainMax: number;
  background: Rgb | null;
  roleColours: RoleColour[];
  edgeStyle: EdgeStyle;
  labelDensity: LabelDensity;
  sun: boolean;
  motion: MotionPolicy;
  textScale: number;
}
