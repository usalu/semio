
    registry.finalize();
}

#[cfg(any(test, target_arch = "wasm32"))]
fn module_registry() -> Registry {
    let mut registry = Registry::new();
    register(&mut registry);
    registry
}

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_extension_sdk::{build_manifest_json, evaluate_json};

    fn point(x: f64, y: f64, z: f64) -> Dictionary {
        Dictionary::with_schema("point").insert("x", Value::Atom(Atom::Decimal(x))).insert("y", Value::Atom(Atom::Decimal(y))).insert("z", Value::Atom(Atom::Decimal(z)))
    }

    fn vector(x: f64, y: f64, z: f64) -> Dictionary {
        Dictionary::with_schema("vector").insert("x", Value::Atom(Atom::Decimal(x))).insert("y", Value::Atom(Atom::Decimal(y))).insert("z", Value::Atom(Atom::Decimal(z)))
    }

    fn test_serial() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    fn channel_payload(out: &Dictionary, channel: &str) -> Dictionary {
        out.get(channel).and_then(|v| v.as_dictionary()).cloned().expect("channel payload")
    }

    fn reset_test_kernel() {
        if let Ok(mut guard) = kernel().write() {
            *guard = Box::new(Brep::new());
        }
        if let Ok(mut cache) = mesh_cache().lock() {
            cache.clear();
        }
    }

    #[test]
    fn box_emits_geometry_handle() {