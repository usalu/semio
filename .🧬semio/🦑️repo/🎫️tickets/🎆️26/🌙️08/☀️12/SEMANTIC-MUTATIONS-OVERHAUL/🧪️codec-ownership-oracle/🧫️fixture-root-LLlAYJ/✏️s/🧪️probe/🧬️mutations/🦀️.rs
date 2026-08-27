#![feature(negative_impls)]
#![deny(deprecated)]
mod insert_page { pub struct Mutation; pub mod text { pub const CODEC: &str = "insert-page"; pub fn decode(_: &[u8]) {} } }
mod remove_page { pub struct Mutation; }
trait CodecAtom {}
impl CodecAtom for &[u8] {}
impl CodecAtom for u8 {}
impl !CodecAtom for PageMutation {}
impl !CodecAtom for &PageMutation {}
mod serde_json { use super::CodecAtom; pub fn to_vec<T: CodecAtom>(_value: T) {} pub fn to_string<T: CodecAtom>(_value: T) {} pub fn to_value<T: CodecAtom>(_value: T) {} pub fn to_writer<W, T: CodecAtom>(_writer: W, _value: T) {} pub fn from_slice<T: CodecAtom>(_value: &[u8]) -> Result<T, ()> { unreachable!() } pub fn from_str<T: CodecAtom>(_value: &str) -> Result<T, ()> { unreachable!() } pub fn from_reader<T: CodecAtom, R>(_value: R) -> Result<T, ()> { unreachable!() } pub fn from_value<T: CodecAtom>(_value: u8) -> Result<T, ()> { unreachable!() } }
pub enum PageMutation { #[deprecated] InsertPage(insert_page::Mutation), #[deprecated] RemovePage(remove_page::Mutation) }
