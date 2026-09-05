/** 📷 generation2d update-camera payload — mirrors `UpdateCamera` (…/🎛️set-camera/🦠️mutation/🦀️.rs:15-17). */
export interface CameraJson {
  x: number;
  y: number;
  zoom: number;
}

export interface UpdateCamera {
  camera: CameraJson;
}
