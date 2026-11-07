# Native MSVC Acquisition And Extraction Review

## Actionable findings

- **Failure while capturing output ownership leaves extracted files behind.** main_windows.go:47-59 installs its cleanup defer only after captureOwnedTree succeeds. A walk or stat error at line 51 returns after compiler extraction without cleaning its outputs. At lines 60-68, a failed second capture leaves the existing defer holding only the compiler snapshot, so SDK outputs also survive. Capture failure can be induced by a concurrent directory change or filesystem error. Return and accumulate the creation-time ownership records from both extraction operations, and install cleanup before later fallible operations; exercise both capture failures.

- **Acquisition can claim and delete a replacement file.** acquire.go:43-61 obtains the actual created file identity but discards it on success. The enclosing get re-reads the pathname at lines 123-127 and records whatever object is there. If it is replaced between close and that Lstat, a later failure removes the replacement as if owned. Return the created handle identity from acquirePayload and verify path identity before recording it; test replacement in that gap.

- **CAB output creation has a pathname race.** cabinet_windows.go:149-161 checks target absence and then asks SetupAPI to create it by pathname, without exclusive creation or a held parent directory. Another actor can create or redirect a target between the check and SetupAPI opening it. Lines 186-194 subsequently record the observed pathname identity rather than an identity captured at creation. The implementation therefore does not establish exclusive output creation or identity-safe cleanup under concurrent modification. Use an extraction interface with exclusive creation/held directory custody or explicitly establish and enforce exclusive stage custody; add an adversarial race test.

## Scope And Evidence

Read-only source review of the Go acquire, request, recipe, CLI, archive, cabinet, cleanup, locking and preflight paths. No new tests or live acquisition were run. The already resolved final-callback cancellation, disconnected MSI root, system DLL and superscript-device cases were not reopened. Full live acquisition, publisher source binding, stage publication and canonical consumer verification remain unestablished as described in the acquisition checkpoint. The race findings require concurrent mutation of the allocated directories; this review did not demonstrate such mutation on this host.
