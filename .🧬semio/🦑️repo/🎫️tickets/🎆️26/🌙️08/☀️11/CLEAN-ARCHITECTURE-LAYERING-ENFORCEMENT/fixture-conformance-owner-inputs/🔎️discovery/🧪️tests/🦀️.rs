//! 🧪️ The same closed owner withdrawal corpus binds borrowed native discovery.
use super::*;
use crate::conformance::providers::ProviderTargetInput;
use serde_json::Value;
struct Control{frontier:Option<usize>,budget:usize}
impl ProviderOperationControl for Control{fn checkpoint(&mut self,completed:usize)->Result<(),ProviderRefusal>{if self.frontier==Some(completed){return Err(ProviderRefusal::Cancelled);}if completed>=self.budget{return Err(ProviderRefusal::Budget);}Ok(())}}
fn strings(value:Option<&Value>)->Option<Vec<&str>>{value?.as_array()?.iter().map(Value::as_str).collect()}
fn check(values:&[Value],control:&mut Control)->(Result<(),ProviderRefusal>,usize,Vec<String>){
 let declarations=values.iter().map(|value|&value["declaration"]).collect::<Vec<_>>();
 let keys=declarations.iter().map(|value|value.as_object().map(|object|object.keys().map(String::as_str).collect::<Vec<_>>()).unwrap_or_default()).collect::<Vec<_>>();
 let raw=declarations.iter().map(|value|value["law-targets"].as_array()).collect::<Vec<_>>();
 let target_keys=raw.iter().map(|targets|targets.map(|targets|targets.iter().map(|target|target.as_object().map(|object|object.keys().map(String::as_str).collect::<Vec<_>>()).unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default()).collect::<Vec<_>>();
 let laws=raw.iter().map(|targets|targets.map(|targets|targets.iter().map(|target|strings(target.get("laws"))).collect::<Vec<_>>()).unwrap_or_default()).collect::<Vec<_>>();
 let features=raw.iter().map(|targets|targets.map(|targets|targets.iter().map(|target|strings(target.get("features"))).collect::<Vec<_>>()).unwrap_or_default()).collect::<Vec<_>>();
 let targets=raw.iter().enumerate().map(|(owner,targets)|targets.map(|targets|targets.iter().enumerate().map(|(index,target)|ProviderTargetInput{keys:&target_keys[owner][index],kind:target["target-kind"].as_str(),name:target["target-name"].as_str(),laws:laws[owner][index].as_deref(),features:features[owner][index].as_deref(),default_features:target["default-features"].as_bool()}).collect::<Vec<_>>())).collect::<Vec<_>>();
 let recipes=declarations.iter().enumerate().map(|(index,value)|ProviderInput{keys:&keys[index],schema_version:value["schema-version"].as_u64(),targets:targets[index].as_deref()}).collect::<Vec<_>>();
 let inputs=values.iter().enumerate().map(|(index,value)|ConformanceContributionInput{eligible:value["eligible"].as_bool().unwrap(),name:value["name"].as_str().unwrap(),manifest:value["manifest"].as_str().unwrap(),declaration:if declarations[index].is_null(){None}else{Some(&recipes[index])}}).collect::<Vec<_>>();
 let mut discovery=ConformanceDiscovery::default();let mut packages=Vec::new();let result=discovery.discover(&inputs,control,&mut|owner,target|{assert!(!target.laws.is_empty());packages.push(owner.name.to_owned());});(result,discovery.completed(),packages)
}
#[test]
fn shared_current_owner_outcomes(){let corpus:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔎️discovery/🔣️.json")).unwrap();for row in corpus["cases"].as_array().unwrap(){let(result,_,packages)=check(row["inputs"].as_array().unwrap(),&mut Control{frontier:None,budget:65536});let outcome=match result{Ok(())=>"ready",Err(ProviderRefusal::MissingDeclaration)=>"missing-declaration",Err(ProviderRefusal::InvalidDeclaration)=>"invalid-declaration",other=>panic!("unexpected refusal {other:?}")};assert_eq!(outcome,row["outcome"].as_str().unwrap(),"{}",row["id"]);assert_eq!(serde_json::to_value(packages).unwrap(),row["packages"],"{}",row["id"]);}}
#[test]
fn every_native_checkpoint_refuses_and_preserves_prefix(){let corpus:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔎️discovery/🔣️.json")).unwrap();for row in corpus["cases"].as_array().unwrap().iter().filter(|row|row["outcome"]=="ready"){let values=row["inputs"].as_array().unwrap();let(_,total,packages)=check(values,&mut Control{frontier:None,budget:65536});for frontier in 0..total{let(result,completed,prefix)=check(values,&mut Control{frontier:Some(frontier),budget:65536});assert_eq!(result,Err(ProviderRefusal::Cancelled));assert_eq!(completed,frontier);assert_eq!(prefix,packages[..prefix.len()]);let(result,completed,prefix)=check(values,&mut Control{frontier:None,budget:frontier});assert_eq!(result,Err(ProviderRefusal::Budget));assert_eq!(completed,frontier);assert_eq!(prefix,packages[..prefix.len()]);}}}
