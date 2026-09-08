# Lowpoly CAD Fixture Dependency

Native532 emitted a Cargo warning that Lowpoly's default-features=false override was ignored for its inherited CAD artifact dependency. The current CAD artifact package declares no default or plugin-entry feature, and its artifact root exports no plugin component entry point. The plugin-entry feature belongs to the separate CAD plugin package.

Removed the ineffective override and the obsolete comment describing the former plugin dependency. The optional cad-fixtures dependency remains enabled by the same feature. Fresh Cargo metadata and compilation must confirm the manifest warning is gone; native532 had already loaded the old manifest.

Changed: `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml`.

The retained worker in the ticket's `📜️script.ts` now records distinct warning lines from both Cargo metadata and compiler stderr alongside structured Rust diagnostics. Native532 started before this receipt change and has a known old-manifest warning in its log, so its eventual structured diagnostic count alone cannot establish warning-free output.


Pass534 fresh Cargo metadata exited 0 with 0 warning lines: []. This validates manifest loading only; compiler and link verification remain separate.
