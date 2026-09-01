/** 🎥️ Shooting direct-mutation discriminated union. */
import type { ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingShot } from "../📸️snapshot/🟦️component.ts";

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
  new_name: string;
}

/** 🔗 Sets an asset's mesh `url`. */
export interface ChangeAssetUrl {
  id: string;
  new_url: string;
}

/** 🔀 Repositions an asset within the display-ordered `assets` list. */
export interface ReorderAssets {
  id: string;
  to_index: number;
}

/** ↔️ The bulk relative-offset gesture over multiple assets (gumball drag). */
export interface DragAssets {
  asset_ids: string[];
  dx: number;
  dy: number;
  dz: number;
}

/** 🔄 The bulk axis-angle rotation gesture over multiple assets. */
export interface RotateAssets {
  asset_ids: string[];
  ax: number;
  ay: number;
  az: number;
  angle: number;
}

/** ↕️ The bulk multiplicative-scale gesture over multiple assets. */
export interface ScaleAssets {
  asset_ids: string[];
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
  new_label: string;
}

/** 📏 Sets a shot's render `width`. */
export interface ChangeShotWidth {
  id: string;
  new_width: number;
}

/** 📐 Sets a shot's render `height`. */
export interface ChangeShotHeight {
  id: string;
  new_height: number;
}

/** 🖼️ Sets a shot's export `format`. */
export interface ChangeShotFormat {
  id: string;
  new_format: string;
}

/** ✂️ Sets a shot's crop `shape`. */
export interface ChangeShotShape {
  id: string;
  new_shape: string;
}

/** 🔃 Repositions a shot within the display-ordered `shots` list. */
export interface ReorderShots {
  id: string;
  to_index: number;
}

/** 📷 Overwrites the saved camera `shot_id` references with a new pose. */
export interface ReplaceShotCamera {
  shot_id: string;
  new_camera: ShootingCamera;
}

/** 🎥 Brings a new saved camera into existence (append-only apply). */
export interface CreateSavedCamera {
  saved_camera: ShootingSavedCamera;
  index: number | null;
}

/** 🧹 Removes a saved camera by id; inverse recreates it. */
export interface DeleteSavedCamera {
  id: string;
}

/** 🪪 Changes a saved camera's identity `label` field. */
export interface RenameSavedCamera {
  id: string;
  new_label: string;
}

/** 🎞️ Whole-value swap of a saved camera's `camera` pose. */
export interface ReplaceSavedCameraView {
  id: string;
  new_camera: ShootingCamera;
}

/** 🔁 Repositions a saved camera within the display-ordered `savedCameras` list. */
export interface ReorderSavedCameras {
  id: string;
  to_index: number;
}

/** 🎯 A narrow addressed single-field setter for the document's active shot. */
export interface SetActiveShot {
  shot_id: string | null;
}

/** 📌 A narrow addressed single-field setter for the document's active asset. */
export interface SetActiveAsset {
  asset_id: string | null;
}

/** ☀️ One of the scene's independently-settable fields — toggles the sun. */
export interface ChangeSceneSunEnabled {
  new_enabled: boolean;
}

/** 🧭 One of the scene's independently-settable fields — the sun's azimuth. */
export interface ChangeSceneSunAzimuth {
  new_azimuth: number;
}

/** 🌅 One of the scene's independently-settable fields — the sun's elevation. */
export interface ChangeSceneSunElevation {
  new_elevation: number;
}

/** 💡 One of the scene's independently-settable fields — the sun's intensity. */
export interface ChangeSceneSunIntensity {
  new_intensity: number;
}

/** 🔅️ One of the scene's independently-settable fields — the ambient light intensity. */
export interface ChangeSceneAmbientIntensity {
  new_intensity: number;
}

/** 🌑 One of the scene's independently-settable fields — toggles shadows. */
export interface ChangeSceneShadowEnabled {
  new_enabled: boolean;
}

/** 🪨 One of the scene's independently-settable fields — the material roughness. */
export interface ChangeSceneMaterialRoughness {
  new_roughness: number;
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
