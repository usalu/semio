import { generateWgpuBrowserClosure, checkWgpuBrowserClosure } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📽️projection/🔭️browser-closure/🟦️.ts";
const root = process.cwd();
console.log(JSON.stringify(await generateWgpuBrowserClosure(root)));
await checkWgpuBrowserClosure(root);
console.log("CLOSURE CHECK OK");
