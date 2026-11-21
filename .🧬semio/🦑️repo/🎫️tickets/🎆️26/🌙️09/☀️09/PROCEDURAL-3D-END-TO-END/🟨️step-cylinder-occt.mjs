import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

const [folder, fixture] = process.argv.slice(2);
const { default: createOcct } = await import(new URL(folder + "/occt-import-js.cjs", import.meta.url));
const occt = await createOcct();
const dot = (a,b) => a.reduce((sum,v,i)=>sum+v*b[i],0);
const cross = ([x,y,z],[a,b,c]) => [y*c-z*b,z*a-x*c,x*b-y*a];
const minus = (a,b) => a.map((v,i)=>v-b[i]);
const close = (actual,expected,tolerance) => {
    if (Math.abs(actual-expected)>tolerance) throw new Error(`Open CASCADE: ${actual} vs ${expected}, tolerance ${tolerance}`);
};

for (const row of JSON.parse(readFileSync(fixture,"utf8")).cases) {
    const source = readFileSync(row.file ? resolve(dirname(fixture),row.file) : resolve(folder,row.kind + '.step'));
    const result = occt.ReadStepFile(source,{
        linearUnit:"millimeter", linearDeflectionType:"absolute_value", linearDeflection:0.001, angularDeflection:0.01,
    });
    if (!result.success || result.meshes.length!==1) throw new Error("Open CASCADE did not load the authored cylinder");
    const mesh = result.meshes[0];
    if (mesh.brep_faces.length !== row.faces) throw new Error("Open CASCADE did not preserve the three authored faces");
    const point = i => mesh.attributes.position.array.slice(3*i,3*i+3);
    const indices = mesh.index.array;
    const lo = [Infinity,Infinity,Infinity], hi = [-Infinity,-Infinity,-Infinity];
    let signedVolume = 0, area = 0;
    for (let i=0; i<indices.length; i+=3) {
        const [a,b,c] = [point(indices[i]),point(indices[i+1]),point(indices[i+2])];
        for (const p of [a,b,c]) p.forEach((axis,j)=>{
            lo[j]=Math.min(lo[j],axis);
            hi[j]=Math.max(hi[j],axis);
        });
        signedVolume += dot(a,cross(b,c))/6;
        area += Math.hypot(...cross(minus(b,a),minus(c,a)))/2;
    }
    lo.forEach((actual,i)=>close(actual,row.bounds[0][i],0.001));
    hi.forEach((actual,i)=>close(actual,row.bounds[1][i],0.001));
    close(signedVolume,row.volume,row.volume*0.0001);
    close(area,row.area,row.area*0.0001);
    console.log(`Open CASCADE independently read the full cylinder at ${row.base.join(",")} with axis ${row.axis.join(",")}`);
}
