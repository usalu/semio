# Ticket Lifecycle Transport

The final incremental UTF-8 transport probe passed initialization and discovered the actual repository ticket_close tool before closure.

The retained MCP client decodes stdout incrementally as UTF-8, preserving Unicode across process-pipe chunk boundaries. The final pre-close probe repeats initialization and tool discovery with that transport.

The repository MCP ticket-close API accepts the complete changed-file list. This ticket's retained move manifests exceed the normal one-megabyte transport limit. A ticket-private Go build overlay raises only the transport's payload limit to 32 MiB; repository source files and lifecycle handlers remain unchanged. The retained input is `🧑‍💻coordination/🔌️ticket-mcp/📜️script.ts`.

The actual MCP binary built successfully through Bun/Nx. A protocol initialization and tools-list probe confirmed that its repository ticket lifecycle tools are available. This preparation does not close the ticket; closure follows final validation and records the complete authored manifest.
