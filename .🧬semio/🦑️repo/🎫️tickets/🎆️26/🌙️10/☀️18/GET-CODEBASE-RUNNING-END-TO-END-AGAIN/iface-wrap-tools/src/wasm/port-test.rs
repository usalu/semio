//! 🔌️ The wasmtime port: the re-exports name the pinned crate's own types, an empty component loads and instantiates through them alone, and a WASI guest context builds. See [`super`].

use crate::wasm;

#[test]
fn the_port_names_the_pinned_crates_own_types() {
    let engine: wasm::Engine = wasmtime::Engine::default();
    let config: wasmtime::Config = wasm::Config::new();
    let table: wasm::component::ResourceTable = wasmtime::component::ResourceTable::new();
    let store: wasmtime::Store<()> = wasm::Store::new(&engine, ());
    drop((config, table, store));
}

#[test]
fn an_empty_component_instantiates_through_the_port_alone() {
    let mut config = wasm::Config::new();
    config.wasm_component_model(true);
    let engine = wasm::Engine::new(&config).expect("an engine builds from a port config");
    let component = wasm::component::Component::new(&engine, crate::minimal_component_without_actor_world()).expect("the empty component parses");
    let linker = wasm::component::Linker::<()>::new(&engine);
    let mut store = wasm::Store::new(&engine, ());
    linker.instantiate(&mut store, &component).expect("the empty component instantiates");
}

#[test]
fn a_wasi_guest_context_and_its_diagnostics_pipe_build_through_the_port() {
    let diagnostics = wasm::wasi::p2::pipe::MemoryOutputPipe::new(16);
    let _context: wasm::wasi::WasiCtx = wasm::wasi::WasiCtxBuilder::new().stderr(diagnostics.clone()).build();
    assert!(diagnostics.contents().is_empty());
}
