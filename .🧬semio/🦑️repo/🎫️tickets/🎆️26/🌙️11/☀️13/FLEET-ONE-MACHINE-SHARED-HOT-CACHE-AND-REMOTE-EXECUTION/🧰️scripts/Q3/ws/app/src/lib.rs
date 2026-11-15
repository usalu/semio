use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize)] pub struct P{pub a:u32}
bitflags::bitflags!{ pub struct F:u32{ const A=1; } }
pub fn f()->usize{ let mut m=indexmap::IndexMap::<u32,u32>::new(); m.insert(1,2); let _=heck::ToSnakeCase::to_snake_case("AbC"); let _=semver::Version::new(1,2,3); let _:anyhow::Result<()> = Ok(()); serde_json::to_string(&P{a:1}).unwrap().len()+m.len() }
