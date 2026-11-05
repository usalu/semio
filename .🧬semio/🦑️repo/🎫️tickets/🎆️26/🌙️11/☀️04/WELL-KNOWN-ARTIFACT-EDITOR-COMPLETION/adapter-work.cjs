
const fs=require("fs"),cp=require("child_process");
const base="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts";
const files=cp.execFileSync("rg",["-l","impl.*PagedOneItemEdit","--glob","*.rs",base],{encoding:"utf8"}).trim().split("\n");
const staged=[];
function one(s,a,b){if(s.split(a).length!==2)throw Error("expected one "+a.slice(0,90));return s.replace(a,b);}
for(const file of files){
 let before=fs.readFileSync(file,"utf8"),s=before;
 const type=file.includes("📜️docx")?"DocxXmlEdit":file.includes("🎒️zip")?"ZipTextEdit":file.includes("🔺️mesh")?"MeshVertexEdit":"BrepVertexEdit";
 s=one(s,"struct "+type+" {","struct "+type+" {\n    last_work: u64,");
 const implAt=s.indexOf("impl PagedOneItemEdit<"),fnAt=s.indexOf("    fn advance(&mut self,",implAt),bodyAt=s.indexOf("{\n",fnAt)+2;
 if(implAt<0||fnAt<0||bodyAt<2)throw Error("advance boundary");
 s=s.slice(0,bodyAt)+`        self.last_work=0;
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PagedOneItemEditStep::Progress(Default::default()))}
`+s.slice(bodyAt);
 s=one(s,"    fn begin_close(&mut self) {","    fn last_turn_work(&self)->u64{self.last_work}\n\n    fn begin_close(&mut self) {");
 if(type==="DocxXmlEdit"){
  s=one(s,"            measure_snapshot(post).map_err(","            self.last_work+=1;\n            measure_snapshot(post).map_err(");
  s=one(s,"            let reserve = measure_mutation(mutation)","            self.last_work+=1;\n            let reserve = measure_mutation(mutation)");
  s=one(s,"        let prepared = apply_addressed_xml_mutation_in_place(post, mutation)?;","        self.last_work+=1;\n        let prepared = apply_addressed_xml_mutation_in_place(post, mutation)?;");
  s=one(s,"\n        measure_snapshot(post).map_err(","\n        self.last_work+=1;\n        measure_snapshot(post).map_err(");
  s=one(s,"        let retained = measure_mutation(&prepared.inverse)","        self.last_work+=1;\n        let retained = measure_mutation(&prepared.inverse)");
 }else{
  const scanStart=s.indexOf("    fn scan("),scanEnd=s.indexOf("\n    fn ",scanStart+10);
  let scan=s.slice(scanStart,scanEnd);
  if(type==="ZipTextEdit"){
   s=one(s,"        if structural_items(post).is_none_or","        self.last_work+=1;\n        if structural_items(post).is_none_or");
   scan=one(scan,"            if self.matches != 1 || self.collision {","            self.last_work+=1;\n            if self.matches != 1 || self.collision {");
   scan=one(scan,"        if entry.name == name {","        self.last_work+=1;\n        if entry.name == name {");
  }else if(type==="MeshVertexEdit"){
   scan=one(scan,"            if self.mesh_matches != 1 || self.primitive_matches != 1 {","            self.last_work+=1;\n            if self.mesh_matches != 1 || self.primitive_matches != 1 {");
   scan=one(scan,"            self.mesh_index += 1;\n            return","            self.last_work+=1;\n            self.mesh_index += 1;\n            return");
   scan=one(scan,"            self.mesh_matches += 1;","            self.last_work+=1;\n            self.mesh_matches += 1;");
   scan=one(scan,"        if primitive.id == target.primitive_id {","        self.last_work+=1;\n        if primitive.id == target.primitive_id {");
  }else{
   scan=one(scan,"            if self.matches != 1 {","            self.last_work+=1;\n            if self.matches != 1 {");
   scan=one(scan,"        if vertex.id == target.vertex_id {","        self.last_work+=1;\n        if vertex.id == target.vertex_id {");
  }
  const currentStart=s.indexOf("    fn scan("),currentEnd=s.indexOf("\n    fn ",currentStart+10);
  s=s.slice(0,currentStart)+scan+s.slice(currentEnd);
  if(type==="ZipTextEdit"){
   s=one(s,"        let index = self.located.ok_or_else","        self.last_work+=1;\n        let index = self.located.ok_or_else");
   s=one(s,"        let previous = std::mem::replace(&mut post.comment,","        self.last_work+=1;\n        let previous = std::mem::replace(&mut post.comment,");
   s=s.replaceAll("finish(step, post)","finish(step, post, &mut self.last_work)");
   s=one(s,"fn finish(step: PagedOneItemEditStep<ZipMutation>, post: &ZipSnapshot)","fn finish(step: PagedOneItemEditStep<ZipMutation>, post: &ZipSnapshot, last_work:&mut u64)");
   s=one(s,"    if matches!(step, PagedOneItemEditStep::Complete(..)) {","    if matches!(step, PagedOneItemEditStep::Complete(..)) {\n        *last_work+=1;");
  }else{
   const needle=type==="MeshVertexEdit"?"        let (mesh_index, primitive_index) = self.located.ok_or_else":"        let index = self.located.ok_or_else";
   s=one(s,needle,"        self.last_work+=1;\n"+needle);
  }
 }
 staged.push([file,before,s]);
}
for(const[file,before]of staged)if(fs.readFileSync(file,"utf8")!==before)throw Error("concurrent edit "+file);
for(const[file,,s]of staged){fs.writeFileSync(file,s);console.log(file);}
