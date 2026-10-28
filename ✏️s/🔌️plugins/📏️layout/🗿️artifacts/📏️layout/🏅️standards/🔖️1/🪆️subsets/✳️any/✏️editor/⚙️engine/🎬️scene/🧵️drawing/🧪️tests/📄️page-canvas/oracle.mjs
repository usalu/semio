import {createHash} from 'node:crypto';
/** ⚖️ Third-party scene matrices independently verify viewport placement. */
import {Matrix3,Vector2} from 'three';
export function map(canvas,target,points){const c=Math.cos(target.rotation),s=Math.sin(target.rotation),cx=target.x+target.width/2,cy=target.y+target.height/2;const rotation=new Matrix3().set(c,-s,cx-c*cx+s*cy,s,c,cy-s*cx-c*cy,0,0,1),viewport=new Matrix3().set(target.width/canvas.width,0,target.x,0,target.height/canvas.height,target.y,0,0,1),matrix=rotation.multiply(viewport),m=matrix.elements;return{affine:{xx:m[0],xy:m[3],yx:m[1],yy:m[4],dx:m[6],dy:m[7]},points:points.map(p=>new Vector2(p[0],p[1]).applyMatrix3(matrix).toArray())};}

export function identity(pageId,childId,artifactId){const source=[pageId,childId,artifactId].map(id=>Buffer.byteLength(id,'utf8')+':'+id).join(''),digest=createHash('sha256').update(source,'utf8').digest('hex').slice(0,16);return{childId:'page-drawing-child-'+digest,artifactId:'page-drawing-artifact-'+digest};}
