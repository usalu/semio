# MCP Source Escrow Guard Qualification

The native Rust regression `header_guard::tests::full_escrow_reclaims_one_acknowledged_row_before_minting_its_guard` passed on November 21, 2026. The Rust runner reported one test passed, zero failed, and zero filtered.

| Acknowledged rows of 32 | Guard admitted | Items charged | Work charged | Heap bytes requested and released |
| ---: | :--- | ---: | ---: | :--- |
| 0 | No | 0 | 0 | 0 and 0 |
| 1 | Yes | 2 | 2 | 0 and 0 |
| 32 | Yes | 2 | 2 | 0 and 0 |

The fixture binds a genuine Source and admits its Core wire delivery and escrow publisher before filling the actual escrow table. It samples the live retained-item and work counters immediately around guard preparation. The production guard asks the Source for one row before taking its mint snapshot. With an acknowledged row, the Source reclaims one row and charges one item and one work unit. The guard then charges one item and one work unit for minting. The fixture caller links the repository's global heap witness and checks its own observed admission allocation receipts before the guard measurement. The test releases any minted permit before closing each Source.

The pinned native fallback ran `mcp-prepared-startup-check.ts --guard-reclaim-test` against the compatible dependency artifacts in `native-host`. It rebuilt Job Context with its `testkit` feature, Native Turn, Fixture Caller, and the Return owner's test binary from the live source. The original Cargo command could not activate the Return owner's dev dependencies through the current root workspace membership. The linked build puts its outputs in `🗑️generated/guard-rust` and uses the existing native-host outputs as read-only inputs. This run qualifies the Rust guard regression. The parent task tracks MCP SDK and transport verification.
