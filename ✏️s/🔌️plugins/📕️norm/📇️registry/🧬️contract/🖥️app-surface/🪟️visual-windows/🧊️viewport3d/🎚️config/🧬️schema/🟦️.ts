/** 🎚️ `Viewport3dWindowConfig` wire twin, mirrors `../🦀️.rs` (canonical JSON: camelCase keys, unit enums as camelCase strings). Vec3, LocalizedCopy and LoadKind come from the structural scene schema. */
import type { LoadKind, LocalizedCopy, Vec3 } from "../../../../../../../🎬️structural-scene/🧬️schema/🟦️.ts";

export type CameraPreset = "iso" | "plan" | "elevationFront" | "elevationBack" | "elevationLeft" | "elevationRight";
export type ColourRamp = "viridis" | "inferno" | "coolwarm" | "grayscale";
export type LabelDensity = "off" | "low" | "normal" | "high";
export type LengthUnit = "millimetre" | "metre";
export type DeviceClass = "desktop" | "mobile" | "tablet";

export interface SectionPlane {
  origin: Vec3;
  normal: Vec3;
  cap: boolean;
}

export interface SavedView {
  name: LocalizedCopy;
  position: Vec3;
  target: Vec3;
}

export interface Viewport3dWindowConfig {
  preset: CameraPreset;
  ramp: ColourRamp;
  domainMax: number;
  sun: boolean;
  hiddenLayers: string[];
  soloLayer?: string;
  hiddenLoadKinds: LoadKind[];
  loadGlyphScale: number;
  deformationScale: number;
  deformationPhase: number;
  playbackSpeed: number;
  activeField?: string;
  section?: SectionPlane;
  labelDensity: LabelDensity;
  showDimensions: boolean;
  unit: LengthUnit;
  device: DeviceClass;
  savedViews: SavedView[];
}
