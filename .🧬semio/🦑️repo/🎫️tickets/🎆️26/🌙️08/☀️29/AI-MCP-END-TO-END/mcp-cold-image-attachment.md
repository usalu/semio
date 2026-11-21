# Cold prepared-image attachment

The regression runs the native catalog reader with the same closed model manifest used by the launcher. A fresh process reads the model, compiles the actual Owned component and checks the image extent and SHA-256 before opening the image codec. A changed byte leaves the extent unchanged but produces `PluginUnavailable` and leaves the shared cache empty. A retry with the exact retained image reports restore progress and publishes the verified handle.

The fixture's ordinary Wasm start leaves bytes `[17, 53]` at the actual channel pointer. Its preparation constructor changes those bytes to `[50, 53]`, the canonical channel version 25. The fresh Source-bound native startup becomes ready using those bytes and exactly two measured instructions for the real `ChannelVersion` export. It then retires the startup and caller under their original receipts. The independent Bun Wasm/Ajv test produced both states and rejected the altered image digest in 12 assertions.

The direct pinned host test rebuilt the current interpreter, runtime, guest host, host facade and catalog from source against retained dependencies. Its linked smoke passed the cold attachment test and the earlier publication authority regression. The test used a private subprocess for model environment custody and kept generated outputs under this ticket. No Cargo or shared target writer was used.

The refreshed run included the current native unit escrow reader. It passed startup custody 1/1 and the two cache tests 2/2. The existing independent unit escrow suite passed 3/3 with 13 assertions. The full runtime Cargo unit suite was outside the direct run. The actual descriptor emitter source passed the separate pinned metadata check against the refreshed runtime.
