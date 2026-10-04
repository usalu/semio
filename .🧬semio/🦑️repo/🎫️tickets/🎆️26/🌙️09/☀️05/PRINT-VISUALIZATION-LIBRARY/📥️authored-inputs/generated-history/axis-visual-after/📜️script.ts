import {compileVizProbeDocument} from '../../../../../../../../../🧰️framework/🛍️products/📓️print/🔨️modules/🧪️viz-probe/🟦️.ts';
import {inferVizChart} from '../../../../../../../../../🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🟦️.ts';
import type {VizChartSpecification} from '../../../../../../../../../🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🟦️.ts';
const body:{raw:string}[]=[];
for(const orient of ['top','bottom','angular'] as const){
 const angular=orient==='angular',options=angular?{center:'50,40',outerRadius:10,startAngle:0,endAngle:180,tickSize:2,titleGap:5}:{tickSize:2,titleGap:5};
 const spec:VizChartSpecification={width:100,height:80,language:'en',margin:{left:8,right:2,top:2,bottom:8},tables:[],layers:[],scales:[{name:'x',kind:'linear',domain:[0,10],range:angular?[0,180]:[10,90]}],guides:[{kind:'axis',scale:'x',orient:angular?'bottom':orient,tickValues:[0,5,10],title:{en:orient.toUpperCase(),de:orient.toUpperCase()},options:{...options,orient}}]};
 const result=await inferVizChart({chart:spec});if(!result.complete)throw Error(JSON.stringify(result.diagnostics));
 const native='\\begin{VizFigure}[width=100,height=80]\\SemioVizScale{x}{linear}{0,10}{'+(angular?'0,180':'10,90')+'}\\SemioVizAxis[scale=x,orient='+orient+',tickValues={0,5,10},title='+orient.toUpperCase()+',tickSize=2,titleGap=5'+(angular?',center={50,40},outerRadius=10,startAngle=0,endAngle=180':'')+']\\end{VizFigure}';
 body.push({raw:'\\par Native '+orient+'\\par '+native+'\\par Canonical '+orient+'\\par '+result.tikz});
}
await compileVizProbeDocument({case:'axis-visual',scenario:'after',documentClass:'semio',documentClassOptions:'type=paper,language=en',packages:['semio-viz'],geometry:true,preamble:['\\title{Axis Visual Proof}','\\author{Semio}','\\date{}'],body},{workDir:import.meta.dir,keepWorkDir:true});
