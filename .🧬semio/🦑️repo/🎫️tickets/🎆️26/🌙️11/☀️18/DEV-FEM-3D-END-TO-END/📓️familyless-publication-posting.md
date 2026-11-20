# Familyless publication posting (fem3d `setActiveExample`, seq 7)

## Symptom
`original posted publication receipt lacks its unique family or actual recipient binding`:
`families=[]`, one posting of 1 item / 112 copy bytes / 0 capacity inside
`advance_typed_operation_publication`.

## Culprit
`🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs`,
`publish_mounted_typed_operation_pump`, the generic `else` arm (about line 88-95).
A unit that returns without `publication_ownership_progress` (`receipt == None`) only
called `cx.settle_receiving_scope()`. Several units post straight into `cx` and return
no receipt, so their posting never reached `spent`, hence no preparation family:

- `admit_actor_capture` (`🧵️retained-command/🔄️full-operation/🦀️.rs:256`): `receive_original_retained_work`
  posts the actor capture (1 item, copy bytes only) whenever the emit carries artifact, config,
  draft or window-config mutations. `setActiveExample` is such an action. This is the 1 item / 112 B / 0 capacity posting.
- Same shape in the sibling unit steps of `publish_mounted_typed_operation_unit`
  (`completion.take()`, tool settlement stage, stream abort generation): each `consume_retained`s and returns `Ok(())`.

The `owned_child_route` arm and the `receipt == Some` arm already handled this (delta added to
`spent`, or `original_lifecycle_unposted` consumed); only the receipt-less arm leaked.

## Fix
In the receipt-less arm, after `settle_receiving_scope()`, add the unit's direct posting
(`original_lifecycle_delta(original_before, cx.retained_progress())`) to `spent`. The pump already
records `spent` as the operation's `publication_ownership_progress` (preparation family) with the
turn grant, so the window now carries exactly one family whose progress equals all postings.
`collect_original_publication_receipts` is unchanged.

## Regression test
`🧪️tests/📤️original-emission/🦀️.rs`: around every real `advance_typed_operation_publication`
the test measures the retained delta and asserts it is zero or equal to exactly one
operation family receipt (preparation xor retirement), with `progress == posted`.

## Verification
See the final report of the run (cargo test results recorded in chat).

## Test results
- `cargo check -p semio-plugin-instance-app`: ok (the crate holding the fix).
- `cargo test -p semio-plugin-command`: 15 passed, 0 failed.
- `semio-plugin-tests-16` (hosts the original-emission regression test): does not compile, because of unrelated errors in
  `🔬️plugin-runtime-plugin-builder-contract-part-3` (`PeerPresenceRootRetirement` missing, undefined `operations`, `native_pages`, `issued`).
  The new assertion in `📤️original-emission` therefore has NOT been run.
- Not run: reactor receipt tests, fem component `cargo check`.
- `C:/b/fem3d-fam` deleted.
