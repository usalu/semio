import path from 'node:path';import {pathToFileURL} from 'node:url';
export async function run(root){const oracle=await import(pathToFileURL(path.join(root,'🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🎟️work/🧪️tests/🔮️oracle.mjs')).href);return{funding:oracle.run(root),outlines:await oracle.runOutline(root),paragraphs:await(await import('./paragraph-check.mjs')).run(root)};}
