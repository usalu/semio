#!/usr/bin/env python3
import re, sys

RULES = [
    (r"\bToolTreeProps\b", "UtilityTreeProps"),
    (r"\bToolTree\b", "UtilityTree"),
    (r"\bToolNode\b", "UtilityNode"),
    (r"\bToolCategory\b", "UtilityCategory"),
    (r"\bToolDefinition\b", "UtilityDefinition"),
    (r"\bDerivedToolSpec\b", "DerivedUtilitySpec"),
    (r"\bToolCollectionNode\b", "UtilityCollectionNode"),
    (r"\bToolLeaf\b", "UtilityLeaf"),
    (r"\bToolToolbarItems\b", "UtilityToolbarItems"),
    (r"\bFrameworkSyncToolLeaf\b", "FrameworkSyncUtilityLeaf"),
    (r"\bbuildFrameworkSyncTools\b", "buildFrameworkSyncUtilities"),
    (r"\bgroupToolNodesByCategory\b", "groupUtilityNodesByCategory"),
    (r"\bderiveToolNodes\b", "deriveUtilityNodes"),
    (r"\btoolDefinitionToSpec\b", "utilityDefinitionToSpec"),
    (r"\btagSetActiveToolWindow\b", "tagSetActiveUtilityWindow"),
    (r"\bwindowToolbarNode\b", "windowUtilityBarNode"),
    (r"\bWINDOW_TOOL_CATEGORIES\b", "WINDOW_UTILITY_CATEGORIES"),
    (r"\binjectActiveTool\b", "injectActiveUtility"),
    (r"\bresolveWindowToolNodes\b", "resolveWindowUtilityNodes"),
    (r"\bresolveWindowTools\b", "resolveWindowUtilities"),
    (r"\bactiveToolByWindowIdRef\b", "activeUtilityByWindowIdRef"),
    (r"\bactiveToolByWindowId\b", "activeUtilityByWindowId"),
    (r"\bactiveToolId\b", "activeUtilityId"),
    (r"\btoolId\b", "utilityId"),
    (r"\bSET_ACTIVE_TOOL_ACTION_ID\b", "SET_ACTIVE_UTILITY_ACTION_ID"),
    (r"\bSET_ACTIVE_TOOL\b", "SET_ACTIVE_UTILITY"),
    (r"\bsortToolNodes\b", "sortUtilityNodes"),
    (r"\bdedupeToolNodesById\b", "dedupeUtilityNodesById"),
    (r"\bisInteractiveToolNode\b", "isInteractiveUtilityNode"),
    (r"\bhasInteractiveToolNodes\b", "hasInteractiveUtilityNodes"),
    (r"\bhasInteractiveToolLeaves\b", "hasInteractiveUtilityLeaves"),
    (r"\btoolNodeCategory\b", "utilityNodeCategory"),
    (r"\bTOOL_CATEGORY_ORDER\b", "UTILITY_CATEGORY_ORDER"),
    (r"\bTOOL_CATEGORY_ICON_ID\b", "UTILITY_CATEGORY_ICON_ID"),
    (r"\breconcileToolPath\b", "reconcileUtilityPath"),
    (r"\bframeworkHistoryToolNodes\b", "frameworkHistoryUtilityNodes"),
    (r"\bframeworkToolsHistoryTab\b", "frameworkUtilitiesHistoryTab"),
    (r"\bresolveToolActivation\b", "resolveUtilityActivation"),
    (r'"setActiveTool"', '"setActiveUtility"'),
    (r'"toolId"', '"utilityId"'),
    (r"\bsyncTools\b", "syncUtilities"),
    # Precise field/prop access patterns for the AppDefinition.tools / AppWindowKindDefinition.tools
    # struct field rename -> .utilities. Deliberately NOT a blanket \btools\b rule: many "tools"
    # occurrences in these files are literal ToolCategory enum member values ("tools") or the
    # internal ToolbarRibbonSegment "tools" tag, which must stay unchanged.
    (r'Pick<AppDefinition, "tools" \| "controllerId">', 'Pick<AppDefinition, "utilities" | "controllerId">'),
    (r'Pick<AppDefinition, "controllerId" \| "tools">', 'Pick<AppDefinition, "controllerId" | "utilities">'),
    (r'Pick<AppDefinition, "tools">', 'Pick<AppDefinition, "utilities">'),
    (r'Pick<AppWindowKindDefinition, "tools">', 'Pick<AppWindowKindDefinition, "utilities">'),
    (r"\bapp\.tools\b", "app.utilities"),
    (r"\bwindowKind\.tools\b", "windowKind.utilities"),
]

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
