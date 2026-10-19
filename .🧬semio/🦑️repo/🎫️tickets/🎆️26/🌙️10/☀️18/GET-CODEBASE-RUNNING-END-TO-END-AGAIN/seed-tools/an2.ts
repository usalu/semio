const R="/Users/ueli/Documents/semio/";
const L=Bun.JSONC.parse(await Bun.file(R+".vscode/launch.json").text());
const S=Bun.JSONC.parse(await Bun.file(R+".vscode/🧩️launch.seed.jsonc").text());
const lc:any[]=L.configurations, sc:any[]=S.configurations;
const show=(c:any)=>console.log(JSON.stringify(c));
for(const n of ["🛠️dev📐️cad⚛️react","🛠️dev📐️cad🧊️wgpu🌐️wasm","🛠️dev📐️cad🧊️wgpu🖥️native"]){console.log("L");lc.filter(c=>c.name===n).forEach(show);console.log("S");sc.filter(c=>c.name===n).forEach(show)}
const idx=sc.findIndex(c=>c==="@generated:cad:react"); console.log(idx, JSON.stringify(sc.slice(idx-2,idx+6)));
const li=lc.findIndex(c=>c.name==="🛠️dev📐️cad⚛️react"); console.log(li, lc.slice(li-2,li+4).map(c=>c.name));
console.log(lc.filter(c=>/^🛠️dev/.test(c.name)).length, sc.filter(c=>typeof c!=='string'&&/^🛠️dev/.test(c.name)).length);
// the users
console.log(lc.filter(c=>/space|🖥️s/.test(c.name)&&/dev/.test(c.name)).map(c=>c.name+" "+c.presentation?.order).slice(0,30));
