/** 🧭️ Exact local geometry composes once with the PDF y-up page projection. */
export type PdfAffine=readonly[number,number,number,number,number,number];
export function pdfPathMatrix(a:PdfAffine,height:number):PdfAffine{return[a[0],-a[1],a[2],-a[3],a[4],height-a[5]];}
export function pdfTextMatrix(a:PdfAffine,origin:readonly[number,number],height:number,italic:boolean):PdfAffine{const skew=italic?0.22:0;return[a[0],-a[1],a[0]*skew-a[2],a[3]-a[1]*skew,a[0]*origin[0]+a[2]*origin[1]+a[4],height-(a[1]*origin[0]+a[3]*origin[1]+a[5])];}
export function pdfImageMatrix(a:PdfAffine,rect:readonly[number,number,number,number],height:number):PdfAffine{const[x,y,width,h]=rect;return[a[0]*width,-a[1]*width,-a[2]*h,a[3]*h,a[0]*x+a[2]*(y+h)+a[4],height-(a[1]*x+a[3]*(y+h)+a[5])];}
