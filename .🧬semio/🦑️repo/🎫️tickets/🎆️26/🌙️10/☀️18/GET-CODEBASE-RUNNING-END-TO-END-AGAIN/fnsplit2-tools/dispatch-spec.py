import json, re, sys
V = "️"
F = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🎯️dispatch-action/🦀️.rs"
def child(e, name, text):
    return {"dir": e + V + name, "doc": "//! %s%s %s See [`super`]." % (e, V, text)}
g = {
  "settings": child("🎛", "general-settings-actions", "General settings actions: appearance, driver, locale, terminology, default app, projection template and merge policy of the framework settings panel."),
  "commands": child("⌨", "command-and-action-staging-actions", "Command and action staging actions: keybindings, command and action staging, expansion, engagement input and search possibles."),
  "layouts": child("🪟", "layout-and-window-actions", "Layout and window actions: layouts, display windows and conflict resolution."),
  "plugins": child("🧩", "plugin-extension-and-chat-actions", "Plugin, extension and chat actions: install, reload and uninstall of plugins and extensions, chat drafts and tool call cancellation."),
  "themes": child("🎨", "theme-actions", "Theme actions: theme selection, token edits, saving, export, import and deletion."),
}
rows = [
 (191, "setAppearance", "settings", "🌓", "Stores the appearance id and republishes the general settings tab."),
 (199, "setDriver", "settings", "🚗", "Stores the ui driver id, clears the driver draft and republishes the general settings tab."),
 (208, "setDriverField", "settings", "🛠", "Edits one field of the driver draft."),
 (216, "setDriverSaveLabel", "settings", "🏷", "Stores the label under which the driver draft is saved."),
 (225, "saveDriver", "settings", "💾", "Saves the driver draft as a custom driver."),
 (239, "deleteDriver", "settings", "🗑", "Deletes one custom driver."),
 (252, "setLocale", "settings", "🌐", "Installs the chosen locale and republishes the settings panels."),
 (270, "setTerminology", "settings", "📖", "Stores the terminology id."),
 (283, "setThemeId", "themes", "🎭", "Selects the theme by id."),
 (296, "runOsCommand", "commands", "🖥", "Runs the os command the action names."),
 (303, "setLayout", "layouts", "🧱", "Applies the chosen layout."),
 (311, "setDefaultApp", "settings", "📌", "Stores the default app of a document kind."),
 (322, "captureKeybinding", "commands", "⌨", "Starts capturing a keybinding for a command."),
 (329, "resetKeybinding", "commands", "↩", "Resets the keybinding of a command."),
 (335, "setCommandExpanded", "commands", "🔽", "Expands or collapses a command row."),
 (344, "executeResolvedCommand", "commands", "▶", "Executes the command resolved from the action arguments."),
 (359, "stageCommandArg", "commands", "📝", "Stages one command argument."),
 (368, "resetStagedCommand", "commands", "🧽", "Clears the staged command."),
 (374, "executeStagedCommand", "commands", "🚀", "Executes the staged command."),
 (396, "setActionExpanded", "commands", "📂", "Expands or collapses an action row."),
 (413, "stageActionArg", "commands", "✍", "Stages one action argument."),
 (422, "resetActionArgs", "commands", "🧼", "Clears the staged action arguments."),
 (430, "executeStagedAction", "commands", "🎯", "Executes the staged action."),
 (438, "setEngagementInput", "commands", "👆", "Stores the engagement input text."),
 (446, "toggleSearchPossibles", "commands", "🔍", "Toggles the search possibles panel."),
 (454, "setWorldProjectionTemplate", "settings", "🗺", "Stores the world projection template."),
 (462, "setLayoutSaveLabel", "layouts", "📛", "Stores the label under which the current layout is saved."),
 (468, "saveCurrentLayout", "layouts", "📋", "Saves the current layout under the stored label."),
 (483, "applyNamedLayout", "layouts", "📐", "Applies a saved layout by name."),
 (491, "deleteUserLayout", "layouts", "❌", "Deletes a saved user layout."),
 (499, "openDisplayWindow", "layouts", "🖼", "Opens a display window."),
 (507, "selectConflict", "layouts", "⚖", "Selects a conflict to resolve."),
 (513, "resolveConflict", "layouts", "🤝", "Resolves the selected conflict."),
 (521, "installPlugin", "plugins", "🔌", "Installs or reloads a plugin."),
 (531, "uninstallPlugin", "plugins", "🔧", "Uninstalls a plugin."),
 (539, "installExtensionUrl", "plugins", "🌍", "Installs an extension from a url."),
 (553, "installExtensionFile", "plugins", "📦", "Installs an extension from a file."),
 (559, "setExtensionEnabled", "plugins", "🔛", "Enables or disables an extension."),
 (569, "uninstallExtension", "plugins", "🚮", "Uninstalls an extension."),
 (577, "setChatDraft", "plugins", "💬", "Stores the chat draft."),
 (583, "sendChatDraft", "plugins", "📨", "Sends the chat draft."),
 (587, "cancelToolCall", "plugins", "🛑", "Cancels a running tool call."),
 (597, "setMergePolicy", "settings", "🔀", "Stores the merge policy."),
 (607, "setThemeColor", "themes", "🌈", "Edits one theme token: color, spacing, font stack, stroke, radius, opacity or metric."),
 (617, "setThemeSaveLabel", "themes", "🔖", "Stores the label under which the theme is saved."),
 (625, "saveTheme", "themes", "🗄", "Saves the edited theme."),
 (642, "exportTheme", "themes", "📤", "Exports the theme."),
 (649, "importTheme", "themes", "📥", "Imports a theme."),
 (663, "applyImportedTheme", "themes", "✅", "Applies the imported theme."),
 (681, "resetThemeId", "themes", "🔄", "Resets the theme selection."),
 (692, "deleteThemeId", "themes", "🧯", "Deletes a custom theme."),
]
def snake(s):
    return re.sub(r"([A-Z])", lambda m: "_" + m.group(1).lower(), s)
used = set()
arms = []
for line, aid, grp, emoji, text in rows:
    assert emoji not in used, (emoji, aid)
    used.add(emoji)
    name = "framework_" + snake(aid)
    if aid == "installPlugin": name = "framework_install_or_reload_plugin"
    if aid == "setThemeColor": name = "framework_set_theme_token"
    arms.append({"line": line, "name": name, "doc": "%s %s" % (emoji, text), "child": g[grp], "allow_returns": True, "binds": {}})
spec = {
    "file": F, "function": "dispatch_action", "match_line": 190, "context": ["action"], "types": {"action": "&ActionDescriptor"},
    "argexpr": {"action": "&action"}, "ret": "Result<(), String>",
    "method": {"self_param": "&mut self", "async": True, "impl_header": "impl<'ctx> ActionsCtx<'ctx>", "call": "return self.{name}({args}).await"},
    "arms": arms,
}
json.dump(spec, open(sys.argv[1], "w", encoding="utf8"), ensure_ascii=False, indent=1)
