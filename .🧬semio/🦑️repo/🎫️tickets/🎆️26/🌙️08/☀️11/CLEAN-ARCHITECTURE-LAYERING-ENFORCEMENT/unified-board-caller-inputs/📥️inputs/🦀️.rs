//! 📥️ A caller-owned strict JSON cursor retains admitted grammar owners on refusal.
use semio_framework_pack_json::{JsonGrammarCursor,JsonMemberPolicy,JsonParsedValue};
use semio_framework_value::{NativeDecodeControl,ValueError};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor};

/// 🧵️ Each input owns its unchanged source contract and a separate retained grammar cursor.
pub struct BoardJsonCursor<V:JsonParsedValue>{cursor:JsonGrammarCursor<V>}
impl<V:JsonParsedValue> BoardJsonCursor<V>{
 /// 🌱️ Starts the closed duplicate-refusing grammar without allocating input owners.
 pub fn new()->Self{Self{cursor:JsonGrammarCursor::new(JsonMemberPolicy::Reject)}}
 /// 🚦️ Advances genuine grammar callbacks using only the supplied cumulative capability.
 pub fn decode(&mut self,source:&str,control:&mut NativeDecodeControl<'_>)->Result<V,ValueError>{
  loop{if let Some(value)=self.cursor.step(source,64,control).map_err(semio_framework_pack_json::JsonError::into_value_error)?{return Ok(value);}}
 }
}
impl<V:JsonParsedValue> RetireOwned for BoardJsonCursor<V>{
 fn retirement(self)->Box<dyn RetirementCursor>{self.cursor.retirement()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
