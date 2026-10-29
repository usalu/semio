# Domain Host Native Test Doors

All three targets use existing packages and their actual artifact workspaces. No native execution is claimed here. Run sequentially in the single compiler lane after the shared SDK source is coherent.

Process:

```sh
repo cargo --store=shared --configuration=prototype -- test --manifest-path "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml" -p semio-s-artifact-process-process3d-editor --test original_host_configuration -- --nocapture
```

Cad:

```sh
repo cargo --store=shared --configuration=prototype -- test --manifest-path "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml" -p semio-s-artifact-cad-cad-editor --test original_host_configuration -- --nocapture
```

Sourcing:

```sh
repo cargo --store=shared --configuration=prototype -- test --manifest-path "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/Cargo.toml" -p semio-s-artifact-sourcing-curation --test original_host_configuration -- --nocapture
```
