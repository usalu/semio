# Proctor process sample

One normal 768 MiB capacity-run diagnostic (unified session 17822) exited inner and outer 0. Its exact command lines, images, creation times and both samples are retained under `🗑️generated/nov07-proctor-sample.log`; the input is `nov07-proctor-sample.ps1`. It selected exactly one Bun command with the proctor target and repository identity.

Its process tree at the first sample was Bun 20508 → Bun 24824 → Bun 25516. All three retained PID, creation time and image at the second sample ten seconds later. Over that interval their CPU-time increases were respectively 0, 0.046875 and 8.734375 seconds; read-transfer increases were 2,725, 2,586 and 8,343,145 bytes. These measurements show activity in that child during this interval; they do not prove overall progress, causation, or a hang. PID 24824 was the sampled proctor child, not established as an orphan.
