/** ✂️ Scalar text splices preserve UTF-8 intervals and overlapping authored style order. */
import type { TextStory } from "../🟦️.ts";
const bytes = (value: string): bigint => BigInt(new TextEncoder().encode(value).length);

/** 🧵️ Right-biased inserted formatting, with left inheritance at EOF and retained empty anchors. */
export function spliceStory(story: TextStory, offset: number, count: number, insert: string): TextStory | null {
  const characters = [...story.content], endIndex = offset + count;
  if (!Number.isSafeInteger(offset) || !Number.isSafeInteger(count) || offset < 0 || count < 0 || !Number.isSafeInteger(endIndex) || endIndex > characters.length) return null;
  const start = bytes(characters.slice(0, offset).join("")), end = bytes(characters.slice(0, endIndex).join("")), inserted = bytes(insert), length = bytes(story.content);
  const shift = (value: bigint): bigint => value - (end - start) + inserted;
  const styleRuns = story.styleRuns.flatMap(run => {
    if (run.start === run.end) {
      const at = run.start < start ? run.start : run.start >= end ? shift(run.start) : start + inserted;
      return [{ ...run, start: at, end: at }];
    }
    const pieces: [bigint, bigint][] = [];
    if (run.start < start) pieces.push([run.start, run.end < start ? run.end : start]);
    if (inserted > 0n && (run.start <= start && start < run.end || start === length && run.start < start && run.end === start)) pieces.push([start, start + inserted]);
    if (run.end > end) pieces.push([shift(run.start > end ? run.start : end), shift(run.end)]);
    const kept = pieces.filter(([a,b])=>a<b);
    return kept.length ? [{ ...run, start: kept[0][0], end: kept[kept.length-1][1] }] : [];
  });
  return { ...story, content: characters.slice(0,offset).join("") + insert + characters.slice(endIndex).join(""), styleRuns };
}
