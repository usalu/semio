#!/usr/bin/env python3
import re, sys

RULES = [
    # Types
    (r"\bToolDefinition\b", "UtilityDefinition"),
    (r"\bToolCategory\b", "UtilityCategory"),
    (r"\bToolRef\b", "UtilityRef"),
    (r"\bToolNode\b", "UtilityNode"),
    (r"\bDerivedToolSpec\b", "DerivedUtilitySpec"),
    # functions
    (r"\btool_toggle_node\b", "utility_toggle_node"),
    (r"\bderive_tool_nodes\b", "derive_utility_nodes"),
    (r"\bresolve_window_tools\b", "resolve_window_utilities"),
    (r"\bpartition_tools_by_category\b", "partition_utilities_by_category"),
    (r"\bapply_set_active_tool\b", "apply_set_active_utility"),
    (r"\btool_button\b", "utility_button"),
    (r"\btool_toggle\b", "utility_toggle"),
    (r"\btool_separator\b", "utility_separator"),
    (r"\btool_collection\b", "utility_collection"),
    (r"\bnote_tool\b", "note_utility"),
    (r"\bpuzzle2d_active_tool\b", "puzzle2d_active_utility"),
    (r"\bpuzzle2d_tool\b", "puzzle2d_utility"),
    (r"\blowpoly_tool\b", "lowpoly_utility"),
    (r"\braster_tool\b", "raster_utility"),
    (r"\bdraw_tool\b", "draw_utility"),
    (r"\bcad_transform_tool\b", "cad_transform_utility"),
    (r"\bprocess3d_active_tool\b", "process3d_active_utility"),
    (r"\bview_with_tool\b", "view_with_utility"),
    # enum variant / action id
    (r"\bSetActiveTool\b", "SetActiveUtility"),
    (r"\bSET_ACTIVE_TOOL_ACTION_ID\b", "SET_ACTIVE_UTILITY_ACTION_ID"),
    # fields / vars
    (r"\bactive_tool_by_window\b", "active_utility_by_window"),
    (r"\bactive_tool_initial\b", "active_utility_initial"),
    (r"\bactive_tool_id\b", "active_utility_id"),
    (r"\bactive_tool\b", "active_utility"),
    (r"\btool_id\b", "utility_id"),
    (r"\bWindowToolRegistry\b", "WindowUtilityRegistry"),
    # wire strings
    (r'"setActiveTool"', '"setActiveUtility"'),
    (r'"toolId"', '"utilityId"'),
]

# bare "tools" field/var -> "utilities" (word-bounded, applied last)
RULES.append((r"\btools\b", "utilities"))

def apply(path):
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    original = text
    for pattern, repl in RULES:
        text = re.sub(pattern, repl, text)
    if text != original:
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)
        print(f"updated {path}")
    else:
        print(f"no change {path}")

for p in sys.argv[1:]:
    apply(p)
