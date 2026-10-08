import { expect, test } from "bun:test";
import { parseSurfaceAppId } from "@semio-tech/framework";
import corpus from "../../🧫️fixtures/🚀️activation/🔣️.json";

test("CAD activation coordinates retain the owned relay contract", () => {
  const parsed = parseSurfaceAppId(`${corpus.artifactRef}#${corpus.role}`);
  expect(parsed).toEqual({ dialect: { artifactKind: corpus.artifactKinds[0], standard: "1", subset: "*" }, role: corpus.role });
});
