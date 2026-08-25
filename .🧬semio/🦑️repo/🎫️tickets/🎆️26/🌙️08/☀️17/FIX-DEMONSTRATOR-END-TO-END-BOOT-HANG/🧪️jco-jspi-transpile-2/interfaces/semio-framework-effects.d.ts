/** @module Interface semio:framework/effects@1.0.0 **/
export type MessageEndpoint = import('./semio-framework-types.js').MessageEndpoint;
export type Pack = import('./semio-framework-types.js').Pack;
export interface SendMessageEffect {
  target: MessageEndpoint,
  payload: Pack,
}
export interface PublishEventEffect {
  topic: string,
  payload: Pack,
}
export type RequestId = import('./semio-framework-types.js').RequestId;
export interface BlobLoadParams {
  hash: string,
}
export interface BlobLoadEffect {
  req: RequestId,
  params: BlobLoadParams,
}
export interface BlobWriteParams {
  mediaType: Pack,
  bytes: Pack,
}
export interface BlobWriteEffect {
  req: RequestId,
  params: BlobWriteParams,
}
export interface HttpParams {
  method: string,
  url: string,
  headers: Array<[string, string]>,
  body?: Pack,
  streaming: boolean,
}
export interface HttpRequestEffect {
  req: RequestId,
  params: HttpParams,
}
export interface DocumentReadParams {
  doc: bigint,
  lane: string,
}
export interface DocumentReadEffect {
  req: RequestId,
  params: DocumentReadParams,
}
export interface DocumentWriteParams {
  doc: bigint,
  lane: string,
  ops: Pack,
}
export interface DocumentWriteEffect {
  req: RequestId,
  params: DocumentWriteParams,
}
export interface LinkResolveEffect {
  req: RequestId,
  link: Pack,
}
export interface RegistryQueryParams {
  kind: string,
  filter: Pack,
}
export interface RegistryQueryEffect {
  req: RequestId,
  params: RegistryQueryParams,
}
export interface IoComposeParams {
  key: Pack,
  sources: Pack,
}
export interface IoComposeEffect {
  req: RequestId,
  params: IoComposeParams,
}
export interface IoRunParams {
  source: string,
  target: string,
  payload: Pack,
}
export interface IoRunEffect {
  req: RequestId,
  params: IoRunParams,
}
export interface CacheDeriveParams {
  engineId: string,
  input: Pack,
}
export interface CacheDeriveEffect {
  req: RequestId,
  params: CacheDeriveParams,
}
export interface CacheReadParams {
  engineId: string,
  key: Pack,
}
export interface CacheReadEffect {
  req: RequestId,
  params: CacheReadParams,
}
export interface OpenWindowParams {
  kind: string,
  params: Pack,
}
export interface OpenWindowEffect {
  req: RequestId,
  params: OpenWindowParams,
}
export interface CloseWindowEffect {
  window: bigint,
}
export interface DispatchActionParams {
  action: string,
  args?: Pack,
  delayMs: bigint,
}
export interface DispatchActionEffect {
  req: RequestId,
  params: DispatchActionParams,
}
export interface InvokeExtensionParams {
  extensionId: string,
  capability: string,
  payload: Pack,
}
export interface InvokeExtensionEffect {
  req: RequestId,
  params: InvokeExtensionParams,
}
export interface NotifyEffect {
  message: string,
}
export interface ClipboardWriteEffect {
  fragment: Pack,
}
export interface NavigateEffect {
  uri: string,
}
export interface OpenExternalUrlEffect {
  url: string,
}
export interface SetPanelEffect {
  panelJson: string,
}
export interface SetActiveUtilityEffect {
  windowId: string,
  utilityId: string,
}
export interface SetActiveToolEffect {
  toolId: string,
}
export interface PatchWorld3dChromeEffect {
  selectionJson: string,
  vorticesJson?: string,
  documentSelectedIds: Array<string>,
  documentHighlightedIds?: Array<string>,
}
export interface ReplayShellCommandEffect {
  actionId: string,
  args?: Pack,
}
export interface SpawnPluginInstanceParams {
  pluginId: string,
  appId: string,
  osInstanceId?: string,
  label?: string,
  documentJson?: string,
}
export interface SpawnPluginInstanceEffect {
  req: RequestId,
  params: SpawnPluginInstanceParams,
}
export interface OpenPluginInstanceEffect {
  pluginId: string,
  appId: string,
  osInstanceId?: string,
}
export interface OpenDialogParams {
  dialogId: string,
  args?: Pack,
}
export interface OpenDialogEffect {
  req: RequestId,
  params: OpenDialogParams,
}
export interface IconRenderExportEffect {
  items: Pack,
}
export interface DownloadMediaExportEffect {
  filename: string,
  mimeType: string,
  data: string,
  encoding?: string,
}
export interface RequestFileOpenParams {
  accept: string,
  readAs?: string,
  importAction: string,
  multiple: boolean,
}
export interface RequestFileOpenEffect {
  req: RequestId,
  params: RequestFileOpenParams,
}
export interface RequestMediaFramesParams {
  accept: string,
  frameAction: string,
  doneAction: string,
  fallbackAction: string,
  sampleStride: number,
  maxFrames: number,
  maxLongEdgePx: number,
  fpsHint: number,
  payload?: string,
  args?: Pack,
}
export interface RequestMediaFramesEffect {
  req: RequestId,
  params: RequestMediaFramesParams,
}
export interface LoadDocumentEffect {
  docPack: Pack,
  spr: Pack,
}
export interface SetTimerEffect {
  id: bigint,
  afterMs: number,
  repeat: boolean,
}
/**
 * # Variants
 * 
 * ## `"inline"`
 * 
 * ## `"isolated"`
 * 
 * ## `"exclusive"`
 */
export type JobPlacement = 'inline' | 'isolated' | 'exclusive';
export interface SpawnJobEffect {
  job: bigint,
  kind: string,
  input: Pack,
  placement: JobPlacement,
}
export interface CancelJobEffect {
  job: bigint,
}
export type RespondResult = RespondResultOk | RespondResultFault;
export interface RespondResultOk {
  tag: 'ok',
  val: Pack,
}
export interface RespondResultFault {
  tag: 'fault',
  val: Pack,
}
export interface RespondEffect {
  req: RequestId,
  outcome: RespondResult,
}
export interface StorageReadParams {
  key: string,
}
export interface StorageReadEffect {
  req: RequestId,
  params: StorageReadParams,
}
export interface StorageWriteParams {
  key: string,
  value: Pack,
}
export interface StorageWriteEffect {
  req: RequestId,
  params: StorageWriteParams,
}
export interface StorageDeleteParams {
  key: string,
}
export interface StorageDeleteEffect {
  req: RequestId,
  params: StorageDeleteParams,
}
export type CapabilityId = import('./semio-framework-capabilities.js').CapabilityId;
export interface RequestCapabilityParams {
  id: CapabilityId,
  scope: string,
  reason: string,
  optional: boolean,
}
export interface RequestCapabilityEffect {
  req: RequestId,
  params: RequestCapabilityParams,
}
export interface ReleaseCapabilityEffect {
  id: CapabilityId,
}
export interface SubscribeEffect {
  topic: string,
}
export type Effect = EffectSendMessage | EffectPublishEvent | EffectBlobLoad | EffectBlobWrite | EffectHttpRequest | EffectDocumentRead | EffectDocumentWrite | EffectLinkResolve | EffectRegistryQuery | EffectIoCompose | EffectIoRun | EffectCacheDerive | EffectCacheRead | EffectOpenWindow | EffectCloseWindow | EffectDispatchAction | EffectInvokeExtension | EffectNotify | EffectClipboardWrite | EffectNavigate | EffectOpenExternalUrl | EffectSetPanel | EffectSetActiveUtility | EffectSetActiveTool | EffectPatchWorld3dChrome | EffectReplayShellCommand | EffectSpawnPluginInstance | EffectOpenPluginInstance | EffectOpenDialog | EffectIconRenderExport | EffectDownloadMediaExport | EffectRequestFileOpen | EffectRequestMediaFrames | EffectLoadDocument | EffectRequestSync | EffectSetTimer | EffectSpawnJob | EffectCancelJob | EffectRespond | EffectStorageRead | EffectStorageWrite | EffectStorageDelete | EffectRequestCapability | EffectReleaseCapability | EffectSubscribe | EffectUnsubscribe;
export interface EffectSendMessage {
  tag: 'send-message',
  val: SendMessageEffect,
}
export interface EffectPublishEvent {
  tag: 'publish-event',
  val: PublishEventEffect,
}
export interface EffectBlobLoad {
  tag: 'blob-load',
  val: BlobLoadEffect,
}
export interface EffectBlobWrite {
  tag: 'blob-write',
  val: BlobWriteEffect,
}
export interface EffectHttpRequest {
  tag: 'http-request',
  val: HttpRequestEffect,
}
export interface EffectDocumentRead {
  tag: 'document-read',
  val: DocumentReadEffect,
}
export interface EffectDocumentWrite {
  tag: 'document-write',
  val: DocumentWriteEffect,
}
export interface EffectLinkResolve {
  tag: 'link-resolve',
  val: LinkResolveEffect,
}
export interface EffectRegistryQuery {
  tag: 'registry-query',
  val: RegistryQueryEffect,
}
export interface EffectIoCompose {
  tag: 'io-compose',
  val: IoComposeEffect,
}
export interface EffectIoRun {
  tag: 'io-run',
  val: IoRunEffect,
}
export interface EffectCacheDerive {
  tag: 'cache-derive',
  val: CacheDeriveEffect,
}
export interface EffectCacheRead {
  tag: 'cache-read',
  val: CacheReadEffect,
}
export interface EffectOpenWindow {
  tag: 'open-window',
  val: OpenWindowEffect,
}
export interface EffectCloseWindow {
  tag: 'close-window',
  val: CloseWindowEffect,
}
export interface EffectDispatchAction {
  tag: 'dispatch-action',
  val: DispatchActionEffect,
}
export interface EffectInvokeExtension {
  tag: 'invoke-extension',
  val: InvokeExtensionEffect,
}
export interface EffectNotify {
  tag: 'notify',
  val: NotifyEffect,
}
export interface EffectClipboardWrite {
  tag: 'clipboard-write',
  val: ClipboardWriteEffect,
}
export interface EffectNavigate {
  tag: 'navigate',
  val: NavigateEffect,
}
export interface EffectOpenExternalUrl {
  tag: 'open-external-url',
  val: OpenExternalUrlEffect,
}
export interface EffectSetPanel {
  tag: 'set-panel',
  val: SetPanelEffect,
}
export interface EffectSetActiveUtility {
  tag: 'set-active-utility',
  val: SetActiveUtilityEffect,
}
export interface EffectSetActiveTool {
  tag: 'set-active-tool',
  val: SetActiveToolEffect,
}
export interface EffectPatchWorld3dChrome {
  tag: 'patch-world3d-chrome',
  val: PatchWorld3dChromeEffect,
}
export interface EffectReplayShellCommand {
  tag: 'replay-shell-command',
  val: ReplayShellCommandEffect,
}
export interface EffectSpawnPluginInstance {
  tag: 'spawn-plugin-instance',
  val: SpawnPluginInstanceEffect,
}
export interface EffectOpenPluginInstance {
  tag: 'open-plugin-instance',
  val: OpenPluginInstanceEffect,
}
export interface EffectOpenDialog {
  tag: 'open-dialog',
  val: OpenDialogEffect,
}
export interface EffectIconRenderExport {
  tag: 'icon-render-export',
  val: IconRenderExportEffect,
}
export interface EffectDownloadMediaExport {
  tag: 'download-media-export',
  val: DownloadMediaExportEffect,
}
export interface EffectRequestFileOpen {
  tag: 'request-file-open',
  val: RequestFileOpenEffect,
}
export interface EffectRequestMediaFrames {
  tag: 'request-media-frames',
  val: RequestMediaFramesEffect,
}
export interface EffectLoadDocument {
  tag: 'load-document',
  val: LoadDocumentEffect,
}
export interface EffectRequestSync {
  tag: 'request-sync',
}
export interface EffectSetTimer {
  tag: 'set-timer',
  val: SetTimerEffect,
}
export interface EffectSpawnJob {
  tag: 'spawn-job',
  val: SpawnJobEffect,
}
export interface EffectCancelJob {
  tag: 'cancel-job',
  val: CancelJobEffect,
}
export interface EffectRespond {
  tag: 'respond',
  val: RespondEffect,
}
export interface EffectStorageRead {
  tag: 'storage-read',
  val: StorageReadEffect,
}
export interface EffectStorageWrite {
  tag: 'storage-write',
  val: StorageWriteEffect,
}
export interface EffectStorageDelete {
  tag: 'storage-delete',
  val: StorageDeleteEffect,
}
export interface EffectRequestCapability {
  tag: 'request-capability',
  val: RequestCapabilityEffect,
}
export interface EffectReleaseCapability {
  tag: 'release-capability',
  val: ReleaseCapabilityEffect,
}
export interface EffectSubscribe {
  tag: 'subscribe',
  val: SubscribeEffect,
}
export interface EffectUnsubscribe {
  tag: 'unsubscribe',
  val: SubscribeEffect,
}
