import { expect, test } from "bun:test";
import { PerspectiveCamera } from "three";
import fixture from "../🧫️fixtures/📷️shot-camera/🔣️.json";

test("shooting captured camera remains stable through viewport navigation", () => {
  const live = new PerspectiveCamera();
  live.position.fromArray(fixture.captured);
  const captured = live.clone();
  live.position.fromArray(fixture.navigated);
  expect(captured.position.toArray()).toEqual(fixture.expected);
  expect(live.position.toArray()).toEqual(fixture.navigated);
});
