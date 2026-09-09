# Extension Workspace Default Features

The new Generation 3D headless geometry test dependencies requested default-features=false through workspace inheritance, but Cargo ignored that setting because the workspace edge omitted it. Declare the pure-library default at both shared math and BREP edges. The only other consumers already use direct path dependencies with default features disabled; independently built extension component roots retain their own manifest defaults. TOML was parsed and both inherited settings checked before writing.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- Cargo.toml
