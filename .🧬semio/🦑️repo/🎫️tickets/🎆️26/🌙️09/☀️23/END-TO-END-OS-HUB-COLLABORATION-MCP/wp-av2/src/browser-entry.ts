/** 🎥️ AV2 page entry: runs the overlay's `🎥️VideoRenderHost` in a real browser over a deck-shaped program (the demo figure,
 * an outlined overview and four tile slides — what animate's `video_render_program_from_deck` builds for a 2×2 grid), once
 * on the page's platform encoder (WebCodecs) and once on the first-party encoder, and posts both MP4s back. */
import { MEDIA_VIDEO_RENDER_CAPABILITY, type VideoRenderProgram } from "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay/🧰️framework/🔨️modules/🎠️kernel/🟦️.ts";
import { browserVideoRenderEnvironmentV1, runVideoRenderExportV1, videoRenderExportJobLogV1 } from "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎥️VideoRenderHost/🟦️.ts";

const width = 1280;
const height = 720;
const fps = 15;
const aspect = 2560 / 1707;
const margin = Math.min(width, height) * 0.05;
const fit = (crop: readonly number[]): [number, number, number, number, number, number] => {
  const pictureAspect = (crop[2]! * aspect) / crop[3]!;
  const roomWidth = width - 2 * margin;
  const roomHeight = height - 2 * margin;
  const [w, h] = pictureAspect >= roomWidth / roomHeight ? [roomWidth, roomWidth / pictureAspect] : [roomHeight * pictureAspect, roomHeight];
  return [w, 0, 0, h, (width - w) / 2, (height - h) / 2];
};
const crops = [
  [0, 0, 0.5, 0.5],
  [0.5, 0, 0.5, 0.5],
  [0, 0.5, 0.5, 0.5],
  [0.5, 0.5, 0.5, 0.5],
] as const;
const overview = fit([0, 0, 1, 1]);
const outline = (crop: readonly number[]) => {
  const left = overview[4] + crop[0]! * overview[0];
  const top = overview[5] + crop[1]! * overview[3];
  const right = left + crop[2]! * overview[0];
  const bottom = top + crop[3]! * overview[3];
  return { verbs: "MLLLZ", points: [left, top, right, top, right, bottom, left, bottom] };
};
const program: VideoRenderProgram = {
  schema: "semio.video-render.program.v1",
  width,
  height,
  fps,
  background: [0, 0, 0, 1],
  paths: crops.map(outline),
  images: [{ url: "/figure.png" }],
  scenes: [
    { ops: [{ kind: "image", image: 0, crop: [0, 0, 1, 1], transform: overview, opacity: 1 }, ...crops.map((_, index) => ({ kind: "stroke" as const, path: index, transform: [1, 0, 0, 1, 0, 0] as const, color: [1, 0.78, 0.2, 1] as const, width: 3 }))] },
    ...crops.map((crop) => ({ ops: [{ kind: "image" as const, image: 0, crop: [...crop] as [number, number, number, number], transform: fit(crop), opacity: 1 }] })),
  ],
  timeline: [{ scene: 0, frames: 45 }, ...crops.map((_, index) => ({ scene: index + 1, frames: 30 }))],
};

async function post(name: string, bytes: Uint8Array): Promise<void> {
  await fetch(`/result/${name}`, { method: "POST", body: bytes });
}

async function main(): Promise<unknown> {
  const platform = browserVideoRenderEnvironmentV1();
  const probe = platform.webCodecs ? await platform.webCodecs.isConfigSupported({ codec: "avc1.42E01F", width, height, framerate: fps, avc: { format: "avc" } }) : false;
  const outcomes: unknown[] = [];
  for (const [tier, environment] of [["platform", platform], ["first-party", { ...platform, webCodecs: null }]] as const) {
    const started = performance.now();
    let delivered: Uint8Array | null = null;
    const outcome = await runVideoRenderExportV1({ filename: `deck-${tier}.mp4`, owner: "animate", program, capabilities: [MEDIA_VIDEO_RENDER_CAPABILITY] }, { environment, deliver: (_name, bytes) => (delivered = bytes), locale: () => "en" });
    if (delivered) await post(`deck-${tier}.mp4`, delivered);
    outcomes.push({ tier, outcome: outcome.outcome, text: outcome.text, ms: Math.round(performance.now() - started) });
  }
  return { webCodecsH264: probe, outcomes, events: videoRenderExportJobLogV1().map((event) => event.kind) };
}

(globalThis as { __av2?: Promise<unknown> }).__av2 = main().catch((error: unknown) => ({ error: String(error instanceof Error ? (error.stack ?? error.message) : error) }));
