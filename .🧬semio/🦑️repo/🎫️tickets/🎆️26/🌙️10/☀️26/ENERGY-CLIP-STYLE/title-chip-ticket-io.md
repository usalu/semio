# Ticket File Mapping Refresh

Native ticket reopen repeatedly failed because Windows held a user-mapped section open. The ticket JSON was atomically refreshed with byte-identical content; no metadata field or state was edited. Repository MCP remains the only interface used to change ticket state.

SHA-256 of the unchanged bytes: 2d4647695b577f1e9e8ccb0f8c3b7befa56555d4716e354865daeb1a4b014770

The same byte-preserving atomic refresh was used before final MCP ticket closure; no state field was edited by the file operation.
