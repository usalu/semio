# Agent Lane Preview Proof Audit

The final `live-v1-21.log` run failed the agent-lane test with `mounted tool metadata lease must return through its original close`. A separate voxel-dimension test failed because fixture preparation receipt progress exceeded the original caller grant.

The retained v1 binary reproduced the agent-lane failure alone with `RUST_MIN_STACK=67108864` and `RUST_BACKTRACE=1`. The backtrace identifies the dropped ownership chain as `QualifiedToolProof` → `DeclaredToolRegistrationAlias` → `DeclaredToolCatalogAlias`. Its immediate caller is `CommandCtx::begin_agent_lane_preview`, reached through `preview_addressed_action`, `preview_fixture_on_original`, and `declared_verb_agent_dispatch`.

In `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs`, `begin_agent_lane_preview` consumes `AdmittedToolCommand`, reads its proof's key, contract and schema ID, then installs the request without transferring or explicitly retiring the proof. Its scope exit drops that proof and its still-live lease. Early error exits also require paid ownership handling. The preview must retain and close the original proof under the caller's lifecycle grant, including error paths. The trace does not establish which exit path was taken in this particular invocation.

The focused native test failed as expected; no source or binary was modified and no rebuild was run.

Both preview routes now finish typed preparation while the ingress record retains its own residual. Once the typed source is ready, the next preview call uses that source directly. Pipeline validation, root capture, revision validation and fallible snapshot reads borrow the command and admission in place. A refused handoff preserves both owners for the original source close.

The preview request receives the funded typed command and raw wire together. The original typed source retains the admission verb, qualified proof and wire admission until its existing controlled close returns them. The request is installed before the context birth receipt can return an error, so that error leaves all transferred owners reachable by normal teardown. The separate declared-action and declared-command routes share this handoff.

The original failure is reproduced by the native declared-verb agent dispatch law. The fix is included in the combined native rebuild.
