macro_rules! env { ($name:literal) => { concat!(::std::env!("CARGO_MANIFEST_DIR"), "/redirect") }; }
