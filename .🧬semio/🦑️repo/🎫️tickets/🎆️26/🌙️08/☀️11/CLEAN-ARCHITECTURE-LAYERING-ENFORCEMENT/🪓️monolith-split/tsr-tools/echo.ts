import { decodePackValue } from "@semio-tech/framework/packs";
import { textEditorScenePacket, sameScenePackV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/📤️outbox-and-scene-packets/🟦️.ts";
import fixture from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧪️tests/🧫️fixtures/🪞️echo-pack/🔣️.json";
for (const row of fixture.cases as any[]) {
  const previous = textEditorScenePacket(row.previous, row.external);
  const echo = textEditorScenePacket(row.echo, row.external);
  console.log(row.id, JSON.stringify(decodePackValue(echo)), "same", sameScenePackV1(previous, echo));
}
