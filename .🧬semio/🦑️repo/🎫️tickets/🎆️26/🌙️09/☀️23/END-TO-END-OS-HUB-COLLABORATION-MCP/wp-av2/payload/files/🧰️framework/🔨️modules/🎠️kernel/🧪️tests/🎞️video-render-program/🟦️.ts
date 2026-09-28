import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { MEDIA_VIDEO_RENDER_CAPABILITY, type VideoRenderProgram, videoRenderDurationMilliseconds, videoRenderFrameCount, videoRenderProgramFromWire, videoRenderProgramProblem } from "../../🟦️.ts";
import { wireEffectToFriendly } from "../../../🎭️actor/🖼️wire-turn/🟦️.ts";

/** 🎞️ TypeScript twin of `🧪️tests/🎞️video-render-program/🦀️.rs`, driven from the SAME fixture
 * (`🧫️fixtures/🎞️video-render-program/🔣️.json`): every host admits exactly the programs the Rust kernel admits, refuses
 * the rest with the same code, and the wire decoder hands a `video-render-export` effect over with its program intact. */

interface ProgramFixture {
  readonly capability: string;
  readonly valid: readonly { readonly id: string; readonly program: VideoRenderProgram; readonly frameCount: number; readonly durationMs: number }[];
  readonly invalid: readonly { readonly id: string; readonly error: string; readonly patch: Record<string, unknown> }[];
  readonly invalidBase: string;
}

function loadFixture(): ProgramFixture {
  return JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🎞️video-render-program/🔣️.json"), "utf8")) as ProgramFixture;
}

/** ⚖️ The whole law, callable without vitest so the nx lane runs it as a plain oracle. */
export function testVideoRenderProgramContract(): void {
  const fixture = loadFixture();
  for (const row of fixture.valid) {
    assert.equal(videoRenderProgramProblem(row.program), null, `${row.id}: admitted`);
    assert.equal(videoRenderFrameCount(row.program), row.frameCount, `${row.id}: frames`);
    assert.equal(videoRenderDurationMilliseconds(row.program), row.durationMs, `${row.id}: duration`);
    const effect = wireEffectToFriendly({ tag: "video-render-export", val: { filename: `${row.id}.mp4`, program: row.program } }, (bytes) => [...bytes]);
    assert.deepEqual(effect, { videoRenderExport: { filename: `${row.id}.mp4`, program: row.program } }, `${row.id}: wire decoder`);
  }
  const base = fixture.valid.find((row) => row.id === fixture.invalidBase)!.program;
  for (const row of fixture.invalid) {
    assert.equal(videoRenderProgramProblem({ ...base, ...row.patch } as VideoRenderProgram), row.error, row.id);
    assert.equal(videoRenderProgramProblem(videoRenderProgramFromWire({ ...base, ...row.patch })), row.error, `${row.id}: after the wire decoder`);
  }
  const unknownKind = { ...base, scenes: [{ ops: [{ kind: "blur", radius: 2 }] }, ...base.scenes.slice(1)] };
  assert.equal(videoRenderProgramProblem(videoRenderProgramFromWire(unknownKind)), "schema", "an unknown op kind is undecodable, like the Rust host's failed decode");
  assert.equal(videoRenderProgramProblem(videoRenderProgramFromWire({ ...base, timeline: [{ scene: 0n, frames: 3n }] })), null, "bigint integers from the pack decoder are admitted");
  assert.equal(fixture.capability, MEDIA_VIDEO_RENDER_CAPABILITY);
  console.log(`video-render-program valid=${fixture.valid.length} invalid=${fixture.invalid.length}`);
}
