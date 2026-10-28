import {createRequire} from 'node:module';
export function checkSlab(root){
const {EllipseCurve}=createRequire(root+'/package.json')('three');
const half=Math.asin(2000/5200),count=Math.ceil(half/(2*Math.PI/72)-1e-12),outline=[[0,0],[1000,0]];
for(const [cx,begin] of [[-3800,-half],[4800,Math.PI-half]]){if(cx===4800)outline.push([0,4000]);for(const [a,b]of [[begin,begin+half],[begin+half,begin+2*half]])outline.push(...new EllipseCurve(cx,2000,5200,5200,a,b,false,0).getPoints(count).slice(1).map(p=>[p.x,p.y]));}
const area=p=>Math.abs(p.reduce((s,a,i)=>{const b=p[(i+1)%p.length];return s+a[0]*b[1]-b[0]*a[1];},0))/2;
const circle=new EllipseCurve(100,300,50,50,0,Math.PI*2,false,0).getPoints(72).slice(0,-1).map(p=>[p.x,p.y]);
const gross=area(outline)*200,through=area(circle)*200,recess=1000*500*50;
return {implementation:'Three EllipseCurve',halfSpan:half,samplesPerHalf:count,gross,through,recess,net:gross-through-recess};
}
