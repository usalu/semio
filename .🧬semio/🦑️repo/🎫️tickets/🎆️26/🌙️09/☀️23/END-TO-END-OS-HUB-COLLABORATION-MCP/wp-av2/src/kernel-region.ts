//#region 🎞️VideoRenderProgram
/** 🎟️ The host capability a plugin requests before any host renders its `videoRenderExport` — twin of Rust
 * `kernel::MEDIA_VIDEO_RENDER_CAPABILITY`. */
export const MEDIA_VIDEO_RENDER_CAPABILITY = "media.video-render";

/** 🎞️ The schema id every {@link VideoRenderProgram} states — twin of Rust `kernel::VIDEO_RENDER_PROGRAM_SCHEMA`; both drive
 * `🧫️fixtures/🎞️video-render-program/🔣️.json`. */
export const VIDEO_RENDER_PROGRAM_SCHEMA = "semio.video-render.program.v1";

/** 📏️ The largest picture edge a program may ask for. */
export const VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE = 4096;

/** 🎞️ The fastest frame rate a program may ask for. */
export const VIDEO_RENDER_PROGRAM_MAXIMUM_FPS = 120;

/** ⏱️ The most frames one program may render. */
export const VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES = 36_000;

/** ✏️ One path of the shared table: one verb letter per segment (`M`, `L`, `Q`, `C`, `Z`) over flat `x, y` points. */
export interface VideoRenderPath {
  readonly verbs: string;
  readonly points: readonly number[];
}

/** 🖼️ One picture a program draws from: a same-origin absolute path or a `data:image/…` URL. */
export interface VideoRenderImage {
  readonly url: string;
}

/** 🧮️ `[a, b, c, d, e, f]` into device pixels, y down. */
export type VideoRenderTransform = readonly [number, number, number, number, number, number];

/** 🎨️ Straight RGBA, every channel `0..=1`. */
export type VideoRenderColor = readonly [number, number, number, number];

/** 🎨️ One paint operation, composited in list order — twin of Rust `kernel::VideoRenderOp` (tagged by `kind`). */
export type VideoRenderOp =
  | { readonly kind: "fill"; readonly path: number; readonly transform: VideoRenderTransform; readonly color: VideoRenderColor }
  | { readonly kind: "stroke"; readonly path: number; readonly transform: VideoRenderTransform; readonly color: VideoRenderColor; readonly width: number }
  | { readonly kind: "image"; readonly image: number; readonly crop: readonly [number, number, number, number]; readonly transform: VideoRenderTransform; readonly opacity: number };

/** 🖼️ One distinct picture. */
export interface VideoRenderScene {
  readonly ops: readonly VideoRenderOp[];
}

/** ⏯️ Scene `scene` shown for `frames` consecutive frames. */
export interface VideoRenderRun {
  readonly scene: number;
  readonly frames: number;
}

/** 🎞️ Everything a host needs to render a video — twin of Rust `kernel::VideoRenderProgram`. */
export interface VideoRenderProgram {
  readonly schema: string;
  readonly width: number;
  readonly height: number;
  readonly fps: number;
  readonly background: VideoRenderColor;
  readonly paths: readonly VideoRenderPath[];
  readonly images: readonly VideoRenderImage[];
  readonly scenes: readonly VideoRenderScene[];
  readonly timeline: readonly VideoRenderRun[];
}

/** 🚨️ The fixture's refusal vocabulary — Rust `VideoRenderProgramError::code`. */
export type VideoRenderProgramErrorCode = "schema" | "dimensions" | "frameRate" | "empty" | "tooLong" | "timeline" | "pathVerbs" | "imageUrl" | "pathIndex" | "imageIndex" | "paint";

/** 🎞️ Frames the timeline plays. */
export function videoRenderFrameCount(program: VideoRenderProgram): number {
  return program.timeline.reduce((sum, run) => sum + run.frames, 0);
}

/** ⏱️ Playing time in milliseconds, rounded half up. */
export function videoRenderDurationMilliseconds(program: VideoRenderProgram): number {
  return Math.floor((videoRenderFrameCount(program) * 1000 + Math.floor(program.fps / 2)) / Math.max(1, program.fps));
}

const VIDEO_RENDER_VERB_POINTS: Readonly<Record<string, number>> = { M: 2, L: 2, Q: 4, C: 6, Z: 0 };

/** ✏️ A path's verbs consume exactly its points, start with a move and every coordinate is finite. */
export function videoRenderPathIsWellFormed(path: VideoRenderPath): boolean {
  let needed = 0;
  for (const [index, verb] of [...path.verbs].entries()) {
    const points = VIDEO_RENDER_VERB_POINTS[verb];
    if (points === undefined || (index === 0 && verb !== "M")) return false;
    needed += points;
  }
  return needed === path.points.length && path.points.every(Number.isFinite);
}

/** 🖼️ A same-origin absolute path (never protocol-relative) or an inline `data:image/…` URL — Rust
 * `video_render_image_url_is_admitted`. */
export function videoRenderImageUrlIsAdmitted(url: string): boolean {
  return (url.startsWith("/") && !url.startsWith("//")) || url.startsWith("data:image/");
}

/** ✂️ A crop `[x, y, width, height]` inside the unit picture with a non-empty area — Rust `video_render_crop_is_admitted`. */
export function videoRenderCropIsAdmitted(crop: readonly number[]): boolean {
  const [x = Number.NaN, y = Number.NaN, width = Number.NaN, height = Number.NaN] = crop;
  return crop.length === 4 && crop.every(Number.isFinite) && x >= 0 && y >= 0 && width > 0 && height > 0 && x + width <= 1 + 1e-9 && y + height <= 1 + 1e-9;
}

const videoRenderUnit = (value: number): boolean => Number.isFinite(value) && value >= 0 && value <= 1;
const videoRenderIndex = (value: number, length: number): boolean => Number.isInteger(value) && value >= 0 && value < length;

/** 🚦️ The ONE admission rule every host applies before it renders a frame — `null` admits, a code refuses. Twin of Rust
 * `VideoRenderProgram::validate`, same order, same codes. */
export function videoRenderProgramProblem(program: VideoRenderProgram): VideoRenderProgramErrorCode | null {
  if (program.schema !== VIDEO_RENDER_PROGRAM_SCHEMA) return "schema";
  const edge = (value: number): boolean => Number.isInteger(value) && value >= 2 && value <= VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE && value % 2 === 0;
  if (!edge(program.width) || !edge(program.height)) return "dimensions";
  if (!Number.isInteger(program.fps) || program.fps <= 0 || program.fps > VIDEO_RENDER_PROGRAM_MAXIMUM_FPS) return "frameRate";
  const frames = videoRenderFrameCount(program);
  if (frames === 0) return "empty";
  if (frames > VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES) return "tooLong";
  if (program.timeline.some((run) => !Number.isInteger(run.frames) || run.frames <= 0 || !videoRenderIndex(run.scene, program.scenes.length))) return "timeline";
  if (!program.paths.every(videoRenderPathIsWellFormed)) return "pathVerbs";
  if (!program.images.every((image) => videoRenderImageUrlIsAdmitted(image.url))) return "imageUrl";
  if (!program.background.every(videoRenderUnit)) return "paint";
  for (const scene of program.scenes) {
    for (const op of scene.ops) {
      if (op.kind === "image") {
        if (!videoRenderIndex(op.image, program.images.length)) return "imageIndex";
        if (!op.transform.every(Number.isFinite) || !videoRenderCropIsAdmitted(op.crop) || !videoRenderUnit(op.opacity)) return "paint";
      } else {
        if (!videoRenderIndex(op.path, program.paths.length)) return "pathIndex";
        if (!op.transform.every(Number.isFinite) || !op.color.every(videoRenderUnit) || (op.kind === "stroke" && (!Number.isFinite(op.width) || op.width < 0))) return "paint";
      }
    }
  }
  return null;
}

/** 🎁️ A program as it leaves the wire decoder (`pack` → plain value, integers possibly `bigint`), normalised to
 * {@link VideoRenderProgram}'s number fields. Admission stays {@link videoRenderProgramProblem}'s job: an op of an unknown
 * `kind` makes the whole program schema-less, exactly as the Rust host's failed decode falls back to the default program,
 * so both hosts refuse it as `schema`. */
export function videoRenderProgramFromWire(value: unknown): VideoRenderProgram {
  const record = (raw: unknown): Record<string, unknown> => (raw !== null && typeof raw === "object" ? (raw as Record<string, unknown>) : {});
  const list = (raw: unknown): readonly unknown[] => (Array.isArray(raw) ? raw : []);
  const numbers = (raw: unknown): number[] => list(raw).map(Number);
  const tuple = <T,>(raw: unknown): T => numbers(raw) as unknown as T;
  const program = record(value);
  let decodable = true;
  const op = (raw: unknown): VideoRenderOp => {
    const fields = record(raw);
    const transform = tuple<VideoRenderTransform>(fields.transform);
    if (fields.kind === "image") return { kind: "image", image: Number(fields.image ?? Number.NaN), crop: tuple(fields.crop), transform, opacity: Number(fields.opacity ?? Number.NaN) };
    if (fields.kind === "stroke") return { kind: "stroke", path: Number(fields.path ?? Number.NaN), transform, color: tuple(fields.color), width: Number(fields.width ?? Number.NaN) };
    decodable &&= fields.kind === "fill";
    return { kind: "fill", path: Number(fields.path ?? Number.NaN), transform, color: tuple(fields.color) };
  };
  const scenes = list(program.scenes).map((raw) => ({ ops: list(record(raw).ops).map(op) }));
  return {
    schema: decodable ? String(program.schema ?? "") : "",
    width: Number(program.width ?? 0),
    height: Number(program.height ?? 0),
    fps: Number(program.fps ?? 0),
    background: tuple(program.background),
    paths: list(program.paths).map((raw) => ({ verbs: String(record(raw).verbs ?? ""), points: numbers(record(raw).points) })),
    images: list(program.images).map((raw) => ({ url: String(record(raw).url ?? "") })),
    scenes,
    timeline: list(program.timeline).map((raw) => ({ scene: Number(record(raw).scene ?? 0), frames: Number(record(raw).frames ?? 0) })),
  };
}
//#endregion 🎞️VideoRenderProgram

//#region 🧵️VideoRenderJob
/** 🧩️ Which encoder produced a finished render — Rust `kernel::VideoRenderEncoderTier`. */
export type VideoRenderEncoderTier = "platform" | "first-party";

/** 🏁️ How one host video render job ended — Rust `kernel::VideoRenderJobOutcome` (tagged by `status`). */
export type VideoRenderJobOutcome =
  | { readonly status: "done"; readonly bytes: number; readonly tier: VideoRenderEncoderTier }
  | { readonly status: "cancelled"; readonly completed: number }
  | { readonly status: "refused"; readonly code: string }
  | { readonly status: "failed"; readonly reason: string };

/** 🧵️ One fact of a host video render job's life — Rust `kernel::VideoRenderJobEvent` (tagged by `kind`). The task list
 * is the fold of these ({@link VideoRenderJobLedger}); nothing mutates it directly. */
export type VideoRenderJobEvent =
  | { readonly kind: "started"; readonly job: number; readonly owner: string; readonly filename: string; readonly frames: number; readonly atMs: number }
  | { readonly kind: "progressed"; readonly job: number; readonly completed: number }
  | { readonly kind: "cancelRequested"; readonly job: number }
  | { readonly kind: "finished"; readonly job: number; readonly outcome: VideoRenderJobOutcome };

/** 🧵️ One running job as a task list shows it — Rust `kernel::VideoRenderJobRow`. */
export interface VideoRenderJobRow {
  readonly job: number;
  readonly owner: string;
  readonly filename: string;
  readonly frames: number;
  readonly completed: number;
  readonly cancelling: boolean;
  readonly startedAtMs: number;
}

/** 🚨️ The fixture's refusal vocabulary — Rust `VideoRenderJobEventError::code`. */
export type VideoRenderJobEventErrorCode = "staleJob" | "unknownJob" | "progressRegressed" | "progressOverrun" | "alreadyCancelling";

/** 📒️ The fold of a host's {@link VideoRenderJobEvent} log — twin of Rust `kernel::VideoRenderJobLedger`. `running()` keeps
 * its identity until an event changes it, so a `useSyncExternalStore` reader re-renders only on real change. */
export class VideoRenderJobLedger {
  #lastJob = 0;
  #running: readonly VideoRenderJobRow[] = [];

  /** 📒️ Folds a whole log; a refusal names the index of the first event refused. */
  static fold(events: readonly VideoRenderJobEvent[]): VideoRenderJobLedger | { readonly index: number; readonly error: VideoRenderJobEventErrorCode } {
    const ledger = new VideoRenderJobLedger();
    for (const [index, event] of events.entries()) {
      const error = ledger.apply(event);
      if (error !== null) return { index, error };
    }
    return ledger;
  }

  /** ➕️ Applies one event (`null`), or refuses it with a code and leaves the ledger as it was. */
  apply(event: VideoRenderJobEvent): VideoRenderJobEventErrorCode | null {
    if (event.kind === "started") {
      if (event.job <= this.#lastJob) return "staleJob";
      this.#lastJob = event.job;
      this.#running = [...this.#running, { job: event.job, owner: event.owner, filename: event.filename, frames: event.frames, completed: 0, cancelling: false, startedAtMs: event.atMs }];
      return null;
    }
    const row = this.#running.find((candidate) => candidate.job === event.job);
    if (!row) return "unknownJob";
    if (event.kind === "progressed") {
      if (event.completed < row.completed) return "progressRegressed";
      if (event.completed > row.frames) return "progressOverrun";
      if (event.completed !== row.completed) this.#running = this.#running.map((candidate) => (candidate === row ? { ...row, completed: event.completed } : candidate));
      return null;
    }
    if (event.kind === "cancelRequested") {
      if (row.cancelling) return "alreadyCancelling";
      this.#running = this.#running.map((candidate) => (candidate === row ? { ...row, cancelling: true } : candidate));
      return null;
    }
    this.#running = this.#running.filter((candidate) => candidate !== row);
    return null;
  }

  /** 🧵️ Every running job, in start order. */
  running(): readonly VideoRenderJobRow[] {
    return this.#running;
  }

  /** 🔎️ Running job `job`, if any. */
  row(job: number): VideoRenderJobRow | undefined {
    return this.#running.find((candidate) => candidate.job === job);
  }

  /** 🔢️ The last job id issued (`0` before the first). */
  lastJob(): number {
    return this.#lastJob;
  }
}
//#endregion 🧵️VideoRenderJob
