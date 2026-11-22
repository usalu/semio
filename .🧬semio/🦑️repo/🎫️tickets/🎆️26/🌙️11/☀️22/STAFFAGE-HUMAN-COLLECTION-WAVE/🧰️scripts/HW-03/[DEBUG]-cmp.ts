const { posed, outlineOf } = await import("./trace.ts");
const [file, sc, view, id] = process.argv.slice(2);
const a = (await Bun.file(file!).json()).asset;
const { state, worlds } = posed(a, sc!);
const ring = outlineOf(worlds, new Set([id!]), view as any, sc!);
console.log("volume outline", JSON.stringify(ring?.map((p: number[]) => p.map((x) => +x.toFixed(3)))));
const s = state.shapes[view!].find((x: any) => x.shape.id === id);
console.log("drawn", JSON.stringify(s?.rings[0].map((p: any) => [+p.x.toFixed(3), +p.y.toFixed(3)])));
console.log("shape points", JSON.stringify(s?.shape.points));
