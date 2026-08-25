/** @module Interface semio:framework/ui@1.0.0 **/
export type InstanceId = import('./semio-framework-types.js').InstanceId;
export interface SurfaceRef {
  instance: InstanceId,
  surface: number,
}
export type Revision = import('./semio-framework-types.js').Revision;
export type Pack = import('./semio-framework-types.js').Pack;
export interface PatchUpsert {
  node: Pack,
}
export type NodeId = bigint;
export interface PatchSetComponent {
  node: NodeId,
  component: Pack,
}
export interface PatchSetLayout {
  node: NodeId,
  layout: Pack,
}
export interface PatchSetActivity {
  node: NodeId,
  activity: Pack,
}
export interface PatchSetChildren {
  node: NodeId,
  children: BigUint64Array,
}
export interface PatchSetStyle {
  node: NodeId,
  style: Pack,
}
export interface PatchSetAccessibility {
  node: NodeId,
  accessibility: Pack,
}
export interface PatchSetBindings {
  node: NodeId,
  bindings: Pack,
}
export interface PatchSetMenu {
  node: NodeId,
  menu: Pack,
}
export type PatchOp = PatchOpUpsert | PatchOpSetComponent | PatchOpSetLayout | PatchOpSetActivity | PatchOpSetChildren | PatchOpSetStyle | PatchOpSetAccessibility | PatchOpSetBindings | PatchOpSetMenu | PatchOpRemove | PatchOpSetRoot;
export interface PatchOpUpsert {
  tag: 'upsert',
  val: PatchUpsert,
}
export interface PatchOpSetComponent {
  tag: 'set-component',
  val: PatchSetComponent,
}
export interface PatchOpSetLayout {
  tag: 'set-layout',
  val: PatchSetLayout,
}
export interface PatchOpSetActivity {
  tag: 'set-activity',
  val: PatchSetActivity,
}
export interface PatchOpSetChildren {
  tag: 'set-children',
  val: PatchSetChildren,
}
export interface PatchOpSetStyle {
  tag: 'set-style',
  val: PatchSetStyle,
}
export interface PatchOpSetAccessibility {
  tag: 'set-accessibility',
  val: PatchSetAccessibility,
}
export interface PatchOpSetBindings {
  tag: 'set-bindings',
  val: PatchSetBindings,
}
export interface PatchOpSetMenu {
  tag: 'set-menu',
  val: PatchSetMenu,
}
export interface PatchOpRemove {
  tag: 'remove',
  val: NodeId,
}
export interface PatchOpSetRoot {
  tag: 'set-root',
  val: NodeId,
}
export interface UiPatch {
  surface: SurfaceRef,
  revision: Revision,
  baseRevision: Revision,
  ops: Array<PatchOp>,
}
