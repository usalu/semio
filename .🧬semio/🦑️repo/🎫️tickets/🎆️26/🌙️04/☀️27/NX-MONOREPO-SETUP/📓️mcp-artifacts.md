# OS MCP Artifact Restoration

The fixed debug executable restored from Nx cache with identical bytes and modes after its entire staged output directory was removed. The independent installed MCP SDK connected to the restored executable over stdio, verified server identity, and observed identical tools/list results twice. Compiler target state was not consulted by the consumer.

{
  "server": {
    "name": "semio-os-mcp",
    "version": "0.1.0"
  },
  "toolCount": 26,
  "artifacts": {
    ".nx-artifact.json": {
      "hash": "57683d01499ddc0a827764bf2765bd0ebcf6a423fc747108df8b3914a940c5ef",
      "mode": 420
    },
    "semio-os-mcp": {
      "hash": "152c4a5502e3abfda841ae2e4b3d698464685e47ceeec10879105e5726a1a77c",
      "mode": 493
    }
  }
}
