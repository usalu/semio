# Cold Modern Protocol Review

The native server accepts the modern stateless `server/discover` opener. The real-binary end-to-end helper uses exactly that opener without the legacy `initialize` sequence.

The cold TypeScript gateway previously required both `initialize` and `notifications/initialized` before spawning its staged native server. A discover-first client would therefore remain queued even after compilation succeeded. The launch executor owns the neutral fixture and gateway correction, preserving the actual native discovery response and keeping legacy initialization ordering intact.

The shared binary producer now supplies the same owner manifest, package, executable and source record to both the native build target and cold startup through the original admitted build interface. It forwards the actual `scriptInvocationEnvironment`; a plain environment lacks the required parent receipt. Bare readiness isolates this binary publication from catalog generation, while the full configured readiness run still remains required.

Ticket bookkeeping note: changing the ticket title through `ticket_reopen` also changed its folder slug. The original title and `AI-MCP-END-TO-END` path were restored immediately through the same MCP tool. Existing reports and the generated directory remain present. The tool retained the existing repository goal metadata; no repository goal was opened, closed or reopened.
