# Float32 Source-Sink Review

Full serial JSON companion44530 reached actual compilation and failed E0004 on a new peer-added JsonWriteNode::Float32 variant before test bodies. Engine added the narrow source-sink branch before the ownership handoff; Surface then independently reviewed and completed the neutral regression. Root read the exact branch, existing formatter, schema, fixture and law.

The sink calls existing write_float32_to directly into fixed ScalarText and then the existing bounded raw output path. It does not promote the source to a generic f64 number. The canonical formatter preserves signed zero and defines nonfinite output as null. Ten bit-pattern vectors cover zero, negative zero,0.1,1.0,minimum normal/subnormal,maximum finite,infinities and NaN. The real Rust law compares exact bytes and returned length to independent serde_json serialization of f32. Surface reported schema/DataView checks and parse-only success; those facts do not substitute for native execution. Root found no source-level objection.

After the failed compiler child ended, Surface verified old wrapper57520/58344 had no Cargo/test child, retired only owned44530 with exit1 and verified old admission gone. One full serial replacement36168 now uses generated/json-companion-serial2-2026-11-05.txt. No native Float32 or full-suite GREEN has been claimed. JPG99108 remains untouched.
