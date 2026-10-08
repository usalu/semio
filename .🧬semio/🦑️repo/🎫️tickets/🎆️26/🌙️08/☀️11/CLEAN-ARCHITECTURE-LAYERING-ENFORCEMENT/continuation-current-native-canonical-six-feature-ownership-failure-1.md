# Current Native Canonical Six Feature Ownership Failure

A fresh Nx-orchestrated execution of the current General UI package's actual `test feature-ownership` command exited with status 1. Its six tests produced two passes and four failures. The output is temporarily held in `🗑️generated/current-native-canonical-6/feature-ownership.log`.

The source fixture still expects 73 test names, ten engine-owned names, original helpers, and exact body hashes. Independent Node hashing ran successfully and agreed with Bun's observed source hashes, but disagreed with five authored hashes:

| Law | Authored Hash | Current Hash |
| --- | --- | --- |
| `closing_input_reports_each_admitted_action_receipt_once` | `d62f01aeea53feb246b3e33c0c5c7bf15dba6186325eb0cd21e7254292ffc5fa` | `f853e0e61303c7847e557317d1153cec2dd3860fa2c015a8fb30290299c3fca0` |
| `an_enabled_shadow_pass_measures_every_caster_before_its_receivers` | `b7f1d2c424706b9097d1d8b58d8305782cd2452a57e86e95c0077a6673b6d2e6` | `1baf7aa76754ad05a032d912d30ad77a9e6a1267256dc6ec4bb44da2d1f757a7` |
| `a_procedural_grid_is_one_prepared_scalar_between_textures_and_opaque_geometry` | `a644a52837a1c0d37199dacb77f61fb3d239905ac454d745fa15493ee7452fcc` | `dabed9136ead496a739ad7a2116425b0b08b68aec287b4af2bb534de3f9e1cc1` |
| `a_disabled_shadow_never_publishes_a_gpu_shadow_scalar` | `49b3b5e62aba499b4884dcbdb80ce4fd9e479703106fcfaadafb1253ce6182ec` | `c8899499667b6802b0474e9b681301a465cc5455162963a2240939b70d3e0ea1` |
| `no_wgpu_target_paints_a_hand_written_colour_literal` | `e51ca9be3e47f14c6b9d3e8e179b6017d2190d9b12a0025819ea833f25fabdda` | `d0df0e535a75d33b0e094ebdd8e9482313ee46db7017566560375529abde5161` |

The theme group also fails because its authored helper `channels_are_hand_written` is absent from the current portable source. No expected hash or helper was rewritten to force a pass. An intentional current corpus revision requires assessing the source change and capability before updating its authority.

The General UI owning whole engine route is independently live and has reached actual Cargo compilation. Its owning route logs show the repository's `native owner-command` wrapper dispatching `test wgpu-engine long`. This separate progress does not discharge the feature ownership failures.

A static Bun build of the current UI package script succeeded with 17 input modules and no diagnostics. A combined static build of UI, repository Cargo owner, and Product renderer entrypoints failed because Playwright's bundled core references two absent optional `chromium-bidi` submodules. The combined build threw before a metafile receipt could be published. This is a static graph limitation, not an observed runtime native command failure, and not a complete external package custody witness.
