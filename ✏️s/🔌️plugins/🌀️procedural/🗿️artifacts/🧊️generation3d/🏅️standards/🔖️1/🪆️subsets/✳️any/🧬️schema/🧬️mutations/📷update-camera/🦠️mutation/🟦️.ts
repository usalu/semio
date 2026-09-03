/** 🔁 generation3d direct `update-camera` payload mirror of `UpdateCamera`. */
export interface CameraJson {
  x: number;
  y: number;
  zoom: number;
}

export interface UpdateCamera {
  camera: CameraJson;
}
