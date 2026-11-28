# 🧩 Design: Framework Image + Thin Plugin Module

## Problem

Every plugin component is one `wasm32-wasip2` cdylib of 130-450 MB (puzzle about 230 MB, process about 400 MB core). A
plugin-only edit relinks the whole framework closure (wasm-ld over every rlib, 3-6 GiB peak, minutes under the
capacity gate), transpiles the whole core module and recompiles it in the browser. Measurements are in
`measurements.md`.

## Why not a shared-nothing component boundary

The plugin code is not a client of a small framework API. It instantiates framework generics for its own types
(`derive(Mutations)`, retirement frontiers, receiving-frame drivers: see the MIT-BESTAND `🎈️component-bloat.md`
analysis, about 750k functions, most of them per-type instances). A WIT boundary cannot carry Rust generics, so a
`framework` component with its own memory would have to re-export the whole Rust API as WIT and the plugin would
still contain the instances. Rejected.

## Chosen boundary: shared-everything split at the core-module level

One component per plugin stays the runtime contract (`semio:framework/actor` world, `📜️.wit`). Its core code is
produced from two separately built parts:

| part | what it holds | built when | cache key |
|---|---|---|---|
| **framework image** | std, wasi-libc, every framework rlib of the plugin's closure, the allocator shim, the wit-bindgen import glue; exports exactly the symbols the plugin module imports (functions and data addresses), its table and its stack pointer | framework rlibs change, or the plugin imports a symbol the image does not export | sha256 of toolchain and every framework rlib (path, size, mtime); the export set only grows per key |
| **plugin module** | only the object files of the plugin's own crates (instances of framework generics for plugin types included), statically linked by `wasm-ld --no-entry --import-memory --import-table --allow-undefined --emit-relocs --gc-sections --global-base=1024 --table-base=<image table size>` with the root crate's unmangled symbols as exports | every plugin edit | its own bytes |

Both parts come from one cargo invocation, `cargo rustc --manifest-path <component> --crate-type rlib`: the root is an rlib, so
cargo compiles the closure with object code (the cdylib lane's thin LTO makes every dependency bitcode-only and moves all
framework codegen into the final link, see `measurements.md`). No relocation-model or profile change is needed, so the
repository compiler policy stays as it is.

The framework image is a `rustc --crate-type cdylib` of a two-line root (`extern crate` of the component's direct
dependencies) against the same rlibs, linked with `--export=@exports.rsp --export-table --growable-table
--global-base=<64 MiB reservation> --skip-wit-component`. Because it is the same rlibs, every mangled name the plugin
module imports exists in the image with the same crate disambiguator.

The link step (`semio-native-transpile link`) appends the plugin module to the framework image:

1. Framework functions, globals, element and data segments are copied byte for byte; framework code is never
   re-encoded, so the step is linear in the plugin size only.
2. Plugin `env` function imports bind to the image's exports by symbol name.
3. Rust's wasm codegen reaches data of other crates through `GOT.data.internal.<symbol>` globals; `wasm-ld` leaves the
   unresolved ones at 0. Each becomes the image's exported address of that symbol. A direct memory relocation against
   an undefined data symbol (found through `--emit-relocs`) is refused instead of linked silently.
4. The plugin's `__stack_pointer` becomes the image's; its data must lie in `[1024, image data start)` and its table
   entries at or after the image table, both checked.
5. `_initialize` becomes a synthesized function: image `_initialize`, then the plugin's `__wasm_call_ctors`.
6. The result is validated (`wasmparser`) and componentized (`wit-component`, as `wasm-component-ld` does), then goes
   through the unchanged transpile, descriptor and publication path.

The first design (spike, below) used a position-independent plugin module (`-shared --experimental-pic`). It works, but
it needs `-C relocation-model=pic` for the plugin crates, which the repository's closed compiler policy does not offer
(`repo cargo` refuses caller `--config`); the static plugin module needs no flag change.

## Contract (schema-first)

`🧬️schema/🔣️.json` of the link command describes the three documents:

- `FrameworkImage`: key, image path, exported symbol set, memory reservation (`[1024, reserveEnd)`).
- `PluginModule`: path, imported symbol set (functions and GOT data).
- `SplitLinkReceipt`: sizes, layout (plugin data range, table range, GOT data count), durations.

The reuse rule is a pure function of the two documents: reuse when `PluginModule.imports ⊆ FrameworkImage.exports`
and the plugin's data fits the reservation; otherwise relink the image with the union of exports (the export
set only grows, so it converges after the first edits).

## Spike evidence (2026-11-27, `🗑️generated/spike`, removed at close)

PIC variant first, then the static variant: a non-PIC plugin reading `fw::TABLE` links statically to 18,146 B with
`linking`/`reloc.*` kept; the reference is `global.get $GOT.data.internal.fw::TABLE` with initializer 0 and no direct
memory relocation against the undefined symbol.


- `fw` (framework stand-in) + `plug` (plugin, generic `fw::Store<Mine>`, a mutable static, a closure through
  `&dyn Fn`) built with `-C relocation-model=pic` for `wasm32-wasip2`.
- Plugin module from the plugin rlib only: 13,865 B, imports `env.memory`, `env.__indirect_function_table`,
  `env.__stack_pointer`, `env.__memory_base`, `env.__table_base` and 14 functions (`fw::greet`, `fw::apply`, alloc
  shims, `core::panicking`); `fw::Store<plug::Mine>::push` is defined in the plugin.
- Framework image (`img` cdylib, `--export=@exports.rsp --export-table --growable-table --global-base=1048576
  --skip-wit-component`): 110,898 B, exports the 14 symbols with identical mangled names.
- Merge: 118,052 B in 3 ms, validates; node runs `plugin_run()` = 29 then 31, the same as the monolithic semantics.

## Not covered by this boundary

- The plugin crates' own codegen (rustc on plugin crates) stays on the edit path.
- The browser still compiles the merged core module; splitting it into two core modules inside the component (so
  the framework part hits the browser code cache) is a follow-up that needs a component encoder for two modules.
