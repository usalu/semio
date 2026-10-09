# Guest SQLite Current Contract

Read-only observation, 2026-10-09. No compilation or Guest execution performed. Paths below are relative to the repository. Concurrent control-signature edits are visible.

## Authentic capability already exists

The component contract is semantic SQLite, not merely native pack/JSON. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1451–1455` declares `sqlite-schema`, `sqlite-export`, and `sqlite-import`, exact dialect coordinates, both native encodings, SQL limits, and structured rejection. Projection/reconstruction are exposed through export/import rather than separate row callbacks.

The original SDK export macro in `🔌️plugin/🦀️.rs:45490–45531` initializes the real plugin then calls `plugin_snapshot_sqlite_*`. The owned interpreter exports do the same at 45763/45775. The implementation at 42759–42813 resolves the component's actual Kernel native registry, routes native dialect ↔ SQLite through `io_run_with_snapshot_control`, and returns the file/native carrier. Thus each Guest's real registered `ArtifactCodec` must already have its individually authored provider. The SDK does not manufacture a schema or opaque blob fallback.

An authentic existing component owner is `🔌️plugin/🖥️host/🧪️testing/🧩️component/🧬️schema/📸️snapshot/🦀️.rs:18–19`, whose actual `Snapshot` exposes its SQL codec. Its sibling `🪶️sqlite/🦀️.rs:35–48` implements semantic `fixture_counter` projection/reconstruction against handwritten `🗄️.sql`. The actual component subset at `🧩️component/🦀️.rs:55` constructs `ArtifactCodec::bare::<Snapshot,Mutation>(KIND)` with its authored dialect and surfaces. This is a genuine compiled test component, not evidence that every Guest production owner has SQL.

## Real host chain

`🔌️plugin/🖥️host/🦀️.rs:1789–1800` dispatches the owned engine; 2824–2858 dispatches actual WIT exports under Wasmtime; 3026–3039 routes engines and refuses mock/recording SQLite calls. `🌉️mcp/🏠️workspace/🦀️.rs:3250–3282` compiles original component bytes, cross-checks its own pack hash, queries its own SQL declaration, installs `Guest { plugin_id, compiled.package_hash, schema }`, retains the compiled route, and publishes document binding plus exact native registration atomically. The provider's `snapshot_type: None` is appropriate for real Guest provenance, not a fake Rust type.

Workspace export 3185–3204 calls the original component, imports its physical file, and checks exact dialect/encoding metadata. Import 3207–3227 attaches real metadata, serializes the database, invokes the component reconstruction, and checks returned encoding. No SQL adapter is needed to establish this capability contract.

## Concrete current integration gaps

- SDK macro calls at 45508/45531 and owned exports 45763/45775 still pass only `input`; current functions 42767/42791 require original `IoRunControl` and `SqliteSnapshotControl`. Real component invocation must receive caller-owned operation authority and controls, not a fresh unlimited synthetic control.
- Workspace 3198/3221 still calls runtime export/import without the new trailing `EntityIdentityAuthority`; host methods 1794/1799, 2833/2845, 3031/3037 require it. The current Store SQLite provider callback receives SQL control only, so the original public IO owner must expose/forward its actual authority into this Guest bridge. Do not fabricate an actor or receipt to satisfy compilation.
- Wasmtime 2840/2852 accepts `identity` but directly uses `payload.to_vec()` and dialect `to_string()` without quoting that authority. An added argument alone does not fund original transport copies or validate ownership accounting.
- Existing compiled-component laws at host `🧪️tests/🔬️owned-instance-open/🪶️lease/🦀️.rs:16–69` and component SQL tests 117/121 also retain old signatures. They cover both carriers, cancellation and explicit rejection in source; no runtime qualification claimed here.

## Remaining verification

Run the original compiled component on both actual engines, through installed public IO rather than SDK registration only. Preserve authentic component hash/schema/dialect; test independent physical SQLite edits, malformed metadata and DDL, all positive limit axes, caller-owned cumulative native/SQL charges, cancellation after work and retained cleanup. Then enumerate all authentic Guest bindings before filtering SQL absence: this one existing fixture establishes a possible real route, not universal owner coverage.
