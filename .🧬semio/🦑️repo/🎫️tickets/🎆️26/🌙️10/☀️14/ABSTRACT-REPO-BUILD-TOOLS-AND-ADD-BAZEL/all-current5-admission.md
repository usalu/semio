# ALL5 Admission Snapshot

One read-only queue/descriptor/process snapshot was captured at 2026-10-15 00:07:02 UTC. It acquired no leases, opened no FactStore, probed no lock and controlled no peer.

The owned producer child 44462 was the sole exclusive global repository-graph-facts queue ticket and had its actual resource database open as FD16. It also had a shared graph-image ticket and FD19 on that database. The separate process 32359 held a shared graph-image ticket and actual FDs16/32; its command was repo:test-sdk-snapshot with prototype, skip-cache and exclude-dependencies flags. There was no exclusive graph-image ticket and no competing global-facts ticket. No catalog, graph-store or graph-charge database descriptor appeared in this snapshot.

The producer had reached global fact-writer admission. Its observed CPU was 49.4 percent and RSS 610416 KiB. This snapshot does not establish a peer exclusive blocker or a lease cycle. macOS lsof returned blank lock-mode fields; ticket-declared mode plus the actual descriptor are reported without claiming a separate kernel-mode measurement.

Evidence: 🗑️generated/all-current5-lease-snapshot.json. No accepted graph pin is implied.
