# job-receiving admission contract: caller migration

Each owner now holds its own `construction: OriginalConstructionAdmission`, passes it to `built`/`built_with`/child `admit`, adopts it via `adopted`, retires it with `unbuilt`/`close_step` in close, and includes it in `terminal_is_empty`.

## Edited (all under 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin)
- 👷️job/📨️emission/🏛️host (semio-plugin-host-configuration): `OriginalHostConfiguration::admit` takes `admission`; advance/take_into use the resident admission.
- 👷️job/🏛️reserved/🪆️child-vcs (+📌️checkpoint, 🪆️cascade): unit, checkpoint unit, cascade.
- 👷️job/🏛️reserved/📌️checkpoint: `OriginalCheckpointRoute`; one `construction` passed to every child admit.
- 👷️job/📨️emission/🎟️capture: `OriginalEmissionSource` (resident), `OriginalEmissionCapture::admit(cx,admission,output)`, `admit_original_emission_receipt(op,cx,admission,output)`.
- 👷️job/🦀️.rs: re-exports `adopted`, `unbuilt`; public `original_steps`.
- ⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest (+metadata.rs): command, receiving, metadata own admissions; `admit_original_construction` 4-arg calls moved to `built`.
- ⚛️reactor/🔌️app: `InstanceBirth` carries the admission (uses `original_steps::unit`); opening/cleanup-pumps hold `birth_construction`, closed in the opening close path.

## Verification
`check -p semio-plugin-job-child-vcs/-host-configuration/-job/-reactor-app --lib` compile (job-receiving itself had a transient cfg-owned-item source error from another agent, later gone).

## Open
- semio-plugin-plugin-program (⚛️reactor/🪟️surfaces/read.rs, native.rs): 121 errors, mostly unrelated framework API drift plus 4-arg `admit_original_construction`; not migrated, blocks runtime-ops check.
- Callers still passing the old signatures: `OriginalEmissionCapture::admit` and `admit_original_emission_receipt` users (🧵️retained-command emission `OriginalEmissionAdmission`, 🧵️instance 📤️emit/🎟️original), `OriginalHostConfiguration::admit` users in tool-machine (other agent), window-view/intent/response.rs in runtime-ops. Their owners need a `construction` field.
- Tests not run.

## Round 2 (emission, runtime-ops, tools)
- 👷️job/📨️emission/🎟️capture: `OriginalEmissionCapture::admit(cx,admission,output)`, `admit_original_emission_receipt(op,cx,admission,output)`, resident `construction` in `OriginalEmissionSource`.
- 🧵️retained-command/🔄️full-operation (+📥️emission): `OriginalEmissionAdmission` owns `pub construction`; `admit(cx,admission,output)`; `admit_original_reserved(...,cx,admission,output)` via `built`; decoder identity migrated; lease close via `retired_poll` and `lease_retirement_step` effect; local `built` removed in favour of `semio_plugin_job::original_steps`. `check -p semio-plugin-retained-command --lib`: no errors remain in 📥️emission; 4 errors remain in other files (mounted/🦀️.rs:108, mounted/♻️frontier:76/81).
- 🧵️instance: tools gain `original_emission_construction` (inventory field 38, test clock now 39, close ladder + maintenance terminal), `MediaTransfers.reserved_mount_construction` (close handled in plugin-app-impl); `capture_child_content` helper (original_emission) stores `SharedOriginalStorage<ChildContentView>` via `funded` + `admit_from`; used by both phase-6 sites. Not compiled (instance-dispatch blocked upstream).
- ⚛️reactor/🏃️runtime (runtime-ops): local `built`/`built_with` replaced by `original_steps` re-export; intent, response, window-view, metadata, manifest command/receiving hold admissions; surface refresh/capture admits pass `construction`; tests (metadata-tests, original-view-read, response-tests) carry per-slot admissions. Blocked from verification by reactor-lifecycle publication/🦀️.rs:56 (unrelated API drift).
- ⚛️reactor/🔄️turn/📥️receiving: `RetainedResponse`/`RetainedIntents` own an admission, closed before witness return.
- host-configuration tests: grants from the settled quote, admitted decoder, multi-turn admit, retired helper. Still failing at decoder teardown (`decoder retains receiving stock`, tests 🦀️.rs:23); cause not yet found.
- Tool-machine users of `OriginalHostConfiguration::admit` left to the tool-machine agent.
