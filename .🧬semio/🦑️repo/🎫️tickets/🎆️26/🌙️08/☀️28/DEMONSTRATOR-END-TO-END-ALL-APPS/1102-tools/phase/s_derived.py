from ph import *
c="semio-framework-os-store-replay-operation"
D="🌱️derived/🦀️.rs"
E(c,D,"let Err(error)=std::mem::replace(&mut prepared.inverse,Ok(Default::default()))else{unreachable!()};",'let error=match std::mem::replace(&mut prepared.inverse,Ok(Default::default())){Err(error)=>error,Ok(inverse)=>{prepared.inverse=Ok(inverse);return Err(ValueError::invariant("derived inverse lost its refusal"))}};')
E(c,D,"}else if !prepared.inverse.as_ref().unwrap().is_empty(){*self.mutation=prepared.inverse.as_mut().unwrap().pop();","}else if prepared.inverse.as_ref().is_ok_and(|inverse|!inverse.is_empty()){*self.mutation=prepared.inverse.as_mut().ok().and_then(|inverse|inverse.pop());")
E(c,D,"}else if !prepared.inverse.as_ref().unwrap().terminal_is_empty(){let p=prepared.inverse.as_mut().unwrap().release_empty_page(",'}else if prepared.inverse.as_ref().is_ok_and(|inverse|!inverse.terminal_is_empty()){let p=prepared.inverse.as_mut().map_err(|_|ValueError::invariant("derived inverse lost its page owner"))?.release_empty_page(')
for fld,slot in (("producer",4),("snapshots",0),("mutations",1)):
    E(c,D,f"}}else if self.{fld}.is_some(){{let factory:Arc<dyn semio_framework_value::FactoryRetirement>=self.{fld}.take().unwrap();",f"}}else if let Some(factory)=self.{fld}.take(){{let factory:Arc<dyn semio_framework_value::FactoryRetirement>=factory;")
E(c,D,"let factory=self.factory_close[index].as_mut().unwrap();",'let Some(factory)=self.factory_close[index].as_mut()else{return Err(ValueError::invariant("derived factory closure is absent"))};')
if __name__=="__main__":build()
