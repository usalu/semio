# Cargo fleet handoff during host admission

The Cargo front door checked per-unit fleet eligibility once. A unit could initially fit even while the larger Cargo request needed to queue. If unit pressure rose later, the Cargo process kept its old local queue position while the fleet was available.

The host capacity gate can now take an alternate continuation on an existing waiting progress tick. Cargo uses its existing fleet policy to select that continuation. The gate checks the caller's authority before and after the probe, uses the original cancellation signal, and removes only its own waiter before the fleet continuation launches Cargo. Once the local gate grants capacity, it runs the local body without probing again. A transient probe failure leaves normal host admission queued. The front door reads explicit offload mode with the same parser the Cargo kernel uses. The fleet continuation keeps the original arguments and the launcher's independently sized local compiler lease.

Nine Cargo capacity tests passed, including the shared JSON cases for explicit `auto`, `always`, and `never`, cancellation while the probe is active, and the host grant taking priority. The combined Cargo and host admission suites passed 40 tests. The neighboring fleet policy test and `git diff --check` passed.

The installed `repo` binary delegates to the repository's pinned runtime and front door. `SEMIO_REPO_SEAL=live` selects the live front door without promoting a snapshot or rebuilding the launcher. The installed launcher returned live-mode capacity status and Cargo help. The existing Shooting waiter was started before the change and keeps its original implementation. The fresh invocation from the workspace root is:

```sh
SEMIO_REPO_SEAL=live repo cargo --offload=auto --store=shared --configuration=prototype --parallelism=2 -- test --manifest-path '✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.toml' --workspace --lib --no-fail-fast
```

No live build or process cancellation was performed during this scheduler verification.
