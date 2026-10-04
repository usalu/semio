# Additional Unmounted Assembly Lifetime Negative Clients

Two immutable compiler clients extend the original26scenario proposal at the lifetime frontier without editing its rows. Native should combine each same client with exact original and exact current definitions plus the previously explicit adjacent data stubs. Expected before accepts/after refuses are pending actual compiler evidence.

## implicit-scope

Client `🧫️assembly-lifetime-native-inputs/🧫️implicit-scope-client.rs`, SHA-256 `8f8c361207b25a7f6c62b50fb6c2e45ae765c90f644e8b75cf887b585fda7885`. Expected current compiler code `E0597`.

```rust
fn main() {
    let guards;
    {
        let transaction = begin_artifact_assembly().unwrap();
        guards = acquire_artifact_assembly_store_registry_guards(&transaction).unwrap();
    }
    drop(guards);
}

```

## escaping-return

Client `🧫️assembly-lifetime-native-inputs/🧫️escaping-return-client.rs`, SHA-256 `145dda95aa70748da48447dd06bb9570068f5e8d684294ca2fdd6e7ee4bc7d5a`. Expected current compiler code `E0515`.

```rust
fn escape_guard() -> impl Sized + 'static {
    let transaction = begin_artifact_assembly().unwrap();
    acquire_artifact_assembly_store_registry_guards(&transaction).unwrap()
}
fn main() {
    drop(escape_guard());
}

```

The implicit client observes automatic transaction destruction at an inner scope boundary. The escaping-return client returns an opaque static guard from a function that owns its local transaction; it avoids adding lifetime generic arguments which would make the original API fail merely on type arity. Public opaque impl Sized is native std language syntax, not a foreign crate API. No compilation/runtime was performed by this author.
