# WFC Component Activation and Cancellation

The native gateway `b55d45cc16540e3d49c9c0228fc2c201e62489eab5c6dbc80c541b93e2f9a8d3` accepted the genuine folder-seeded WFC job and recorded cancellation 132 ms after acceptance. With a longer SDK observation window, it recorded terminal cancellation 51.166 seconds after the request. The SDK ping succeeded and the independent native storage reader verified identical Pack and SPR bytes.

Two four-second samples covered the disposable gateway after the SDK observed cancellation request acknowledgment. All 1,441 and 1,428 samples from the inference worker were within `RoutingArtifactChannel::resolved_route`, `open_plugin_artifact_channel_scoped` and `scoped_component_hash`. Of those, 1,283 and 1,266 passed through the framework's BLAKE3 update. The 0.35 progress message preceded route resolution. The samples locate the job in first component activation.

The inference command now shares the job token with the component activation scope. A cold hash reports the bytes it has completed and checks cancellation after each 4 MiB chunk. Component cache access and preparation keep their cancellation checkpoints. The inference route reports the opening phase and checks cancellation again after origin registration, in the branch that retires the origin.

The retained SDK test reads [the neutral cancellation fixture](wfc-activation-cancel.json), waits for actual hash progress, sends cancellation for the accepted handle, and checks the terminal job and exact durable pair. It cross-checks the terminal projection with Ajv.

The updated native gateway `9603c9ffcfe0a44222d1be0fc799fae33cf4fa7ccf745e26bcef8918f2707e4b` reported hash progress at `0.2752348331321686`. The job was running when the SDK requested cancellation, and recorded terminal `cancelled` 72 ms later. Ajv verified the terminal projection, the SDK ping succeeded, and the native storage reader again verified the exact 405-byte Pack and 211-byte SPR. The live solve and cancellation during the subsequent guest computation still require independent verification.
