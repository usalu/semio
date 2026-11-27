import { expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "../🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const validate = new Ajv().compile(schema);
const encoder = new TextEncoder();

type Callback = { args: Record<string, unknown>; repeat?: { text: string; count: number } };

const payload = ({ args, repeat }: Callback) => (repeat ? { ...args, payload: repeat.text.repeat(repeat.count) } : args);

test("a callback is accepted when the picker schema holds and the payload fits the GeoJSON byte limit", () => {
  for (const callback of fixture.callbacks) {
    const input = payload(callback as Callback);
    const bytes = typeof input.payload === "string" ? encoder.encode(input.payload).length : 0;
    expect(validate(input) && bytes <= fixture.limits.geoJsonBytes).toBe(callback.accepted);
  }
});

test("the wire envelope holds two elevation files and the elevation limit exceeds the GeoJSON limit", () => {
  expect(fixture.limits.wireBytes).toBe(2 * fixture.limits.elevationBytes + 4096);
  expect(fixture.limits.geoJsonBytes).toBeLessThan(fixture.limits.elevationBytes);
  expect(new Set(fixture.fileActions).size).toBe(fixture.fileActions.length);
});
