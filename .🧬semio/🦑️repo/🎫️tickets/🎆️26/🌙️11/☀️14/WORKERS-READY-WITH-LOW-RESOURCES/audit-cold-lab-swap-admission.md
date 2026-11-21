# Cold Lab Swap Admission

The already running private lab passed native with an exit receipt and entered repo-mcp at 09:46:36 UTC on 2026-11-21. At 09:52:27 its private capacity sample reported 8192 MB simulated total, four cores, 6947 MB available, warning pressure, 16103 MB swap used and 40.04 MB/s swap traffic. Its queue held repo-mcp:install requiring 3327 MB for its minimum job, and the private grants directory was empty. The log named swap use above the ceiling as the reason for waiting despite available memory. A later sample reported 16229 MB swap used and 81.56 MB/s traffic.

The Darwin sampler obtains actual host vm_stat and sysctl swap counters, substitutes the declared host size and cores, then retains those swap readings. The admission decision derives the swap ceiling from the simulated total and enforces its hard swap condition before normal memory admission. Thus the observed reason is the host swap guard applied to the simulated host, not evidence of double borrowing between outer and inner ledgers. High free memory alone cannot bypass the hard condition. Progress depends on subsequent samples meeting the swap policy and the remaining gates. No gate, environment, mount or process was changed.

Source: 🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts lines 108–117, 154 and 197–208; sibling 📏️decision/🟦️.ts lines 201–216.
