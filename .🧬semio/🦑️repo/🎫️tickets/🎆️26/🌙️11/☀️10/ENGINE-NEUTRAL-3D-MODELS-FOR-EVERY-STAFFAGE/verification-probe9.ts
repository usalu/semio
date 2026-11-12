import { readFileSync } from "node:fs";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { loadFbx } = await import(`${S}/🧪️tests/🧰️support/🧊️.ts`);
const f = new Uint8Array(readFileSync(`${S}/📦️packages/🟦️typescript/📤️dist/🧊️models/shower-head/shower-head-turn-on-and-run-light-solid-model-1to50.fbx`));
const o = loadFbx(f);
const dump = (x: any, d = 0): void => { console.log(" ".repeat(d * 2) + x.type + " " + x.name + (x.isMesh ? " MESH tris=" + x.geometry.attributes.position.count / 3 : "")); x.children.forEach((c: any) => dump(c, d + 1)); };
dump(o);
