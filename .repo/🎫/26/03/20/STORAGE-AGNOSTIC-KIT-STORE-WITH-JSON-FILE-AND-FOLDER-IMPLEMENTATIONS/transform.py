#!/usr/bin/env python3
"""Transform Sketchpad.tsx to remove Yjs entity stores and use abstract KitStore."""
import re, sys

FILE = "/workspaces/semio/compose/sketchpad/sketchpad/Sketchpad.tsx"

with open(FILE, "r") as f:
    lines = f.readlines()

total = len(lines)
print(f"Read {total} lines from {FILE}")

# Phase 1: Remove entity stores (lines 1501-6355, 1-indexed)
# These are Yjs-backed entity store classes (AttributeStore through DesignStore)
# that will be replaced by KitStore snapshot access.
ENTITY_STORE_START = 1500  # 0-indexed (line 1501)
ENTITY_STORE_END = 6355    # 0-indexed exclusive (line 6355 inclusive)

# Phase 2: Remove ConceptStore + SyncIdMap + CollaborativeKitStore (lines 6357-7770)
# Keep Kit scope providers and hooks (from line ~7771)
COLLAB_START = 6356  # 0-indexed (line 6357)
# Find exact end of CollaborativeKitStore (look for the closing brace before KitScope)
collab_end = None
for i in range(6356, min(7800, total)):
    line = lines[i]
    if '// #endregion 🔖KitStore Interface Implementation' in line:
        # The class closing brace is on next non-empty line after this
        for j in range(i+1, min(i+5, total)):
            if lines[j].strip() == '}':
                collab_end = j + 1  # 0-indexed exclusive (line after closing brace)
                break
        break

if collab_end is None:
    # Fallback: search for KitScope type definition
    for i in range(6356, min(7800, total)):
        if 'type KitScope = { guid: string }' in lines[i]:
            collab_end = i
            break

print(f"Entity stores: lines {ENTITY_STORE_START+1}-{ENTITY_STORE_END}")
print(f"CollaborativeKitStore: lines {COLLAB_START+1}-{collab_end}")

# Build the new file content
output = []

# Part 1: Keep everything before entity stores (lines 1-1500)
output.extend(lines[:ENTITY_STORE_START])

# Part 2: Replace entity stores with a single comment
output.append("\n")
output.append("// Entity stores removed - using KitStore snapshot access\n")
output.append("\n")

# Part 3: Keep everything between entity stores end and Kit region start (lines 6356)
# Actually entity stores end at line 6355 and Kit region starts at 6357
# Line 6355 is "// #endregion Design", line 6356 is empty, line 6357 is "// #region Kit"
# So we skip from end of entity stores to start of Kit region (nothing between them)

# Part 4: Replace ConceptStore + CollaborativeKitStore with Kit region header + executeKitCommand
output.append("// #region Kit\n")
output.append("\n")
output.append("// [🏘️compose📚js🗃️sketchpad💻sketchpad🔖kit](composerepo://p/u/compose/b/l/js/fd/org/sketchpad/f/Sketchpad.tsx/s/Kit)\n")
output.append("// Storage-agnostic kit store hooks using KitStore interface.\n")
output.append("\n")
output.append("/**\n")
output.append(" * Execute a kit command against a KitStore.\n")
output.append(" * Looks up the command in kitCommands, builds context from snapshot, applies diff.\n")
output.append(" **/\n")
output.append("async function executeKitCommand(kitStore: KitStore, command: string, origin?: string, ...args: any[]): Promise<KitCommandResult> {\n")
output.append("  const callback = kitCommands[command as keyof typeof kitCommands];\n")
output.append("  if (!callback) throw new Error(`Command \"${command}\" not found in kit commands`);\n")
output.append("  const context: KitCommandContext = {\n")
output.append("    kit: kitStore.getSnapshot().kit,\n")
output.append("    fileUrls: new Map(),\n")
output.append("    origin,\n")
output.append("  };\n")
output.append("  const result = (callback as any)(context, ...args);\n")
output.append("  if (result.diff) {\n")
output.append("    kitStore.apply(result.diff, { origin });\n")
output.append("  }\n")
output.append("  return result;\n")
output.append("}\n")
output.append("\n")

# Part 5: Keep Kit scope providers + hook definitions from after CollaborativeKitStore
# The KitScope type starts at line ~7771 (after CollaborativeKitStore close)
output.extend(lines[collab_end:total])

with open(FILE, "w") as f:
    f.writelines(output)

new_total = len(output)
print(f"Wrote {new_total} lines (removed {total - new_total} lines)")
print("Phase 1-2 complete: Entity stores + CollaborativeKitStore removed")
