import { readFileSync } from "node:fs";
const { default: createOcct } = await import(new URL(process.argv[2] + "/occt-import-js.cjs", import.meta.url));

const [folder,fixturePath,mode] = process.argv.slice(2);
const fixtures = JSON.parse(readFileSync(fixturePath,"utf8"));
const occt = await createOcct();
const dot = (a,b) => a.reduce((sum,v,i)=>sum+v*b[i],0);
const cross = ([x,y,z],[a,b,c]) => [y*c-z*b,z*a-x*c,x*b-y*a];
const minus = (a,b) => a.map((v,i)=>v-b[i]);
const close = (a,b,tol=1e-5) => {
    if (Math.abs(a-b)>tol) throw new Error(`Open CASCADE readback mismatch ${a} vs ${b}`);
};
for (const [index, fixture] of fixtures.cases.entries()) {
    const row = fixtures.stepExport ? {
        ...fixture,
        kind: mode ? `nurbs-${mode}-${index}` : `nurbs-${index}`,
        vertices: fixtures.stepExport.counts.VERTEX_POINT,
        edges: fixtures.stepExport.counts.EDGE_CURVE,
        faces: fixtures.stepExport.counts.ADVANCED_FACE,
        bounds: fixture.corners,
    } : fixture;
    const source = readFileSync(`${folder}/${row.kind}.step`);
    const result = occt.ReadStepFile(source, { linearUnit:"millimeter" });
    if (!result.success || result.meshes.length!==1) throw new Error(`Open CASCADE did not load one ${row.kind}`);
    const mesh = result.meshes[0];
    if (mesh.brep_faces.length!==row.faces) throw new Error("Open CASCADE lost the original planar faces");
    const point = i => mesh.attributes.position.array.slice(3*i,3*i+3);
    const indices = mesh.index.array;
    const minima = [Infinity,Infinity,Infinity], maxima=[-Infinity,-Infinity,-Infinity];
    const corners = new Set();
    const uses = new Map();
    let volume=0, area=0;
    for (let i=0;i<indices.length;i+=3) {
        const [a,b,c]=[point(indices[i]),point(indices[i+1]),point(indices[i+2])];
        volume+=dot(a,cross(b,c))/6;
        area+=Math.hypot(...cross(minus(b,a),minus(c,a)))/2;
        for (const [from,to] of [[a,b],[b,c],[c,a]]) {
            from.forEach((component,axis)=>{
                minima[axis]=Math.min(minima[axis],component);
                maxima[axis]=Math.max(maxima[axis],component);
            });
            const f=from.join(","), t=to.join(",");
            corners.add(f);
            const key=f<t ? f+"|"+t : t+"|"+f;
            const [count,sign] = uses.get(key) ?? [0,0];
            uses.set(key, [count+1,sign+(f<t ? 1 : -1)]);
        }
    }
    for (const [count,sign] of uses.values()) {
        if (count!==2 || sign!==0) throw new Error("Open CASCADE readback mesh is not an oriented closed boundary");
    }
    if (corners.size!==row.vertices) throw new Error("Open CASCADE lost the original vertices");
    if (uses.size - (indices.length/3-row.faces)!==row.edges) throw new Error("Open CASCADE lost the original edges");
    minima.forEach((value,axis)=>close(value,row.bounds[0][axis]));
    maxima.forEach((value,axis)=>close(value,row.bounds[1][axis]));
    close(volume,row.volume);
    close(area,row.area);
    console.log(`Open CASCADE STEP readback passed for ${row.kind}`);
}
