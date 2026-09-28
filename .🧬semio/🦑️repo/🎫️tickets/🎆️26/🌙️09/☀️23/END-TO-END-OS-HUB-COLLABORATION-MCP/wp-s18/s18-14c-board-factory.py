# -*- coding: utf-8 -*-
"""S18 §14c hub leg: a hub document's program (`<plugin>@<bundle sha256>`, hubProgramIdV1) resolves its plugin's own
app-surface session factory — the 2d puzzle board threw "The current app has no registered board session factory" on
every hub-opened puzzle document because the registrations are keyed by the plugin id. Removes the temporary
`[DEBUG] s18 board` probe line from ShellHost in the same pass. Idempotent."""
import json
import pathlib

LOADER = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx")
SHELL = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx")
FIXTURE = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧫️fixtures/🔣️session-factory.json")
HUB_SHA = "fbe769993f80590cf71f4caa0d6005a54d8bc845efbe0b63741e25d6bfd8e223"

EDITS = [
    (LOADER,
     """/** 🪪️ Joins one exact app-owned constructor to the current shell instance without constructing it. */
export function resolveAppSurfaceSessionFactory(registrations: readonly AppSurfaceSessionFactory[], identity: { readonly pluginId: string; readonly appId: string; readonly instanceId: number } | null): ScopedBoardSessionFactory | null {
  if (!identity) return null;
  const matches = registrations.filter((registration) => registration.kind === "board-2d" && registration.pluginId === identity.pluginId && registration.appId === identity.appId);""",
     """/** 🪪️ Joins one exact app-owned constructor to the current shell instance without constructing it. A hub document's
 * program (`<plugin>@<bundle sha256>`, {@link parseHubProgramIdV1}) runs its plugin's own app, so it joins that plugin's
 * constructor: the registrations are keyed by plugin id, and a hub-opened 2d puzzle threw "no registered board session
 * factory" (ticket 26/09/23 S18 §14c). The scoped factory keeps the program's own id. */
export function resolveAppSurfaceSessionFactory(registrations: readonly AppSurfaceSessionFactory[], identity: { readonly pluginId: string; readonly appId: string; readonly instanceId: number } | null): ScopedBoardSessionFactory | null {
  if (!identity) return null;
  const pluginId = parseHubProgramIdV1(identity.pluginId)?.pluginId ?? identity.pluginId;
  const matches = registrations.filter((registration) => registration.kind === "board-2d" && registration.pluginId === pluginId && registration.appId === identity.appId);"""),
    (SHELL,
     "  useEffect(() => { console.warn(\"[DEBUG] s18 board\", JSON.stringify({ focusedProgram, factory: boardSessionFactory !== null, registrations: (surfaceSessionFactories ?? []).map((entry) => `${entry.pluginId}/${entry.appId}`), spawned: (panel?.spawnedApps ?? []).map((entry) => `${entry.id}:${entry.pluginId}/${entry.appId}`), browserActors: browserActorUiVersion })); }, [focusedProgram, boardSessionFactory, surfaceSessionFactories, panel?.spawnedApps, browserActorUiVersion]);\n",
     ""),
]

SCOPES = [
    {"pluginId": f"puzzle@{HUB_SHA}", "appId": "s.puzzle2d@1/*#editor", "instanceId": 3, "matches": True},
    {"pluginId": f"other@{HUB_SHA}", "appId": "s.puzzle2d@1/*#editor", "instanceId": 3, "matches": False},
]


def main() -> None:
    for path, old, new in EDITS:
        text = path.read_text(encoding="utf-8")
        if old in text:
            assert text.count(old) == 1, (path.name, old[:80])
            text = text.replace(old, new)
            path.write_text(text, encoding="utf-8")
    text = LOADER.read_text(encoding="utf-8")
    if "parseHubProgramIdV1 }" not in text and "import { parseHubProgramIdV1 }" not in text:
        first_import = text.index("import ")
        text = text[:first_import] + 'import { parseHubProgramIdV1 } from "../../../../🔌️plugin/📇️registry/🌎️hub-source/🔍️resolution/🟦️.ts";\n' + text[first_import:]
        LOADER.write_text(text, encoding="utf-8")
    raw = FIXTURE.read_text(encoding="utf-8")
    added = [scope for scope in SCOPES if scope not in json.loads(raw)["scopes"]]
    if added:
        last = '    { "pluginId": "puzzle", "appId": "s.puzzle3d@1/*#editor", "instanceId": 1, "matches": false }\n  ]'
        assert raw.count(last) == 1
        rows = "".join(f',\n    {{ "pluginId": "{scope["pluginId"]}", "appId": "{scope["appId"]}", "instanceId": {scope["instanceId"]}, "matches": {str(scope["matches"]).lower()} }}' for scope in added)
        raw = raw.replace(last, last[: -len("\n  ]")] + rows + "\n  ]")
        json.loads(raw)
        FIXTURE.write_text(raw, encoding="utf-8")
    print("ok")


main()
