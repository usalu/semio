extern crate self as semio_framework_os_kernel;

#[path = "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🦀️.rs"]
mod value;
pub use value::{DslValue, FromValue, ToValue, ValueError};

pub mod os_pack {
    pub mod json {
        use crate::{DslValue, FromValue, ToValue};

        pub fn from_json_str<T: FromValue>(raw: &str) -> Result<T, String> {
            let value: serde_json::Value = serde_json::from_str(raw).map_err(|error| error.to_string())?;
            T::from_value(DslValue::from(value)).map_err(|error| error.to_string())
        }

        pub fn to_json_string<T: ToValue>(value: &T) -> String {
            serde_json::to_string(&serde_json::Value::from(value.to_value())).expect("DslValue is JSON-safe")
        }
    }
}

#[path = "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs"]
pub mod os_directory;

pub mod os_identity {
    use std::sync::atomic::{AtomicU64, Ordering};

    pub fn fill_entropy(bytes: &mut [u8]) -> std::io::Result<()> {
        let seed = NEXT.fetch_add(1, Ordering::Relaxed).to_le_bytes();
        for (index, byte) in bytes.iter_mut().enumerate() { *byte = seed[index % seed.len()] ^ index as u8; }
        Ok(())
    }

    pub fn time_ordered_id() -> String {
        let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_nanos());
        format!("{nanos:032x}{:016x}", NEXT.fetch_add(1, Ordering::Relaxed))
    }

    static NEXT: AtomicU64 = AtomicU64::new(0);
}
