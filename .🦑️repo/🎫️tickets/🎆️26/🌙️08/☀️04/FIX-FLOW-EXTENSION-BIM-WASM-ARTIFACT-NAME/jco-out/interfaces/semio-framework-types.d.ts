/** @module Interface semio:framework/types **/
export interface MigrateDocumentInput {
  fromVersion: string,
  toVersion: string,
  data: Uint8Array,
}
export interface MigrateDocumentOutput {
  data: Uint8Array,
}
export type PluginError = PluginErrorMessage;
export interface PluginErrorMessage {
  tag: 'message',
  val: string,
}
