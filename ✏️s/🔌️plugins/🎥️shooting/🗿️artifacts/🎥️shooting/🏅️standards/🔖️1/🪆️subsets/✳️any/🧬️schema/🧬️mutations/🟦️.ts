/** 🎥️ Shooting direct-mutation discriminated union. */
import type { ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingShot } from "../📸️snapshot/🟦️.ts";

/** 🌱 Brings a new asset into existence (append-only apply). */
export interface CreateAsset {
  asset: ShootingAsset;
  index: number | null;
}

/** 🗑️ Removes an asset by id; inverse recreates it. */
export interface DeleteAsset {
  id: string;
}

/** ✏️ Changes an asset's identity `name` field. */
export interface RenameAsset {
  id: string;
  newName: string;
}

/** 🔗 Sets an asset's mesh `url`. */
export interface ChangeAssetUrl {
  id: string;
  newUrl: string;
}

/** 🔀 Repositions an asset within the display-ordered `assets` list. */
export interface ReorderAssets {
  id: string;
  toIndex: number;
}

/** ↔️ The bulk relative-offset gesture over multiple assets (gumball drag). */
export interface DragAssets {
  assetIds: string[];
  dx: number;
  dy: number;
  dz: number;
}

/** 🔄 The bulk axis-angle rotation gesture over multiple assets. */
export interface RotateAssets {
  assetIds: string[];
  ax: number;
  ay: number;
  az: number;
  angle: number;
}

/** ↕️ The bulk multiplicative-scale gesture over multiple assets. */
export interface ScaleAssets {
  assetIds: string[];
  sx: number;
  sy: number;
  sz: number;
}

/** 📸 Brings a new shot into existence (append-only apply). */
export interface CreateShot {
  shot: ShootingShot;
  index: number | null;
}

/** 🚮 Removes a shot by id; inverse recreates it. */
export interface DeleteShot {
  id: string;
}

/** 🏷️ Changes a shot's identity `label` field. */
export interface RenameShot {
  id: string;
  newLabel: string;
}

/** 📏 Sets a shot's render `width`. */
export interface ChangeShotWidth {
  id: string;
  newWidth: number;
}

/** 📐 Sets a shot's render `height`. */
export interface ChangeShotHeight {
  id: string;
  newHeight: number;
}

/** 🖼️ Sets a shot's export `format`. */
export interface ChangeShotFormat {
  id: string;
  newFormat: string;
}

/** ✂️ Sets a shot's crop `shape`. */
export interface ChangeShotShape {
  id: string;
  newShape: string;
}

/** 🔃 Repositions a shot within the display-ordered `shots` list. */
export interface ReorderShots {
  id: string;
  toIndex: number;
}

/** 📷 Overwrites the saved camera `shotId` references with a new pose. */
export interface ReplaceShotCamera {
  shotId: string;
  newCamera: ShootingCamera;
}

/** 🎥 Brings a new saved camera into existence (append-only apply). */
export interface CreateSavedCamera {
  savedCamera: ShootingSavedCamera;
  index: number | null;
}

/** 🧹 Removes a saved camera by id; inverse recreates it. */
export interface DeleteSavedCamera {
  id: string;
}

/** 🪪 Changes a saved camera's identity `label` field. */
export interface RenameSavedCamera {
  id: string;
  newLabel: string;
}

/** 🎞️ Whole-value swap of a saved camera's `camera` pose. */
export interface ReplaceSavedCameraView {
  id: string;
  newCamera: ShootingCamera;
}

/** 🔁 Repositions a saved camera within the display-ordered `savedCameras` list. */
export interface ReorderSavedCameras {
  id: string;
  toIndex: number;
}

/** 🎯 A narrow addressed single-field setter for the document's active shot. */
export interface SetActiveShot {
  shotId: string | null;
}

/** 📌 A narrow addressed single-field setter for the document's active asset. */
export interface SetActiveAsset {
  assetId: string | null;
}

/** ☀️ One of the scene's independently-settable fields — toggles the sun. */
export interface ChangeSceneSunEnabled {
  newEnabled: boolean;
}

/** 🧭 One of the scene's independently-settable fields — the sun's azimuth. */
export interface ChangeSceneSunAzimuth {
  newAzimuth: number;
}

/** 🌅 One of the scene's independently-settable fields — the sun's elevation. */
export interface ChangeSceneSunElevation {
  newElevation: number;
}

/** 💡 One of the scene's independently-settable fields — the sun's intensity. */
export interface ChangeSceneSunIntensity {
  newIntensity: number;
}

/** 🔅️ One of the scene's independently-settable fields — the ambient light intensity. */
export interface ChangeSceneAmbientIntensity {
  newIntensity: number;
}

/** 🌑 One of the scene's independently-settable fields — toggles shadows. */
export interface ChangeSceneShadowEnabled {
  newEnabled: boolean;
}

/** 🪨 One of the scene's independently-settable fields — the material roughness. */
export interface ChangeSceneMaterialRoughness {
  newRoughness: number;
}

export type ShootingMutation =
  | ({ mutation: "createAsset" } & CreateAsset)
  | ({ mutation: "deleteAsset" } & DeleteAsset)
  | ({ mutation: "renameAsset" } & RenameAsset)
  | ({ mutation: "changeAssetUrl" } & ChangeAssetUrl)
  | ({ mutation: "reorderAssets" } & ReorderAssets)
  | ({ mutation: "dragAssets" } & DragAssets)
  | ({ mutation: "rotateAssets" } & RotateAssets)
  | ({ mutation: "scaleAssets" } & ScaleAssets)
  | ({ mutation: "createShot" } & CreateShot)
  | ({ mutation: "deleteShot" } & DeleteShot)
  | ({ mutation: "renameShot" } & RenameShot)
  | ({ mutation: "changeShotWidth" } & ChangeShotWidth)
  | ({ mutation: "changeShotHeight" } & ChangeShotHeight)
  | ({ mutation: "changeShotFormat" } & ChangeShotFormat)
  | ({ mutation: "changeShotShape" } & ChangeShotShape)
  | ({ mutation: "reorderShots" } & ReorderShots)
  | ({ mutation: "replaceShotCamera" } & ReplaceShotCamera)
  | ({ mutation: "createSavedCamera" } & CreateSavedCamera)
  | ({ mutation: "deleteSavedCamera" } & DeleteSavedCamera)
  | ({ mutation: "renameSavedCamera" } & RenameSavedCamera)
  | ({ mutation: "replaceSavedCameraView" } & ReplaceSavedCameraView)
  | ({ mutation: "reorderSavedCameras" } & ReorderSavedCameras)
  | ({ mutation: "setActiveShot" } & SetActiveShot)
  | ({ mutation: "setActiveAsset" } & SetActiveAsset)
  | ({ mutation: "changeSceneSunEnabled" } & ChangeSceneSunEnabled)
  | ({ mutation: "changeSceneSunAzimuth" } & ChangeSceneSunAzimuth)
  | ({ mutation: "changeSceneSunElevation" } & ChangeSceneSunElevation)
  | ({ mutation: "changeSceneSunIntensity" } & ChangeSceneSunIntensity)
  | ({ mutation: "changeSceneAmbientIntensity" } & ChangeSceneAmbientIntensity)
  | ({ mutation: "changeSceneShadowEnabled" } & ChangeSceneShadowEnabled)
  | ({ mutation: "changeSceneMaterialRoughness" } & ChangeSceneMaterialRoughness);
