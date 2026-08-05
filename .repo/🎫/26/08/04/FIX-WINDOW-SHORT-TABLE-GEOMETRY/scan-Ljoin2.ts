#!/usr/bin/env bun
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
const png = process.argv[2];
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const { loadImage, createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const img = await loadImage(png);
const c = createCanvas(img.width, img.height);
c.getContext("2d").drawImage(img, 0, 0);
const { data } = c.getContext("2d").getImageData(0,0,img.width,img.height);
function L(x:number,y:number){const i=(y*img.width+x)*4;return 0.2126*data[i]+0.7152*data[i+1]+0.0722*data[i+2];}
function rgb(x:number,y:number){const i=(y*img.width+x)*4;return [data[i],data[i+1],data[i+2]];}
// for each y, find all x with mid-grey
for (let y of [0,50,100,150,158,159,160,200,250,300,302,303,304,320]) {
  if (y>=img.height) continue;
  const rules:number[]=[];
  for(let x=0;x<img.width;x++){
    const l=L(x,y);
    if(l>80&&l<195){
      const [r,g,b]=rgb(x,y);
      if(Math.abs(r-g)<40&&Math.abs(g-b)<40) rules.push(x);
    }
  }
  console.log(`[DEBUG] y=${y} ruleXs=${rules.slice(0,20).join(",")} count=${rules.length}`);
}
