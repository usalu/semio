import {readFileSync,writeFileSync,renameSync} from 'node:fs';
const file='C:/git/semio/🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🧭️legend/🟦️.ts';
const temporary=import.meta.dir+'/source.tmp';writeFileSync(temporary,readFileSync(file,'utf8').replace('.context(context)()','.context(context as unknown as CanvasRenderingContext2D)()'));renameSync(temporary,file);
