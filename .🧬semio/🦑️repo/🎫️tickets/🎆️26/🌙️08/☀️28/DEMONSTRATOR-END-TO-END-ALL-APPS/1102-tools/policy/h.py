H=[]
OS="🧰️framework/🛍️products/💻️os/🔨️modules"
def add(rel,edits,note):H.append((rel,edits,note))
E='return Err(ValueError::invariant("native I/O state invariant violated").into())'
N=f"{OS}/🛎️services/🚪️native-io"
add(f"{N}/📋️rows/advance.rs",[
 ("else{unreachable!()}","else{"+E+"}",6),
 ("Some((self.modified_path.take().unwrap(),time))",'Some((self.modified_path.take().ok_or_else(||ValueError::invariant("native I/O modified path absent"))?,time))')],"native-io rows: let-else arms return ValueError::invariant -> OriginalTurnFailure (unreachable!/unwrap removed); 7 sites")
add(f"{N}/📖️buffer/read.rs",[
 ('_=>unreachable!("only original file observation states")','_=>'+E),
 ("else{unreachable!()}","else{"+E+"}",3),
 ("_=>unreachable!()};\n   if writer.chunk_complete()","_=>"+E+"};\n   if writer.chunk_complete()"),
 ("_=>unreachable!()};}\n    semio_framework_job","_=>"+E+"};}\n    semio_framework_job"),
 ("Some(*cursor>=*length)),_=>unreachable!()};","Some(*cursor>=*length)),_=>"+E+"};"),
 ("file.read(bytes),_=>unreachable!()};","file.read(bytes),_=>"+E+"};"),
 ("_=>unreachable!()\n  };\n  semio_framework_job::receive_original_retained_work::<ValueError>","_=>"+E+"\n  };\n  semio_framework_job::receive_original_retained_work::<ValueError>")],"native-io read: all state-machine unreachable!() arms return ValueError::invariant -> OriginalTurnFailure; 9 sites")

PL="✏️s/🔌️plugins"
SC=f"{PL}/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene"
DE=lambda m:'_=>return Err(DocumentSceneError::literal("'+m+'"))'
add(f"{SC}/🎨️paint/📋️prepare/🦀️.rs",[
 ('painted scene")),_=>unreachable!()','painted scene")),_=>return Err(DocumentSceneError::literal("Drawing scene content kind is not paintable"))'),
 ('"complete"=>{},_=>unreachable!()','"complete"=>{},_=>return Err(DocumentSceneError::literal("Drawing scene prepare phase is invalid"))')],"DocumentSceneError::literal; 2 sites")
add(f"{SC}/🎨️paint/🦀️.rs",[
 ('"complete"=>return Ok(false),_=>unreachable!(),}Ok(true)}','"complete"=>return Ok(false),_=>return Err(DocumentSceneError::literal("Drawing scene paint phase is invalid")),}Ok(true)}'),
 ('"complete"=>return Ok(false),_=>unreachable!(),','"complete"=>return Ok(false),_=>return Err(DocumentSceneError::literal("Drawing scene paint phase is invalid")),')],"DocumentSceneError::literal; 2 sites")
add(f"{SC}/📍️placement/🦀️.rs",[('"complete"=>Ok(false),_=>unreachable!()}}','"complete"=>Ok(false),_=>Err(DocumentSceneError::literal("Drawing scene placement phase is invalid"))}}')],"DocumentSceneError::literal; 1 site")
IV=lambda m:'.ok_or_else(||ValueError::invariant("'+m+'"))?'
add("🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/📥️source/🦀️.rs",[
 ("let original=self.original.as_ref().unwrap();let source=match self.field{0=>original.kind.as_bytes(),1=>original.input.as_ref().unwrap(),2=>original.checkpoint.as_ref().unwrap(),_=>unreachable!()};",
  "let original=self.original.as_ref()"+IV("actor source original is absent")+";let source=match self.field{0=>original.kind.as_bytes(),1=>original.input.as_ref()"+IV("actor source input is absent")+",2=>original.checkpoint.as_ref()"+IV("actor source checkpoint is absent")+',_=>return Err(ValueError::invariant("actor source field is invalid"))};')],"ValueError::invariant; 4 sites")
add("🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs",[
 ("let Some(result)=self.result.take()else{unreachable!()};",'let Some(result)=self.result.take()else{return Err(ValueError::invariant("fold cleanup result is absent"))};'),
 ("let Some(Err(error))=self.result.as_mut()else{unreachable!()};",'let Some(Err(error))=self.result.as_mut()else{return Err(ValueError::invariant("fold cleanup error is absent"))};')],"ValueError::invariant; 2 sites")
add(f"{OS}/🔌️plugin/👷️job/🛬️receiving/📥️pack/🦀️.rs",[("3|4|5=>unreachable!(),","3|4|5=>return Err(invalid().into()),")],"ValueError via invalid(); 1 site")
add(f"{OS}/🔌️plugin/👷️job/🛬️receiving/🔀️transaction/🦀️.rs",[("Ok(((),receipt(d)))})?;},_=>unreachable!(),",'Ok(((),receipt(d)))})?;},_=>return Err(invalid("transaction receiving phase is invalid").into()),')],"ValueError via invalid(reason); 1 site")

OSC=f"{OS}/🏪️store/📦️codec/🧬️owned-schema"
LF=f"{OSC}/🧱️fields/📥️lifted/🧬️owned-schema"
DG=lambda m:'.ok_or_else(||diagnostic.with_native(semio_framework_value::ValueError::invariant("'+m+'")))?'
add(f"{OSC}/🧮️decode-owner-bundle-and-bounded-array-authority/🦀️.rs",[('return match self.retirement_factory.as_ref().expect("history entry retains its factory")','return match self.retirement_factory.as_ref()'+DG("history entry factory is absent"))],"SchemaDecodeDiagnostic::with_native(ValueError::invariant) via the in-scope `diagnostic`; 1 site")
add(f"{LF}/🎟️field-decoder-registry-and-empty-array-authority/🦀️.rs",[('match self.retirement_factory.as_ref().expect("history value retains its factory")','match self.retirement_factory.as_ref()'+DG("history value factory is absent"))],"same; 1 site")
add(f"{LF}/🧰️owned-spr-mutation-target-and-array-authority/🦀️.rs",[('return match self.mutation_factory.as_ref().expect("mutation target retains original factory")','return match self.mutation_factory.as_ref()'+DG("mutation target factory is absent"))],"same; 1 site")
add(f"{LF}/🧵️owned-spr-edit-authority-and-history-decoder/🦀️.rs",[
 ('match self.retirement_factory.as_ref().expect("edit retains value factory")','match self.retirement_factory.as_ref()'+DG("edit value factory is absent")),
 ('self.strings[1].take().expect("original actor text retained")','self.strings[1].take().ok_or_else(||OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-owner",offset:token.start,line:0,column:0,path,refusal_kind:semio_framework_value::ValueRefusalKind::InvariantViolated,retained_progress:Default::default()})?')],"same; explicit diagnostic for the actor text; 2 sites")

W="semio_framework_value::RetainedWorkOutcome{result,work:1}"
add(f"{OS}/🌊️flow/🖥️host/📥️evaluation/🦀️.rs",[
 ('3=>{semio_framework_job::close_original_retained_step(cx,d,|g|{let result=(||self.decoder.as_mut().unwrap().close_step(g))();'+W+'})?;if self.decoder.as_ref().unwrap().terminal_is_empty(){',
  '3=>{let decoder=self.decoder.as_mut().ok_or_else(||invalid("flow evaluation decoder is absent"))?;semio_framework_job::close_original_retained_step(cx,d,|g|{let result=decoder.close_step(g);'+W+'})?;if decoder.terminal_is_empty(){'),
 ('8=>{semio_framework_job::close_original_retained_step(cx,d,|g|{let result=(||self.source_close.as_mut().unwrap().step(g))();'+W+'})?;if self.source_close.as_ref().unwrap().terminal_is_empty(){',
  '8=>{let close=self.source_close.as_mut().ok_or_else(||invalid("flow evaluation source close is absent"))?;semio_framework_job::close_original_retained_step(cx,d,|g|{let result=close.step(g);'+W+'})?;if close.terminal_is_empty(){')],"flow evaluation: receiver hoisted out of the close closure (ValueError via invalid); 4 sites")
add(f"{OS}/🌊️flow/🖥️host/📥️preview/🦀️.rs",[
 ('6=>{semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=(||self.source_close.as_mut().unwrap().step(grant))();'+W+'})?;if self.source_close.as_ref().unwrap().terminal_is_empty(){',
  '6=>{let close=self.source_close.as_mut().ok_or_else(||invalid("flow preview source close is absent"))?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=close.step(grant);'+W+'})?;if close.terminal_is_empty(){'),
 ('10=>{semio_framework_job::close_original_retained_step(cx,demand,|g|{let result=(||self.fields.as_mut().unwrap().displaced.as_mut().unwrap().step(g))();'+W+'})?;if self.fields.as_ref().unwrap().displaced.as_ref().unwrap().terminal_is_empty(){self.fields.as_mut().unwrap().displaced=None;',
  '10=>{let displaced=self.fields.as_mut().ok_or_else(||invalid("flow preview fields are absent"))?.displaced.as_mut().ok_or_else(||invalid("flow preview displaced close is absent"))?;semio_framework_job::close_original_retained_step(cx,demand,|g|{let result=displaced.step(g);'+W+'})?;if displaced.terminal_is_empty(){self.fields.as_mut().ok_or_else(||invalid("flow preview fields are absent"))?.displaced=None;')],"flow preview: receivers hoisted out of the close closures; 4 sites")
