# Wave 6 Flip (integrator only)
1. Rewrite ✏️s/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs as native Brep delegating to modules.
2. Drop brepkit-* from Cargo.toml + cargo update lock.
3. Rename BrepkitKernel→Brep across consumers (flow brep, cad engine, process3d, os host, demonstrator, lowpoly test, benches).
4. Keep BrepKernel async trait + block_on (do not drop async-trait this wave).
5. Delete differential harness deps if any.
6. cargo test -p semio-s-3d; consumer builds.
