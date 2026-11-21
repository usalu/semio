import sys
from pathlib import Path

source = Path("🧰️framework/🔨️modules/🌱️value/✨️derive/🛬️receive/🔀️enum/🦀️.rs")
text = source.read_text()

def edit(before, after, count=1):
    global text
    actual = text.count(before)
    if actual != count:
        raise SystemExit(f"{actual} matching spans; expected {count}: {before}")
    text = text.replace(before, after)

edit("let mut slots=Vec::new();", "")
edit("let mut direct_slots=Vec::new();", "")
edit('let slot=format_ident!("variant_{}",id.to_string().to_lowercase());', "")
edit('direct_types.push(direct_type);direct_slots.push(format_ident!("direct_{}",id.to_string().to_lowercase()));', "direct_types.push(direct_type);")
edit("carriers.push(carrier);slots.push(slot);wires.push(wire);", "carriers.push(carrier);wires.push(wire);")
edit("let all_slots=slots.iter().chain(direct_slots.iter()).collect::<Vec<_>>();", "")
edit(
    'let pending=all_indices.iter().map(|i|format_ident!("pending_{}",i)).collect::<Vec<_>>();',
    'let selected=format_ident!("{}NativeSelectedVariant",name);let variants=all_indices.iter().map(|i|format_ident!("Variant{}",i)).collect::<Vec<_>>();',
)

edit("let child_plans=all_indices.iter().zip(all_slots.iter()).zip(all_types.iter()).map(|((i,slot),ty)|quote!(#i=>match self.phase{",
     "let child_plans=variants.iter().zip(all_types.iter()).map(|(variant,ty)|quote!(#selected::#variant{slot,..}=>match self.phase{")
edit("let child_apply=all_indices.iter().zip(all_slots.iter()).zip(pending.iter()).zip(all_constructors.iter()).map(|(((i,slot),pending),construct)|quote!(#i=>match self.phase{",
     "let child_apply=variants.iter().zip(all_constructors.iter()).map(|(variant,construct)|quote!(#selected::#variant{slot,pending}=>match self.phase{")
edit("let child_close_plan=all_indices.iter().zip(all_slots.iter()).zip(pending.iter()).zip(all_types.iter()).map(|(((i,slot),pending),ty)|quote!(#i=>{",
     "let child_close_plan=variants.iter().zip(all_types.iter()).map(|(variant,ty)|quote!(#selected::#variant{slot,pending}=>{")
edit("let child_close=all_indices.iter().zip(all_slots.iter()).zip(pending.iter()).map(|((i,slot),pending)|quote!(#i=>{",
     "let child_close=variants.iter().map(|variant|quote!(#selected::#variant{slot,pending}=>{")
edit("self.#slot.", "slot.", 21)
edit("&mut self.#pending", "&mut **pending", 2)
edit("self.#pending.", "pending.", 5)
edit("self.selected=None;Self::finish", "self.child=None;Self::finish")

edit(
    '#(#all_slots:#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#all_types>,#pending: ::std::mem::ManuallyDrop<Option<#all_types>>,)*',
    'child:Option<#selected>,',
)
edit('retirement:#value::retained_clone::RetainedCloneClose,selected:Option<usize>,direct:bool,',
     'retirement:#value::retained_clone::RetainedCloneClose,direct:bool,')
edit('#(#all_slots:#tree::NativeReceiveSlot::default(),#pending: ::std::mem::ManuallyDrop::new(None),)*',
     'child:None,')
edit('retirement:Default::default(),selected:None,direct:false,',
     'retirement:Default::default(),direct:false,')
edit(
    '#vis struct #receiver<I:#tree::NativeValueSource+?Sized>',
    'enum #selected where #(#all_types:#native::native_decoding::NativeReceive<dyn #tree::NativeValueSource>,)*{#(#variants{slot:#tree::NativeReceiveSlot<dyn #tree::NativeValueSource,#all_types>,pending: ::std::mem::ManuallyDrop<Option<#all_types>>},)*}\n #vis struct #receiver<I:#tree::NativeValueSource+?Sized>',
)
edit('::std::mem::size_of::<Option<usize>>()+::std::mem::size_of::<bool>()', '::std::mem::size_of::<Option<#selected>>()+::std::mem::size_of::<bool>()')
edit('::std::mem::size_of::<Option<usize>>()),1))}', '::std::mem::size_of::<Option<#selected>>()),1))}')
edit('match self.selected.ok_or_else(#tree::invalid)?{#(#child_plans,)*_=>Err(#tree::invalid())}',
     'match self.child.as_ref().ok_or_else(#tree::invalid)?{#(#child_plans,)*}')
edit('self.selected=Some(selected.index+if selected.direct{#variant_count}else{0});self.phase=1;work=3',
     'let index=selected.index+if selected.direct{#variant_count}else{0};self.child=Some(match index{#(#all_indices=>#selected::#variants{slot:Default::default(),pending: ::std::mem::ManuallyDrop::new(None)},)*_=>return Err(#tree::invalid().into())});self.phase=1;work=3')
edit('match self.selected.ok_or_else(#tree::invalid)?{#(#child_apply,)*_=>return Err(#tree::invalid().into())}',
     'match self.child.as_mut().ok_or_else(#tree::invalid)?{#(#child_apply,)*}')
edit('match self.selected{#(Some(#all_indices)=>#owner::native_close_settle(effect,|effect,reply|self.#all_slots.begin_close_into(effect,reply)).map(|s|#value::RetainedCloneStep::Progress(s.progress())),)*_=>',
     'match self.child.as_mut(){#(Some(#selected::#variants{slot,..})=>#owner::native_close_settle(effect,|effect,reply|slot.begin_close_into(effect,reply)).map(|s|#value::RetainedCloneStep::Progress(s.progress())),)*None=>')
edit('match self.selected{#(Some(#all_indices)=>#owner::native_close_settle(effect,|effect,reply|self.#all_slots.close_into(body,effect,reply)).map(|s|#value::RetainedCloneStep::Progress(s.progress())),)*_=>',
     'match self.child.as_mut(){#(Some(#selected::#variants{slot,..})=>#owner::native_close_settle(effect,|effect,reply|slot.close_into(body,effect,reply)).map(|s|#value::RetainedCloneStep::Progress(s.progress())),)*None=>')
edit('if let Some(selected)=self.selected{return match selected{#(#child_close_plan,)*_=>Err(#tree::invalid())}}',
     'if let Some(child)=self.child.as_ref(){return match child{#(#child_close_plan,)*}}')
edit('if let Some(selected)=self.selected{return match selected{#(#child_close,)*_=>Complete(#value::RetainedWorkOutcome{result:Err(#tree::invalid().into()),work:0})}}',
     'if let Some(child)=self.child.as_mut(){return match child{#(#child_close,)*}}')
edit('self.selected.is_none()&&self.output.is_none()&&self.retirement.is_empty()#(&&self.#all_slots.terminal_is_empty())*#(&&pending.is_none())*',
     'self.child.is_none()&&self.output.is_none()&&self.retirement.is_empty()')
edit('match self.state.selected{\n    #(Some(#all_indices)=>self.state.#all_slots.advance_work_demand(source,body),)*\n    _=>Err(#tree::invalid()),',
     'match self.state.child.as_ref(){\n    #(Some(#selected::#variants{slot,..})=>slot.advance_work_demand(source,body),)*\n    None=>Err(#tree::invalid()),')
edit('match self.state.selected{#(Some(#all_indices)=>self.state.#all_slots.advance(source,control,witness,body,maximum,g).map(|s|#value::RetainedCloneStep::Progress(s.progress())),)*_=>',
     'match self.state.child.as_mut(){#(Some(#selected::#variants{slot,..})=>slot.advance(source,control,witness,body,maximum,g).map(|s|#value::RetainedCloneStep::Progress(s.progress())),)*None=>')
edit('self.state.selector.advance_pending() #(||self.state.#all_slots.advance_pending())*',
     'self.state.selector.advance_pending()||self.state.child.as_ref().is_some_and(|child|match child{#(#selected::#variants{slot,..}=>slot.advance_pending(),)*})')
for old in ("#all_slots", "self.#pending", "self.selected", "self.state.selected"):
    if old in text:
        raise SystemExit(f"Unconverted old discriminator storage {old!r}")
if sys.argv[1:] == ["--write"]:
    source.write_text(text)
print(f"Selected native enum derive validated ({len(text.encode())} source bytes)")
