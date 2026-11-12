import { readFileSync } from "node:fs";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { loadFbx } = await import(`${S}/🧪️tests/🧰️support/🧊️.ts`);
const loadAssimp = (await import("assimpjs")).default as any;
const f = new Uint8Array(readFileSync(`${S}/📦️packages/🟦️typescript/📤️dist/🧊️models/shower-head/shower-head-turn-on-and-run-light-solid-model-1to50.fbx`));
const o = loadFbx(f); let a = 0, b = 0, n = 0; o.traverse((x: any) => { if (x.isMesh) { n++; a += x.geometry.index ? x.geometry.index.count / 3 : 0; b += x.geometry.attributes.position.count / 3; console.log(x.name, x.isSkinnedMesh, x.geometry.index?.count, x.geometry.attributes.position.count, x.geometry.groups.length); } });
console.log("three meshes", n, "index tris", a, "pos tris", b);
const ai = await loadAssimp(); const l = new ai.FileList(); l.AddFile("m.fbx", f); const r = ai.ConvertFileList(l, "assjson"); const s = JSON.parse(new TextDecoder().decode(r.GetFile(0).GetContent()));
console.log("assimp meshes", s.meshes.length, "faces", s.meshes.reduce((n: number, m: any) => n + m.faces.length, 0));
