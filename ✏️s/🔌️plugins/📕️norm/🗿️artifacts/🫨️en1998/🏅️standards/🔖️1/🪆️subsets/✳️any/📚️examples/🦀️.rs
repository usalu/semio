//! 📚️ EN 1998 example documents.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🏢️seismic-rc-frame/🧪️tests/🧩️example/🦀️.rs"]
mod example;
#[path = "🏢️seismic-multipart/🦀️.rs"]
pub mod seismic_multipart;
#[path = "⚠️seismic-multipart-fail/🦀️.rs"]
pub mod seismic_multipart_fail;
#[path = "🏢️seismic-rc-frame/🦀️.rs"]
pub mod seismic_rc_frame;
#[path = "⚠️seismic-rc-frame-fail/🦀️.rs"]
pub mod seismic_rc_frame_fail;
//#endregion 🪢️Mounts
