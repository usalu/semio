/** 🧠 Hand-authored architecture contracts retain explicit incidence and existing native ownership. */
import { readFileSync, openSync, writeSync, closeSync, ftruncateSync, mkdirSync } from "node:fs";
import { join } from "node:path";
const root="C:/git/semio",product=join(root,"🧰️framework/🛍️products/📓️print");
export const architectures=[
 {kind:"feed-forward-neural-network",mode:"feedforward",render:"units",layers:[[3,"input","Input 3"],[4,"dense","Dense 4"],[2,"output","Output 2"]],links:"1/1/2/1,1/1/2/2,1/1/2/3,1/1/2/4,1/2/2/1,1/2/2/2,1/2/2/3,1/2/2/4,1/3/2/1,1/3/2/2,1/3/2/3,1/3/2/4,2/1/3/1,2/1/3/2,2/2/3/1,2/2/3/2,2/3/3/1,2/3/3/2,2/4/3/1,2/4/3/2"},
 {kind:"cnn-architecture",mode:"cnn",render:"blocks",layers:[[3,"image","Image 3"],[4,"convolution","Convolution 4"],[2,"pooling","Pool 2"],[2,"classifier","Classes 2"]],links:"1/1/2/1,2/1/3/1,3/1/4/1"},
 {kind:"rnn-architecture",mode:"rnn",render:"units",layers:[[2,"input","Sequence 2"],[3,"state","Recurrent state 3"],[2,"output","Output 2"]],links:"1/1/2/1,1/2/2/2,2/1/3/1,2/2/3/2,2/3/2/1"},
 {kind:"lstm-architecture",mode:"lstm",render:"blocks",layers:[[2,"input","Sequence"],[3,"memory","LSTM memory"],[2,"output","Output"]],links:"1/1/2/1,2/1/3/1,2/3/2/1"},
 {kind:"transformer-architecture",mode:"transformer",render:"blocks",layers:[[3,"token","Token embedding"],[4,"attention","Self attention"],[4,"feedforward","Feed forward"],[2,"output","Output projection"]],links:"1/1/2/1,2/1/3/1,3/1/4/1,1/1/3/4"},
 {kind:"attention-block",mode:"transformer",render:"blocks",layers:[[3,"query","Query"],[3,"key","Key"],[3,"value","Value"],[2,"context","Context"]],links:"1/1/4/1,2/1/4/1,3/1/4/2"},
 {kind:"encoder-decoder-diagram",mode:"encoder-decoder",render:"blocks",layers:[[4,"input","Input 4"],[3,"encoder","Encoder 3"],[2,"latent","Latent 2"],[3,"decoder","Decoder 3"],[4,"output","Output 4"]],links:"1/1/2/1,2/1/3/1,3/1/4/1,4/1/5/1"},
 {kind:"residual-network",mode:"residual",render:"units",layers:[[3,"input","Input 3"],[3,"block","Block A 3"],[3,"block","Block B 3"],[2,"output","Output 2"]],links:"1/1/2/1,1/2/2/2,1/3/2/3,2/1/3/1,2/2/3/2,2/3/3/3,1/1/3/1,1/2/3/2,1/3/3/3,3/1/4/1,3/2/4/2"},
 {kind:"gan-architecture",mode:"gan",render:"blocks",layers:[[2,"noise","Noise"],[3,"generator","Generator"],[3,"fake","Generated sample"],[3,"real","Real sample"],[2,"discriminator","Discriminator"]],links:"1/1/2/1,2/1/3/1,3/1/5/1,4/1/5/2"},
 {kind:"autoencoder",mode:"autoencoder",render:"blocks",layers:[[4,"input","Input 4"],[3,"encoder","Encoder 3"],[1,"latent","Code 1"],[3,"decoder","Decoder 3"],[4,"output","Reconstruction 4"]],links:"1/1/2/1,2/1/3/1,3/1/4/1,4/1/5/1"},
 {kind:"graph-neural-network",mode:"gnn",render:"units",layers:[[3,"vertex","Vertex features 3"],[3,"message","Messages 3"],[3,"update","Updated vertices 3"]],links:"1/1/2/1,1/2/2/2,1/3/2/3,1/1/2/2,1/2/2/3,1/3/2/1,2/1/3/1,2/2/3/2,2/3/3/3"},
 {kind:"computational-graph",mode:"computational-graph",render:"units",layers:[[2,"input","Inputs 2"],[2,"operation","Operations 2"],[1,"result","Result 1"]],links:"1/1/2/1,1/2/2/2,2/1/3/1,2/2/3/1"},
 {kind:"tensor-shape-diagram",mode:"tensor",render:"blocks",layers:[[4,"input","Tensor 4x3"],[3,"reshape","Reshape 3x4"],[2,"output","Projection 2x4"]],links:""},
 {kind:"model-pipeline",mode:"pipeline",render:"blocks",layers:[[3,"source","Data"],[2,"preprocess","Preprocess"],[3,"model","Model"],[2,"postprocess","Postprocess"]],links:"1/1/2/1,2/1/3/1,3/1/4/1"},
 {kind:"training-loop",mode:"training-loop",render:"blocks",layers:[[3,"batch","Batch"],[3,"model","Forward model"],[2,"loss","Loss"],[2,"gradient","Gradient"],[2,"update","Update"]],links:"1/1/2/1,2/1/3/1,3/1/4/1,4/1/5/1,5/1/2/3"},
 {kind:"diffusion-process-diagram",mode:"diffusion",render:"blocks",layers:[[4,"data","Clean sample"],[4,"noise","Noised sample"],[3,"denoiser","Denoiser"],[4,"sample","Reconstructed sample"]],links:"1/1/2/1,2/1/3/1,3/1/4/1,3/3/2/3"}
] as const;
function replace(path:string,before:string,after:string):void{const current=readFileSync(path),source=current.toString();if(source.split(before).length!==2)throw Error("Owned span missing or repeated "+path);const next=Buffer.from(source.replace(before,after));const file=openSync(path,"r+");try{if(!readFileSync(path).equals(current))throw Error("Concurrent source changed "+path);writeSync(file,next,0,next.length,0);ftruncateSync(file,next.length);}finally{closeSync(file);}}
if(process.argv[2]==="labels"){
 const de=[['Eingabe 3','Dichte Schicht 4','Ausgabe 2'],['Bild 3','Faltung 4','Pooling 2','Klassen 2'],['Sequenz 2','Rekurrenter Zustand 3','Ausgabe 2'],['Sequenz','LSTM Speicher','Ausgabe'],['Token Einbettung','Selbstaufmerksamkeit','Vorwärtsnetz','Ausgabeprojektion'],['Abfrage','Schlüssel','Wert','Kontext'],['Eingabe 4','Kodierer 3','Latent 2','Dekodierer 3','Ausgabe 4'],['Eingabe 3','Block A 3','Block B 3','Ausgabe 2'],['Rauschen','Generator','Erzeugte Probe','Reale Probe','Diskriminator'],['Eingabe 4','Kodierer 3','Code 1','Dekodierer 3','Rekonstruktion 4'],['Knotenmerkmale 3','Nachrichten 3','Aktualisierte Knoten 3'],['Eingaben 2','Operationen 2','Ergebnis 1'],['Tensor 4x3','Umformung 3x4','Projektion 2x4'],['Daten','Vorverarbeitung','Modell','Nachverarbeitung'],['Stapel','Vorwärtsmodell','Verlust','Gradient','Aktualisierung'],['Saubere Probe','Verrauschte Probe','Entrauscher','Rekonstruierte Probe']];
 const style=join(product,'🖋️latex/semio-viz-network-graph.sty'),fixturePath=join(product,'🧪️tests/🧬️native-chart-grammar/🔣️.json'),fixture=JSON.parse(readFileSync(fixturePath,'utf8'));
 for(const [index,entry]of architectures.entries()){const rows=entry.layers.map(([units,kind,label],row)=>`{${row+1},${units},${kind},{${label}}}`).join(', '),localized=entry.layers.map(([units,kind,label],row)=>`{${row+1},${units},${kind},{\\SemioVizLocalized{${label}}{${de[index]![row]}}}}`).join(', ');replace(style,`\\clist_map_inline:nn { ${rows} }`,`\\clist_map_inline:nn { ${localized} }`);const actual=fixture.nativeNeuralTopologyControls.vectors.architectures.find((item:{kind:string})=>item.kind===entry.kind);actual.labelsDe=de[index];}
 const old=readFileSync(fixturePath,'utf8'),pattern=/"nativeNeuralTopologyControls":\s*\{[\s\S]*?(?=\n\s*"nativeNeuralArchitectureControls":)/,span=old.match(pattern)?.[0];if(!span)throw Error('Topology fixture span absent');replace(fixturePath,span,'"nativeNeuralTopologyControls": '+JSON.stringify(fixture.nativeNeuralTopologyControls,null,2).replaceAll('\n','\n  ')+',');
 console.log('[DEBUG] Sixteen stock architecture layer captions now resolve explicit EN/de document language');
}
if(process.argv[2]==="label-oracle")replace(join(product,'🧪️tests/🧬️native-chart-grammar/🟦️.ts'),'entry.layers.forEach(([, ,label])=>{if(!text.replace(/\\s+/g," ").includes(String(label)))throw Error("Missing authored layer caption "+label);});','entry.layers.forEach(([, ,label],index)=>{const expected=language==="de"&&"labelsDe"in entry?entry.labelsDe[index]:label;if(!text.replace(/\\s+/g," ").includes(String(expected)))throw Error("Missing authored layer caption "+expected);});');
if(process.argv[2]==="caption-spaces"){
 const path=join(product,'🖋️latex/semio-viz-network-graph.sty');for(const line of readFileSync(path,'utf8').split('\n').filter(line=>line.includes('\\clist_map_inline:nn')&&line.includes('\\SemioVizLocalized'))){replace(path,line,line.replace(/\\SemioVizLocalized\{([^}]*)\}\{([^}]*)\}/g,(_,en,de)=>`\\SemioVizLocalized{${en.replaceAll(' ','~')}}{${de.replaceAll(' ','~')}}`));}console.log('[DEBUG] Neural stock label spaces preserved under native expl3 tokenization');
}
if(process.argv[2]==="caption-wrap"){
 const path=join(product,'🖋️latex/semio-viz-network-graph.sty');for(const line of readFileSync(path,'utf8').split('\n').filter(line=>line.includes('\\clist_map_inline:nn')&&line.includes('\\SemioVizLocalized'))){replace(path,line,line.replace(/\\SemioVizLocalized\{([^}]*)\}\{([^}]*)\}/g,(_,en,de)=>`\\SemioVizLocalized{${en.replaceAll('~','\\space{}')}}{${de.replaceAll('~','\\space{}')}}`));}
 replace(join(product,'🧪️tests/🧬️native-chart-grammar/🟦️.ts'),'if(!text.replace(/\\s+/g," ").includes(String(expected)))throw Error("Missing authored layer caption "+expected);','if(!text.replace(/-\\s+/g,"").replace(/\\s+/g," ").includes(String(expected)))throw Error("Missing authored layer caption "+expected);');console.log('[DEBUG] Neural stock captions use breakable spaces and per-layer paragraph width');
}
if(process.argv[2]==="guard-log")replace(join(product,'🧪️tests/🧬️native-chart-grammar/🟦️.ts'),'join(directory,"🧪️probe-out",entry.id+".log"),"utf8").replace(/\\s+/g," ")','join(directory,"🧪️probe-out",entry.id+".log"),"utf8").replace(/\\(semio-viz\\)/g,"").replace(/\\s+/g," ")');
if(process.argv[2]==="curve-fixture"){
 const path=join(product,'🧪️tests/🧬️native-chart-grammar/🔣️.json'),fixture=JSON.parse(readFileSync(path,'utf8')),bend:Record<string,number>={'rnn-architecture':.7,'lstm-architecture':3,'transformer-architecture':.35,'attention-block':.3,'residual-network':.35,'gan-architecture':-.35,'training-loop':-.35,'diffusion-process-diagram':-.65};for(const entry of fixture.nativeNeuralTopologyControls.vectors.architectures)entry.curvature=bend[entry.kind]??0;const source=readFileSync(path,'utf8'),span=source.match(/"nativeNeuralTopologyControls":\s*\{[\s\S]*?(?=\n\s*"nativeNeuralArchitectureControls":)/)?.[0];if(!span)throw Error('Owned topology span absent');replace(path,span,'"nativeNeuralTopologyControls": '+JSON.stringify(fixture.nativeNeuralTopologyControls,null,2).replaceAll('\n','\n  ')+',');
 const helper=join(product,'🧪️tests/🧬️native-chart-grammar/🟦️.ts');replace(helper,'options={...isBaselineGraph?{}:{mode:"feedforward",unitSize:c.unitSize}','options={curvature:"curvature"in entry?entry.curvature:0,...isBaselineGraph?{}:{mode:"feedforward",unitSize:c.unitSize}');
 replace(helper,'record.scenario===id&&record.key==="geometry/link"','record.scenario===id&&["geometry/link","geometry/curve"].includes(record.key)');
 replace(helper,'equal(actual[n]!.values.slice(0,4).map(Number),[...position(a!,u!),...position(b!,v!)],id+" complete directed incidence "+n,.001);if(actual[n]!.values[6]!=="arrow")throw Error("Missing directed edge "+n);','const source=position(a!,u!),target=position(b!,v!),bend="curvature"in entry?Number(entry.curvature):0,curved=b!==a!+1&&Math.abs(bend)>1e-9;if(actual[n]!.key!==(curved?"geometry/curve":"geometry/link"))throw Error("Missing visible bypass/recurrent route "+n);const control=[(source[0]!+target[0]!)/2-bend*(target[1]!-source[1]!),(source[1]!+target[1]!)/2+bend*(target[0]!-source[0]!)];equal(actual[n]!.values.slice(0,curved?6:4).map(Number),curved?[...source,...control,...target]:[...source,...target],id+" complete directed incidence "+n,.001);if(actual[n]!.values[curved?8:6]!=="arrow")throw Error("Missing directed edge "+n);');
 replace(helper,'const strokes=paint.paths.filter(path=>!path.closed&&path.points.length===2&&Math.hypot(path.bounds[2]!-path.bounds[0]!,path.bounds[3]!-path.bounds[1]!)>1);if(strokes.length!==links.length)','const strokes=paint.paths.filter(path=>!path.closed&&(path.points.length===2||path.points.length===129)&&Math.hypot(path.bounds[2]!-path.bounds[0]!,path.bounds[3]!-path.bounds[1]!)>1);if(strokes.length!==links.length)');
 replace(helper,'const end=path.points[1]!,distance=Math.hypot','const bend="curvature"in entry?Number(entry.curvature):0,curved=b!==a!+1&&Math.abs(bend)>1e-9;if(curved){if(path.points.length!==129)throw Error("Actual PDF bypass curve absent "+n);const control=[(source[0]!+target[0]!)/2-bend*(target[1]!-source[1]!),(source[1]!+target[1]!)/2+bend*(target[0]!-source[0]!)],oracle=d3Path();oracle.moveTo(source[0]!,source[1]!);oracle.bezierCurveTo(control[0]!,control[1]!,control[0]!,control[1]!,target[0]!,target[1]!);if(!oracle.toString().includes("C"))throw Error("Independent D3 curve absent");for(const sample of[32,64]){const t=sample/128,point=source.map((value,axis)=>(1-t)**3*value+3*(1-t)*t*control[axis]!+t**3*target[axis]!);equal(path.points[sample]!,point,id+" D3 actual bypass body "+n,.25);}if(Math.hypot(path.points.at(-1)![0]!-target[0]!,path.points.at(-1)![1]!-target[1]!)>1.5)throw Error("Actual PDF bypass target "+n);return;}const end=path.points[1]!,distance=Math.hypot');
 console.log('[DEBUG] Neutral curved bypass/recurrence contracts and independent D3 cubic/probe/PDF body oracle authored before product repair');
}
if(process.argv[2]==="route-filter")replace(join(product,'🧪️tests/🧬️native-chart-grammar/🟦️.ts'),'links:id==="empty"?neutral.custom.empty:neutral.custom.links,id}))];','links:id==="empty"?neutral.custom.empty:neutral.custom.links,id}))].filter(entry=>!process.env.PRINT_NATIVE_NEURAL_ROUTE_ONLY||("curvature"in entry&&entry.curvature!==0));');
if(process.argv[2]==="tip-fixture"){
 const path=join(product,'🧪️tests/🧬️native-chart-grammar/🔣️.json'),fixture=JSON.parse(readFileSync(path,'utf8')),contract=fixture.nativeNeuralTopologyControls;contract.vectors.visibleDirectedTips=true;contract.schema.required.push('visibleDirectedTips');contract.schema.properties.visibleDirectedTips={const:true};const source=readFileSync(path,'utf8'),span=source.match(/"nativeNeuralTopologyControls":\s*\{[\s\S]*?(?=\n\s*"nativeNeuralArchitectureControls":)/)?.[0];if(!span)throw Error('Owned neural topology contract missing');replace(path,span,'"nativeNeuralTopologyControls": '+JSON.stringify(contract,null,2).replaceAll('\n','\n  ')+',');console.log('[DEBUG] Neutral neural directed tip visibility requires actual target-fill exclusion before producer repair');
}
if(process.argv[2]==="tip-oracle"){
 const helper=join(product,'🧪️tests/🧬️native-chart-grammar/🟦️.ts');replace(helper,'const bend="curvature"in entry?Number(entry.curvature):0,curved=b!==a!+1&&Math.abs(bend)>1e-9;if(curved){if(path.points.length!==129)','const bend="curvature"in entry?Number(entry.curvature):0,curved=b!==a!+1&&Math.abs(bend)>1e-9,control=[(source[0]!+target[0]!)/2-bend*(target[1]!-source[1]!),(source[1]!+target[1]!)/2+bend*(target[0]!-source[0]!)],at=(t:number)=>source.map((value,axis)=>{if(!curved)return interpolateNumber(value,target[axis]!)(t);const p=interpolateNumber(value,control[axis]!)(t),q=control[axis]!,r=interpolateNumber(control[axis]!,target[axis]!)(t);return interpolateNumber(interpolateNumber(p,q)(t),interpolateNumber(q,r)(t))(t);}),mark=entry.render==="blocks"?marks[b!-1]:marks.find(mark=>Math.hypot((mark.bounds[0]!+mark.bounds[2]!)/2-target[0]!, (mark.bounds[1]!+mark.bounds[3]!)/2-target[1]!)<c.toleranceMm);if(!mark)throw Error("Tip target body missing "+n);const outline=mark.width/2,distance=(point:number[])=>{if(entry.render==="units")return Math.hypot(point[0]!-target[0]!,point[1]!-target[1]!)-c.unitSize;const qx=Math.abs(point[0]!-(mark.bounds[0]!+mark.bounds[2]!)/2)-(mark.bounds[2]!-mark.bounds[0]!)/2,qy=Math.abs(point[1]!-(mark.bounds[1]!+mark.bounds[3]!)/2)-(mark.bounds[3]!-mark.bounds[1]!)/2;return Math.hypot(Math.max(qx,0),Math.max(qy,0))+Math.min(Math.max(qx,qy),0);};let lower=0,upper=1;for(let iteration=0;iteration<48;iteration++){const middle=(lower+upper)/2;if(distance(at(middle))>outline)lower=middle;else upper=middle;}const boundary=(lower+upper)/2,terminal=at(boundary),tip=records.filter(record=>record.scenario===id&&record.key==="geometry/neural-tip")[n];if(!tip)throw Error("Native visible tip record absent "+n);equal(tip.values.slice(0,2).map(Number),terminal,id+" independent D3 target boundary "+n,.001);equal([Number(tip.values[2])],[boundary],id+" independent D3 boundary parameter "+n,.0001);equal([Number(tip.values[3])],[outline],id+" actual target outline "+n,.001);if(curved){if(path.points.length!==129)');
 replace(helper,'const t=sample/128,point=source.map((value,axis)=>(1-t)**3*value+3*(1-t)*t*control[axis]!+t**3*target[axis]!);equal(path.points[sample]!,point,id+" D3 actual bypass body "+n,.25);','const point=at(boundary*sample/128);equal(path.points[sample]!,point,id+" D3 actual bypass body "+n,.25);');
 replace(helper,'Math.hypot(path.points.at(-1)![0]!-target[0]!,path.points.at(-1)![1]!-target[1]!)>1.5','Math.hypot(path.points.at(-1)![0]!-terminal[0]!,path.points.at(-1)![1]!-terminal[1]!)>1.5');
 replace(helper,'const end=path.points[1]!,distance=Math.hypot(target[0]!-source[0]!,target[1]!-source[1]!),cross=','const end=path.points[1]!,length=Math.hypot(target[0]!-source[0]!,target[1]!-source[1]!),cross=');
 replace(helper,'if(Math.abs(cross)>c.toleranceMm*distance||Math.hypot(end[0]!-target[0]!,end[1]!-target[1]!)>1.5)throw Error("Actual PDF directed target "+n);','if(Math.abs(cross)>c.toleranceMm*length||Math.hypot(end[0]!-terminal[0]!,end[1]!-terminal[1]!)>1.5)throw Error("Actual PDF directed target "+n);');
 replace(helper,'" architecture inventories, captions, directed PDF/probe incidence matched D3");','" architecture inventories, captions, directed PDF/probe incidence matched D3");');
 console.log('[DEBUG] Existing D3 cubic oracle now adjudicates independently computed unit/block boundary prefix and actual visible head envelopes');
}
if(process.argv[2]==="curve-product"){
 const path=join(product,'🖋️latex/semio-viz-network-graph.sty'),source=readFileSync(path,'utf8'),span=source.match(/\\cs_new_protected:Npn \\semio_viz_nn_explicit_draw:n #1 \{[\s\S]*?\n\}(?=\n% 🧠 The connections)/)?.[0];if(!span)throw Error('Explicit native incidence owner absent');const args=String.raw`    { \semio_viz_nn_x:n { \l_semio_viz_nn_from_layer_int } }
    { \semio_viz_nn_y:nn { \l_semio_viz_nn_from_unit_int } { \l_semio_viz_nn_from_count_int } }
    { \semio_viz_nn_x:n { \l_semio_viz_nn_to_layer_int } }
    { \semio_viz_nn_y:nn { \l_semio_viz_nn_to_unit_int } { \l_semio_viz_nn_to_count_int } }
    { \l_semio_viz_net_link_width_fp }`;replace(path,span,String.raw`\cs_new_protected:Npn \semio_viz_nn_explicit_draw:n #1 {
  \semio_viz_nn_explicit_read:n {#1}
  \tl_set:Nn \l_semio_viz_net_stroke_tl { semio-chrome-border-normal }
  \fp_compare:nTF { abs(\l_semio_viz_net_curvature_fp)>1e-9 && \l_semio_viz_nn_to_layer_int!=\l_semio_viz_nn_from_layer_int+1 } {
    \semio_viz_net_draw_curve:nnnnnn
${args}
    { \l_semio_viz_net_curvature_fp }
  } {
    \semio_viz_net_draw_link:nnnnn
${args}
  }
}`);
 const catalogue=join(product,'🖼️assets/🔣️viz-catalog.json'),entries=JSON.parse(readFileSync(catalogue,'utf8')).kinds,fixture=JSON.parse(readFileSync(join(product,'🧪️tests/🧬️native-chart-grammar/🔣️.json'),'utf8'));for(const value of fixture.nativeNeuralTopologyControls.vectors.architectures.filter((entry:{curvature:number})=>entry.curvature!==0)){const entry=entries.find((entry:{slug:string})=>entry.slug===value.kind),before=JSON.stringify(entry,null,2).replaceAll('\n','\n    ');entry.options.curvature=value.curvature;replace(catalogue,before,JSON.stringify(entry,null,2).replaceAll('\n','\n    '));}
 console.log('[DEBUG] Existing native curve producer now honors explicit bypass/recurrence curvature and8 stock routed contracts');
}
if(process.argv[2]==="training-captions"){
 const style=join(product,'🖋️latex/semio-viz-network-graph.sty');replace(style,'\\SemioVizLocalized{Forward\\space{}model}{Vorwärtsmodell}','\\SemioVizLocalized{Forward\\space{}model}{Modell}');replace(style,'\\SemioVizLocalized{Update}{Aktualisierung}','\\SemioVizLocalized{Update}{Anpassen}');const path=join(product,'🧪️tests/🧬️native-chart-grammar/🔣️.json'),fixture=JSON.parse(readFileSync(path,'utf8')),entry=fixture.nativeNeuralTopologyControls.vectors.architectures.find((entry:{kind:string})=>entry.kind==='training-loop'),before=JSON.stringify(entry.labelsDe,null,2).replaceAll('\n','\n        ');entry.labelsDe=['Stapel','Modell','Verlust','Gradient','Anpassen'];replace(path,before,JSON.stringify(entry.labelsDe,null,2).replaceAll('\n','\n        '));console.log('[DEBUG] German five-slot training captions retain their role within the available text width');
}
if(process.argv[2]==="rust-emission")replace(join(product,'🧪️tests/🧬️native-chart-grammar/🟦️.ts'),'await Bun.write(join(directory,id+"-emitted.tex"),inferred.tikz);body.push({raw:"\\\\clearpage\\\\SemioVizProbeBegin{neural-topology}{"+id+"}"+inferred.tikz.replace','const emission=process.env.PRINT_NATIVE_NEURAL_TIKZ&&id!=="strings"?readFileSync(join(process.env.PRINT_NATIVE_NEURAL_TIKZ,id+"-"+language+".tex"),"utf8"):inferred.tikz;await Bun.write(join(directory,id+"-emitted.tex"),emission);body.push({raw:"\\\\clearpage\\\\SemioVizProbeBegin{neural-topology}{"+id+"}"+emission.replace');
if(process.argv[2]==="fixture"){
 const path=join(product,"🧪️tests/🧬️native-chart-grammar/🔣️.json"),fixture=JSON.parse(readFileSync(path,"utf8"));if(fixture.nativeNeuralTopologyControls)throw Error("Topology fixture already exists");
 const neutral={schema:{type:"object",required:["frame","padding","unitSize","toleranceMm","architectures"],properties:{frame:{type:"array",items:{type:"number"},minItems:2,maxItems:2},padding:{type:"number"},unitSize:{type:"number"},toleranceMm:{type:"number"},architectures:{type:"array",minItems:16,maxItems:16,items:{type:"object",required:["kind","mode","render","layers","links"],properties:{kind:{type:"string"},mode:{type:"string"},render:{enum:["units","blocks"]},layers:{type:"array",items:{type:"array",minItems:3,maxItems:3}},links:{type:"string"}}}}}},vectors:{frame:[80,50],padding:3,unitSize:1.1,toleranceMm:.04,architectures},invalid:["1/1/2","1/1/2/1/3","0/1/2/1","1/0/2/1","1/1/4/1","1/1/2/4","1.5/1/2/1","1/1/2/x","1/1/2/0"],custom:{layers:[[2,"input","Input"],[3,"hidden","Hidden"],[2,"output","Output"]],links:"1/1/2/1,1/2/2/2,2/3/2/1,2/1/3/1,2/2/3/2",empty:""}};
 replace(path,'"nativeNeuralArchitectureControls":',`"nativeNeuralTopologyControls": ${JSON.stringify(neutral,null,2).replaceAll("\n","\n  ")},\n  "nativeNeuralArchitectureControls":`);
 console.log("[DEBUG] Authored16 neutral architecture topologies and9 invalid incidence vectors");
}
if(process.argv[2]==="product"){
 const owner=join(product,"🖋️latex/semio-viz-network-graph.sty"),schemaPath=join(product,"🧬️schema/🔣️.json"),catalogPath=join(product,"🖼️assets/🔣️viz-catalog.json");
 const demos=architectures.map(a=>({name:"demo-neural-"+a.kind,columns:["layer","units","kind","label"],rows:a.layers.map(([units,kind,label],index)=>({layer:index+1,units,kind,label})),description:{en:`Architecture overview: ${a.layers.map(row=>row[2]).join(" → ")}.`,de:`Architekturübersicht: ${a.layers.map(row=>row[2]).join(" → ")}.`}}));
 const source=JSON.parse(readFileSync(schemaPath,"utf8")),catalog=JSON.parse(readFileSync(catalogPath,"utf8"));
 for(const a of architectures){const entry=catalog.kinds.find((entry:{slug:string})=>entry.slug===a.kind);if(!entry)throw Error("Missing owned stock "+a.kind);const before=JSON.stringify(entry,null,2).replaceAll("\n","\n    ");entry.family="neural-network";entry.options={variant:a.kind,mode:a.mode,render:a.render,connections:"none",links:a.links,directed:true,maxUnits:10,padding:3,unitSize:1.1};entry.data="demo-neural-"+a.kind;replace(catalogPath,before,JSON.stringify(entry,null,2).replaceAll("\n","\n    "));}
 const before=JSON.stringify(source["x-semio-demo-tables"].find((entry:{name:string})=>entry.name==="demo-layers"),null,2).replaceAll("\n","\n    ");replace(schemaPath,before,before+",\n    "+demos.map(demo=>JSON.stringify(demo,null,2).replaceAll("\n","\n    ")).join(",\n    "));
 const blocks=demos.map(demo=>`  \\seq_if_in:NnF \\g_semio_viz_table_names_seq { ${demo.name} } {\n    \\semio_viz_table_new:nn { ${demo.name} } { layer, units, kind, label }\n    \\clist_map_inline:nn { ${demo.rows.map(row=>`{${row.layer},${row.units},${row.kind},{${row.label}}}`).join(", ")} }\n      { \\semio_viz_table_row:nn { ${demo.name} } {##1} }\n  }`).join("\n");
 replace(owner,"  \\seq_if_in:NnF \\g_semio_viz_table_names_seq { demo-commits } {",blocks+"\n  \\seq_if_in:NnF \\g_semio_viz_table_names_seq { demo-commits } {");
 console.log("[DEBUG] Handcrafted16 stock neural architecture tables, labels, explicit incidence and schema declarations");
}
if(process.argv[2]==="baseline-harness"){
 const path=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts");
 replace(path,'},options={mode:"feedforward",padding:c.padding,unitSize:c.unitSize,labels:true,...isCustom?', '},isBaselineGraph=process.env.PRINT_NATIVE_NEURAL_BASELINE&&entry.kind==="graph-neural-network",options={...isBaselineGraph?{}:{mode:"feedforward",unitSize:c.unitSize},padding:c.padding,labels:true,...isCustom?');
 replace(path,'original=JSON.stringify(base),changed=changeVizChartValue(base,{path:["presets","0","options","mode"],value:entry.mode})','original=JSON.stringify(base),changed=changeVizChartValue(base,{path:["presets","0","options",isBaselineGraph?"labels":"mode"],value:isBaselineGraph?false:entry.mode})');
 console.log("[DEBUG] Baseline graph GNN preserves declared graph options until native stock topology RED");
}
if(process.argv[2]==="rust-limit")replace(join(product,"🧬️schema/💡️inferences/🦀️.rs"),'let maximum=selected("maxUnits").and_then(numeric).unwrap_or(10.0);','let maximum=match selected("maxUnits"){Some(value)=>numeric(value).ok_or_else(||"neural visible unit limit requires a positive integer literal".to_string())?,None=>10.0};');
if(process.argv[2]==="graph-fixture"){
 const cases=[{id:"known",nodes:["A","B"],source:"A",target:"B",valid:true},{id:"spaces",nodes:[" A ","B"],source:" A ",target:"B",valid:true},{id:"numbers",nodes:[0,1],source:0,target:1,valid:true},{id:"booleans",nodes:[false,true],source:false,target:true,valid:true},{id:"duplicate",nodes:["A","A"],source:"A",target:"A",valid:false,diagnostic:"Duplicate graph node ID"},{id:"empty-node",nodes:["","B"],source:"",target:"B",valid:false,diagnostic:"Graph node IDs must be present"},{id:"null-node",nodes:[null,"B"],source:null,target:"B",valid:false,diagnostic:"Graph node IDs must be present"},{id:"unknown-source",nodes:["A","B"],source:"Z",target:"B",valid:false,diagnostic:"Unknown graph endpoint"},{id:"unknown-target",nodes:["A","B"],source:"A",target:"Z",valid:false,diagnostic:"Unknown graph endpoint"},{id:"empty-source",nodes:["A","B"],source:"",target:"B",valid:false,diagnostic:"Graph node IDs must be present"},{id:"empty-target",nodes:["A","B"],source:"A",target:"",valid:false,diagnostic:"Graph node IDs must be present"},{id:"null-source",nodes:["A","B"],source:null,target:"B",valid:false,diagnostic:"Graph node IDs must be present"},{id:"missing-target",nodes:["A","B"],source:"A",valid:false,diagnostic:"Graph node IDs must be present"}];
 const contract={schema:{type:"array",minItems:13,maxItems:13,items:{type:"object",required:["id","nodes","source","valid"],properties:{id:{type:"string"},nodes:{type:"array",minItems:2,maxItems:2,items:{type:["string","number","boolean","null"]}},source:{type:["string","number","boolean","null"]},target:{type:["string","number","boolean","null"]},valid:{type:"boolean"},diagnostic:{type:"string"}}}},vectors:cases};
 replace(join(product,"🧪️tests/🧬️native-chart-grammar/🔣️.json"),'"nativeNeuralTopologyControls":',`"nativeGraphReferenceControls":${JSON.stringify(contract,null,2)},\n"nativeNeuralTopologyControls":`);console.log("[DEBUG] Authored13 scalar graph identity and endpoint cases");
}
if(process.argv[2]==="fixture-kind")replace(join(product,"🧪️tests/🧬️native-chart-grammar/🔣️.json"),'"kind": "computational-graph-67"','"kind": "computational-graph"');
if(process.argv[2]==="graph-oracle")replace(join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts"),'&&new Set(ids).size===ids.length&&from!==undefined','&&d3Group(ids,id=>id).size===ids.length&&from!==undefined');
if(process.argv[2]==="numeric-links"){
 const ts=join(product,"🧬️schema/💡️inferences/📚️catalogue/🟦️.ts"),rs=join(product,"🧬️schema/💡️inferences/🦀️.rs");
 replace(ts,'const layer=String(settings.layerColumn??"layer"),units=String(settings.unitsColumn??"units"),maximum=Number(settings.maxUnits??10);','const numeric=(value:unknown)=>typeof value==="number"?value:typeof value==="string"&&/^[+-]?(?:\\d+(?:\\.\\d*)?|\\.\\d+)(?:[eE][+-]?\\d+)?$/.test(value.trim())?Number(value):NaN;const layer=String(settings.layerColumn??"layer"),units=String(settings.unitsColumn??"units"),maximum=numeric(settings.maxUnits??10);');
 replace(ts,'layer:Number(row[layer])','layer:numeric(row[layer])');replace(ts,'ordered.map(({row})=>Number(row[units]))','ordered.map(({row})=>numeric(row[units]))');replace(ts,'counts.some(count=>!Number.isSafeInteger(count)||count<1)','counts.some(count=>!Number.isSafeInteger(count)||count<0||count>2147483647)');replace(ts,'maximum<1)throw Error("Neural visible','(maximum<1||maximum>2147483647))throw Error("Neural visible');
 replace(rs,'value.as_str().and_then(|value|value.parse::<f64>().ok())','value.as_str().and_then(|value|value.trim().parse::<f64>().ok())');replace(rs,'!(1.0..=9007199254740991.0).contains(&maximum)','!(1.0..=2147483647.0).contains(&maximum)');replace(rs,'(1.0..=9007199254740991.0).contains(value)','(0.0..=2147483647.0).contains(value)');
}
if(process.argv[2]==="graph-proof-fixes"){
 const ts=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts");replace(ts,'ids.includes(from)&&ids.includes(to)','ids.includes(from??"")&&ids.includes(to??"")');replace(ts,'"diagnostic"in entry&&log.replace','"diagnostic"in entry&&typeof entry.diagnostic==="string"&&log.replace');
 const path=join(product,"🖋️latex/semio-viz-network.sty"),source=readFileSync(path,"utf8"),line=source.match(/  \\tl_if_empty:NT \\l_semio_viz_network_cell_three_tl[^\r\n]+[\r\n]+  \\semio_viz_table_scalar:N \\l_semio_viz_network_cell_three_tl/);if(!line)throw Error("Graph weight span absent");const eol=line[0].includes("\r\n")?"\r\n":"\n";replace(path,line[0],"  \\semio_viz_table_scalar:N \\l_semio_viz_network_cell_three_tl"+eol+line[0].split(/\r?\n/)[0]);
}
if(process.argv[2]==="block-oracle"){
 const path=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts");replace(path,'position(n+1,1)[1]!-c.unitSize','Math.min(position(n+1,1)[1]!,height/2-c.unitSize)');replace(path,'position(n+1,Number(entry.layers[n]![0]))[1]!+c.unitSize','Math.max(position(n+1,Number(entry.layers[n]![0]))[1]!,height/2+c.unitSize)');
}
if(process.argv[2]==="typed-neural"){
 const source=join(product,"🧬️schema/💡️inferences/📚️catalogue/🟦️.ts"),tests=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts");replace(source,'METADATA["x-semio-demo-tables"].find(table=>table.name===data)as unknown as {rows?:readonly VizRow[]}|undefined','METADATA["x-semio-demo-tables"].find(table=>table.name===data)');
 replace(tests,'process.env.PRINT_NATIVE_NEURAL_BASELINE?[]:["custom","empty"]','process.env.PRINT_NATIVE_NEURAL_BASELINE?[]:["custom","empty","strings"]');replace(tests,'links:id==="custom"?neutral.custom.links:neutral.custom.empty','links:id==="empty"?neutral.custom.empty:neutral.custom.links');replace(tests,'rows:entry.layers.map(([units,kind,label],index)=>({layer:index+1,units,kind,label}))','rows:entry.layers.map(([units,kind,label],index)=>({layer:id==="strings"?String(index+1):index+1,units:id==="strings"?String(units):units,kind,label}))');replace(tests,'[DEBUG] Neural36 bilingual/theme stock/custom/empty architecture','[DEBUG] Neural38 bilingual/theme stock/custom/empty/string-scalar architecture');
}
if(process.argv[2]==="tip-tuples"){
 const path=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts");
 replace(path,"polygonContains(mark.points,point)","polygonContains(mark.points.map(value=>[value[0]!,value[1]!]as[number,number]),[point[0]!,point[1]!])");
 console.log("[DEBUG] Actual PDF polygon coordinates narrowed to explicit two-axis D3 tuples");
}
if(process.argv[2]==="tip-analytic-boundary"){
 const path=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts");
 replace(path,"const qx=Math.abs(point[0]!-(mark.bounds[0]!+mark.bounds[2]!)/2)-(mark.bounds[2]!-mark.bounds[0]!)/2,qy=Math.abs(point[1]!-(mark.bounds[1]!+mark.bounds[3]!)/2)-(mark.bounds[3]!-mark.bounds[1]!)/2;","const qx=Math.abs(point[0]!-x(b!-.5))-.34*(width-2*c.padding)/entry.layers.length,qy=Math.abs(point[1]!-height/2)-Math.max(c.unitSize,position(b!,Number(entry.layers[b!-1]![0]))[1]!-height/2);");
 console.log("[DEBUG] D3 target-box intersection now derives exact neutral extents, independently verified against actual PDF marks");
}
if(process.argv[2]==="neural-custom-default-contract"){
 const path=join(product,"🧬️schema/🔣️.json");
 replace(path,"Directed fromLayer/fromUnit/toLayer/toUnit incidences use one-based ordered layer ranks and visible units; supplied empty links draw no connections.","Directed fromLayer/fromUnit/toLayer/toUnit incidences use one-based ordered layer ranks and visible units; supplied empty links draw no connections. Authored data, layerColumn, unitsColumn, maxUnits or connections select the implicit connection policy unless links are explicitly supplied.");
 replace(path,"Gerichtete fromLayer/fromUnit/toLayer/toUnit-Inzidenzen verwenden einsbasierte geordnete Schichtränge und sichtbare Einheiten; explizit leere links zeichnen keine Verbindungen.","Gerichtete fromLayer/fromUnit/toLayer/toUnit-Inzidenzen verwenden einsbasierte geordnete Schichtränge und sichtbare Einheiten; explizit leere links zeichnen keine Verbindungen. Explizite data, layerColumn, unitsColumn, maxUnits oder connections wählen die implizite Verbindungsvorgabe, sofern links nicht explizit angegeben sind.");
 console.log("[DEBUG] Neural stock/custom connection inheritance contract declared in bilingual schema metadata");
}
if(process.argv[2]==="neural-custom-defaults"){
 const native=join(product,"🖋️latex/semio-viz-network-graph.sty"),plot=join(product,"🖋️latex/semio-viz-plot.sty"),ts=join(product,"🧬️schema/💡️inferences/📚️catalogue/🟦️.ts"),rs=join(product,"🧬️schema/💡️inferences/🦀️.rs");
 replace(native,"\\bool_new:N \\l_semio_viz_nn_links_supplied_bool",String.raw`\bool_new:N \l_semio_viz_nn_links_supplied_bool
\bool_new:N \l_semio_viz_nn_catalog_custom_bool
\tl_new:N \l_semio_viz_nn_catalog_options_tl
% 🧩 Structural selectors choose the implicit policy while authored links keep precedence.
\cs_new_protected:Npn \semio_viz_nn_catalog_authored:n #1 {
  \str_case:nn {#1} {
    {data}{\bool_set_true:N\l_semio_viz_nn_catalog_custom_bool}
    {layerColumn}{\bool_set_true:N\l_semio_viz_nn_catalog_custom_bool}
    {unitsColumn}{\bool_set_true:N\l_semio_viz_nn_catalog_custom_bool}
    {maxUnits}{\bool_set_true:N\l_semio_viz_nn_catalog_custom_bool}
    {connections}{\bool_set_true:N\l_semio_viz_nn_catalog_custom_bool}
  }
}
\cs_new_protected:Npn \semio_viz_nn_catalog_authored:nn #1#2 { \semio_viz_nn_catalog_authored:n {#1} }
\cs_new_protected:Npn \semio_viz_nn_catalog_keep:n #1 {
  \str_case:nnF {#1} {{links}{}{connections}{}} { \tl_put_right:Nn\l_semio_viz_nn_catalog_options_tl {#1,} }
}
\cs_new_protected:Npn \semio_viz_nn_catalog_keep:nn #1#2 {
  \str_case:nnF {#1} {{links}{}{connections}{}} { \tl_put_right:Nn\l_semio_viz_nn_catalog_options_tl {#1={#2},} }
}
\cs_new_protected:Npn \semio_viz_nn_catalog_options:Nn #1#2 {
  \bool_set_false:N \l_semio_viz_nn_catalog_custom_bool
  \keyval_parse:NNn \semio_viz_nn_catalog_authored:n \semio_viz_nn_catalog_authored:nn {#2}
  \bool_if:NT \l_semio_viz_nn_catalog_custom_bool {
    \tl_clear:N \l_semio_viz_nn_catalog_options_tl
    \exp_args:NNNV \keyval_parse:NNn \semio_viz_nn_catalog_keep:n \semio_viz_nn_catalog_keep:nn #1
    \tl_set_eq:NN #1 \l_semio_viz_nn_catalog_options_tl
  }
}`);
 replace(plot,"      \\tl_if_empty:nF {#2} { \\tl_put_right:Nn \\l_semio_viz_kind_options_tl { , #2 } }","      \\str_if_eq:VnT \\l_semio_viz_kind_family_tl { neural-network } { \\semio_viz_nn_catalog_options:Nn \\l_semio_viz_kind_options_tl {#2} }\n      \\tl_if_empty:nF {#2} { \\tl_put_right:Nn \\l_semio_viz_kind_options_tl { , #2 } }");
 replace(ts,'if(entry.family==="neural-network"){const effective={...entry.options,...preset.options},neuralData=String(data??effective.data??entry.data);validateNeuralLinks(effective,neuralData==="demo"?entry.data:neuralData,spec.tables??[]);}','if(entry.family==="neural-network"){const structural=data!==undefined||["data","layerColumn","unitsColumn","maxUnits","connections"].some(key=>preset.options?.[key]!==undefined),effective={...entry.options,...preset.options,...structural&&preset.options?.links===undefined?{links:undefined}:{}},neuralData=String(data??effective.data??entry.data);validateNeuralLinks(effective,neuralData==="demo"?entry.data:neuralData,spec.tables??[]);}');
 replace(rs,'fn validate_neural_links(chart:&DslValue,preset:&DslValue,entry:&DslValue)->Result<(),String>{','fn validate_neural_links(chart:&DslValue,preset:&DslValue,entry:&DslValue)->Result<(),String>{\n if preset.get("options").and_then(|options|options.get("links")).is_none()&&(preset.get("data").is_some()||["data","layerColumn","unitsColumn","maxUnits","connections"].iter().any(|key|preset.get("options").and_then(|options|options.get(*key)).is_some())){return Ok(());}');
 console.log("[DEBUG] Native/TS/Rust neural stock link inheritance yields to authored structural/connection controls with explicit links preserved");
}
if(process.argv[2]==="implicit-tip-fixture"){
 const json=join(product,"🧪️tests/🧬️native-chart-grammar/🔣️.json"),ts=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts"),source=readFileSync(json,"utf8"),fixture=JSON.parse(source),control=fixture.nativeNeuralTopologyControls;control.schema.required.push("implicitConnections");control.schema.properties.implicitConnections={type:"array",items:{enum:["full","adjacent","none"]},minItems:3,maxItems:3,uniqueItems:true};control.vectors.implicitConnections=["full","adjacent","none"];const span=source.match(/"nativeNeuralTopologyControls":\s*\{[\s\S]*?(?=\n\s*"nativeNeuralArchitectureControls":)/)?.[0];if(!span)throw Error("Owned topology fixture missing");replace(json,span,'"nativeNeuralTopologyControls": '+JSON.stringify(control,null,2).replaceAll('\n','\n  ')+',');
 replace(ts,'const entries=[...c.architectures,...(process.env.PRINT_NATIVE_NEURAL_BASELINE?[]:["custom","empty","strings"]).map(id=>({kind:"feed-forward-neural-network",mode:"rnn",render:"units",layers:neutral.custom.layers,links:id==="empty"?neutral.custom.empty:neutral.custom.links,id}))]','const implicit=c.implicitConnections.map(connections=>({kind:"feed-forward-neural-network",mode:"rnn",render:"units",layers:neutral.custom.layers,connections,id:"implicit-"+connections,links:neutral.custom.layers.slice(1).flatMap((layer,index)=>Array.from({length:Number(neutral.custom.layers[index]![0])},(_,from)=>Array.from({length:Number(layer[0])},(_,to)=>connections==="none"||connections==="adjacent"&&from!==to?[]:[`${index+1}/${from+1}/${index+2}/${to+1}`]).flat()).flat()).join(",")}));\n const entries=[...c.architectures,...(process.env.PRINT_NATIVE_NEURAL_BASELINE?[]:["custom","empty","strings"]).map(id=>({kind:"feed-forward-neural-network",mode:"rnn",render:"units",layers:neutral.custom.layers,links:id==="empty"?neutral.custom.empty:neutral.custom.links,id})),...process.env.PRINT_NATIVE_NEURAL_BASELINE?[]:implicit]');
 replace(ts,'data:table.name,render:entry.render,directed:true,links:entry.links,connections:"full"','data:table.name,render:entry.render,directed:true,..."connections"in entry?{connections:entry.connections}:{links:entry.links,connections:"full"}');
 replace(ts,'process.env.PRINT_NATIVE_NEURAL_TIKZ&&id!=="strings"','process.env.PRINT_NATIVE_NEURAL_TIKZ&&id!=="strings"&&!id.startsWith("implicit-")');
 console.log("[DEBUG] Three neutral implicit custom connection policies now share complete actual directed target-boundary assertions");
}
if(process.argv[2]==="implicit-rust-corpus"){
 const rs=join(product,"🧪️tests/🧬️chart-mutations/🦀️.rs"),ts=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts");
 replace(rs,' }eprintln!("[DEBUG] Neural Rust{count} neutral en/de stock/custom/empty/invalid incidence cases passed mutation, inverse and canonical inference");',String.raw`  for connections in source["vectors"]["implicitConnections"].as_array().unwrap(){let before=ChartSnapshot::from_value(DslValue::from(serde_json::json!({"chart":{"width":80,"height":50,"language":language,"layers":[],"tables":[{"name":"neural-custom","columns":["layer","units","kind","label"],"rows":layers}],"presets":[{"kind":"feed-forward-neural-network","options":{"data":"neural-custom","connections":"residual","mode":"rnn","directed":true,"curvature":0}}]}}))).unwrap();let mutation=ChangeChartValue::from_value(DslValue::from(serde_json::json!({"path":["presets","0","options","connections"],"value":connections}))).unwrap();let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());let current=outcome.diff().apply(&before).unwrap();assert_eq!(outcome.diff().inverse(&before).apply(&current).unwrap(),before);let result=ChartInference::infer(&current).unwrap();assert!(result.complete,"{language}/{connections}: {:?}",result.diagnostics);if let Some(root)=std::env::var_os("PRINT_NATIVE_NEURAL_TIKZ"){std::fs::create_dir_all(&root).unwrap();std::fs::write(std::path::Path::new(&root).join(format!("implicit-{}-{language}.tex",connections.as_str().unwrap())),result.tikz).unwrap();}count+=1;}
 }eprintln!("[DEBUG] Neural Rust{count} neutral en/de stock/custom/empty/implicit/invalid incidence cases passed mutation, inverse and canonical inference");`.replaceAll('\\"','"'));
 replace(ts,'process.env.PRINT_NATIVE_NEURAL_TIKZ&&id!=="strings"&&!id.startsWith("implicit-")','process.env.PRINT_NATIVE_NEURAL_TIKZ&&id!=="strings"');
 console.log("[DEBUG] Rust canonical producer now emits six implicit policy sources alongside existing 36 stock/custom/empty sources");
}
if(process.argv[2]==="caption-slot-fixture"){
 const json=join(product,"🧪️tests/🧬️native-chart-grammar/🔣️.json"),ts=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts"),source=readFileSync(json,"utf8"),fixture=JSON.parse(source),control=fixture.nativeNeuralTopologyControls;control.schema.required.push("captionsWithinSlots");control.schema.properties.captionsWithinSlots={const:true};control.vectors.captionsWithinSlots=true;const span=source.match(/"nativeNeuralTopologyControls":\s*\{[\s\S]*?(?=\n\s*"nativeNeuralArchitectureControls":)/)?.[0];if(!span)throw Error("Owned topology fixture missing");replace(json,span,'"nativeNeuralTopologyControls": '+JSON.stringify(control,null,2).replaceAll('\n','\n  ')+',');
 replace(ts,'entries=c.architectures.filter(entry=>["feed-forward-neural-network","cnn-architecture"].includes(entry.kind)),failures:string[]=[];if(!c.visibleDirectedTips)','entries=c.architectures,failures:string[]=[];if(!c.captionsWithinSlots)throw Error("Neural caption slot neutral contract absent");if(!c.visibleDirectedTips)');
 replace(ts,'if(heads.length!==links.length)throw Error("Directed arrowhead inventory "+heads.length);','const step=(c.frame[0]-2*c.padding)/entry.layers.length;for(const glyph of paint.glyphs){if(!glyph.origin||!glyph.advance)throw Error("Actual caption advance absent");const left=Math.min(glyph.origin[0]!,glyph.origin[0]!+glyph.advance[0]!),right=Math.max(glyph.origin[0]!,glyph.origin[0]!+glyph.advance[0]!),slot=Math.round(((left+right)/2-c.padding)/step-.5);if(slot<0||slot>=entry.layers.length||left<c.padding+slot*step+.5-c.toleranceMm||right>c.padding+(slot+1)*step-.5+c.toleranceMm)throw Error("Caption advance exceeds independent D3 layer slot "+glyph.text);}if(heads.length!==links.length)throw Error("Directed arrowhead inventory "+heads.length);');
 replace(ts,'console.log("[DEBUG] Neural46 actual bilingual/theme unit/block directed arrowhead polygons remain outside target opaque fill by independent D3 membership");','console.log("[DEBUG] Neural"+2*entries.reduce((count,entry)=>count+(entry.links?entry.links.split(",").length:0),0)+" actual bilingual/theme stock directed heads remain outside opaque targets and all caption advances fit independent D3 slots");');
 console.log("[DEBUG] All stock caption advances and all stock directed arrowheads now have neutral actual PDF slot/membership assertions");
}
if(process.argv[2]==="caption-empty-links"){
 replace(join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts"),'links=entry.links.split(",").map(record=>record.split("/").map(Number)),x=scaleLinear([0,entry.layers.length],[c.padding,c.frame[0]-c.padding])','links=entry.links?entry.links.split(",").map(record=>record.split("/").map(Number)):[],x=scaleLinear([0,entry.layers.length],[c.padding,c.frame[0]-c.padding])');
}
if(process.argv[2]==="selected-diagram-count"){
 replace(join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts"),'console.log("[DEBUG] Native diagram "+control.cases.length*2','console.log("[DEBUG] Native diagram "+cases.length*2');
}
if(process.argv[2]==="implicit-and-head-repair"){
 const native=join(product,"🖋️latex/semio-viz-network-graph.sty"),json=join(product,"🧪️tests/🧬️native-chart-grammar/🔣️.json"),ts=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts");
 const source=readFileSync(json,"utf8"),fixture=JSON.parse(source),control=fixture.nativeNeuralTopologyControls;control.schema.required.push("directedTipClearanceMm");control.schema.properties.directedTipClearanceMm={const:.45};control.vectors.directedTipClearanceMm=.45;const span=source.match(/"nativeNeuralTopologyControls":\s*\{[\s\S]*?(?=\n\s*"nativeNeuralArchitectureControls":)/)?.[0];if(!span)throw Error("Owned topology fixture missing");replace(json,span,'"nativeNeuralTopologyControls": '+JSON.stringify(control,null,2).replaceAll('\n','\n  ')+',');
 replace(ts,'if(distance(at(middle))>outline)lower=middle;else upper=middle;','if(distance(at(middle))>outline+c.directedTipClearanceMm)lower=middle;else upper=middle;');
 replace(native,'\\fp_compare:nNnTF { \\l_semio_viz_net_tip_distance_fp } > { \\l_semio_viz_net_tip_outline_fp }','\\fp_compare:nNnTF { \\l_semio_viz_net_tip_distance_fp } > { \\l_semio_viz_net_tip_outline_fp+.45 }');
 replace(native,String.raw`\cs_new_protected:Npn \semio_viz_nn_link:nnn #1#2#3 {
  \semio_viz_net_draw_link:nnnnn
    { \semio_viz_nn_x:n { #1 - 1 } } { \semio_viz_nn_y:nn {#2} { \l_semio_viz_nn_b_int } }
    { \semio_viz_nn_x:n {#1} } { \semio_viz_nn_y:nn {#3} { \l_semio_viz_nn_a_int } }
    { \l_semio_viz_net_link_width_fp }
}`,String.raw`\cs_new_protected:Npn \semio_viz_nn_link:nnn #1#2#3 {
  \bool_if:NTF \l_semio_viz_net_directed_bool {
    \int_set:Nn \l_semio_viz_nn_from_layer_int {#1-1}
    \int_set:Nn \l_semio_viz_nn_to_layer_int {#1}
    \int_set:Nn \l_semio_viz_nn_from_unit_int {#2}
    \int_set:Nn \l_semio_viz_nn_to_unit_int {#3}
    \int_set_eq:NN \l_semio_viz_nn_from_count_int \l_semio_viz_nn_b_int
    \int_set_eq:NN \l_semio_viz_nn_to_count_int \l_semio_viz_nn_a_int
    \fp_zero:N \l_semio_viz_net_c_fp
    \semio_viz_nn_directed_draw:
  } {
    \semio_viz_net_draw_link:nnnnn
      { \semio_viz_nn_x:n { #1 - 1 } } { \semio_viz_nn_y:nn {#2} { \l_semio_viz_nn_b_int } }
      { \semio_viz_nn_x:n {#1} } { \semio_viz_nn_y:nn {#3} { \l_semio_viz_nn_a_int } }
      { \l_semio_viz_net_link_width_fp }
  }
}`);
 for(const[before,after]of[["{Discriminator}",String.raw`{Dis\-crim\-in\-ator}`],["{Diskriminator}",String.raw`{Dis\-kri\-mi\-na\-tor}`],["{Selbstaufmerksamkeit}",String.raw`{Selbst\-auf\-merk\-sam\-keit}`],["{Vorwärtsnetz}",String.raw`{Vor\-wärts\-netz}`],["{Ausgabeprojektion}",String.raw`{Aus\-gabe\-pro\-jek\-tion}`],["Reconstruction\\space{}4",String.raw`Recon\-struc\-tion\space{}4`],["Rekonstruktion\\space{}4",String.raw`Re\-kon\-struk\-tion\space{}4`],["{Vorverarbeitung}",String.raw`{Vor\-ver\-ar\-bei\-tung}`],["{Nachverarbeitung}",String.raw`{Nach\-ver\-ar\-bei\-tung}`]])replace(native,before,after);
 console.log("[DEBUG] Implicit neural arrows reuse clipped owner, .9mm heads get half-width target clearance, stock compound captions use native discretionary hyphenation");
}
if(process.argv[2]==="implicit-head-resume"){
 const native=join(product,"🖋️latex/semio-viz-network-graph.sty");
 replace(native,String.raw`\fp_compare:nNnTF { \l_semio_viz_net_tip_distance_fp } > { \l_semio_viz_net_tip_outline_fp }`,String.raw`\fp_compare:nNnTF { \l_semio_viz_net_tip_distance_fp } > { \l_semio_viz_net_tip_outline_fp+.45 }`);
 replace(native,String.raw`\cs_new_protected:Npn \semio_viz_nn_link:nnn #1#2#3 {
  \semio_viz_net_draw_link:nnnnn
    { \semio_viz_nn_x:n { #1 - 1 } } { \semio_viz_nn_y:nn {#2} { \l_semio_viz_nn_b_int } }
    { \semio_viz_nn_x:n {#1} } { \semio_viz_nn_y:nn {#3} { \l_semio_viz_nn_a_int } }
    { \l_semio_viz_net_link_width_fp }
}`,String.raw`\cs_new_protected:Npn \semio_viz_nn_link:nnn #1#2#3 {
  \bool_if:NTF \l_semio_viz_net_directed_bool {
    \int_set:Nn \l_semio_viz_nn_from_layer_int {#1-1}
    \int_set:Nn \l_semio_viz_nn_to_layer_int {#1}
    \int_set:Nn \l_semio_viz_nn_from_unit_int {#2}
    \int_set:Nn \l_semio_viz_nn_to_unit_int {#3}
    \int_set_eq:NN \l_semio_viz_nn_from_count_int \l_semio_viz_nn_b_int
    \int_set_eq:NN \l_semio_viz_nn_to_count_int \l_semio_viz_nn_a_int
    \fp_zero:N \l_semio_viz_net_c_fp
    \semio_viz_nn_directed_draw:
  } {
    \semio_viz_net_draw_link:nnnnn
      { \semio_viz_nn_x:n { #1 - 1 } } { \semio_viz_nn_y:nn {#2} { \l_semio_viz_nn_b_int } }
      { \semio_viz_nn_x:n {#1} } { \semio_viz_nn_y:nn {#3} { \l_semio_viz_nn_a_int } }
      { \l_semio_viz_net_link_width_fp }
  }
}`);
 for(const[before,after]of[["{Discriminator}",String.raw`{Dis\-crim\-in\-ator}`],["{Diskriminator}",String.raw`{Dis\-kri\-mi\-na\-tor}`],["{Selbstaufmerksamkeit}",String.raw`{Selbst\-auf\-merk\-sam\-keit}`],["{Vorwärtsnetz}",String.raw`{Vor\-wärts\-netz}`],["{Ausgabeprojektion}",String.raw`{Aus\-gabe\-pro\-jek\-tion}`],["Reconstruction\\space{}4",String.raw`Recon\-struc\-tion\space{}4`],["Rekonstruktion\\space{}4",String.raw`Re\-kon\-struk\-tion\space{}4`],["{Vorverarbeitung}",String.raw`{Vor\-ver\-ar\-bei\-tung}`],["{Nachverarbeitung}",String.raw`{Nach\-ver\-ar\-bei\-tung}`]])replace(native,before,after);
 console.log("[DEBUG] Implicit neural arrows reuse clipped owner, .9mm heads get half-width target clearance, stock compound captions use native discretionary hyphenation");
}

if(process.argv[2]==="implicit-head-exact"){
 const native=join(product,"🖋️latex/semio-viz-network-graph.sty"),source=readFileSync(native,"utf8"),old=source.match(/\\cs_new_protected:Npn \\semio_viz_nn_link:nnn #1#2#3 \{[\s\S]*?\r?\n\}/)?.[0];if(!old)throw Error("Owned implicit link span absent");
 replace(native,old,String.raw`\cs_new_protected:Npn \semio_viz_nn_link:nnn #1#2#3 {
  \bool_if:NTF \l_semio_viz_net_directed_bool {
    \int_set:Nn \l_semio_viz_nn_from_layer_int {#1-1}
    \int_set:Nn \l_semio_viz_nn_to_layer_int {#1}
    \int_set:Nn \l_semio_viz_nn_from_unit_int {#2}
    \int_set:Nn \l_semio_viz_nn_to_unit_int {#3}
    \int_set_eq:NN \l_semio_viz_nn_from_count_int \l_semio_viz_nn_b_int
    \int_set_eq:NN \l_semio_viz_nn_to_count_int \l_semio_viz_nn_a_int
    \fp_zero:N \l_semio_viz_net_c_fp
    \semio_viz_nn_directed_draw:
  } {
    \semio_viz_net_draw_link:nnnnn
      { \semio_viz_nn_x:n { #1 - 1 } } { \semio_viz_nn_y:nn {#2} { \l_semio_viz_nn_b_int } }
      { \semio_viz_nn_x:n {#1} } { \semio_viz_nn_y:nn {#3} { \l_semio_viz_nn_a_int } }
      { \l_semio_viz_net_link_width_fp }
  }
}`);
 for(const[before,after]of[["{Discriminator}",String.raw`{Dis\-crim\-in\-ator}`],["{Diskriminator}",String.raw`{Dis\-kri\-mi\-na\-tor}`],["{Selbstaufmerksamkeit}",String.raw`{Selbst\-auf\-merk\-sam\-keit}`],["{Vorwärtsnetz}",String.raw`{Vor\-wärts\-netz}`],["{Ausgabeprojektion}",String.raw`{Aus\-gabe\-pro\-jek\-tion}`],["Reconstruction\\space{}4",String.raw`Recon\-struc\-tion\space{}4`],["Rekonstruktion\\space{}4",String.raw`Re\-kon\-struk\-tion\space{}4`],["{Vorverarbeitung}",String.raw`{Vor\-ver\-ar\-bei\-tung}`],["{Nachverarbeitung}",String.raw`{Nach\-ver\-ar\-bei\-tung}`]])replace(native,before,after);
 console.log("[DEBUG] Exact-span implicit arrow reuse and standard stock compound discretionary hyphenation landed after actual RED");
}
if(process.argv[2]==="owned-head-clearance"){
 const native=join(product,"🖋️latex/semio-viz-network-graph.sty"),source=readFileSync(native,"utf8"),old=source.match(/\\cs_new_protected:Npn \\semio_viz_nn_directed_draw: \{[\s\S]*?(?=\r?\n% 🧠 The connections)/)?.[0];if(!old)throw Error("Owned neural directed renderer absent");const before=String.raw`\fp_compare:nNnTF { \l_semio_viz_net_tip_distance_fp } > { \l_semio_viz_net_tip_outline_fp }`;if(old.split(before).length!==2)throw Error("Owned neural target distance guard absent");replace(native,old,old.replace(before,String.raw`\fp_compare:nNnTF { \l_semio_viz_net_tip_distance_fp } > { \l_semio_viz_net_tip_outline_fp+.45 }`));console.log("[DEBUG] Verified sole Neural directed distance guard now includes declared .45mm head half-width");
}
if(process.argv[2]==="self-loop-neutral"){
 const json=join(product,"🧪️tests/🧬️native-chart-grammar/🔣️.json"),ts=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts"),source=readFileSync(json,"utf8"),fixture=JSON.parse(source),control=fixture.nativeNeuralTopologyControls;control.schema.required.push("selfLoops");control.schema.properties.selfLoops={type:"array",minItems:4,maxItems:4,items:{type:"object",required:["id","render","directed"],properties:{id:{type:"string"},render:{enum:["units","blocks"]},directed:{type:"boolean"}}}};control.vectors.selfLoops=[{id:"unit-directed",render:"units",directed:true},{id:"unit-undirected",render:"units",directed:false},{id:"block-directed",render:"blocks",directed:true},{id:"block-undirected",render:"blocks",directed:false}];const span=source.match(/"nativeNeuralTopologyControls":\s*\{[\s\S]*?(?=\n\s*"nativeNeuralArchitectureControls":)/)?.[0];if(!span)throw Error("Owned topology fixture absent");replace(json,span,'"nativeNeuralTopologyControls": '+JSON.stringify(control,null,2).replaceAll('\n','\n  ')+',');
 replace(ts,'if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="neural-tip")','if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="neural-self"){await compileNativeNeuralSelfLoopControls(join(workDir,"neural-self"));return;}\n  if(process.env.PRINT_NATIVE_GRAMMAR_PHASE==="neural-tip")');
 const old=readFileSync(ts,"utf8");replace(ts,old,old+String.raw`
/** 🔄️ Same-unit recurrence retains visible native loops and independent D3 arc/target geometry. */
export async function compileNativeNeuralSelfLoopControls(workDir:string):Promise<void>{
 const neutral=fixture.nativeNeuralTopologyControls,c=neutral.vectors,validate=new Ajv2020({strict:false}).compile(neutral.schema),failures:string[]=[];if(!validate(c))throw Error("Neural self-loop neutral "+JSON.stringify(validate.errors));
 for(const[language,appearance]of [["en","light"],["de","dark"]]as const){const directory=join(workDir,appearance),body:{raw:string}[]=[];
  for(const entry of c.selfLoops){const base={chart:{width:c.frame[0],height:c.frame[1],language,theme:{appearance},layers:[],tables:[{name:"neural-self",columns:["layer","units","kind","label"],rows:neutral.custom.layers.map(([units,kind,label],index)=>({layer:index+1,units,kind,label}))}],presets:[{kind:"feed-forward-neural-network",options:{data:"neural-self",mode:"rnn",render:entry.render,directed:entry.directed,padding:c.padding,unitSize:c.unitSize,labels:false,curvature:.35,links:"2/1/2/1"}}]}}as unknown as VizChartSnapshot,before=JSON.stringify(base),changed=changeVizChartValue(base,{path:["presets","0","options","links"],value:"2/2/2/2"}),replayed=applyVizChartDiff(base,changed.diff),inverse=applyVizChartDiff(replayed.snapshot,inverseVizChartDiff(base,changed.diff));if(changed.messages.length||changed.diff.edits.length!==1||replayed.messages.length||inverse.messages.length||JSON.stringify(base)!==before)throw Error("Self-loop mutation/purity");deepStrictEqual(inverse.snapshot,base);const result=await inferVizChart(replayed.snapshot);if(!result.complete)throw Error("Self-loop canonical admission "+JSON.stringify(result.diagnostics));mkdirSync(directory,{recursive:true});await Bun.write(join(directory,entry.id+"-emitted.tex"),result.tikz);body.push({raw:"\\clearpage\\SemioVizProbeBegin{neural-self}{"+entry.id+"}"+result.tikz.replace(/(\\begin\{VizFigure\}[^\n]*\n)/,"$1\\draw[line width=.01mm] (0,0) rectangle (80,50);\\special{pdf:literal direct /SemioVizNeuralSelf"+entry.id+" BMC}\n").replace("\\end{VizFigure}","\\special{pdf:literal direct EMC}\\end{VizFigure}")});}
  const records=await compileVizProbeDocument({case:"neural-self",scenario:appearance,geometry:true,documentClass:"semio",documentClassOptions:"type=paper,language="+language+",theme="+appearance,packages:["semio-viz"],preamble:["\\title{Neural Recurrence}","\\author{Semio}","\\date{}"],body},{workDir:directory,scenario:undefined,keepWorkDir:true}),pdf=await getDocument({data:new Uint8Array(readFileSync(join(directory,"🧪️probe-out",appearance+".pdf"))),useSystemFonts:true}).promise;
  try{for(const entry of c.selfLoops){let found=false;for(let number=1;number<=pdf.numPages;number++){const operators=await(await pdf.getPage(number)).getOperatorList(),marker="SemioVizNeuralSelf"+entry.id;if(!operators.argsArray.some((args,index)=>[OPS.beginMarkedContent,OPS.beginMarkedContentProps].includes(operators.fnArray[index]!)&&String(args?.[0]?.name??args?.[0])===marker))continue;found=true;try{const paint=nativeControlPdfPaint(operators,c.frame,marker),routes=paint.paths.filter(path=>!path.fill&&path.points.length>2&&Math.hypot(path.bounds[2]!-path.bounds[0]!,path.bounds[3]!-path.bounds[1]!)>1);if(routes.length!==1)throw Error("Visible same-unit loop absent "+routes.length);const target=[c.frame[0]/2,c.frame[1]/2],step=(c.frame[0]-2*c.padding)/3,halfHeight=Math.max(c.unitSize,3*c.unitSize),radius=(entry.render==="units"?c.unitSize:halfHeight)*1.1,offset=(entry.render==="units"?c.unitSize:halfHeight)*1.4,center=entry.render==="units"?[target[0]!+offset,target[1]!]:[target[0]!,target[1]!+offset],mark=paint.paths.find(path=>path.fill&&path.closed&&Math.hypot((path.bounds[0]!+path.bounds[2]!)/2-target[0]!, (path.bounds[1]!+path.bounds[3]!)/2-target[1]!)<c.toleranceMm);if(!mark)throw Error("Actual self-loop target absent");const angleOffset=entry.render==="units"?-90:0,at=(angle:number)=>[center[0]!+radius*Math.cos((angle+angleOffset)*Math.PI/180),center[1]!+radius*Math.sin((angle+angleOffset)*Math.PI/180)],distance=(point:number[])=>{if(entry.render==="units")return Math.hypot(point[0]!-target[0]!,point[1]!-target[1]!)-c.unitSize;const qx=Math.abs(point[0]!-target[0]!)-.34*step,qy=Math.abs(point[1]!-target[1]!)-halfHeight;return Math.hypot(Math.max(qx,0),Math.max(qy,0))+Math.min(Math.max(qx,qy),0);};let lower=180,upper=270;for(let index=0;index<48;index++){const middle=(lower+upper)/2;if(distance(at(middle))>mark.width/2+c.directedTipClearanceMm)lower=middle;else upper=middle;}const boundary=(lower+upper)/2,start=entry.directed?540-boundary+angleOffset:0,end=entry.directed?360+boundary+angleOffset:360,actual=records.find(record=>record.scenario===entry.id&&record.key==="geometry/neural-self-loop");if(!actual)throw Error("Native self-loop geometry absent");equal(actual.values.slice(0,5).map(Number),[...center,radius,start,end],entry.id+" independent D3 loop incidence",.002);if(actual.values[5]!== (entry.directed?"arrow":"line"))throw Error("Self-loop directed policy absent");const oracle=d3Path();oracle.arc(center[0]!,center[1]!,radius,start*Math.PI/180,end*Math.PI/180);if(!oracle.toString().includes("A"))throw Error("Independent D3 loop route absent");for(const point of routes[0]!.points)equal([Math.hypot(point[0]!-center[0]!,point[1]!-center[1]!)],[radius],entry.id+" actual D3 loop radius",.025);const heads=paint.paths.filter(path=>path.closed&&path.bounds[2]!-path.bounds[0]!<2&&path.bounds[3]!-path.bounds[1]!<2);if(heads.length!==(entry.directed?1:0))throw Error("Actual directed loop head policy");for(const head of heads)for(const point of head.points)if(polygonContains(mark.points.map(value=>[value[0]!,value[1]!]as[number,number]),[point[0]!,point[1]!]))throw Error("Loop head covered by target");}catch(error){failures.push(language+"/"+entry.id+" "+String(error));}}if(!found)throw Error("Self-loop PDF marker absent "+entry.id);}}finally{await pdf.destroy();}
 }if(failures.length)throw Error("Native Neural self-loops:\n"+failures.join("\n"));console.log("[DEBUG] Neural self8 actual bilingual/theme unit/block directed/undirected same-unit loops matched D3 arcs, target membership and canonical inverse/purity");
}
`);
 const launch=join(root,".vscode/🧩️launch.seed.jsonc"),launchSource=readFileSync(launch,"utf8"),entry=launchSource.match(/    \{\s*"name": "⚖️test📓️print🎯️neural-visible-tips"[\s\S]*?\n    \},/)?.[0];if(!entry)throw Error("Owned launch neighbour absent");replace(launch,entry,entry+"\n"+entry.replace("neural-visible-tips","neural-self-loops").replace('"neural-tip"','"neural-self"').replace("900.0371299952","900.0371299953"));
 console.log("[DEBUG] Same-unit neural recurrence schema-first four-policy neutral actual native proof and registered launch added before implementation");
}
if(process.argv[2]==="caption-umlaut"){
 replace(join(product,"🖋️latex/semio-viz-network-graph.sty"),String.raw`Vor\-w\u00E4rts\-netz`,String.raw`Vor\-wärts\-netz`);
 console.log("[DEBUG] Authored German transformer caption retains actual Unicode ä");
}
if(process.argv[2]==="self-loop-native"){
 const native=join(product,"🖋️latex/semio-viz-network-graph.sty"),source=readFileSync(native,"utf8"),old=source.match(/\\cs_new_protected:Npn \\semio_viz_nn_explicit_draw:n #1 \{[\s\S]*?(?=\r?\n% 🎯 Directed neural links)/)?.[0];if(!old)throw Error("Owned neural explicit renderer absent");const split=String.raw`  \tl_set:Nn \l_semio_viz_net_stroke_tl { semio-chrome-border-normal }`;
 const helper=String.raw`\fp_new:N \l_semio_viz_nn_loop_x_fp
\fp_new:N \l_semio_viz_nn_loop_y_fp
\fp_new:N \l_semio_viz_nn_loop_r_fp
\fp_new:N \l_semio_viz_nn_loop_angle_fp
\cs_new_protected:Npn \semio_viz_nn_loop_distance: {
  \semio_viz_nn_tip_distance:
  \fp_sub:Nn \l_semio_viz_net_tip_distance_fp { .45 }
}
\cs_new_protected:Npn \semio_viz_nn_self_loop: {
  \group_begin:
  \fp_set:Nn \l_semio_viz_nn_loop_x_fp { \semio_viz_nn_x:n{\l_semio_viz_nn_to_layer_int} }
  \fp_set:Nn \l_semio_viz_nn_loop_y_fp { \semio_viz_nn_y:nn{\l_semio_viz_nn_to_unit_int}{\l_semio_viz_nn_to_count_int} }
  \fp_set_eq:NN \l_semio_viz_nn_loop_r_fp \l_semio_viz_nn_unit_fp
  \fp_set:Nn \l_semio_viz_nn_loop_angle_fp { -90 }
  \str_if_eq:VnT \l_semio_viz_nn_render_tl { blocks } {
    \fp_set:Nn \l_semio_viz_nn_loop_y_fp { (\l_semio_viz_net_yhi_fp+\l_semio_viz_net_ylo_fp)/2 }
    \fp_set:Nn \l_semio_viz_nn_loop_r_fp { max(\l_semio_viz_nn_unit_fp,\semio_viz_nn_y:nn{\l_semio_viz_nn_to_count_int}{\l_semio_viz_nn_to_count_int}-\l_semio_viz_nn_loop_y_fp) }
    \fp_zero:N \l_semio_viz_nn_loop_angle_fp
  }
  \fp_set_eq:NN \l_semio_viz_net_size_fp \l_semio_viz_nn_loop_r_fp
  \fp_set_eq:NN \l_semio_viz_net_e_fp \l_semio_viz_net_link_width_fp
  \cs_set:Npn \semio_viz_net_nx:n ##1 { \fp_use:N\l_semio_viz_nn_loop_x_fp }
  \cs_set:Npn \semio_viz_net_ny:n ##1 { \fp_use:N\l_semio_viz_nn_loop_y_fp }
  \cs_set_protected:Npn \semio_viz_net_node_size:n ##1 { \fp_set_eq:NN\l_semio_viz_net_v_fp\l_semio_viz_nn_loop_r_fp }
  \cs_set_eq:NN \semio_viz_net_tip_distance: \semio_viz_nn_loop_distance:
  \use:x { \exp_not:N\begin{scope}[rotate~around={\fp_use:N\l_semio_viz_nn_loop_angle_fp:(\fp_use:N\l_semio_viz_nn_loop_x_fp,\fp_use:N\l_semio_viz_nn_loop_y_fp)}] }
  \bool_if:NTF \l_semio_viz_net_directed_bool { \semio_viz_net_directed_loop:n {1} } { \semio_viz_net_loop_draw:n {1} }
  \end{scope}
  \semio_viz_probe_geometry:nx { neural-self-loop } {
    \fp_eval:n{round(\l_semio_viz_nn_loop_x_fp-1.4*\l_semio_viz_nn_loop_r_fp*sind(\l_semio_viz_nn_loop_angle_fp),4)},
    \fp_eval:n{round(\l_semio_viz_nn_loop_y_fp+1.4*\l_semio_viz_nn_loop_r_fp*cosd(\l_semio_viz_nn_loop_angle_fp),4)},
    \fp_eval:n{round(1.1*\l_semio_viz_nn_loop_r_fp,4)},
    \bool_if:NTF\l_semio_viz_net_directed_bool{\fp_eval:n{round(\l_semio_viz_net_loop_start_fp+\l_semio_viz_nn_loop_angle_fp,4)}}{0},
    \bool_if:NTF\l_semio_viz_net_directed_bool{\fp_eval:n{round(\l_semio_viz_net_loop_end_fp+\l_semio_viz_nn_loop_angle_fp,4)}}{360},
    \bool_if:NTF\l_semio_viz_net_directed_bool{arrow}{line}
  }
  \group_end:
}
`;
 const modified=old.replace(split,split+String.raw`
  \bool_lazy_and:nnTF { \int_compare_p:n{\l_semio_viz_nn_from_layer_int=\l_semio_viz_nn_to_layer_int} } { \int_compare_p:n{\l_semio_viz_nn_from_unit_int=\l_semio_viz_nn_to_unit_int} } { \semio_viz_nn_self_loop: } {`).replace(/\r?\n\}$/, "\n  }\n}");if(modified===old)throw Error("Owned self-loop dispatch absent");replace(native,old,helper+modified);console.log("[DEBUG] Neural identical-unit incidence reuses locally scoped existing generic directed/undirected loop primitives");
}
if(process.argv[2]==="self-loop-contract"){
 const schema=join(product,"🧬️schema/🔣️.json");
 replace(schema,"Directed fromLayer/fromUnit/toLayer/toUnit incidences use one-based ordered layer ranks and visible units; supplied empty links draw no connections.","Directed fromLayer/fromUnit/toLayer/toUnit incidences use one-based ordered layer ranks and visible units; an identical source and target unit draws a visible recurrence loop using the directed policy, beside unit glyphs and above layer blocks. Supplied empty links draw no connections.");
 replace(schema,"Gerichtete fromLayer/fromUnit/toLayer/toUnit-Inzidenzen verwenden einsbasierte geordnete Schichtränge und sichtbare Einheiten; explizit leere links zeichnen keine Verbindungen.","Gerichtete fromLayer/fromUnit/toLayer/toUnit-Inzidenzen verwenden einsbasierte geordnete Schichtränge und sichtbare Einheiten; eine identische Quell- und Zieleinheit zeichnet eine sichtbare Wiederholungsschleife gemäß directed, neben Einheitsglyphen und über Schichtblöcken. Explizit leere links zeichnen keine Verbindungen.");
 console.log("[DEBUG] Existing neural links schema declares identical-unit recurrence before native implementation");
}
if(process.argv[2]==="self-loop-rust"){
 const rust=join(product,"🧪️tests/🧬️chart-mutations/🦀️.rs"),source=readFileSync(rust,"utf8"),old=source.match(/fn canonical_neural_architecture_incidence_vectors\(\)\{[\s\S]*?(?=\r?\n\/\/\/|\r?\n#\[test\]|$)/)?.[0];if(!old)throw Error("Owned Rust canonical neural test absent");
 const loop=String.raw`  for entry in source["vectors"]["selfLoops"].as_array().unwrap(){let before=ChartSnapshot::from_value(DslValue::from(serde_json::json!({"chart":{"width":80,"height":50,"language":language,"theme":{"appearance":if language=="de"{"dark"}else{"light"}},"layers":[],"tables":[{"name":"neural-self","columns":["layer","units","kind","label"],"rows":layers}],"presets":[{"kind":"feed-forward-neural-network","options":{"data":"neural-self","mode":"rnn","render":entry["render"].clone(),"directed":entry["directed"].clone(),"padding":source["vectors"]["padding"].clone(),"unitSize":source["vectors"]["unitSize"].clone(),"labels":false,"curvature":0.35,"links":"2/1/2/1"}}]}}))).unwrap();let mutation=ChangeChartValue::from_value(DslValue::from(serde_json::json!({"path":["presets","0","options","links"],"value":"2/2/2/2"}))).unwrap();let outcome=mutation.diff(&before);assert!(outcome.messages().is_empty());let current=outcome.diff().apply(&before).unwrap();assert_eq!(outcome.diff().inverse(&before).apply(&current).unwrap(),before);let result=ChartInference::infer(&current).unwrap();assert!(result.complete,"{language}/{}: {:?}",entry["id"],result.diagnostics);if let Some(root)=std::env::var_os("PRINT_NATIVE_NEURAL_TIKZ"){std::fs::create_dir_all(&root).unwrap();std::fs::write(std::path::Path::new(&root).join(format!("self-{}-{language}.tex",entry["id"].as_str().unwrap())),result.tikz).unwrap();}count+=1;}
`;
 const needle=' }eprintln!("[DEBUG] Neural Rust{count}';if(old.split(needle).length!==2)throw Error("Owned Rust count capsule absent");replace(rust,old,old.replace(needle,loop+needle).replace(' }eprintln!',' }assert_eq!(count,68);eprintln!').replace('stock/custom/empty/implicit/invalid','stock/custom/empty/implicit/self-loop/invalid'));
 const ts=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts"),tsSource=readFileSync(ts,"utf8"),span=tsSource.match(/export async function compileNativeNeuralSelfLoopControls\(workDir:string\):Promise<void>\{[\s\S]*?(?=\r?\n\/\*\*|$)/)?.[0];if(!span)throw Error("Owned neural self-loop native test absent");const before='mkdirSync(directory,{recursive:true});await Bun.write(join(directory,entry.id+"-emitted.tex"),result.tikz);body.push({raw:',after='mkdirSync(directory,{recursive:true});const tikz=process.env.PRINT_NATIVE_NEURAL_TIKZ?readFileSync(join(process.env.PRINT_NATIVE_NEURAL_TIKZ,"self-"+entry.id+"-"+language+".tex"),"utf8"):result.tikz;await Bun.write(join(directory,entry.id+"-emitted.tex"),tikz);body.push({raw:';if(span.split(before).length!==2)throw Error("Owned neural self emission absent");replace(ts,span,span.replace(before,after).replace('+result.tikz.replace(', '+tikz.replace('));console.log("[DEBUG] Rust neutral canonical68 now emits50 actual sources including8 same-unit policies; native self-loop corpus requires exact Rust files when selected");
}
if(process.argv[2]==="caption-umlaut-bytes"){
 const native=join(product,"🖋️latex/semio-viz-network-graph.sty");replace(native,"Vor\\-w\\u00E4rts\\-netz","Vor\\-wärts\\-netz");const source=readFileSync(native,"utf8");if(!source.includes("Vor\\-wärts\\-netz")||source.includes("Vor\\-w\\u00E4rts\\-netz"))throw Error("Unicode authored caption byte verification failed");console.log("[DEBUG] Verified actual authored Unicode ä bytes with ordinary escaped literal; Bun raw Unicode escape reproduction retained");
}
if(process.argv[2]==="self-loop-docstring"){
 replace(join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts"),"/** \\u{1f504}\\uFE0F Same-unit recurrence","/** 🔄️ Same-unit recurrence");console.log("[DEBUG] Neural self-loop docstring retains actual Unicode emoji bytes");
}
if(process.argv[2]==="self-loop-rotation-key"){
 replace(join(product,"🖋️latex/semio-viz-network-graph.sty"),String.raw`rotate~around={\fp_use:N\l_semio_viz_nn_loop_angle_fp:(`,String.raw`rotate~around={\fp_eval:n{\l_semio_viz_nn_loop_angle_fp}:(`);console.log("[DEBUG] Loop rotation separates expl3 variable from TikZ colon syntax");
}
if(process.argv[2]==="rust-source-receipt"){
 const {createHash}=await import("node:crypto"),{readdirSync,writeFileSync}=await import("node:fs"),ticket=join(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY"),fixture=JSON.parse(readFileSync(join(product,"🧪️tests/🧬️native-chart-grammar/🔣️.json"),"utf8")),neural=fixture.nativeNeuralTopologyControls.vectors,construction=fixture.scientificConstructionWindows.vectors;
 const expectedNeural=[...neural.architectures.map((entry:{kind:string})=>entry.kind),"custom","empty",...neural.implicitConnections.map((name:string)=>"implicit-"+name),...neural.selfLoops.map((entry:{id:string})=>"self-"+entry.id)].flatMap(name=>[name+"-en.tex",name+"-de.tex"]).sort(),expectedConstruction=construction.cases.flatMap((_:unknown,index:number)=>construction.variants.flatMap((mode:number)=>["Cv"+index+"M"+mode+"-light.tex","Cv"+index+"M"+mode+"-dark.tex"])).sort();
 let report="# Registered Rust Source Receipt\n\nWhole registered print-rs:test85368 actual terminal0: 27 tests passed, 0 failed; exact runner log generated/native-completion-rust-self-producer/runner.log. Neural canonical68 assertions include mutation/inverse/stock/custom/empty/implicit/self/invalid. Construction producer is data-derived. Native geometric results remain separate.\n";
 for(const[name,expected]of[["native-neural-rust-emission",expectedNeural],["native-construction-rust-emission",expectedConstruction]]as const){const directory=join(ticket,"🗑️generated",name),actual=readdirSync(directory).filter(name=>name.endsWith(".tex")).sort();if(JSON.stringify(actual)!==JSON.stringify(expected))throw Error("Exact Rust corpus inventory mismatch "+name);const rows=actual.map(name=>name+" "+createHash("sha256").update(readFileSync(join(directory,name))).digest("hex")),digest=createHash("sha256").update(rows.join("\n")).digest("hex");report+="\n"+name+": "+actual.length+" exact expected sources; sorted filename-plus-SHA256 inventory digest "+digest+".\n\n```\n"+rows.join("\n")+"\n```\n";console.log("[DEBUG] "+name+" "+actual.length+" exact files "+digest);}
 writeFileSync(join(ticket,"📓️rust-neural-construction-source-receipt-2026-10-06.md"),report);
}
if(process.argv[2]==="self-loop-colon-category"){
 replace(join(product,"🖋️latex/semio-viz-network-graph.sty"),String.raw`rotate~around={\fp_eval:n{\l_semio_viz_nn_loop_angle_fp}:(`,String.raw`rotate~around={\fp_eval:n{\l_semio_viz_nn_loop_angle_fp}\c_colon_str(`);console.log("[DEBUG] Existing TikZ rotation parser receives category12 colon via expl3 c_colon_str");
}
if(process.argv[2]==="self-loop-probe-and-paint"){
 const native=join(product,"🖋️latex/semio-viz-network-graph.sty"),source=readFileSync(native,"utf8"),span=source.match(/\\cs_new_protected:Npn \\semio_viz_nn_self_loop: \{[\s\S]*?(?=\r?\n\\cs_new_protected:Npn \\semio_viz_nn_explicit_draw:n)/)?.[0];if(!span)throw Error("Owned neural loop adapter absent");replace(native,span,span.replace(String.raw`  \end{scope}
  \semio_viz_probe_geometry:nx`,String.raw`  \semio_viz_probe_geometry:nx`).replace(String.raw`  \group_end:`,String.raw`  \end{scope}
  \group_end:`));
 const ts=join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts"),text=readFileSync(ts,"utf8"),helper=text.match(/export async function compileNativeNeuralSelfLoopControls\(workDir:string\):Promise<void>\{[\s\S]*?(?=\r?\n\/\*\*|$)/)?.[0];if(!helper)throw Error("Owned self-loop observer absent");const before='routes=paint.paths.filter(path=>!path.fill&&path.points.length>2',after='routes=paint.paths.filter(path=>[OPS.stroke,OPS.closeStroke].includes(path.paint)&&path.points.length>2';if(helper.split(before).length!==2)throw Error("Owned self stroke classification absent");replace(ts,helper,helper.replace(before,after));console.log("[DEBUG] Self-loop probe captures angles before scope restoration; observer classifies actual PDF stroke operator instead of RGB fill array truthiness");
}
if(process.argv[2]==="self-shaft-red"){
 const {getDocument,OPS}=await import("pdfjs-dist/legacy/build/pdf.mjs"),{nativeControlPdfPaint}=await import(join(product,"🧪️tests/🧬️native-chart-grammar/🟦️.ts")),{path}=await import("d3-path"),ticket=join(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY"),pdf=await getDocument({data:new Uint8Array(readFileSync(join(ticket,"🗑️generated/native-completion-self-colon-ts/neural-self/light/🧪️probe-out/light.pdf"))),useSystemFonts:true}).promise;
 try{const paint=nativeControlPdfPaint(await(await pdf.getPage(2)).getOperatorList(),[80,50],"SemioVizNeuralSelfunit-directed"),route=paint.paths.find(value=>[OPS.stroke,OPS.closeStroke].includes(value.paint)&&value.points.length>2);if(!route)throw Error("Actual directed loop stroke absent");const oracle=path();oracle.arc(41.54,25,1.21,0,2*Math.PI);if(!oracle.toString().includes("A"))throw Error("D3 circle oracle absent");const radii=route.points.map(point=>Math.hypot(point[0]!-41.54,point[1]!-25));console.log("[DEBUG] Actual declared R1.21 circular directed loop shaft radius min="+Math.min(...radii)+" max="+Math.max(...radii)+" samples="+radii.length);if(radii.some(radius=>Math.abs(radius-1.21)>.025))throw Error("Actual directed-loop circular shaft differs from D3 radius by more than.025mm");}finally{await pdf.destroy();}
}
