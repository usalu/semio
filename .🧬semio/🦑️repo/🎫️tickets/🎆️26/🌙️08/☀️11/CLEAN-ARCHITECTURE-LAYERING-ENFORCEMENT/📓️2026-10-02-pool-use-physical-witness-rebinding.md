# Pool Use Physical Witness Rebinding

Read-only source evidence, 2026-10-02. No compiler/test jobs run. The reported portable assertion failures concern physical witness selection; this audit does not claim runtime success.

## Three original lower laws

General async owner `🧰️framework/🔨️modules/⏳️async/🦀️.rs` retains cfg(test) include mounts at2220 and2578. Actual test definitions are now in:

- `🧪️tests/🔬️native-pool-unit/🦀️.rs:20`: worker_pool_use_native_busy_keeps_executor_running_until_final_release.
- Same file61: worker_pool_use_acquire_and_shutdown_linearize_exactly_once.
- `🧪️tests/🔬️wasm-pool-cooperative/🦀️.rs:13`: worker_pool_use_cooperative_busy_keeps_executor_running_until_final_release.

The included files themselves declare `mod tests` at1 and `mod cooperative_tests` at1, preserving the registered native_pool::tests and wasm_pool::cooperative_tests qualified names. Lower permanent router209–255 correctly retains the original three native names, but its source marker check237 searches only the production root, which no longer contains their literal definitions. Read these exact mounted authored test owners for definition presence and separately assert both cfg(test) include mounts. Do not copy test bodies or search an unbounded tree to manufacture membership.

## Fourth retained activity state

DB engine `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs` has three `_pool_use: Arc<WorkerPoolUse>` declarations at325,1702,3163. The fourth is actual `DatabaseCreateCatalogState` at5364 with `pool_use: Mutex<Option<Arc<WorkerPoolUse>>>` at5366. Its typed try_prepare_with_use takes Arc<WorkerPoolUse> at6485 and constructs `pool_use: Mutex::new(Some(pool_use))` at6519. Terminal retirement6438–6440 first releases admission, takes the retained pool use, then publishes finished.

This is a concrete retained use with explicit terminal release, not absence of the fourth ownership state. The higher router1134 exact four identical textual declarations assumption is stale. Preserve four-state accounting as three immutable Arc declarations plus exactly one named create-catalog optional/mutex owner, and preserve its constructor/release witnesses. Database itself additionally owns an optional pool use at7555; count it separately rather than incorrectly treating it as the fourth retained capability/catalog state.

The higher DocumentMountSingleFlightCheckScript now contains the moved original five portable rows, all marker checks and original arithmetic/activity assertions. Rebind only their actual source witnesses; retain native three-law execution and the existing broader document-mount roster.
