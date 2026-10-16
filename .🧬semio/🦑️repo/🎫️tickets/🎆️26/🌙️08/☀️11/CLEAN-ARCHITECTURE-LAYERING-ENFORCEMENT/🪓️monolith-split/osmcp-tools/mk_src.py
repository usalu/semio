import sys, os
sys.path.insert(0, ".")
sys.path.insert(0, "../hostmcp-tools")
import sbx
SCR = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/"
MCPREL = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp"
sbx.mk(SCR + "srcrepo", {MCPREL: SCR + "orig/🌉️mcp"})
print("srcrepo ready")
