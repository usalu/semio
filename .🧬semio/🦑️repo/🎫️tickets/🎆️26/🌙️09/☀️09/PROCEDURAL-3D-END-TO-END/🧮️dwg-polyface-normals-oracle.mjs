import { BufferGeometry, Float32BufferAttribute } from "three";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const test = JSON.parse(readFileSync(resolve("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📤️export/🧪️tests/🧫️fixtures/mesh-projection.json"), "utf8"));
const geometry = new BufferGeometry();
geometry.setAttribute("position", new Float32BufferAttribute([...test.polyfaceVertices, ...test.face3dCorners.slice(0, 3)].flat(), 3));
geometry.setIndex(test.indices);
geometry.computeVertexNormals();
writeFileSync(new URL("./🗑️generated/dwg-polyface-normals.txt", import.meta.url), Array.from(geometry.attributes.normal.array).join("\n"));
