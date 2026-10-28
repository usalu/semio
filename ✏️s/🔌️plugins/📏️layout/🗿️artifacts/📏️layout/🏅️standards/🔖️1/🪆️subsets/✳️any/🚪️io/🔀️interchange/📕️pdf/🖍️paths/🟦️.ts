/** 🖍️ PDF current paths keep construction-time transforms and exact cubic controls. */
export type PdfMatrix=readonly[number,number,number,number,number,number];
export interface PathPoint{x:number;y:number}
export type PathSegment={kind:"moveTo"|"lineTo";to:PathPoint}|{kind:"cubicTo";c1:PathPoint;c2:PathPoint;to:PathPoint}|{kind:"close"};
export class PdfPath{private segments:PathSegment[]=[];private current:PathPoint|null=null;private start:PathPoint|null=null;
 private point(m:PdfMatrix,x:number,y:number,h:number,u:number):PathPoint{const p={x:(m[0]*x+m[2]*y+m[4])*u,y:h-(m[1]*x+m[3]*y+m[5])*u};if(!Number.isFinite(p.x)||!Number.isFinite(p.y))throw Error("layout.pdf.path-non-finite");return p;}
 private push(s:PathSegment):void{if(this.segments.length>=16384)throw Error("layout.pdf.path-segment-limit");this.segments.push(s);}
 moveTo(m:PdfMatrix,x:number,y:number,h:number,u=1):void{const to=this.point(m,x,y,h,u);this.push({kind:"moveTo",to});this.current=to;this.start=to;}
 lineTo(m:PdfMatrix,x:number,y:number,h:number,u=1):void{if(!this.current)throw Error("layout.pdf.path-current-point");const to=this.point(m,x,y,h,u);this.push({kind:"lineTo",to});this.current=to;}
 cubic(m:PdfMatrix,p:readonly number[],h:number,u=1):void{if(!this.current)throw Error("layout.pdf.path-current-point");const c1=this.point(m,p[0],p[1],h,u),c2=this.point(m,p[2],p[3],h,u),to=this.point(m,p[4],p[5],h,u);this.push({kind:"cubicTo",c1,c2,to});this.current=to;}
 initial(m:PdfMatrix,p:readonly number[],h:number,u=1):void{if(!this.current)throw Error("layout.pdf.path-current-point");const c1=this.current,c2=this.point(m,p[0],p[1],h,u),to=this.point(m,p[2],p[3],h,u);this.push({kind:"cubicTo",c1,c2,to});this.current=to;}
 finalCurve(m:PdfMatrix,p:readonly number[],h:number,u=1):void{this.cubic(m,[p[0],p[1],p[2],p[3],p[2],p[3]],h,u);}
 close():void{if(this.current){this.push({kind:"close"});this.current=this.start;}}
 rectangle(m:PdfMatrix,p:readonly number[],h:number,u=1):void{const[x,y,w,height]=p;this.moveTo(m,x,y,h,u);this.lineTo(m,x+w,y,h,u);this.lineTo(m,x+w,y+height,h,u);this.lineTo(m,x,y+height,h,u);this.close();}
 take():PathSegment[]{this.current=null;this.start=null;const rows=this.segments;this.segments=[];return rows;}
 get empty():boolean{return this.segments.length===0;}}
