"""Group leftovers: redundant IDE artifacts that no build, test or ticket reads."""
import os, re
from blib import *

g = "leftovers"
ED(g, ".claude/settings.json", '    "csharp-lsp@claude-plugins-official": true,\n    "gopls-lsp@claude-plugins-official": true,\n    "mcp-apps@mcp-apps": true,\n    "pyright-lsp@claude-plugins-official": true,\n    "rust-analyzer-lsp@claude-plugins-official": true,\n    "typescript-lsp@claude-plugins-official": true\n', '    "mcp-apps@mcp-apps": true\n', (1,), "leftovers-claude")
DELETE(g, ".github/hooks/repo.json", "leftovers-github-hooks"); RMDIR(g, ".github/hooks")

ED(g, ".gitignore", ".idea\n*.suo\n*.ntvs*\n*.njsproj\n", "", (1,), "leftovers-gitignore")
ED(g, ".gitignore", "#--------------------------------------VISUALSTUDIO--------------------------------------\n*.rsuser\n*.suo\n*.user\n*.userosscache\n*.sln.docstates\n*.userprefs\n",
   "#--------------------------------------NATIVE-BUILD-OUTPUT--------------------------------------\n", (1,), "leftovers-gitignore")
ED(g, ".gitignore", "[Ll]ogs/\n.vs/\nGenerated\\ Files/\n", "[Ll]ogs/\nGenerated\\ Files/\n", (1,), "leftovers-gitignore")
ED(g, ".gitignore", "*.vspscc\n*.vssscc\n", "", (1,), "leftovers-gitignore")
ED(g, ".gitignore", "*.vsp\n*.vspx\n", "", (1,), "leftovers-gitignore")
ED(g, ".gitignore", ".mfractor/\n.localhistory/\n.vshistory/\n", "", (1,), "leftovers-gitignore")
ED(g, ".gitignore", ".ionide/\n", "", (1,), "leftovers-gitignore")

import subprocess
tracked = subprocess.run(["git", "-C", ROOT, "ls-files"], capture_output=True, text=True).stdout.split("\n")
for rel in sorted(f for f in tracked if os.path.basename(f).startswith("tsconfig") and f.endswith(".json") and ".🧬semio/🦑️repo/🎫️tickets/" not in f):
    if '"$schema"' in read(rel):
        RX(g, rel, r'^[ \t]*"\$schema":\s*"https://json\.schemastore\.org/tsconfig",?\r?\n', "", (1,), "leftovers-tsconfig", re.M)

ED(g, "pyproject.toml", '    "jupyter>=1.1.1",\n    "notebook>=7.6.0",\n    "ipykernel>=7.3.0",\n', "", (1,), "leftovers-python")
