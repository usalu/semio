/** 📐️ Native Layout frame geometry becomes real STEP millimetres with a page Y flip. */
import type { LayoutBounds } from "../../../../🧬️schema/🟦️.ts";
import { binary64, binary64Value, type Binary64 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type { LayoutBounds, Binary64 };
export type Point3 = [number, number, number];
export const POINT_TO_MILLIMETRE = 25.4 / 72;
/** 🧭️ Applies native centre rotation before the page-coordinate and physical-unit conversion. */
export function position(bounds:LayoutBounds,pageHeight:Binary64,x:number,y:number):Point3{const w=binary64Value(bounds.w),h=binary64Value(bounds.h),angle=binary64Value(bounds.rotation),sin=Math.sin(angle),cos=Math.cos(angle),dx=x-w/2,dy=y-h/2;return[(binary64Value(bounds.x)+w/2+cos*dx-sin*dy)*POINT_TO_MILLIMETRE,(binary64Value(pageHeight)-binary64Value(bounds.y)-h/2-sin*dx-cos*dy)*POINT_TO_MILLIMETRE,0];}
/** 🔲️ Returns a closed rotated frame outline in the canonical drafting writer's physical coordinates. */
export function framePoints(bounds:LayoutBounds,pageHeight:Binary64):Point3[]{const w=binary64Value(bounds.w),h=binary64Value(bounds.h);return[[0,0],[w,0],[w,h],[0,h],[0,0]].map(([x,y])=>position(bounds,pageHeight,x,y));}
/** 🔲️ Recovers native bounds from an orthogonal closed planar STEP path in points. */
export function rectangleBounds(points:readonly Point3[],minX:number,maxY:number):LayoutBounds|null{if(points.length!==5||points.some(point=>point.some(value=>!Number.isFinite(value))))return null;const first=points[0],last=points[4];if(first.some((value,index)=>Math.abs(value-last[index])>1e-7)||points.some(point=>Math.abs(point[2]-first[2])>1e-7))return null;let u=[points[1][0]-first[0],points[1][1]-first[1]],v=[points[3][0]-first[0],points[3][1]-first[1]];if(u[0]*v[1]-u[1]*v[0]>0)[u,v]=[v,u];const width=Math.hypot(...u),height=Math.hypot(...v);if(width<1e-9||height<1e-9||Math.abs(u[0]*v[0]+u[1]*v[1])>width*height*1e-7||Math.abs(points[2][0]-first[0]-u[0]-v[0])>1e-7||Math.abs(points[2][1]-first[1]-u[1]-v[1])>1e-7)return null;return{x:binary64((first[0]+points[2][0])/2-minX-width/2),y:binary64(maxY-(first[1]+points[2][1])/2-height/2),w:binary64(width),h:binary64(height),rotation:binary64(Math.atan2(-u[1],u[0]))};}
/** 🔤️ Recovers the editable glyph-height frame whose baseline is the STEP annotation origin. */
export function annotationBounds(position:Point3,xAxis:Point3,yAxis:Point3,width:number,height:number,minX:number,maxY:number):LayoutBounds{const x=position[0]+xAxis[0]*width/2+yAxis[0]*height/2,y=position[1]+xAxis[1]*width/2+yAxis[1]*height/2;return{x:binary64(x-minX-width/2),y:binary64(maxY-y-height/2),w:binary64(width),h:binary64(height),rotation:binary64(-Math.atan2(xAxis[1],xAxis[0]))};}
