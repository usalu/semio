/** @module Interface semio:framework/types@1.0.0 **/
export type Pack = Uint8Array;
export type PluginError = PluginErrorFault;
export interface PluginErrorFault {
  tag: 'fault',
  val: Pack,
}
export type InstanceId = number;
export type Revision = bigint;
export type MessageEndpoint = MessageEndpointShell | MessageEndpointBackbone | MessageEndpointPluginInstance | MessageEndpointExtension | MessageEndpointTopic;
export interface MessageEndpointShell {
  tag: 'shell',
  val: InstanceId,
}
export interface MessageEndpointBackbone {
  tag: 'backbone',
  val: string,
}
export interface MessageEndpointPluginInstance {
  tag: 'plugin-instance',
  val: InstanceId,
}
export interface MessageEndpointExtension {
  tag: 'extension',
  val: string,
}
export interface MessageEndpointTopic {
  tag: 'topic',
  val: string,
}
export type RequestId = bigint;
