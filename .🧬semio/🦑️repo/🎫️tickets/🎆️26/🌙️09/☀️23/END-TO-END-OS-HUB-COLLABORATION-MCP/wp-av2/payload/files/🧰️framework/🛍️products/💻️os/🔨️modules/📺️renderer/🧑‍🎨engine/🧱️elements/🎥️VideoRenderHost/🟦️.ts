// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/VideoRenderHost/module.ts
/** @emoji 🎥️ `🎥️VideoRenderHost` — the browser host of `Effect::VideoRenderExport` (`🎠️kernel`): a guest has no GPU and no
 * encoder, so it hands the host a `VideoRenderProgram` and the host paints every distinct scene on a 2D canvas, encodes the
 * timeline as H.264 and muxes an MP4 with the raster video tier's first-party writer (`🖌️raster/🎥️video/🟦️.ts`).
 *
 * Two encoder tiers behind the tier's `VideoEncoderPort`: WebCodecs `VideoEncoder` (hardware/platform H.264, AVCC chunks +
 * `avcC` description) whenever the page supports the stream's configuration, otherwise the first-party all-`I_PCM` encoder
 * (exact, larger). Every export is a task the `🧵️TaskManager` lists with frame progress and a cancel control; cancellation is
 * checked before every frame and closes the platform encoder. React-free, so its laws run without a browser
 * (`🧪️tests/🔬️unit/🟦️.ts`), and the wgpu page host reuses it for the same effect.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type VideoRenderPath, type VideoRenderProgram, videoRenderFrameCount, videoRenderProgramProblem } from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import { AvcIntraPcmEncoder, avcConfigurationFromRecord, avcLevelIdc, type AvcDecoderConfiguration, type EncodedVideoSample, type VideoStreamParameters, writeAvcMp4 } from "../../../../../../../🔨️modules/🖌️raster/🎥️video/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🌐️Labels
/** 🌐️ What the host tells the user about an export, in every shell language (English first, German second). */
export const VIDEO_RENDER_EXPORT_TEXT_V1 = {
  started: { en: "Rendering video {filename} — progress and cancel in Task Manager.", de: "Video {filename} wird gerendert — Fortschritt und Abbruch im Task-Manager." },
  done: { en: "Video {filename} exported ({frames} frames).", de: "Video {filename} exportiert ({frames} Bilder)." },
  cancelled: { en: "Video export {filename} cancelled.", de: "Videoexport {filename} abgebrochen." },
  refused: { en: "Video export {filename} refused: the program is invalid ({code}).", de: "Videoexport {filename} abgelehnt: das Programm ist ungültig ({code})." },
  failed: { en: "Video export {filename} failed: {reason}", de: "Videoexport {filename} fehlgeschlagen: {reason}" },
  progress: { en: "Frame {completed} of {total}", de: "Bild {completed} von {total}" },
} as const;

/** 🌐️ One {@link VIDEO_RENDER_EXPORT_TEXT_V1} message in `locale` with its `{placeholders}` filled. */
export function videoRenderExportTextV1(kind: keyof typeof VIDEO_RENDER_EXPORT_TEXT_V1, locale: string, values: Readonly<Record<string, string | number>>): string {
  const template = locale === "de" ? VIDEO_RENDER_EXPORT_TEXT_V1[kind].de : VIDEO_RENDER_EXPORT_TEXT_V1[kind].en;
  return template.replace(/\{(\w+)\}/g, (match, key: string) => (key in values ? String(values[key]) : match));
}
//#endregion 🌐️Labels

//#region 🖌️Paint
/** 🖌️ The 2D context surface this host paints on (`OffscreenCanvasRenderingContext2D` or a DOM canvas's). */
export type VideoRenderContext2dV1 = Pick<CanvasRenderingContext2D, "setTransform" | "clearRect" | "fillRect" | "fill" | "stroke" | "getImageData"> & { fillStyle: unknown; strokeStyle: unknown; lineWidth: number };

/** ✏️ A program path as a `Path2D` (`M`, `L`, `Q`, `C`, `Z` over flat points). */
export function videoRenderPath2dV1(path: VideoRenderPath, create: () => Path2D = () => new Path2D()): Path2D {
  const shape = create();
  let cursor = 0;
  const next = (): number => path.points[cursor++] ?? 0;
  for (const verb of path.verbs) {
    if (verb === "M") shape.moveTo(next(), next());
    else if (verb === "L") shape.lineTo(next(), next());
    else if (verb === "Q") shape.quadraticCurveTo(next(), next(), next(), next());
    else if (verb === "C") shape.bezierCurveTo(next(), next(), next(), next(), next(), next());
    else if (verb === "Z") shape.closePath();
  }
  return shape;
}

/** 🎨️ Straight RGBA 0..=1 as a CSS colour. */
export function videoRenderCssColorV1(color: readonly number[]): string {
  const channel = (value: number | undefined): number => Math.round(Math.min(1, Math.max(0, value ?? 0)) * 255);
  return `rgba(${channel(color[0])}, ${channel(color[1])}, ${channel(color[2])}, ${Math.min(1, Math.max(0, color[3] ?? 1))})`;
}

/** 🖼️ Paints scene `sceneIndex` over the program background, in list order, each op under its own device transform. */
export function drawVideoRenderSceneV1(context: VideoRenderContext2dV1, program: VideoRenderProgram, sceneIndex: number, paths: readonly Path2D[]): void {
  context.setTransform(1, 0, 0, 1, 0, 0);
  context.clearRect(0, 0, program.width, program.height);
  context.fillStyle = videoRenderCssColorV1(program.background);
  context.fillRect(0, 0, program.width, program.height);
  for (const op of program.scenes[sceneIndex]?.ops ?? []) {
    const path = paths[op.path];
    if (!path) continue;
    context.setTransform(op.transform[0], op.transform[1], op.transform[2], op.transform[3], op.transform[4], op.transform[5]);
    if (op.paint === "stroke") {
      context.strokeStyle = videoRenderCssColorV1(op.color);
      context.lineWidth = op.width;
      context.stroke(path);
    } else {
      context.fillStyle = videoRenderCssColorV1(op.color);
      context.fill(path, "nonzero");
    }
  }
}
//#endregion 🖌️Paint

//#region 🎞️Encode
/** 🧩️ Which encoder produced the samples. */
export type VideoRenderEncoderTierV1 = "webcodecs" | "intra-pcm";

/** 🌍️ What the host environment offers; injected so the laws run the whole pipeline without a browser. */
export interface VideoRenderEnvironmentV1 {
  readonly createCanvas: (width: number, height: number) => { readonly context: VideoRenderContext2dV1; readonly source: unknown };
  readonly createPath: () => Path2D;
  readonly webCodecs: WebCodecsVideoEncoderPortV1 | null;
  readonly yieldToHost: () => Promise<void>;
}

/** 🔌️ The slice of WebCodecs this host drives (`VideoEncoder`, `VideoFrame`), injectable for laws. */
export interface WebCodecsVideoEncoderPortV1 {
  isConfigSupported(config: { codec: string; width: number; height: number; framerate: number; avc: { format: "avc" } }): Promise<boolean>;
  open(config: { codec: string; width: number; height: number; framerate: number; avc: { format: "avc" } }, output: (sample: EncodedVideoSample, description: Uint8Array | null) => void, error: (reason: unknown) => void): WebCodecsSessionV1;
}

/** 🎛️ One open platform encoder. */
export interface WebCodecsSessionV1 {
  encode(source: unknown, timestampMicros: number, durationMicros: number, keyFrame: boolean): void;
  readonly queueSize: number;
  flush(): Promise<void>;
  close(): void;
}

/** 📈️ Progress of one render: frames encoded of frames total. */
export type VideoRenderProgressV1 = (completed: number, total: number) => void;

/** 🚨️ Why a render stopped short of a file. */
export class VideoRenderExportErrorV1 extends Error {
  constructor(
    readonly code: "refused" | "cancelled" | "encoder",
    readonly detail: string,
  ) {
    super(`${code}: ${detail}`);
  }
}

/** 📦️ A finished render. */
export interface VideoRenderResultV1 {
  readonly bytes: Uint8Array;
  readonly tier: VideoRenderEncoderTierV1;
  readonly frames: number;
}

/** 🔤️ The WebCodecs codec string asked for: Constrained Baseline at the level the stream needs. */
export function videoRenderWebCodecsCodecV1(parameters: VideoStreamParameters): string {
  return `avc1.42E0${avcLevelIdc(parameters).toString(16).toUpperCase().padStart(2, "0")}`;
}

/** 🎬️ Renders and encodes `program` into MP4 bytes: admission first (`videoRenderProgramProblem`), then one paint per
 * timeline run and one encode per frame, `progress` after every frame, cancellation before every frame. */
export async function renderVideoProgramV1(program: VideoRenderProgram, environment: VideoRenderEnvironmentV1, progress: VideoRenderProgressV1, signal: AbortSignal): Promise<VideoRenderResultV1> {
  const problem = videoRenderProgramProblem(program);
  if (problem !== null) throw new VideoRenderExportErrorV1("refused", problem);
  const parameters: VideoStreamParameters = { width: program.width, height: program.height, fps: program.fps };
  const total = videoRenderFrameCount(program);
  const canvas = environment.createCanvas(program.width, program.height);
  const paths = program.paths.map((path) => videoRenderPath2dV1(path, environment.createPath));
  const codec = videoRenderWebCodecsCodecV1(parameters);
  const config = { codec, width: program.width, height: program.height, framerate: program.fps, avc: { format: "avc" as const } };
  const webCodecs = environment.webCodecs !== null && (await environment.webCodecs.isConfigSupported(config).catch(() => false)) ? environment.webCodecs : null;
  const samples: EncodedVideoSample[] = [];
  let description: Uint8Array | null = null;
  let failure: unknown = null;
  const session = webCodecs?.open(
    config,
    (sample, record) => {
      samples.push(sample);
      if (record !== null) description = record;
    },
    (reason) => {
      failure = reason;
    },
  );
  const pcm = session ? null : new AvcIntraPcmEncoder(parameters);
  const frameMicros = 1_000_000 / program.fps;
  const keyInterval = program.fps * 2;
  let frame = 0;
  try {
    for (const run of program.timeline) {
      drawVideoRenderSceneV1(canvas.context, program, run.scene, paths);
      const rgba = pcm ? canvas.context.getImageData(0, 0, program.width, program.height).data : null;
      for (let repeat = 0; repeat < run.frames; repeat += 1) {
        if (signal.aborted) throw new VideoRenderExportErrorV1("cancelled", `${frame}/${total}`);
        if (failure !== null) throw new VideoRenderExportErrorV1("encoder", String(failure));
        if (session) {
          session.encode(canvas.source, Math.round(frame * frameMicros), Math.round(frameMicros), frame % keyInterval === 0);
          while (session.queueSize > 4) await environment.yieldToHost();
        } else if (pcm && rgba) {
          samples.push(pcm.encode(rgba));
        }
        frame += 1;
        progress(frame, total);
        await environment.yieldToHost();
      }
    }
    if (session) await session.flush();
    if (failure !== null) throw new VideoRenderExportErrorV1("encoder", String(failure));
  } finally {
    session?.close();
  }
  const configuration: AvcDecoderConfiguration | null = pcm ? pcm.configuration() : description ? avcConfigurationFromRecord(description) : null;
  if (configuration === null || samples.length !== total) throw new VideoRenderExportErrorV1("encoder", `encoder answered ${samples.length} of ${total} frames${configuration === null ? " and no avcC" : ""}`);
  return { bytes: writeAvcMp4(parameters, configuration, samples), tier: session ? "webcodecs" : "intra-pcm", frames: total };
}

/** 🌍️ The page's own environment: `OffscreenCanvas` (or a detached `<canvas>`), `Path2D`, WebCodecs when present. */
export function browserVideoRenderEnvironmentV1(): VideoRenderEnvironmentV1 {
  const scope = globalThis as unknown as {
    readonly OffscreenCanvas?: new (width: number, height: number) => { getContext(kind: "2d", options: { willReadFrequently: boolean }): VideoRenderContext2dV1 | null };
    readonly VideoEncoder?: { new (init: { output: (chunk: { readonly type: string; readonly byteLength: number; copyTo(target: Uint8Array): void }, metadata?: { readonly decoderConfig?: { readonly description?: AllowSharedBufferSource } }) => void; error: (error: unknown) => void }): { configure(config: object): void; encode(frame: unknown, options: { keyFrame: boolean }): void; readonly encodeQueueSize: number; flush(): Promise<void>; close(): void; readonly state: string }; isConfigSupported(config: object): Promise<{ readonly supported?: boolean }> };
    readonly VideoFrame?: new (source: unknown, init: { timestamp: number; duration: number }) => { close(): void };
    readonly document?: { createElement(tag: "canvas"): { width: number; height: number; getContext(kind: "2d", options: { willReadFrequently: boolean }): VideoRenderContext2dV1 | null } };
  };
  const VideoEncoderCtor = scope.VideoEncoder;
  const VideoFrameCtor = scope.VideoFrame;
  const webCodecs: WebCodecsVideoEncoderPortV1 | null =
    VideoEncoderCtor && VideoFrameCtor
      ? {
          isConfigSupported: async (config) => (await VideoEncoderCtor.isConfigSupported(config)).supported === true,
          open: (config, output, error) => {
            const encoder = new VideoEncoderCtor({
              output: (chunk, metadata) => {
                const data = new Uint8Array(chunk.byteLength);
                chunk.copyTo(data);
                const description = metadata?.decoderConfig?.description;
                output({ data, sync: chunk.type === "key" }, description ? new Uint8Array(ArrayBuffer.isView(description) ? description.buffer.slice(description.byteOffset, description.byteOffset + description.byteLength) : description) : null);
              },
              error,
            });
            encoder.configure({ ...config, latencyMode: "quality" });
            return {
              encode: (source, timestamp, duration, keyFrame) => {
                const frame = new VideoFrameCtor(source, { timestamp, duration });
                try {
                  encoder.encode(frame, { keyFrame });
                } finally {
                  frame.close();
                }
              },
              get queueSize() {
                return encoder.encodeQueueSize;
              },
              flush: () => encoder.flush(),
              close: () => {
                if (encoder.state !== "closed") encoder.close();
              },
            };
          },
        }
      : null;
  return {
    createCanvas: (width, height) => {
      if (scope.OffscreenCanvas) {
        const source = new scope.OffscreenCanvas(width, height);
        const context = source.getContext("2d", { willReadFrequently: webCodecs === null });
        if (context) return { context, source };
      }
      const element = scope.document?.createElement("canvas");
      if (!element) throw new VideoRenderExportErrorV1("encoder", "no 2D canvas in this environment");
      element.width = width;
      element.height = height;
      const context = element.getContext("2d", { willReadFrequently: webCodecs === null });
      if (!context) throw new VideoRenderExportErrorV1("encoder", "no 2D canvas context in this environment");
      return { context, source: element };
    },
    createPath: () => new Path2D(),
    webCodecs,
    yieldToHost: () => new Promise((resolve) => setTimeout(resolve, 0)),
  };
}
//#endregion 🎞️Encode

//#region 🧵️Tasks
/** 🧵️ One running export as the task manager lists it. */
export interface VideoRenderExportTaskV1 {
  readonly id: string;
  readonly filename: string;
  readonly owner: string;
  readonly startedAtMs: number;
  readonly completed: number;
  readonly total: number;
  readonly cancelling: boolean;
}

const controllers = new Map<string, AbortController>();
const listeners = new Set<() => void>();
let snapshot: readonly VideoRenderExportTaskV1[] = [];
let serial = 0;

function publish(next: readonly VideoRenderExportTaskV1[]): void {
  snapshot = next;
  for (const listener of [...listeners]) listener();
}

/** 🧵️ Every export running in this tab right now (stable identity between changes). */
export function videoRenderExportTasksSnapshotV1(): readonly VideoRenderExportTaskV1[] {
  return snapshot;
}

/** 🧵️ Notifies `listener` whenever an export starts, advances, is cancelled or ends. */
export function subscribeVideoRenderExportTasksV1(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

/** 🛑️ Asks export `id` to stop before its next frame; `false` when no such export runs. */
export function cancelVideoRenderExportV1(id: string): boolean {
  const controller = controllers.get(id);
  if (!controller) return false;
  controller.abort();
  publish(snapshot.map((task) => (task.id === id ? { ...task, cancelling: true } : task)));
  return true;
}

/** 🏁️ How an export ended, with the user-facing sentence the host shows for it. */
export interface VideoRenderExportOutcomeV1 {
  readonly status: "done" | "cancelled" | "refused" | "failed";
  readonly text: string;
  readonly tier: VideoRenderEncoderTierV1 | null;
  readonly bytes: number;
}

/** 🎥️ The whole host answer to one `videoRenderExport` effect: registers a task, renders + encodes, hands the MP4 to
 * `deliver`, and resolves with the outcome (never rejects — every failure is a stated outcome). */
export async function runVideoRenderExportV1(
  request: { readonly filename: string; readonly owner: string; readonly program: VideoRenderProgram },
  host: { readonly environment: VideoRenderEnvironmentV1; readonly deliver: (filename: string, bytes: Uint8Array) => void; readonly locale: () => string; readonly announce?: (text: string) => void },
): Promise<VideoRenderExportOutcomeV1> {
  serial += 1;
  const id = `export:video#${serial}`;
  const controller = new AbortController();
  controllers.set(id, controller);
  const total = videoRenderFrameCount(request.program);
  publish([...snapshot, { id, filename: request.filename, owner: request.owner, startedAtMs: Date.now(), completed: 0, total, cancelling: false }]);
  host.announce?.(videoRenderExportTextV1("started", host.locale(), { filename: request.filename }));
  let lastPublish = 0;
  try {
    const result = await renderVideoProgramV1(
      request.program,
      host.environment,
      (completed) => {
        const now = Date.now();
        if (completed !== total && now - lastPublish < 100) return;
        lastPublish = now;
        publish(snapshot.map((task) => (task.id === id ? { ...task, completed } : task)));
      },
      controller.signal,
    );
    host.deliver(request.filename, result.bytes);
    return { status: "done", text: videoRenderExportTextV1("done", host.locale(), { filename: request.filename, frames: result.frames }), tier: result.tier, bytes: result.bytes.length };
  } catch (error) {
    const known = error instanceof VideoRenderExportErrorV1 ? error : null;
    if (known?.code === "cancelled") return { status: "cancelled", text: videoRenderExportTextV1("cancelled", host.locale(), { filename: request.filename }), tier: null, bytes: 0 };
    if (known?.code === "refused") return { status: "refused", text: videoRenderExportTextV1("refused", host.locale(), { filename: request.filename, code: known.detail }), tier: null, bytes: 0 };
    return { status: "failed", text: videoRenderExportTextV1("failed", host.locale(), { filename: request.filename, reason: known?.detail ?? (error instanceof Error ? error.message : String(error)) }), tier: null, bytes: 0 };
  } finally {
    controllers.delete(id);
    publish(snapshot.filter((task) => task.id !== id));
  }
}
//#endregion 🧵️Tasks
