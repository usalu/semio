# Native Map Provider Census

Source-only bounded framework Rust census. No runtime or compilation was performed. Each conversion must remain an allocation-free move of existing ownership; fallible backing belongs to the staged receiver.

- `🧰️framework/🔨️modules/📡️replication/🆔️ids/🛬️receiving/🦀️.rs`:3 use semio_framework_value_native_owned::native_decoding::{NativeReceive,owned::{NativeReceiveMap,NativeMappedReceiver,tree::NativeValueSource}};
- `🧰️framework/🔨️modules/📡️replication/🆔️ids/🛬️receiving/🦀️.rs`:4 impl NativeReceiveMap<String> for SchemaId{fn from_native(value:String)->Self{Self(value)}}
- `🧰️framework/🔨️modules/📡️replication/🆔️ids/🛬️receiving/🦀️.rs`:11 impl NativeReceiveMap<[u8;32]> for PayloadHash{fn from_native(value:[u8;32])->Self{Self(value)}}
- `🧰️framework/🔨️modules/📡️replication/🆔️ids/🛬️receiving/🧪️tests/🦀️.rs`:16 #[test]fn schema_identity_adopts_the_completed_native_text_owner(){let text=String::from("schema:original");let pointer=text.as_ptr();let owner=<SchemaId as NativeReceiveMap<String>>::from_native(text);assert_eq!(owner.0.as_ptr(),pointer);drain(owner
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs`:67 pub use mapped::{NativeReceiveMap,NativeMappedReceiver};
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔄️map/🦀️.rs`:6 pub trait NativeReceiveMap<T>:RetireOwned{fn from_native(value:T)->Self;}
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔄️map/🦀️.rs`:8 pub struct NativeMappedReceiver<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>>{inner:T::Receiver,pending:ManuallyDrop<Option<T>>,output:ManuallyDrop<Option<U>>,retirement:RetainedCloneClose,closing:bool,marker:PhantomData<fn(&I)->U>}
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔄️map/🦀️.rs`:11 impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>> NativeMappedReceiver<I,T,U>{pub fn new()->Self{Self{inner:T::receiver(),pending:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),closing:false,marker:
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔄️map/🦀️.rs`:12 impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>>NativeReceiver<I,U>for NativeMappedReceiver<I,T,U>{
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔄️map/🦀️.rs`:23 impl<I:'static+?Sized,T:NativeReceive<I>,U:NativeReceiveMap<T>>Drop for NativeMappedReceiver<I,T,U>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"native mapped receiving requires funded closure");if em
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🔄️map/🧪️tests/🦀️.rs`:5 impl NativeReceiveMap<String> for Wrapped{fn from_native(value:String)->Self{Self(value)}}
- `🧰️framework/🔨️modules/🖱️ui/🌐️locale/🏷️label/🛬️receiving/🦀️.rs`:3 use semio_framework_value_native_owned::native_decoding::{NativeReceive,owned::{NativeReceiveMap,NativeMappedReceiver,tree::NativeValueSource}};
- `🧰️framework/🔨️modules/🖱️ui/🌐️locale/🏷️label/🛬️receiving/🦀️.rs`:10 impl NativeReceiveMap<NativeLabelTerminologies> for LocalizedLabel{
- `🧰️framework/🔨️modules/🖱️ui/🌐️locale/🏷️label/🛬️receiving/🦀️.rs`:11  fn from_native(value:NativeLabelTerminologies)->Self{Self{cells:[[Cow::Owned(value.native.en),Cow::Owned(value.native.de)],[Cow::Owned(value.reuse.en),Cow::Owned(value.reuse.de)]]}}
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/🖼️presentation/🗂️expanded/🦀️.rs`:49 impl semio_framework_value_native_owned::native_decoding::owned::NativeReceiveMap<Vec<String>> for DagExpandedPaths{
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🧬️schema/🖼️presentation/🗂️expanded/🦀️.rs`:50  fn from_native(mut values:Vec<String>)->Self{values.sort_unstable();values.dedup();Self::from_admitted(values)}
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📥️receive/🦀️.rs`:4 use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeReceive,NativeReceiver,owned::{NativeDefault,NativeMappedReceiver,NativeReceiveMap,tree::{NativeReceiveSlot,NativeScalarReceiver,NativeScalarValue,NativeValueSource,a
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📥️receive/🦀️.rs`:24 impl NativeReceiveMap<OrderedMap<Value>> for Dictionary{fn from_native(pairs:OrderedMap<Value>)->Self{Self{pairs}}}
