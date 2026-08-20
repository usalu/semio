/** @module Interface semio:framework/ui@1.0.0 **/
export type InstanceId = import('./semio-framework-types.js').InstanceId;
export interface SurfaceRef {
  instance: InstanceId,
  surface: number,
}
export type Revision = import('./semio-framework-types.js').Revision;
export type Pack = import('./semio-framework-types.js').Pack;
export interface PatchReplace {
  path: Uint32Array,
  node: Pack,
}
export interface PatchInsertChild {
  path: Uint32Array,
  index: number,
  node: Pack,
}
export interface PatchRemoveChild {
  path: Uint32Array,
  index: number,
}
export interface PatchSetProps {
  path: Uint32Array,
  props: Pack,
}
export type PatchOp = PatchOpReplace | PatchOpInsertChild | PatchOpRemoveChild | PatchOpSetProps;
export interface PatchOpReplace {
  tag: 'replace',
  val: PatchReplace,
}
export interface PatchOpInsertChild {
  tag: 'insert-child',
  val: PatchInsertChild,
}
export interface PatchOpRemoveChild {
  tag: 'remove-child',
  val: PatchRemoveChild,
}
export interface PatchOpSetProps {
  tag: 'set-props',
  val: PatchSetProps,
}
export interface UiPatch {
  surface: SurfaceRef,
  kind: string,
  revision: Revision,
  baseRevision: Revision,
  ops: Array<PatchOp>,
}
