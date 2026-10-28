import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
EDITS = [(path, f'"{spec}"', f'"{rel(path, target)}"') for path, spec, target in json.load(open(os.path.join(HERE, "specifier-rows.json"), encoding="utf8"))]
MCP = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🟦️.ts"
EDITS.append((MCP, 'export * from "../../🧬️schema-validation/🟦️.ts";\n', ""))
CLOSURES = L + "/🏗️build/📥️source/🛗️zero-touch/🗂️closures/🔣️.json"
EDITS.append((CLOSURES, '''      "reason": "The Nx graph consumer path analyses sources through the installed TypeScript compiler; nx --version never loads it."
    }
''', '''      "reason": "The Nx graph consumer path analyses sources through the installed TypeScript compiler; nx --version never loads it."
    },
    {
      "module": "''' + L + '''/📦️packages/🟦️typescript/🏗️build/🕸️graph/📥️inference/🟨️.mjs",
      "reason": "The public inference entry re-exports the graph inference seam statically, so it is itself post-install; importers reach it by a dynamic import only."
    },
    {
      "module": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🏗️builder/📥️inputs/🔌️nx/🟦️.ts",
      "reason": "The public Nx consumer entry re-exports the Nx graph consumer seam statically, so it is itself post-install; importers reach it by a dynamic import only."
    }
'''))
