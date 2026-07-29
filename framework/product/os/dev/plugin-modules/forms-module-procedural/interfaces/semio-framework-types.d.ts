/** @module Interface semio:framework/types **/
export interface PluginManifestJson {
  json: string,
}
export interface ActionInvocationJson {
  json: string,
}
export interface ActionContextJson {
  json: string,
}
export interface ActionResponseJson {
  json: string,
}
export interface WindowInputJson {
  json: string,
}
export interface WindowOutputJson {
  json: string,
}
export interface PluginToolsJson {
  json: string,
}
export interface PluginWindowEngagementsJson {
  json: string,
}
export interface PluginWindowMeasuresJson {
  json: string,
}
export interface AppLabelsJson {
  json: string,
}
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
