pub use semio_framework_os_spr::channel;
pub use channel::{AppFrame,encode_app_frame};
#[global_allocator]static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🎞️app-frame/🌱️emit/🦀️.rs"]mod original_emit;

pub use semio_framework_os_pack::pack_rt::encode_wire_value;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌉️bridge/🌱️original/🦀️.rs"]mod original_wire_value;
