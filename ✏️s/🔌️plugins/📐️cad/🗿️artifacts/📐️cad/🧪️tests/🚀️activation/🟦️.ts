import { expect, test } from "bun:test";
import { parseSurfaceAppId } from "@semio-tech/framework";
import corpus from "../🧫️fixtures/🚀️activation/🔣️.json";

test("CAD activation coordinates retain the owned relay contract", () => {
  const parsed = parseSurfaceAppId(`${corpus.artifactRef}#${corpus.role}`);
  const expectedAt7x108 = { dialect: { artifactKind: corpus.artifactKinds[0], standard: "1", subset: "*" }, role: corpus.role }; expect<typeof expectedAt7x108>(parsed).toEqual(expectedAt7x108);
});
