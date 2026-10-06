export async function compileNativeScientificCanvasControls(workDir:string,neutral:{schema:object;vectors:{canvas:number[];familyFrame:number[];domain:number[];range:number[];tickDivisions:number[];cases:{family:string;kind:string;options:Record<string,unknown>}[]}}=fixture.scientificCanvasControlEffects):Promise<void>{
  const control=neutral.vectors,before=JSON.stringify(control),failures:string[]=[];
  const validate=new Ajv2020({strict:false}).compile(neutral.schema);if(!validate(control))throw Error("Scientific canvas neutral vectors rejected: "+JSON.stringify(validate.errors));
  const cases=control.cases.flatMap((entry,index)=>[0,1,2].map(mode=>({entry,mode,id:"Cv"+index+"M"+mode}))),[width,height]=control.familyFrame,{domain,range}=control,preamble=scientificCanvasProbePreamble();
  let matched=0;
  const themes=(["light","dark"] as const).filter(theme=>!process.env.PRINT_NATIVE_CONTROL_THEME||theme===process.env.PRINT_NATIVE_CONTROL_THEME);if(!themes.length)throw Error("Unknown scientific canvas theme");
  for(const appearance of themes){
    const language=appearance==="light"?"en":"de",body:{raw:string}[]=[];
    for(const entry of cases){
      const initial={chart:{width:control.canvas[0],height:control.canvas[1],language,theme:{appearance},layers:[],presets:[{kind:entry.entry.kind,options:{...entry.entry.options,width,height,axes:false,grid:false,title:"",xlabel:"",ylabel:"",domain:"",range:""}}]}} as unknown as VizChartSnapshot;
      let snapshot=initial;const source=JSON.stringify(initial),changes:Record<string,string|number|boolean>=entry.mode?{axes:true,grid:true,ticks:control.tickDivisions[entry.mode-1]!,title:"Cvtitle",xlabel:"Cvx",ylabel:"Cvy",...(entry.mode===2?{domain:domain.join(","),range:range.join(",")}: {})}:{};
      for(const[key,value]of Object.entries(changes)){
        const mutation=changeVizChartValue(snapshot,{path:["presets","0","options",key],value});if(mutation.messages.length||mutation.diff.edits.length>1)throw Error(entry.id+" canvas mutation rejected: "+JSON.stringify(mutation.messages));
        const replay=applyVizChartDiff(snapshot,mutation.diff),inverse=applyVizChartDiff(replay.snapshot,inverseVizChartDiff(snapshot,mutation.diff));if(replay.messages.length||inverse.messages.length)throw Error(entry.id+" canvas replay rejected");deepStrictEqual(inverse.snapshot,snapshot);snapshot=replay.snapshot;
      }
      if(JSON.stringify(initial)!==source)throw Error(entry.id+" canvas mutation changed input");const result=await inferVizChart(snapshot);if(!result.complete)throw Error(entry.id+" canvas inference rejected: "+JSON.stringify(result.diagnostics));
      const reference="\\draw[line width=.01mm] (0,0) rectangle ("+control.canvas.join(",")+");\n\\special{pdf:literal direct /SemioVizControl"+entry.id+" BMC}\n",emitted=result.tikz.replace(/(\\begin\{VizFigure\}[^\n]*\n)/,"$1"+reference).replace("\\end{VizFigure}","\\special{pdf:literal direct EMC}\n\\end{VizFigure}");
      body.push({raw:"\\clearpage\\noindent\\texttt{Case~"+entry.id+"}\\par\\ExplSyntaxOn\\tl_set:Nn\\l_semio_viz_test_canvas_case_tl{"+entry.id+"}\\ExplSyntaxOff\n\\SemioVizProbeBegin{scientific-canvas}{"+entry.id+"}\\SemioVizProbeValues{canvas/observer}{1}\n"+emitted});
    }
    const directory=join(workDir,appearance);if(!process.env.PRINT_NATIVE_CANVAS_REPLAY)await compileVizProbeDocument({case:"scientific-canvas",scenario:appearance,documentClass:"semio",documentClassOptions:"type=paper,language="+language+",theme="+appearance,packages:["semio-viz"],geometry:true,preamble,body},{workDir:directory,scenario:undefined,keepWorkDir:true});
    const pdfDirectory=process.env.PRINT_NATIVE_CANVAS_REPLAY?join(process.env.PRINT_NATIVE_CANVAS_REPLAY,appearance):directory,pdf=await getDocument({data:new Uint8Array(readFileSync(join(pdfDirectory,"🧪️probe-out",appearance+".pdf"))),useSystemFonts:true}).promise,paint=new Map<string,{body:ReturnType<typeof nativeControlPdfPaint>;chrome?:ReturnType<typeof nativeControlPdfPaint>}>();
    try{
      for(let page=1;page<=pdf.numPages;page++){
        const operators=await(await pdf.getPage(page)).getOperatorList(),has=(marker:string)=>operators.argsArray.some((args,index)=>[OPS.beginMarkedContent,OPS.beginMarkedContentProps].includes(operators.fnArray[index]!)&&String(args?.[0]?.name??args?.[0])===marker);
        for(const entry of cases)if(has("SemioVizControl"+entry.id)){
          if(paint.has(entry.id))throw Error("Scientific canvas duplicate case page "+entry.id);
          paint.set(entry.id,{body:nativeControlPdfPaint(operators,control.canvas,(has("SemioVizBody"+entry.id)?"SemioVizBody":"SemioVizControl")+entry.id,["SemioVizChrome"+entry.id]),...(has("SemioVizChrome"+entry.id)?{chrome:nativeControlPdfPaint(operators,control.canvas,"SemioVizChrome"+entry.id)}:{})});
        }
      }
      for(const entry of cases)try{
        const actual=paint.get(entry.id),baseline=paint.get(entry.id.replace(/M\d$/,"M0"));if(!actual||!baseline)throw Error("Scientific canvas actual case body missing");if(!baseline.body.paths.length&&!baseline.body.glyphs.length)throw Error("Scientific canvas baseline has no actual body paint");
        const x=scaleLinear(entry.mode===2?domain:[0,width!],[0,width!]),y=scaleLinear(entry.mode===2?range:[0,height!],[0,height!]),scale=entry.mode===2?Math.sqrt(width!/(domain[1]!-domain[0]!)*height!/(range[1]!-range[0]!)):1;
        if(actual.body.paths.length!==baseline.body.paths.length)throw Error("Scientific body paths "+actual.body.paths.length+" expected "+baseline.body.paths.length);
        for(const[index,original]of baseline.body.paths.entries()){
          const value=actual.body.paths[index]!;if(value.paint!==original.paint||value.closed!==original.closed||value.points.length!==original.points.length)throw Error("Scientific body topology changed at "+index);
          for(const[vertex,point]of original.points.entries())equal(value.points[vertex]!,[x(point[0]!),y(point[1]!)],"D3 scientific affine body "+index+"/"+vertex,.035);
          equal(value.bounds,[x(original.bounds[0]!),y(original.bounds[1]!),x(original.bounds[2]!),y(original.bounds[3]!)],"D3 scientific exact affine bounds "+index,.035);
          equal([value.width,value.opacity,value.fillOpacity],[original.width*scale,original.opacity,original.fillOpacity],"Scientific affine paint state "+index,.005);equal(value.stroke,original.stroke,"Scientific stroke colour",.002);equal(value.fill,original.fill,"Scientific fill colour",.002);matched++;
        }
        if(actual.body.glyphs.length!==baseline.body.glyphs.length)throw Error("Scientific body glyph count changed");for(const[index,original]of baseline.body.glyphs.entries()){const value=actual.body.glyphs[index]!;if(value.text!==original.text||!value.origin||!original.origin||!value.advance||!original.advance)throw Error("Scientific body glyph content/geometry missing");equal(value.origin,[x(original.origin[0]!),y(original.origin[1]!)],"D3 scientific affine glyph origin",.035);equal(value.advance,[x(original.advance[0]!)-x(0),y(original.advance[1]!)-y(0)],"D3 scientific affine glyph advance",.035);}
        if(!actual.chrome)throw Error("Shared scientific chrome ownership missing");const chrome=actual.chrome;
        if(!entry.mode){if(chrome.paths.length||chrome.glyphs.length)throw Error("Disabled scientific chrome paints");continue;}
        const ticks=control.tickDivisions[entry.mode-1]!,grid=chrome.paths.filter(path=>Math.abs(path.opacity-.3)<.001&&!path.closed),axes=chrome.paths.filter(path=>path.closed&&Math.abs(path.opacity-1)<.001);
        if(grid.length!==2*(ticks+1)||axes.length!==1)throw Error("Scientific chrome grid/axis paths "+grid.length+"/"+axes.length+" expected "+2*(ticks+1)+"/1");
        for(let tick=0;tick<=ticks;tick++){const px=scaleLinear([0,ticks],[0,width!])(tick),py=scaleLinear([0,ticks],[0,height!])(tick);for(const expected of [[[px,0],[px,height!]],[[0,py],[width!,py]]])if(!grid.some(path=>path.points.length===2&&path.points.every((point,index)=>point.every((value,axis)=>Math.abs(value-expected[index]![axis]!)<.025))))throw Error("Independent D3 scientific grid missing "+JSON.stringify(expected));}
        equal(axes[0]!.bounds,[0,0,width!,height!],"Scientific canvas axis bounds",.025);const text=chrome.glyphs.map(glyph=>glyph.text).join("").replace(/\s/g,"");for(const label of ["Cvtitle","Cvx","Cvy"])if(!text.includes(label))throw Error("Scientific chrome label missing "+label);
        const dx=scaleLinear([0,ticks],entry.mode===2?domain:[0,width!]),dy=scaleLinear([0,ticks],entry.mode===2?range:[0,height!]),format=(value:number)=>String(Number(value.toFixed(2))).replace(".",language==="de"?",":"."),labels=Array.from({length:ticks+1},(_,index)=>format(dx(index))+format(dy(index))).join("");if(!text.replace(/[−–]/g,"-").startsWith(labels))throw Error("Independent localized D3 tick labels "+text+" expected prefix "+labels);
      }catch(error){failures.push(appearance+"/"+entry.entry.family+"/"+entry.id+": "+String(error));}
    }finally{await pdf.destroy();}
  }
  if(JSON.stringify(control)!==before)throw Error("Scientific canvas neutral vectors changed");if(failures.length)throw Error("Scientific physical canvas effects:\n"+failures.join("\n"));console.log("[DEBUG] Scientific canvas "+cases.length*themes.length+" canonical cases and "+matched+" actual body paths matched independent D3 affine paint and isolated chrome");
}

