import { Plane } from "../../../../semio";

/**
 * Interface for any entity that can be transformed in the 3D scene.
 * Every model can have a plane (semio coordinate system).
 * Models that are transformable will show gumball transform controls.
 */
export interface TransformableModel {
  /** Unique identifier for the model */
  guid: string;

  /**
   * The plane defining the model's position and orientation in 3D space.
   * If undefined, the model is not positioned in 3D space.
   */
  plane?: Plane;

  /**
   * Whether the model can be transformed.
   * If false, transform controls will not be shown even if plane exists.
   */
  isTransformable?: boolean;

  /**
   * Whether the model is currently selected.
   */
  isSelected?: boolean;
}

/**
 * Delta representing how a plane has been transformed.
 */
export interface PlaneTransformDelta {
  /** Translation in x, y, z */
  translation?: { x: number; y: number; z: number };

  /** Rotation as quaternion or euler angles */
  rotation?: { x: number; y: number; z: number; w: number };

  /** Scale factor */
  scale?: number;
}

/**
 * Callback for when a model's plane is updated via transform controls.
 */
export type OnPlaneUpdate = (modelGuid: string, newPlane: Plane) => void;

/**
 * Callback for when multiple models' planes are updated simultaneously.
 */
export type OnMultiPlaneUpdate = (updates: Array<{ modelGuid: string; newPlane: Plane }>) => void;
