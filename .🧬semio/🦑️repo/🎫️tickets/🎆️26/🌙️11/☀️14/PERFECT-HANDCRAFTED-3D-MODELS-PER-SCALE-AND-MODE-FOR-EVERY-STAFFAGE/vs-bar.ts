const C = await import(`../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes-next-b/🧵️centreline/🟦️.ts`);
const [hx, hy, hz] = process.argv.slice(2).map((x) => Number(x) / 2);
const corners = [-1, 1].flatMap((x) => [-1, 1].flatMap((y) => [-1, 1].map((z) => [x * hx, y * hy, z * hz])));
const faces = [[0, 1, 3, 2], [4, 6, 7, 5], [0, 4, 5, 1], [2, 3, 7, 6], [0, 2, 6, 4], [1, 5, 7, 3]];
const indices = Uint32Array.from(faces.flatMap(([a, b, c, d]) => [a!, b!, c!, a!, c!, d!]));
const positions = Float64Array.from(corners.flat());
const cl = C.centreLineOf(positions, positions, indices);
console.log(cl && { len: cl.length, th: cl.thickness, c: C.clusterPoints(cl, positions, 0) });
