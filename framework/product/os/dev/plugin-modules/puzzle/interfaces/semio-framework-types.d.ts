/** @module Interface semio:framework/types **/
export interface ProgramManifestJson {
  json: string,
}
export interface ActionInvocationJson {
  json: string,
}
export interface CommandInvocationJson {
  json: string,
}
export interface InvocationContextJson {
  json: string,
}
export interface InvocationResponseJson {
  json: string,
}
export interface WindowInputJson {
  json: string,
}
export interface WindowOutputJson {
  json: string,
}
export interface UiRefreshRequestJson {
  json: string,
}
export interface UiRefreshResponseJson {
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
export interface DocumentTextFiles {
  dsl: string,
  ops: string,
}
export interface DocumentPackFiles {
  pack: Uint8Array,
  ops: string,
}
export interface MediaArtifact {
  descriptorJson: string,
  data: Uint8Array,
}
export type ProgramError = ProgramErrorMessage;
export interface ProgramErrorMessage {
  tag: 'message',
  val: string,
}
