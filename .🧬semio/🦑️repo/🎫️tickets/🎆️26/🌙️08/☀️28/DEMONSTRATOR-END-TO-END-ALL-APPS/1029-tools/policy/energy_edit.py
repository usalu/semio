import sys
M="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
def edit(path,pairs):
    t=open(M+path,encoding="utf-8").read()
    for old,new,count in pairs:
        n=t.count(old)
        assert n==count,(path,old[:80],n,count)
        t=t.replace(old,new)
    open(M+path,"w",encoding="utf-8").write(t)
    print("ok",path[-40:])
R="Result<Self::Row,::semio_framework_value::ValueError>"
INV="::semio_s_artifact_energy_model_native_mutations::scalar::invalid"
edit("🧳️native-lifecycle/🌱️construction/🦀️.rs",[
 ("context:Self::Context)->Self::Row;","context:Self::Context)->Result<Self::Row,ValueError>;",1),
 ("let value=S::finish(payload,fields,self.context);","let value=S::finish(payload,fields,self.context)?;",1)])
edit("🧳️native-lifecycle/🌱️construction/🪡️fields/🦀️.rs",[
 ("fn finish(p:RetainedCloneRef<'_,S::Payload>,_:(),_:())->S::Value{*S::field(p).get()}","fn finish(p:RetainedCloneRef<'_,S::Payload>,_:(),_:())->Result<S::Value,ValueError>{Ok(*S::field(p).get())}",1)])
edit("🧳️native-lifecycle/🌱️construction/🧪️tests/🦀️.rs",[
 ("_:())->StateShadingSurface{StateShadingSurface{id:p.get().id,name,vertices_m,transmittance_schedule_id}}","_:())->Result<StateShadingSurface,ValueError>{Ok(StateShadingSurface{id:p.get().id,name,vertices_m,transmittance_schedule_id})}",1),
 ("rank:usize)->Recreated{Recreated{id:p.get().id.0,name,vertices_m,rank}}","rank:usize)->Result<Recreated,ValueError>{Ok(Recreated{id:p.get().id.0,name,vertices_m,rank})}",1)])
edit("✨️derive/🌱️birth/🦀️.rs",[
 ("_:(),_:())->Self::Row{Vec::new()}}});","_:(),_:())->"+R+"{Ok(Vec::new())}}});",1),
 ("_:(),_:())->Self::Row{crate::model::SetpointManagerKind::from_parts(p.get().#source,[#(p.get().#limits),*]).expect(\"validated original setpoint control law\")}}});","_:(),_:())->"+R+"{crate::model::SetpointManagerKind::from_parts(p.get().#source,[#(p.get().#limits),*]).ok_or_else("+INV+")}}});",1),
 ("_:(),_:())->Self::Row{p.get().#presence.then_some(p.get().#source)}}});","_:(),_:())->"+R+"{Ok(p.get().#presence.then_some(p.get().#source))}}});",1),
 ("_:(),_:())->Self::Row{crate::model::OutsideBoundary::from_parts(p.get().#source,p.get().#partner).expect(\"validated original surface boundary\")}}});","_:(),_:())->"+R+"{crate::model::OutsideBoundary::from_parts(p.get().#source,p.get().#partner).ok_or_else("+INV+")}}});",1),
 ("_:(),_:())->Self::Row{match(payload.get().#source,payload.get().#upper){(Some(min),Some(max))=>Some(crate::schedule::ScheduleLimits{min,max}),_=>None}}}});","_:(),_:())->"+R+"{Ok(match(payload.get().#source,payload.get().#upper){(Some(min),Some(max))=>Some(crate::schedule::ScheduleLimits{min,max}),_=>None})}}});",1),
 ("_:(),_:())->Self::Row{[#(#items),*]}}});","_:(),_:())->"+R+"{Ok([#(#items),*])}}});",1),
 ("_:(),_:())->Self::Row{let mut output=Vec::with_capacity(#length);for index in 0..#length{output.push(payload.get().#target.get()[index]);}output}}});","_:(),_:())->"+R+"{let mut output=Vec::with_capacity(#length);for index in 0..#length{output.push(payload.get().#target.get()[index]);}Ok(output)}}});",1),
 ("_:(),_:())->Self::Row{payload.get().#presence.then_some(payload.get().#source)}}});","_:(),_:())->"+R+"{Ok(payload.get().#presence.then_some(payload.get().#source))}}});",1),
 ("let index=if optional{quote!(Some(u32::try_from(context).expect(\"validated original row rank\")))}else{quote!(u32::try_from(context).expect(\"validated original row rank\"))};","let index=if optional{quote!(Some(u32::try_from(context).map_err(|_|"+INV+"())?))}else{quote!(u32::try_from(context).map_err(|_|"+INV+"())?)};",1),
 ("context:usize)->Self::Row{#name{#(#inverse_fields,)*index:#index}}}}}else{quote!()};","context:usize)->"+R+"{Ok(#name{#(#inverse_fields,)*index:#index})}}}}else{quote!()};",1),
 ("_:())->Self::Row{crate::snapshot::state::#row{#(#construction),*}}} #recreation})","_:())->"+R+"{Ok(crate::snapshot::state::#row{#(#construction),*})}} #recreation})",1)])
SP="::semio_s_artifact_energy_model_native_mutations_lifecycle::construction::RowBirthSpec"
edit("✨️derive/✂️delete/🧱️surface/🦀️.rs",[
 (",rank:usize)->Self::Row{#item::Surface(<<crate::CreateSurface as ::semio_s_artifact_energy_model_native_mutations_lifecycle::construction::RowRecreation>::Spec as "+SP+">::finish(p,f,rank))}}",",rank:usize)->"+R+"{Ok(#item::Surface(<<crate::CreateSurface as ::semio_s_artifact_energy_model_native_mutations_lifecycle::construction::RowRecreation>::Spec as "+SP+">::finish(p,f,rank)?))}}",1),
 ("name:String,rank:usize)->Self::Row{#item::Fenestration(crate::CreateFenestration{","name:String,rank:usize)->"+R+"{Ok(#item::Fenestration(crate::CreateFenestration{",1),
 ("index:Some(u32::try_from(rank).expect(\"admitted original aperture rank\"))})}}","index:Some(u32::try_from(rank).map_err(|_|"+INV+"())?)}))}}",1),
 ("new_vertices_m:Vec<[f64;3]>,_:usize)->Self::Row{#item::Vertices(crate::ReplaceFenestrationVertices{id:p.get().id,new_vertices_m})}}","new_vertices_m:Vec<[f64;3]>,_:usize)->"+R+"{Ok(#item::Vertices(crate::ReplaceFenestrationVertices{id:p.get().id,new_vertices_m}))}}",1)])
edit("✨️derive/✂️delete/🪟️fenestration/🦀️.rs",[
 ("rank:usize)->Self::Row{#bundle{create:Some(crate::CreateFenestration{","rank:usize)->"+R+"{Ok(#bundle{create:Some(crate::CreateFenestration{",1),
 ("index:Some(u32::try_from(rank).expect(\"admitted original aperture rank\"))}),vertices:Some(vertices)}}","index:Some(u32::try_from(rank).map_err(|_|"+INV+"())?)}),vertices:Some(vertices)})}",1),
 ("id:b.create.as_ref().expect(\"admitted original aperture creation\").id,new_vertices_m:b.vertices.take().expect(\"admitted original aperture polygon\")}","id:b.create.as_ref().ok_or_else("+INV+")?.id,new_vertices_m:b.vertices.take().ok_or_else("+INV+")?}",1),
 ("#item::Create(b.create.take().expect(\"admitted original aperture creation\"))","#item::Create(b.create.take().ok_or_else("+INV+")?)",1)])
edit("✨️derive/✂️delete/📆️annual/🦀️.rs",[
 ("rank:usize)->Self::Row{#bundle{create:Some(crate::CreateAnnualSchedule{index:u32::try_from(rank).expect(\"admitted original annual rank\"),id:p.get().id,default_daily_schedule_id:p.get().default_daily_schedule_id,holiday_daily_schedule_id:*p.get().holiday_daily_schedule_id.get()}),rules,holidays}}","rank:usize)->"+R+"{Ok(#bundle{create:Some(crate::CreateAnnualSchedule{index:u32::try_from(rank).map_err(|_|"+INV+"())?,id:p.get().id,default_daily_schedule_id:p.get().default_daily_schedule_id,holiday_daily_schedule_id:*p.get().holiday_daily_schedule_id.get()}),rules,holidays})}",1),
 ("let id=b.create.as_ref().expect(\"admitted original annual prefix\").id;","let id=b.create.as_ref().ok_or_else("+INV+")?.id;",1),
 ("index:u32::try_from(index).expect(\"admitted original holiday rank\")","index:u32::try_from(index).map_err(|_|"+INV+"())?",1),
 ("index:u32::try_from(index).expect(\"admitted original rule rank\")","index:u32::try_from(index).map_err(|_|"+INV+"())?",1),
 ("#item::Create(b.create.take().expect(\"admitted original annual prefix\"))","#item::Create(b.create.take().ok_or_else("+INV+")?)",1)])
edit("✨️derive/✂️delete/🦀️.rs",[
 ("let mut ty=types.pop().unwrap();","let mut ty=types.pop().ok_or_else(||error(\"combined native restrictions require an original last source\"))?;",1)])
