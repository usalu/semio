//! 📁️ Resolves literal and same-file scalar constant path arguments with exact provenance.
use super::tokens::{Kind,Token,tokens,string_value,token_pairs};
use std::collections::{BTreeMap,BTreeSet};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RustPathReferenceContext{Method,Qualified}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RustPathSourceScope{File,Crate}
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RustPathConstantDefinition{pub name:String,pub start:usize,pub end:usize,pub line:usize}
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum RustPathReferenceOrigin{Literal,Constant(RustPathConstantDefinition)}
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct RustPathReference{pub value:String,pub path:String,pub root:String,pub call:String,pub context:RustPathReferenceContext,pub start:usize,pub end:usize,pub line:usize,pub origin:RustPathReferenceOrigin}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct RustPathRootError;
impl RustPathRootError{pub fn code(&self)->&'static str{"invalid-root"}}
impl std::fmt::Display for RustPathRootError{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{formatter.write_str("Rust path root must be unique and canonical relative")}}
impl std::error::Error for RustPathRootError{}

#[derive(Clone,Copy,PartialEq,Eq)]
enum ScopeKind{Module,Function,Block,Associated}
struct Definition{value:Option<String>,definition:RustPathConstantDefinition}
struct Shadow{name:String,start:usize,end:usize}
struct Scope{parent:Option<usize>,start:usize,end:usize,kind:ScopeKind,constants:BTreeMap<String,Vec<Definition>>,modules:BTreeMap<String,Vec<usize>>,shadows:Vec<Shadow>}
struct Meta{kind:ScopeKind,name:Option<String>,parameters:Option<(usize,usize)>,generic_names:Vec<String>}
fn identifier<'a>(token:Option<&'a Token<'_>>)->Option<&'a str>{token.filter(|token|token.kind==Kind::Identifier).map(|token|token.text.strip_prefix("r#").unwrap_or(token.text))}
fn scope(parent:Option<usize>,start:usize,end:usize,kind:ScopeKind)->Scope{Scope{parent,start,end,kind,constants:BTreeMap::new(),modules:BTreeMap::new(),shadows:Vec::new()}}
fn segments(tokens:&[Token<'_>],pairs:&[Option<usize>],start:usize,end:usize)->Vec<(usize,usize)>{let mut output=Vec::new();let mut first=start;let mut at=start;while at<end{if tokens[at].text==","{output.push((first,at));first=at+1;}at=pairs[at].filter(|close|*close>at).unwrap_or(at)+1;}output.push((first,end));output}
fn pattern_names(tokens:&[Token<'_>],start:usize,end:usize)->BTreeSet<String>{(start..end).filter_map(|at|{let name=identifier(tokens.get(at))?;if matches!(name,"ref"|"mut"|"self"|"Self"|"true"|"false"|"in"|"move")||tokens.get(at+1).is_some_and(|token|matches!(token.text,"::"|"("|"{"))||at>0&&tokens[at-1].text=="::"||at+1<end&&tokens[at+1].text==":"{None}else{Some(name.to_owned())}}).collect()}
fn statement_end(tokens:&[Token<'_>],pairs:&[Option<usize>],start:usize)->usize{let mut at=start;while at<tokens.len()&&tokens[at].text!=";"{if tokens[at].text=="}"{return at;}at=pairs[at].filter(|close|*close>at).unwrap_or(at)+1;}at}

fn scope_tree(tokens:&[Token<'_>],pairs:&[Option<usize>])->(Vec<Scope>,Vec<usize>){
 let mut scopes=vec![scope(None,0,tokens.len(),ScopeKind::Module)];let mut membership=vec![0;tokens.len()];let mut kinds=BTreeMap::new();let mut openings=BTreeMap::new();let mut function_headers=Vec::new();
 for at in 0..tokens.len(){
  if tokens[at].text=="mod"&&identifier(tokens.get(at+1)).is_some()&&tokens.get(at+2).is_some_and(|token|token.text=="{"){kinds.insert(at+2,Meta{kind:ScopeKind::Module,name:identifier(tokens.get(at+1)).map(str::to_owned),parameters:None,generic_names:Vec::new()});}
  if tokens[at].text=="fn"&&identifier(tokens.get(at+1)).is_some(){let mut open=at+2;let mut depth=0i32;let mut generic_names=Vec::new();while open<tokens.len(){let text=tokens[open].text;if text=="("&&depth==0||depth==0&&matches!(text,";"|"{"){break;}if text=="const"&&depth>0{if let Some(name)=identifier(tokens.get(open+1)){generic_names.push(name.to_owned());}}if text=="<"{depth+=1;}else if text==">"{depth-=1;}else if text==">>"{depth-=2;}if depth>0{open=pairs[open].filter(|close|*close>open).unwrap_or(open);}open+=1;}if tokens.get(open).is_some_and(|token|token.text=="("){if let Some(close)=pairs[open]{let mut body=close+1;while body<tokens.len()&&!matches!(tokens[body].text,"{"|";"){body=pairs[body].filter(|close|*close>body).unwrap_or(body)+1;}if tokens.get(body).is_some_and(|token|token.text=="{"){function_headers.push((at,body));kinds.insert(body,Meta{kind:ScopeKind::Function,name:None,parameters:Some((open+1,close)),generic_names});}}}}
  if matches!(tokens[at].text,"impl"|"trait"){let mut body=at+1;while body<tokens.len()&&!matches!(tokens[body].text,"{"|";"){body+=1;}if tokens.get(body).is_some_and(|token|token.text=="{"){kinds.insert(body,Meta{kind:ScopeKind::Associated,name:None,parameters:None,generic_names:Vec::new()});}}
 }
 let mut current=0;for at in 0..tokens.len(){membership[at]=current;if tokens[at].text=="{"{if let Some(end)=pairs[at]{let meta=kinds.get(&at);let index=scopes.len();let mut next=scope(Some(current),at+1,end,meta.map_or(ScopeKind::Block,|meta|meta.kind));if let Some(meta)=meta{for name in &meta.generic_names{next.shadows.push(Shadow{name:name.clone(),start:next.start,end:next.end});}}if let Some((start,end))=meta.and_then(|meta|meta.parameters){for(start,end)in segments(tokens,pairs,start,end){let mut colon=start;while colon<end&&tokens[colon].text!=":"{colon=pairs[colon].filter(|close|*close>colon).unwrap_or(colon)+1;}for name in pattern_names(tokens,start,colon){next.shadows.push(Shadow{name,start:next.start,end:next.end});}}}scopes.push(next);openings.insert(at,index);if let Some(name)=meta.and_then(|meta|meta.name.as_ref()){scopes[current].modules.entry(name.clone()).or_default().push(index);}current=index;}}else if tokens[at].text=="}"&&current!=0&&scopes[current].end==at{current=scopes[current].parent.unwrap();}}
 for at in 0..tokens.len(){let index=membership[at];
  if tokens[at].text=="const"&&!function_headers.iter().any(|(start,end)|*start<=at&&at<*end)&&identifier(tokens.get(at+1)).is_some()&&tokens.get(at+2).is_some_and(|token|token.text==":"){let end=statement_end(tokens,pairs,at+3);let mut equal=at+3;while equal<end&&tokens[equal].text!="="{equal=pairs[equal].filter(|close|*close>equal).unwrap_or(equal)+1;}let value=if equal+2==end{tokens.get(equal+1).and_then(string_value)}else{None};let token=&tokens[at+1];let name=identifier(Some(token)).unwrap().to_owned();scopes[index].constants.entry(name.clone()).or_default().push(Definition{value,definition:RustPathConstantDefinition{name,start:token.start,end:token.end,line:token.line}});}
  if tokens[at].text=="let"{let mut equal=at+1;while equal<tokens.len()&&!matches!(tokens[equal].text,"="|";"){equal=pairs[equal].filter(|close|*close>equal).unwrap_or(equal)+1;}let mut pattern_end=at+1;while pattern_end<equal&&tokens[pattern_end].text!=":"{pattern_end=pairs[pattern_end].filter(|close|*close>pattern_end).unwrap_or(pattern_end)+1;}let names=pattern_names(tokens,at+1,pattern_end);if at>0&&matches!(tokens[at-1].text,"if"|"while"){let mut body=equal+1;while body<tokens.len()&&tokens[body].text!="{"{body=pairs[body].filter(|close|*close>body).unwrap_or(body)+1;}if let Some(index)=openings.get(&body).copied(){for name in names{let(start,end)=(scopes[index].start,scopes[index].end);scopes[index].shadows.push(Shadow{name,start,end});}}}else{let start=statement_end(tokens,pairs,equal+1)+1;let end=scopes[index].end;for name in names{scopes[index].shadows.push(Shadow{name,start,end});}}}
  if tokens[at].text=="|"&&at>0&&matches!(tokens[at-1].text,"="|"("|","|"move"|"return"|">"|"{"){let mut close=at+1;while close<tokens.len()&&tokens[close].text!="|"{close=pairs[close].filter(|end|*end>close).unwrap_or(close)+1;}if close<tokens.len(){let mut end=close+1;while end<tokens.len()&&!matches!(tokens[end].text,";"|","|")"|"}"){end=pairs[end].filter(|close|*close>end).unwrap_or(end)+1;}for(start,finish)in segments(tokens,pairs,at+1,close){let mut colon=start;while colon<finish&&tokens[colon].text!=":"{colon=pairs[colon].filter(|close|*close>colon).unwrap_or(colon)+1;}for name in pattern_names(tokens,start,colon){scopes[index].shadows.push(Shadow{name,start:close+1,end});}}}}
  if tokens[at].text=="for"{let mut end=at+1;while end<tokens.len()&&tokens[end].text!="in"{end+=1;}let mut body=end+1;while body<tokens.len()&&tokens[body].text!="{"{body=pairs[body].filter(|close|*close>body).unwrap_or(body)+1;}if let Some(index)=openings.get(&body).copied(){for name in pattern_names(tokens,at+1,end){let(start,end)=(scopes[index].start,scopes[index].end);scopes[index].shadows.push(Shadow{name,start,end});}}}
 }
 (scopes,membership)
}
fn module_of(scopes:&[Scope],mut index:usize)->usize{while scopes[index].kind!=ScopeKind::Module{let Some(parent)=scopes[index].parent else{break;};index=parent;}index}
fn bare<'a>(scopes:&'a[Scope],name:&str,mut index:usize,at:usize)->Option<&'a Definition>{let mut capture=true;loop{let scope=&scopes[index];if scope.kind==ScopeKind::Associated{return None;}if capture&&scope.shadows.iter().any(|shadow|shadow.name==name&&shadow.start<=at&&at<shadow.end){return None;}if let Some(entries)=scope.constants.get(name){return if entries.len()==1&&entries[0].value.is_some(){Some(&entries[0])}else{None};}if scope.kind==ScopeKind::Module{return None;}if scope.kind==ScopeKind::Function{capture=false;}index=scope.parent?;}}
fn resolve_constant<'a>(tokens:&[Token<'_>],scopes:&'a[Scope],membership:&[usize],start:usize,end:usize,source_scope:RustPathSourceScope)->Option<&'a Definition>{
 if(end-start)%2!=1{return None;}let mut names=Vec::new();for at in start..end{if(at-start)%2==1{if tokens[at].text!="::"{return None;}}else{names.push(identifier(tokens.get(at))?);}}
 if names.len()==1{return bare(scopes,names[0],membership[start],start);}let mut index=module_of(scopes,membership[start]);let mut first=0;
 if names[0]=="crate"{if source_scope!=RustPathSourceScope::Crate{return None;}index=0;first+=1;}else if names[0]=="self"{first+=1;}else if names[0]=="super"{while names.get(first)==Some(&"super"){index=module_of(scopes,scopes[index].parent?);first+=1;}}else{let mut lexical=membership[start];let found=loop{let scope=&scopes[lexical];if let Some(found)=scope.modules.get(names[0]){break found;}if scope.kind==ScopeKind::Module{return None;}lexical=scope.parent?;};if found.len()!=1{return None;}index=found[0];first+=1;}
 while first+1<names.len(){let modules=scopes[index].modules.get(names[first])?;if modules.len()!=1{return None;}index=modules[0];first+=1;}let entries=scopes[index].constants.get(names.get(first).copied()?)?;if entries.len()==1&&entries[0].value.is_some(){Some(&entries[0])}else{None}
}
fn qualified_call(call:&str)->bool{matches!(call,"std::path::Path::new"|"std::path::PathBuf::from"|"std::fs::File::open"|"std::fs::File::create")||call.strip_prefix("std::fs::").is_some_and(|name|matches!(name,"read"|"read_to_string"|"read_dir"|"write"|"metadata"|"symlink_metadata"|"canonicalize"|"create_dir"|"create_dir_all"|"remove_file"|"remove_dir"|"remove_dir_all"))}

/// 🧭️ Resolves whole first literal or scalar constant references within explicit source module scope.
pub fn inspect_rust_path_references(source:&str,roots:&[&str],source_scope:RustPathSourceScope)->Result<Vec<RustPathReference>,RustPathRootError>{
 let mut seen=BTreeSet::new();for root in roots{if !seen.insert(*root)||root.chars().any(|value|matches!(value,'\\'|':'|'\0'|'\r'|'\n'))||root.split('/').any(|part|matches!(part,""|"."|"..")){return Err(RustPathRootError);}}
 let tokens=tokens(source);let pairs=token_pairs(&tokens);let(scopes,membership)=scope_tree(&tokens,&pairs);let mut output=Vec::new();
 for open in 1..tokens.len(){if tokens[open].text!="("{continue;}let Some(close)=pairs[open]else{continue;};let Some(name)=identifier(tokens.get(open-1))else{continue;};let method=open>=2&&tokens[open-2].text==".";let mut first=open-1;if !method{while first>=2&&tokens[first-1].text=="::"&&identifier(tokens.get(first-2)).is_some(){first-=2;}}let call=if method{name.to_owned()}else{tokens[first..open].iter().map(|token|token.text).collect::<String>()};if !(if method{matches!(name,"join"|"push")}else{qualified_call(&call)}){continue;}let(start,end)=segments(&tokens,&pairs,open+1,close)[0];if start==end{continue;}let literal=if end==start+1{string_value(&tokens[start])}else{None};let definition=if literal.is_none(){resolve_constant(&tokens,&scopes,&membership,start,end,source_scope)}else{None};let Some(value)=literal.or_else(||definition.and_then(|definition|definition.value.clone()))else{continue;};let normalized=value.replace('\\',"/");let mut path=normalized.as_str();while let Some(next)=path.strip_prefix("./").or_else(||path.strip_prefix("../")){path=next;}let Some(root)=roots.iter().filter(|root|path==**root||path.strip_prefix(**root).is_some_and(|suffix|suffix.starts_with('/'))).max_by_key(|root|root.encode_utf16().count())else{continue;};if path.chars().any(|value|matches!(value,'\0'|'\r'|'\n')){continue;}output.push(RustPathReference{value:value.clone(),path:path.to_owned(),root:(*root).to_owned(),call,context:if method{RustPathReferenceContext::Method}else{RustPathReferenceContext::Qualified},start:tokens[start].start,end:tokens[end-1].end,line:tokens[start].line,origin:definition.map_or(RustPathReferenceOrigin::Literal,|definition|RustPathReferenceOrigin::Constant(definition.definition.clone()))});}
 output.sort_by_key(|reference|reference.start);Ok(output)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
