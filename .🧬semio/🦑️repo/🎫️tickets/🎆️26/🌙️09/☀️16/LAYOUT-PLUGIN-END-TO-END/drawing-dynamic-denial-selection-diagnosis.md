# Dynamic Denial Selection Diagnosis

Core6 actual evidence is two direct work1-versus0 failures in denied checks. The retained log does not print case/axis/demand, so exact first runtime tuple is not claimed yet. No production change or law weakening has been applied.

Defining Source gives a concrete matching counterexample. Dynamic selected-close law uses the real string selected, including cancellation after seven clone turns. That leaves five bytes in the genuine partial Vec<u8> (bind, reserve, five byte writes). Its Collection<u8> close with body4096 selects payload copy5/capacity0/release0 at the current nested depthD. The test's copy denial changes maximum_copy_bytes to4, then calls unqualified cursor.close_step(grant). Dynamic close_step recomputes close_demands(grant.maximum_copy_bytes); Collection.next_selected_work_byte_demand(4) selects copy4, not copy5. Its next_birth_bytes(4) remains0. Therefore the actual lower event is fully granted copy4/capacity0/release0/depthD, truncates four original bytes, reports actual work1, and allocates/releases0. Work1 is genuine and must not be changed to0.

The same mechanism exists in the full law's text partial cancellation:for close body3 and at least three pending u8 elements, original selected copy3 changes to actual selected copy2 when the test supplies maximum_copy_bytes2. Candidate first fixture is variant9/copy3/cancel8, but this is Source-derived, not a retained per-case runtime observation.

Relevant defining paths:
- retained-clone/🌱️dynamic/🦀️.rs:close_step calls close_demands(grant.maximum_copy_bytes), then forwards the admitted grant and exact child work.
- retained-clone/♻️close/🦀️.rs:step_granted likewise queries the actual incoming body before child invocation.
- retirement/🦀️.rs:Collection.next_selected_work_byte_demand selects floor(body/element-width) bounded by original len; its close_step truncates that actual run.
- selected close fixture requests bodies0/1/4096 and cuts0/1/3/7.
- retirement/🎟️tickets/🎯️selection/🦀️.rs:existing RetirementSelection retains original body and quote, preflights ALL selected axes, verifies exact-copy fixed point, normalizes to the selected demand and invokes only afterward. This is the correct semantic model for a denied selected event law.
- RetainedCloneCursor currently exposes only unqualified close_step(grant), not that qualified selection door.

No weakening to actual adaptation is justified:copy4 is a legitimate new selected body. No test-local wrapper may fabricate work0. Genuine selected five-axis denial needs an actual first-party qualified invocation which keeps the intended body fixed while checking a narrower grant. A bounded option is a canonical RetainedCloneCursor close_selected(body,grant) method/default that borrows the fresh same-owner demand for body, returns neutral work0 before invocation on any axis refusal, verifies quote at its exact copy frontier, then forwards the normalized selected grant to the actual owner. This mirrors existing first-party RetirementSelection without allocating, changing stocks, inferring work or adding a compatibility unwrap. Both denied and admitted law calls must use the same qualified door. Unqualified adaptive close_step remains the lower operation, not a false denial oracle.

Root review is required before choosing this interface or changing callers. If exact first runtime identification is required first, add only case/axis/original-vs-fresh-demand diagnostics to the unchanged work0 assertions and let Root coordinate the next genuine owning run. No extra runtime has been launched here.
