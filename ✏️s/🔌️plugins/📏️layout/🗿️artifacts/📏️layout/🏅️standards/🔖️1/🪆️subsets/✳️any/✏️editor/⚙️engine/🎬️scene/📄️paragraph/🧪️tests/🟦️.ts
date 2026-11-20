/** 🧪️ Shared Layout flow agrees with original source offsets and independent selected fonts. */
import {test,expect} from "bun:test";
import inRange from "lodash/inRange";
import fixture from "./🧫️fixtures/🔣️.json";
import {run,repositoryRoot} from "./🧵️flow/🔮️oracle.mjs";
test("shared story flow and address cache",async()=>{const result=await run(repositoryRoot());expect(result.flow.runs).toBe(18);expect(result.flow.cancelled).toBe(6);expect(result.addressed.requests).toBe(60);expect(result.addressed.analyses).toBe(18);expect(result.labels.runs).toBe(15);expect(result.labels.cancelled).toBe(5);});

test("preview projects story flow only in the detailed viewport", () => {
  const [left, right] = fixture.preview.storyXRange;
  if (left === undefined || right === undefined) throw new Error("Missing demo story bounds");
  for (const view of fixture.preview.views) {
    const halfViewport = 2000 / view.zoom;
    const storyVisible = view.zoom >= 0.35 && inRange(view.cameraX, left - halfViewport, right + halfViewport);
    expect(storyVisible).toBe(view.storyVisible);
  }
});
