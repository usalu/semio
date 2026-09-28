/** 🧊️ S19 one-off third-party oracle: parses an exported STL with three.js `STLLoader` and prints triangle count + bounds.
 * usage: bun s19-stl-oracle.ts <file.stl> */
import { readFileSync } from "node:fs";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
const bytes = readFileSync(process.argv[2]!);
const geometry = new STLLoader().parse(bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength));
geometry.computeBoundingBox();
const box = geometry.boundingBox!;
console.log(JSON.stringify({ triangles: geometry.getAttribute("position").count / 3, min: box.min.toArray(), max: box.max.toArray() }));
