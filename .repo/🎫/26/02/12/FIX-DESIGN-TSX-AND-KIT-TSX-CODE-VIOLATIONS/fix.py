#!/usr/bin/env python3
"""Fix all code breachs in Design.tsx and Kit.tsx by adding section summaries, definition summaries, and spec comments."""

import re


def fix_file(filepath, section_summaries, definition_summaries):
    """Fix a single file by adding section summaries and definition summaries."""
    with open(filepath, "r") as f:
        lines = f.readlines()

    insertions = []

    for line_num_1based, summary in section_summaries.items():
        idx = line_num_1based - 1
        if idx < len(lines):
            line = lines[idx]
            if "#region" in line and "🔖" in line:
                insertions.append((idx + 1, f"// {summary}\n"))
            elif "#region" in line:
                insertions.append((idx + 1, f"// {summary}\n"))

    for line_num_1based, (summary, spec) in definition_summaries.items():
        idx = line_num_1based - 1
        if idx < len(lines):
            if spec:
                insertions.append((idx, f"// {spec}\n"))
            insertions.append((idx, f"// {summary}\n"))

    insertions.sort(key=lambda x: x[0], reverse=True)
    for pos, text in insertions:
        lines.insert(pos, text)

    with open(filepath, "w") as f:
        f.writelines(lines)

    return len(insertions)


def fix_design():
    filepath = "/workspaces/semio/compose/js/sketchpad/Design.tsx"

    section_summaries = {
        28: "Imports for Design app MUST include all shared sketchpad, React, and UI dependencies.",
        226: "State management types and interfaces MUST define the Design app selection, presence, hover, diff, and state shape.",
        323: "Commands MUST define all executable Design app actions dispatched by keyboard shortcuts and UI interactions.",
        1033: "Store MUST implement DesignStore extending PlainKitDiffAppStore with undo/redo, selection diff inversion, and state persistence.",
        1264: "Design app plugin registration MUST register the Design app plugin with machine actions, guards, and default state.",
        1423: "Hooks MUST provide the Design app initialization lifecycle within the React component tree.",
        1464: "Components MUST provide Design app scope, actor context, and synchronization wrapper components.",
        1727: "Action hooks MUST provide composable React hooks for Design app selection, hover, focus, panel, and transaction actions.",
        2825: "Footer MUST render dynamic Design app footer items showing selection and transaction state.",
        2941: "Tools MUST define all Design app tool configurations for selection, lasso, and hand modes.",
        3056: "WindowLibrary MUST provide draggable window templates for adding scene, diagram, and table windows.",
        3188: "Details MUST render the Design app detail panels for design, pieces, connections, and connector sections.",
        4529: "Hover Intent Context MUST manage debounced hover state to prevent flickering during rapid mouse movement.",
        4594: "Diagram MUST render the interactive React Flow design diagram with nodes, edges, minimap, and controls.",
        7185: "Scene MUST render the Three.js 3D scene view of design pieces with selection and hover highlighting.",
        7869: "App MUST compose all Design app panels, canvas, toolbar, and footer into the main Design app layout.",
        8570: "Settings MUST render the Design app settings panel with theme, language, device, expertise, and mode toggles.",
        8726: "Config MUST export the Design app configuration with route segments, panel definitions, and path matching.",
    }

    definition_summaries = {
        230: (
            "Tracks the current piece, connection, and connector selection state for the Design app.",
            None,
        ),
        236: ("Diff for added/removed piece GUIDs in a selection change.", None),
        240: ("Diff for added/removed connection GUIDs in a selection change.", None),
        244: (
            "Diff for a selected port change identifying the piece and connector.",
            None,
        ),
        249: (
            "Composite diff combining pieces, connections, and connector selection changes.",
            None,
        ),
        254: ("Enumeration of fullscreen window modes for the Design app.", None),
        259: ("Enumeration of window kinds available in the Design app.", None),
        263: (
            "Presence state for a Design app user including cursor, camera, and diagram viewport.",
            None,
        ),
        269: (
            "Hover state tracking which pieces, connections, connectors, types, and designs are hovered.",
            None,
        ),
        276: (
            "Extended presence for other collaborators including their display name.",
            None,
        ),
        279: ("Complete diff describing all mutable Design app state changes.", None),
        293: (
            "Edit record extending KitDiffAppEdit with Design app selection diff.",
            None,
        ),
        294: ("Complete runtime state for a Design app instance.", None),
        311: (
            "Context passed to Design app commands including app state, GUID, and design data.",
            None,
        ),
        316: (
            "Result returned by Design app commands containing diffs to apply.",
            None,
        ),
        325: (
            "Registry of all named Design app commands mapped to their handler functions.",
            None,
        ),
        1035: (
            "Computes the inverse of a Design app selection diff for undo support.",
            "// MUST return a diff that reverses the given selection diff.",
        ),
        1068: (
            "Checks whether two Design app identifiers refer to the same design.",
            None,
        ),
        1069: ("Checks whether a Design app identifier matches any in a list.", None),
        1071: (
            "DesignStore manages Design app state persistence, undo/redo stacks, and Y.js synchronization.",
            "// MUST extend PlainKitDiffAppStore and synchronize state with the Y.js shared document.",
        ),
        1258: (
            "Initializes the Design app store factory registration.",
            "// MUST register the DesignStore factory exactly once via registerDesignAppStoreFactory.",
        ),
        1466: (
            "Provider component that establishes Design app scope and actor context.",
            "// MUST wrap children with DesignAppScopeContext and DesignAppActorContext providers.",
        ),
        1473: (
            "Returns the current Design app XState actor from context.",
            "// MUST return the actor from DesignAppActorContext.",
        ),
        1477: (
            "Selects derived state from the Design app store.",
            "// MUST resolve the DesignStore from the orchestrator and apply the selector.",
        ),
        1492: (
            "Selects derived state from the Design app XState snapshot.",
            "// MUST use useSelector to reactively track the Design app state slice.",
        ),
        1548: (
            "Returns a reactive field for a Design app selection property.",
            "// MUST create a Field wrapping the selection value and setter.",
        ),
        1557: (
            "Returns a hook result for the current Design app selection.",
            "// MUST provide the current selection, a setter, and a canSet flag.",
        ),
        1561: (
            "Returns a reactive field for the Design app fullscreen window.",
            "// MUST create a Field wrapping the fullscreen value and setter.",
        ),
        1570: (
            "Returns a hook result for the Design app fullscreen window state.",
            "// MUST provide the current fullscreen window, a setter, and a canSet flag.",
        ),
        1574: (
            "Returns a reactive field for the Design app active tool.",
            "// MUST create a Field wrapping the active tool value and setter.",
        ),
        1584: (
            "Returns a hook result for the Design app active tool.",
            "// MUST provide the current active tool, a setter, and a canSet flag.",
        ),
        1588: (
            "Returns a hook result for the Design app diff state.",
            "// MUST provide the current diff, a setter, and a canSet flag.",
        ),
        1592: (
            "Returns other collaborators' presence state for the Design app.",
            "// MUST return a read-only list of other users' presence data.",
        ),
        1603: (
            "Returns a reactive field for the Design app camera.",
            "// MUST create a Field wrapping the camera value and setter.",
        ),
        1612: (
            "Returns a hook result for the Design app camera state.",
            "// MUST provide the current camera, a setter, and a canSet flag.",
        ),
        1616: (
            "Returns a hook result for the Design app diagram center coordinate.",
            "// MUST provide the current diagram center, a setter, and a canSet flag.",
        ),
        1638: (
            "Returns a hook result for the Design app diagram scale.",
            "// MUST provide the current diagram scale, a setter, and a canSet flag.",
        ),
        1659: (
            "Returns a reactive field for the focused piece GUID.",
            "// MUST create a Field wrapping the focused piece GUID value and setter.",
        ),
        1668: (
            "Returns a hook result for the focused piece GUID.",
            "// MUST provide the current focused piece GUID, a setter, and a canSet flag.",
        ),
        1672: (
            "Returns a hook result for the Design app selected model tags.",
            "// MUST provide the current selected model tags, a setter, and a canSet flag.",
        ),
        1691: (
            "Returns a hook result for the Design app hover state.",
            "// MUST provide the current hover, a setter, and a canSet flag.",
        ),
        1714: (
            "Returns a reactive field for Design app panel visibility.",
            "// MUST create a Field wrapping the panel visibility value and setter.",
        ),
        1723: (
            "Returns a hook result for Design app panel visibility.",
            "// MUST provide the current panel visibility, a setter, and a canSet flag.",
        ),
        1729: (
            "Tuple type for action hook results pairing an action callback with a canAct flag.",
            None,
        ),
        1731: (
            "Returns an action to set hover state to a single piece.",
            "// MUST return a callback that sets hover to the given piece GUID.",
        ),
        1740: (
            "Returns an action to set hover state to multiple pieces.",
            "// MUST return a callback that sets hover to the given piece GUIDs.",
        ),
        1749: (
            "Returns an action to set hover state to a single connection.",
            "// MUST return a callback that sets hover to the given connection GUID.",
        ),
        1758: (
            "Returns an action to set hover state to a single port.",
            "// MUST return a callback that sets hover to the given port identifiers.",
        ),
        1767: (
            "Returns an action to set hover state to types.",
            "// MUST return a callback that sets hover to the given type GUIDs.",
        ),
        1776: (
            "Returns an action to set hover state to designs.",
            "// MUST return a callback that sets hover to the given design GUIDs.",
        ),
        1785: (
            "Returns an action to clear the Design app hover state.",
            "// MUST return a callback that clears all hover state.",
        ),
        1794: (
            "Returns an action to select a single piece.",
            "// MUST return a callback that selects the given piece GUID.",
        ),
        1803: (
            "Returns an action to select multiple pieces.",
            "// MUST return a callback that selects the given piece GUIDs.",
        ),
        1812: (
            "Returns an action to add a piece to the current selection.",
            "// MUST return a callback that adds the given piece GUID to selection.",
        ),
        1826: (
            "Returns an action to remove a piece from the current selection.",
            "// MUST return a callback that removes the given piece GUID from selection.",
        ),
        1838: (
            "Returns an action to select a single connection.",
            "// MUST return a callback that selects the given connection GUID.",
        ),
        1847: (
            "Returns an action to add a connection to the current selection.",
            "// MUST return a callback that adds the given connection GUID to selection.",
        ),
        1861: (
            "Returns an action to remove a connection from the current selection.",
            "// MUST return a callback that removes the given connection GUID from selection.",
        ),
        1873: (
            "Returns an action to select a piece port.",
            "// MUST return a callback that selects the given piece-connector port.",
        ),
        1884: (
            "Returns an action to deselect a piece port.",
            "// MUST return a callback that deselects the given piece-connector port.",
        ),
        1896: (
            "Returns an action to deselect all items in the Design app.",
            "// MUST return a callback that clears all selection state.",
        ),
        1905: (
            "Returns an action to select all pieces and connections.",
            "// MUST return a callback that adds all piece and connection GUIDs to selection.",
        ),
        1919: (
            "Returns an action to focus on a specific piece.",
            "// MUST return a callback that sets the focused piece GUID.",
        ),
        1928: (
            "Returns an action to clear the focused piece.",
            "// MUST return a callback that clears the focused piece GUID.",
        ),
        1937: (
            "Returns an action to toggle diagram fullscreen mode.",
            "// MUST return a callback that toggles the diagram fullscreen window state.",
        ),
        1946: (
            "Returns an action to toggle accessl fullscreen mode.",
            "// MUST return a callback that toggles the accessl fullscreen window state.",
        ),
        1955: (
            "Returns an action to toggle a specific panel's visibility.",
            "// MUST return a callback that toggles the given panel's visibility.",
        ),
        1966: (
            "Returns an action to add a model tag for all types.",
            "// MUST return a callback that adds the given tag to all type entries.",
        ),
        1982: (
            "Returns an action to remove a model tag from all types.",
            "// MUST return a callback that removes the given tag from all type entries.",
        ),
        1998: (
            "Interface for transaction action callbacks including start, finalize, and abort.",
            None,
        ),
        2004: (
            "Returns the Design app transaction controller.",
            "// MUST provide start, finalize, and abort transaction actions.",
        ),
        2019: (
            "Provider component that establishes Design app transaction context.",
            "// MUST wrap children with the Design app transaction provider.",
        ),
        2024: (
            "Returns an action to undo the last Design app transaction.",
            "// MUST return a callback that undoes the most recent transaction.",
        ),
        2034: (
            "Returns an action to redo the last undone Design app transaction.",
            "// MUST return a callback that redoes the most recently undone transaction.",
        ),
        2044: (
            "Returns an action to delete all currently selected items.",
            "// MUST return a callback that removes all selected pieces and connections.",
        ),
        2054: (
            "Returns an action to add a piece to the design.",
            "// MUST return a callback that adds a piece with the given type GUID.",
        ),
        2064: (
            "Returns an action to add multiple pieces to the design.",
            "// MUST return a callback that adds pieces with the given type GUIDs.",
        ),
        2074: (
            "Returns an action to remove a piece from the design.",
            "// MUST return a callback that removes the piece with the given GUID.",
        ),
        2084: (
            "Returns an action to remove multiple pieces from the design.",
            "// MUST return a callback that removes the pieces with the given GUIDs.",
        ),
        2094: (
            "Returns an action to update a piece in the design.",
            "// MUST return a callback that updates the piece with the given GUID and partial data.",
        ),
        2104: (
            "Returns an action to update multiple pieces in the design.",
            "// MUST return a callback that updates the pieces with the given GUID-data pairs.",
        ),
        2119: (
            "Returns an action to add a connection to the design.",
            "// MUST return a callback that adds a connection with the given data.",
        ),
        2129: (
            "Returns an action to add multiple connections to the design.",
            "// MUST return a callback that adds connections with the given data array.",
        ),
        2139: (
            "Returns an action to remove a connection from the design.",
            "// MUST return a callback that removes the connection with the given GUID.",
        ),
        2149: (
            "Returns an action to remove multiple connections from the design.",
            "// MUST return a callback that removes the connections with the given GUIDs.",
        ),
        2159: (
            "Returns an action to update a connection in the design.",
            "// MUST return a callback that updates the connection with the given GUID and partial data.",
        ),
        2169: (
            "Returns an action to update multiple connections in the design.",
            "// MUST return a callback that updates the connections with the given GUID-data pairs.",
        ),
        2184: (
            "Returns an action to cluster selected pieces into a new design.",
            "// MUST return a callback that clusters the given piece GUIDs.",
        ),
        2194: (
            "Returns an action to expand a nested design into inline pieces.",
            "// MUST return a callback that expands the design with the given piece GUID.",
        ),
        2261: (
            "Returns the full Design app commands API for programmatic access.",
            "// MUST expose all Design app commands through the store controller.",
        ),
        2361: (
            "Synchronizes Y.js document changes to XState Design app state.",
            "// MUST observe Y.js map changes and dispatch corresponding XState events.",
        ),
        2473: (
            "Provider that makes transaction-changed piece GUIDs available to children.",
            "// MUST compute and provide the set of piece GUIDs changed in the current transaction.",
        ),
        2483: (
            "Returns whether a piece is changed in the current transaction.",
            "// MUST check the transaction pieces context for the given GUID.",
        ),
        2488: (
            "Returns whether a piece is currently hovered in the Design app.",
            "// MUST check the hover state for the given piece GUID.",
        ),
        2566: (
            "Provider that makes transitively hovered piece GUIDs available to children.",
            "// MUST compute and provide the set of piece GUIDs that are transitively hovered.",
        ),
        2576: (
            "Returns whether a piece is transitively hovered via type or design document.",
            "// MUST check the transitive hover pieces for the given GUID.",
        ),
        2582: (
            "Returns whether a type is transitively hovered in the Design app.",
            "// MUST check the hover state for the given type GUID.",
        ),
        2587: (
            "Returns the diff status of a piece for visual indication.",
            "// MUST return DiffStatus from the design diff for the given piece GUID.",
        ),
        2592: (
            "Returns whether a piece is currently selected in the Design app.",
            "// MUST check the selection state for the given piece GUID.",
        ),
        2604: (
            "Returns the computed color for a piece based on its status.",
            "// MUST derive the color from selection, hover, diff status, and type mapping.",
        ),
        2661: (
            "Returns whether a connection is currently hovered in the Design app.",
            "// MUST check the hover state for the given connection GUID.",
        ),
        2673: (
            "Returns whether a connection is currently selected in the Design app.",
            "// MUST check the selection state for the given connection GUID.",
        ),
        2685: (
            "Returns whether a port is currently hovered in the Design app.",
            "// MUST check the hover state for the given piece-connector port.",
        ),
        2699: (
            "Returns the selected connector for the Design app.",
            "// MUST return the currently selected connector from the selection state.",
        ),
        2712: (
            "Returns whether a specific piece port is currently selected.",
            "// MUST check the selection connector state for the given piece-connector pair.",
        ),
        2753: (
            "Returns the diff status of a connection for visual indication.",
            "// MUST return DiffStatus from the design diff for the given connection GUID.",
        ),
        2758: (
            "Returns the computed color for a connection based on its status.",
            "// MUST derive the color from selection, hover, and diff status.",
        ),
        2805: (
            "Returns the center position of a piece on the canvas.",
            "// MUST look up the piece metadata for the given GUID and return its center.",
        ),
        2814: (
            "Returns the plane orientation of a piece.",
            "// MUST look up the piece metadata for the given GUID and return its plane.",
        ),
        2827: (
            "Footer component that renders dynamic Design app footer status items.",
            "// MUST register and unregister footer items based on selection and transaction state.",
        ),
        2943: ("Tool configuration for normal selection mode.", None),
        2949: ("Tool configuration for additive selection mode.", None),
        2955: ("Tool configuration for subtractive selection mode.", None),
        2961: ("Tool configuration for rectangular lasso selection mode.", None),
        2967: ("Tool configuration for freeform lasso selection mode.", None),
        2973: ("Tool configuration for hand/pan mode.", None),
        2979: ("Array of all Design app tool configurations.", None),
        2981: (
            "Settings component for the selection tool group with additive, subtractive, and intersect toggles.",
            "// MUST render toggle buttons for each selection sub-mode.",
        ),
        3020: (
            "Settings component for the hand tool that activates hand mode.",
            "// MUST activate the hand tool on mount.",
        ),
        3032: (
            "Settings component for the lasso tool with rectangular and freeform toggles.",
            "// MUST render toggle group for lasso sub-modes.",
        ),
        3160: (
            "Panel component that renders the draggable window template library.",
            "// MUST render categorized window templates for scene, diagram, and table types.",
        ),
        3190: (
            "Detail section component for the currently open design.",
            "// MUST render the design form fields within a detail panel section.",
        ),
        3601: (
            "Detail section component for the design pieces list.",
            "// MUST render each piece with its type, name, and selection interactions.",
        ),
        4291: (
            "Detail section component for the design connections list.",
            "// MUST render each connection with its connected pieces and ports.",
        ),
        4433: (
            "Detail section component for the currently selected connector.",
            "// MUST render the connector detail form for the selected port.",
        ),
        5494: (
            "Custom minimap node component rendering a colored circle.",
            "// MUST render a circle at the given position with accent color when selected.",
        ),
        7839: ("Props interface for the Design app root component.", None),
        8728: (
            "Exported Design app configuration including routes, panels, and path matching.",
            None,
        ),
    }

    with open(filepath, "r") as f:
        lines = f.readlines()

    insertions = []

    for line_num, summary in section_summaries.items():
        idx = line_num - 1
        if idx < len(lines):
            insertions.append((idx + 1, f"// {summary}\n"))

    for line_num, (summary, spec) in definition_summaries.items():
        idx = line_num - 1
        if idx < len(lines):
            if spec:
                insertions.append((idx, f"{spec}\n"))
            insertions.append((idx, f"// {summary}\n"))

    orphan_line = 7839
    idx = orphan_line - 1
    insertions.append((idx, "// #region 🔖Windows\n"))
    insertions.append(
        (
            idx,
            "// Window components MUST wrap diagram and scene views with hover and transaction providers.\n",
        )
    )

    groups = {}
    for pos, text in insertions:
        groups.setdefault(pos, []).append(text)

    sorted_positions = sorted(groups.keys(), reverse=True)
    for pos in sorted_positions:
        for text in reversed(groups[pos]):
            lines.insert(pos, text)

    for i in range(len(lines) - 1, -1, -1):
        if "// #endregion Components" in lines[i]:
            for j in range(i + 1, min(i + 5, len(lines))):
                if "// #region App" in lines[j] or "// #region 🔖App" in lines[j]:
                    lines.insert(i, "// #endregion 🔖Windows\n\n")
                    break
            break

    with open(filepath, "w") as f:
        f.writelines(lines)


def fix_kit():
    filepath = "/workspaces/semio/compose/js/sketchpad/Kit.tsx"

    section_summaries = {
        29: "Imports for Kit app MUST include all shared sketchpad, React, DnD, and UI dependencies.",
        205: "Design family helper functions MUST traverse the design document to collect related design GUIDs.",
        232: "Constants MUST define artifact kinds and toolbar sub-tool configurations for the Kit app.",
        269: "Internal state management MUST define all Kit app types, interfaces, store, and Y.js synchronization.",
        1050: "Kit app plugin registration MUST register the Kit app plugin with machine actions, guards, and default state.",
        1532: "Action hooks MUST provide composable React hooks for Kit app selection, hover, sort, filter, and transaction actions.",
        1614: "Selection helper hooks MUST provide entity-specific add, remove, toggle, select-single, select-all, and clear operations.",
        1698: "Types selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for type selection.",
        1780: "Designs selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for design selection.",
        1862: "Qualities selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for quality selection.",
        1944: "Ports selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for port selection.",
        2026: "Tags selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for tag selection.",
        2108: "Concepts selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for concept selection.",
        2190: "Files selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for file selection.",
        2272: "Folders selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for folder selection.",
        2354: "Authors selection hooks MUST provide add, remove, toggle, select-single, select-all, and clear for author selection.",
        2436: "Global selection hooks MUST provide select-all across all artifact kinds.",
        2588: "Types MUST provide hover status and color hooks for type visual indication in the Kit app.",
        2657: "Designs MUST provide hover status and color hooks for design visual indication in the Kit app.",
        2726: "Commands MUST define all executable Kit app actions for artifact CRUD, import, and export.",
        3473: "Table MUST render the interactive data table with sortable columns, expandable rows, and drag-drop reordering.",
        6117: "Diagram MUST render the interactive force-directed Kit diagram with type and design nodes.",
        7309: "Tools MUST define Kit app toolbar filter and selection tool components.",
        7374: "Details MUST render the Kit app detail panels for kit, type, port, tag, concept, design, file, folder, and multi-artifact sections.",
        8033: "Settings MUST render the Kit app settings panel with theme, language, device, expertise, mode, and diagram force controls.",
        8201: "Footer MUST render the Kit app footer with selection count status.",
        8221: "Config MUST export the Kit app configuration with route segments, panel definitions, and path matching.",
    }

    definition_summaries = {
        275: (
            "Tracks the current entity selection state across all artifact kinds for the Kit app.",
            None,
        ),
        287: ("Diff for added/removed type GUIDs in a Kit app selection change.", None),
        291: (
            "Diff for added/removed design GUIDs in a Kit app selection change.",
            None,
        ),
        295: (
            "Diff for added/removed quality strings in a Kit app selection change.",
            None,
        ),
        299: ("Diff for added/removed port GUIDs in a Kit app selection change.", None),
        303: ("Diff for added/removed tag GUIDs in a Kit app selection change.", None),
        307: (
            "Diff for added/removed concept GUIDs in a Kit app selection change.",
            None,
        ),
        311: (
            "Diff for added/removed file strings in a Kit app selection change.",
            None,
        ),
        315: (
            "Diff for added/removed folder GUIDs in a Kit app selection change.",
            None,
        ),
        319: (
            "Diff for added/removed author strings in a Kit app selection change.",
            None,
        ),
        323: (
            "Composite diff combining all artifact-kind selection changes for the Kit app.",
            None,
        ),
        334: ("Enumeration of window kinds available in the Kit app.", None),
        338: ("Presence state for a Kit app user including cursor and camera.", None),
        342: (
            "Hover state tracking which single entity is hovered per artifact kind.",
            None,
        ),
        353: (
            "Extended presence for other Kit app collaborators including their display name.",
            None,
        ),
        356: ("Column identifier type for Kit app table sorting.", None),
        357: ("Sort direction type for Kit app table sorting.", None),
        359: (
            "Configuration interface for Kit diagram force simulation parameters.",
            None,
        ),
        366: ("Default force simulation settings for the Kit diagram layout.", None),
        373: ("Complete diff describing all mutable Kit app state changes.", None),
        387: (
            "Edit record extending KitDiffAppEdit with Kit app selection diff.",
            None,
        ),
        388: ("Complete runtime state for a Kit app instance.", None),
        404: (
            "Context passed to Kit app commands including the current app state.",
            None,
        ),
        407: ("Result returned by Kit app commands containing diffs to apply.", None),
        412: (
            "Computes the inverse of a Kit app selection diff for undo support.",
            "// MUST return a diff that reverses the given selection diff across all artifact kinds.",
        ),
        477: ("Checks whether two Kit app identifiers refer to the same kit.", None),
        478: ("Checks whether a Kit app identifier matches any in a list.", None),
        1177: (
            "Overload: returns the KitStore instance when no selector is provided.",
            None,
        ),
        1178: (
            "Overload: returns a derived value when a selector function is provided.",
            None,
        ),
        1179: (
            "Selects derived state or the raw KitStore from the sketchpad orchestrator.",
            "// MUST resolve the KitStore for the current kit scope and apply the optional selector.",
        ),
        1198: (
            "Selects derived state from the Kit app XState snapshot.",
            "// MUST use the sketchpad actor to reactively track the Kit app state slice.",
        ),
        1236: (
            "Returns a hook result for the current Kit app selection.",
            "// MUST provide the current selection, a setter, and a canSet flag.",
        ),
        1253: (
            "Returns a hook result for the Kit app fullscreen window state.",
            "// MUST provide the current fullscreen window, a setter, and a canSet flag.",
        ),
        1270: (
            "Returns other collaborators' presence state for the Kit app.",
            "// MUST return a read-only list of other users' presence data.",
        ),
        1280: (
            "Returns a hook result for the Kit app window layout.",
            "// MUST provide the current window layout, a setter, and a canSet flag.",
        ),
        1297: (
            "Returns the Kit app diagram force settings with an updater.",
            "// MUST provide the current force settings, an updater, and a canSet flag.",
        ),
        1314: (
            "Returns a hook result for the Kit app active tool.",
            "// MUST provide the current active tool, a setter, and a canSet flag.",
        ),
        1337: (
            "Returns a reactive field for the Kit app active tool.",
            "// MUST create a Field wrapping the active tool value and setter.",
        ),
        1342: (
            "Returns a read-only hook result for the Kit app sort column.",
            "// MUST provide the current sort column from the XState snapshot.",
        ),
        1352: (
            "Returns a read-only hook result for the Kit app sort direction.",
            "// MUST provide the current sort direction from the XState snapshot.",
        ),
        1362: (
            "Returns a read-only hook result for the Kit app expanded rows.",
            "// MUST provide the current expanded row set from the XState snapshot.",
        ),
        1372: (
            "Returns the Kit app transaction controller with start, finalize, and abort.",
            "// MUST provide transaction actions dispatching to the XState actor.",
        ),
        1387: (
            "Returns the full Kit app commands API for programmatic access.",
            "// MUST expose all Kit app commands through the store controller.",
        ),
        1534: (
            "Tuple type for action hook results pairing an action callback with a canAct flag.",
            None,
        ),
        1536: (
            "Returns an action to select a single type in the Kit app.",
            "// MUST return a callback that selects the given type GUID.",
        ),
        1549: (
            "Returns an action to deselect a single type in the Kit app.",
            "// MUST return a callback that deselects the given type GUID.",
        ),
        1562: (
            "Returns an action to select a single design in the Kit app.",
            "// MUST return a callback that selects the given design GUID.",
        ),
        1575: (
            "Returns an action to deselect a single design in the Kit app.",
            "// MUST return a callback that deselects the given design GUID.",
        ),
        1588: (
            "Returns an action to set the full Kit app selection.",
            "// MUST return a callback that replaces the entire selection state.",
        ),
        1601: (
            "Returns an action to clear the full Kit app selection.",
            "// MUST return a callback that clears all selection state.",
        ),
        1700: (
            "Returns an action to add a type to the Kit app selection.",
            "// MUST return a callback that adds the given type GUID to selection.",
        ),
        1713: (
            "Returns an action to remove a type from the Kit app selection.",
            "// MUST return a callback that removes the given type GUID from selection.",
        ),
        1726: (
            "Returns an action to toggle a type in the Kit app selection.",
            "// MUST return a callback that toggles the given type GUID in selection.",
        ),
        1739: (
            "Returns an action to select only a single type, clearing others.",
            "// MUST return a callback that clears types and selects the given GUID.",
        ),
        1752: (
            "Returns an action to select multiple types in the Kit app.",
            "// MUST return a callback that selects the given type GUIDs.",
        ),
        1765: (
            "Returns an action to clear all type selections.",
            "// MUST return a callback that clears all type GUIDs from selection.",
        ),
        1782: (
            "Returns an action to add a design to the Kit app selection.",
            "// MUST return a callback that adds the given design GUID to selection.",
        ),
        1795: (
            "Returns an action to remove a design from the Kit app selection.",
            "// MUST return a callback that removes the given design GUID from selection.",
        ),
        1808: (
            "Returns an action to toggle a design in the Kit app selection.",
            "// MUST return a callback that toggles the given design GUID in selection.",
        ),
        1821: (
            "Returns an action to select only a single design, clearing others.",
            "// MUST return a callback that clears designs and selects the given GUID.",
        ),
        1834: (
            "Returns an action to select multiple designs in the Kit app.",
            "// MUST return a callback that selects the given design GUIDs.",
        ),
        1847: (
            "Returns an action to clear all design selections.",
            "// MUST return a callback that clears all design GUIDs from selection.",
        ),
        1864: (
            "Returns an action to add a quality to the Kit app selection.",
            "// MUST return a callback that adds the given quality string to selection.",
        ),
        1877: (
            "Returns an action to remove a quality from the Kit app selection.",
            "// MUST return a callback that removes the given quality string from selection.",
        ),
        1890: (
            "Returns an action to toggle a quality in the Kit app selection.",
            "// MUST return a callback that toggles the given quality string in selection.",
        ),
        1903: (
            "Returns an action to select only a single quality, clearing others.",
            "// MUST return a callback that clears qualities and selects the given string.",
        ),
        1916: (
            "Returns an action to select multiple qualities in the Kit app.",
            "// MUST return a callback that selects the given quality strings.",
        ),
        1929: (
            "Returns an action to clear all quality selections.",
            "// MUST return a callback that clears all quality strings from selection.",
        ),
        1946: (
            "Returns an action to add a port to the Kit app selection.",
            "// MUST return a callback that adds the given port GUID to selection.",
        ),
        1959: (
            "Returns an action to remove a port from the Kit app selection.",
            "// MUST return a callback that removes the given port GUID from selection.",
        ),
        1972: (
            "Returns an action to toggle a port in the Kit app selection.",
            "// MUST return a callback that toggles the given port GUID in selection.",
        ),
        1985: (
            "Returns an action to select only a single port, clearing others.",
            "// MUST return a callback that clears ports and selects the given GUID.",
        ),
        1998: (
            "Returns an action to select multiple ports in the Kit app.",
            "// MUST return a callback that selects the given port GUIDs.",
        ),
        2011: (
            "Returns an action to clear all port selections.",
            "// MUST return a callback that clears all port GUIDs from selection.",
        ),
        2028: (
            "Returns an action to add a tag to the Kit app selection.",
            "// MUST return a callback that adds the given tag GUID to selection.",
        ),
        2041: (
            "Returns an action to remove a tag from the Kit app selection.",
            "// MUST return a callback that removes the given tag GUID from selection.",
        ),
        2054: (
            "Returns an action to toggle a tag in the Kit app selection.",
            "// MUST return a callback that toggles the given tag GUID in selection.",
        ),
        2067: (
            "Returns an action to select only a single tag, clearing others.",
            "// MUST return a callback that clears tags and selects the given GUID.",
        ),
        2080: (
            "Returns an action to select multiple tags in the Kit app.",
            "// MUST return a callback that selects the given tag GUIDs.",
        ),
        2093: (
            "Returns an action to clear all tag selections.",
            "// MUST return a callback that clears all tag GUIDs from selection.",
        ),
        2110: (
            "Returns an action to add a concept to the Kit app selection.",
            "// MUST return a callback that adds the given concept GUID to selection.",
        ),
        2123: (
            "Returns an action to remove a concept from the Kit app selection.",
            "// MUST return a callback that removes the given concept GUID from selection.",
        ),
        2136: (
            "Returns an action to toggle a concept in the Kit app selection.",
            "// MUST return a callback that toggles the given concept GUID in selection.",
        ),
        2149: (
            "Returns an action to select only a single concept, clearing others.",
            "// MUST return a callback that clears concepts and selects the given GUID.",
        ),
        2162: (
            "Returns an action to select multiple concepts in the Kit app.",
            "// MUST return a callback that selects the given concept GUIDs.",
        ),
        2175: (
            "Returns an action to clear all concept selections.",
            "// MUST return a callback that clears all concept GUIDs from selection.",
        ),
        2192: (
            "Returns an action to add a file to the Kit app selection.",
            "// MUST return a callback that adds the given file string to selection.",
        ),
        2205: (
            "Returns an action to remove a file from the Kit app selection.",
            "// MUST return a callback that removes the given file string from selection.",
        ),
        2218: (
            "Returns an action to toggle a file in the Kit app selection.",
            "// MUST return a callback that toggles the given file string in selection.",
        ),
        2231: (
            "Returns an action to select only a single file, clearing others.",
            "// MUST return a callback that clears files and selects the given string.",
        ),
        2244: (
            "Returns an action to select multiple files in the Kit app.",
            "// MUST return a callback that selects the given file strings.",
        ),
        2257: (
            "Returns an action to clear all file selections.",
            "// MUST return a callback that clears all file strings from selection.",
        ),
        2274: (
            "Returns an action to add a folder to the Kit app selection.",
            "// MUST return a callback that adds the given folder GUID to selection.",
        ),
        2287: (
            "Returns an action to remove a folder from the Kit app selection.",
            "// MUST return a callback that removes the given folder GUID from selection.",
        ),
        2300: (
            "Returns an action to toggle a folder in the Kit app selection.",
            "// MUST return a callback that toggles the given folder GUID in selection.",
        ),
        2313: (
            "Returns an action to select only a single folder, clearing others.",
            "// MUST return a callback that clears folders and selects the given GUID.",
        ),
        2326: (
            "Returns an action to select multiple folders in the Kit app.",
            "// MUST return a callback that selects the given folder GUIDs.",
        ),
        2339: (
            "Returns an action to clear all folder selections.",
            "// MUST return a callback that clears all folder GUIDs from selection.",
        ),
        2356: (
            "Returns an action to add an author to the Kit app selection.",
            "// MUST return a callback that adds the given author string to selection.",
        ),
        2369: (
            "Returns an action to remove an author from the Kit app selection.",
            "// MUST return a callback that removes the given author string from selection.",
        ),
        2382: (
            "Returns an action to toggle an author in the Kit app selection.",
            "// MUST return a callback that toggles the given author string in selection.",
        ),
        2395: (
            "Returns an action to select only a single author, clearing others.",
            "// MUST return a callback that clears authors and selects the given string.",
        ),
        2408: (
            "Returns an action to select multiple authors in the Kit app.",
            "// MUST return a callback that selects the given author strings.",
        ),
        2421: (
            "Returns an action to clear all author selections.",
            "// MUST return a callback that clears all author strings from selection.",
        ),
        2438: (
            "Returns an action to select all entities across all artifact kinds.",
            "// MUST return a callback that adds all artifact GUIDs to selection.",
        ),
        2476: (
            "Returns an action to set the Kit app filter search query.",
            "// MUST return a callback that sets the filter search string.",
        ),
        2489: (
            "Returns an action to toggle a row's expanded state in the Kit table.",
            "// MUST return a callback that toggles the given row GUID in expanded rows.",
        ),
        2502: (
            "Returns an action to set the Kit table sort column.",
            "// MUST return a callback that sets the sort column identifier.",
        ),
        2515: (
            "Returns an action to toggle the Kit table sort direction.",
            "// MUST return a callback that toggles between ascending and descending.",
        ),
        2535: (
            "Returns a hook result for the Kit app hover state.",
            "// MUST provide the current hover, a setter, and a canSet flag.",
        ),
        2545: (
            "Returns an action to set the Kit app hover state.",
            "// MUST return a callback that sets hover to the given entity.",
        ),
        2558: (
            "Returns an action to clear the Kit app hover state.",
            "// MUST return a callback that clears all hover state.",
        ),
        2571: (
            "Returns an action to toggle a specific panel's visibility.",
            "// MUST return a callback that toggles the given panel's visibility.",
        ),
        2590: (
            "Returns whether a type is currently hovered in the Kit app.",
            "// MUST check the hover state for the given type GUID.",
        ),
        2598: (
            "Returns the selection/hover status of a type for visual indication.",
            "// MUST derive status from selection and hover states for the given type GUID.",
        ),
        2604: (
            "Returns the computed color for a type based on its status.",
            "// MUST derive the color from the type's hovered and selected state.",
        ),
        2659: (
            "Returns whether a design is currently hovered in the Kit app.",
            "// MUST check the hover state for the given design GUID.",
        ),
        2667: (
            "Returns the selection/hover status of a design for visual indication.",
            "// MUST derive status from selection and hover states for the given design GUID.",
        ),
        2673: (
            "Returns the computed color for a design based on its status.",
            "// MUST derive the color from the design's hovered and selected state.",
        ),
        2728: (
            "Registry of all named Kit app commands mapped to their handler functions.",
            None,
        ),
        7311: (
            "Returns a hook for the Kit app filter search input state.",
            "// MUST provide the current filter string and a setter.",
        ),
        7323: (
            "Filter toolbar component rendering the search input for Kit artifacts.",
            "// MUST render a filter input connected to the Kit app filter search state.",
        ),
        7331: (
            "Toolbar selection tool component for the Kit app.",
            "// MUST render selection mode toggle buttons.",
        ),
        7349: (
            "Toolbar hand tool component for the Kit app.",
            "// MUST activate hand mode on mount.",
        ),
        7376: (
            "Detail section component for the currently open kit.",
            "// MUST render the kit metadata form fields within a detail panel section.",
        ),
        7488: (
            "Detail section component for the selected type.",
            "// MUST render the type form fields within a detail panel section.",
        ),
        7565: (
            "Detail section component for the selected port.",
            "// MUST render the port form fields within a detail panel section.",
        ),
        7628: (
            "Detail section component for the selected tag.",
            "// MUST render the tag form fields within a detail panel section.",
        ),
        7680: (
            "Detail section component for the selected concept.",
            "// MUST render the concept form fields within a detail panel section.",
        ),
        7732: (
            "Detail section component for the selected design.",
            "// MUST render the design form fields within a detail panel section.",
        ),
        7855: (
            "Detail section component for the selected file.",
            "// MUST render the file metadata within a detail panel section.",
        ),
        7917: (
            "Detail section component for the selected folder.",
            "// MUST render the folder metadata within a detail panel section.",
        ),
        8001: (
            "Detail section component for multiple selected artifacts.",
            "// MUST render a summary of all selected artifacts across kinds.",
        ),
        8203: (
            "Footer component that renders the Kit app selection count status.",
            "// MUST register and unregister footer items based on current selection state.",
        ),
        8223: (
            "Exported Kit app configuration including routes, panels, and path matching.",
            None,
        ),
    }

    with open(filepath, "r") as f:
        lines = f.readlines()

    insertions = []

    for line_num, summary in section_summaries.items():
        idx = line_num - 1
        if idx < len(lines):
            insertions.append((idx + 1, f"// {summary}\n"))

    for line_num, (summary, spec) in definition_summaries.items():
        idx = line_num - 1
        if idx < len(lines):
            if spec:
                insertions.append((idx, f"{spec}\n"))
            insertions.append((idx, f"// {summary}\n"))

    groups = {}
    for pos, text in insertions:
        groups.setdefault(pos, []).append(text)

    sorted_positions = sorted(groups.keys(), reverse=True)
    for pos in sorted_positions:
        for text in reversed(groups[pos]):
            lines.insert(pos, text)

    with open(filepath, "w") as f:
        f.writelines(lines)


if __name__ == "__main__":
    print("Fixing Design.tsx...")
    fix_design()
    print("Fixing Kit.tsx...")
    fix_kit()
    print("Done!")
