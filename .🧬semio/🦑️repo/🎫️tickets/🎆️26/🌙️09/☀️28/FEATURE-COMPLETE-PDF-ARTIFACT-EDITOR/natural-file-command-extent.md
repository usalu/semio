# Natural File Command Extent

The browser and native shells submit natural-file opens as one encoded MediaIn command. The shared actor and native kernel admit at most 8,388,608 encoded bytes including the sequence, port, descriptor, and payload length. The browser previously read against 512 MiB, and the native reader used a 32 MiB limit intended for segmented export. Both could read an entire file that command ingress could never admit.

Each shell now measures its own encoded PDF request with an empty payload and reserves the largest permitted sequence and length prefixes before allocating or reading the file. For the registered PDF 1.7 descriptor the resulting conservative limits are 8,388,482 payload bytes in Chromium and 8,388,480 in native WGPU. The two-byte difference comes from the encoded sequence width. The existing command ingress still checks the exact completed command.

Ajv validated the cross-language fixture. SQLite independently calculated the same command allowances using unsigned base-128 lengths. The browser's real bounded reader rejected one byte over budget before asking the Blob for a slice and preserved admitted bytes and progress. The governed Vitest suite passed all 10 cases. Two focused native laws linked the repository's real SPR encoder, Value and Kernel code and verified the encoded allowance plus file read, oversize refusal and cancellation.

Natural PDF import above the assembled command ceiling would require a segmented intake path. The existing streamed original-image chooser carries a different guest-issued loan and does not accept natural PDF input.

- [Browser transfer tests](🗑️generated/pdf-open-wire-limit-vitest.log)
- [Native reader and wire tests](🗑️generated/pdf-open-native-wire-limit-green.log)
- [Browser cancellation verification](browser-natural-file-cancellation.md)
- [Native picker ownership verification](native-picker-ownership.md)

Native Open now observes its cancellation token while the guest import request is in flight. Cancelling drops the kernel request future through its existing abandonment path. The kernel retires the exact command on its closing lane, and the fresh program owner follows the existing instance close path without publishing a document. The focused Rust executable passed three cases, including an observed drop on cancellation, registered PDF descriptor encoding, and the bounded reader. The simulated pending-request case exercises the live shell guard. Mounted native cancellation and reopen still require the completed application run.

- [Native cancellation test](🗑️generated/pdf-open-native-cancellation-green.log)
