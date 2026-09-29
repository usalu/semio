#!/usr/bin/env python3
"""🆔️ WG11 session 14d — set for the first train after the chain (T6): the Display ▸ Windows rows carry ONE element-id scheme on
both renderers, fixed by the schema.

Measured (WG11 isolated re-run, `.🧬semio/🌐hub/s14-wg11-captures/isolated-1.tsv`): `the_display_windows_leaf_publishes_reacts_kind_and_
projection_ids` and `the_expanded_display_taxonomy_paints_in_the_mounted_react_order` red — the wgpu shell composed the projection
rows through `element_id_segment` (`…axonometric.axonometricIsometric`, 09-28) while React's `buildDisplayWindowsTree` still glued raw
ids (`…axonometric.axonometric-isometric`, `…windows.puzzle3d-main`), and the shared fixture pinned React's raw form. Neither is one
scheme: raw kind/template ids are not element ids (`ELEMENT_ID_PATTERN`, `🆔️ElementId/🟦️.tsx` ≡ `is_element_id`, `🛂️manifest`).

Decision (coordinator 07:2x): both renderers compose every Display window row id through the element-id grammar's own composer —
React `childElementId`, wgpu `semio_framework::child_element_id` (byte-identical mirrors) — the section `framework.display.windows.
<kind>`, its `.kind` leaf and the whole projection taxonomy `….projection.<template>…` — and the Projection PANE's own rows
(`framework.worldOrbit.projection.<window>.<template>`, React's `projection-pane` fixture: `…paneTop.threePoint`), which glued the
raw template id so the pane's default selection (stamped under the element id) never matched a row. The schema of the shared fixture declares
the grammar (`definitions.elementId`, the same pattern) for every row/section id, so Ajv refuses a raw id in the fixture itself;
the fixture's ids are rewritten through the grammar; laws: React (mounted Tree DOM ids == fixture, each `isElementId`) and wgpu
(retained keys and painted order == fixture, each `is_element_id`).

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/display-element-ids/` and applies;
`--revert` restores the backups.
"""

import difflib
import json
import re
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SHELL = ENGINE / "🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
PANELS = ENGINE / "🧱️elements/📌️ChromePanels/🟦️.tsx"
PANELS_LAWS = ENGINE / "🧱️elements/📌️ChromePanels/🧪️tests/🧩️component/🟦️.tsx"
CONTRACT_LAWS = ENGINE / "🧪️tests/🔬️engine-contract/🟦️.ts"
FIXTURE = ENGINE / "🧫️fixtures/🪟️window-lifecycle-template-drag/🔣️.json"
SCHEMA = ENGINE / "🧬️schema/🪟️window-lifecycle-template-drag/🔣️.json"
DISPLAY_LAWS = ENGINE / "🧱️elements/🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/display-element-ids"
ELEMENT_ID = re.compile(r"^[a-z][a-zA-Z0-9]*(\.[a-z][a-zA-Z0-9]*)*$")

SHELL_EDITS = [
    (
        '''                world_projection_template_rows(&format!("framework.display.windows.{}.projection", kind.id), &kind.id, 0, &mut 0)''',
        '''                world_projection_template_rows(&semio_framework::child_element_id(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &[kind.id.as_str(), "projection"]), &kind.id, 0, &mut 0)''',
    ),
    (
        '''                id: format!("framework.display.windows.{}.kind", kind.id),''',
        '''                id: semio_framework::child_element_id(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &[kind.id.as_str(), "kind"]),''',
    ),
    (
        '''            sections.push(UiTreeSectionNode { header_toolbar: None, id: format!("framework.display.windows.{}", kind.id), label: Some(Label::data(label)), default_open: Some(false), presence: UiPresence::default(), items, window: None });''',
        '''            sections.push(UiTreeSectionNode {
                header_toolbar: None,
                id: semio_framework::child_element_id(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, &[kind.id.as_str()]),
                label: Some(Label::data(label)),
                default_open: Some(false),
                presence: UiPresence::default(),
                items,
                window: None,
            });''',
    ),
    (
        '''/// [`WORLD_PROJECTION_TEMPLATES`]' own `depth` column: a row's id is its parent's id plus its
/// template id, which is byte-identical to React's growing `idPrefix`
/// (`framework.display.windows.<kind>.projection.parallel.axonometric.axonometric-isometric`).''',
        '''/// [`WORLD_PROJECTION_TEMPLATES`]' own `depth` column: a row's id is its parent's id plus its template
/// id, composed through the element-id grammar exactly as React's `childElementId` composes it
/// (`framework.display.windows.puzzle3dMain.projection.parallel.axonometric.axonometricIsometric`).''',
    ),
    (
        '''        let id = format!("{prefix}.{}", semio_framework::element_id_segment(template.id));''',
        '''        let id = semio_framework::child_element_id(prefix, &[template.id]);''',
    ),
    (
        '''        let id = format!("{prefix}.{}", template.id);''',
        '''        let id = semio_framework::child_element_id(prefix, &[template.id]);''',
    ),
]

PANELS_EDITS = [
    (
        '''  singleTreeLeaf,
  uiDataLabel,
  windowTemplatePaletteTreeDragController,
} from "@semio-tech/ui-react";''',
        '''  singleTreeLeaf,
  uiDataLabel,
  windowTemplatePaletteTreeDragController,
  childElementId,
} from "@semio-tech/ui-react";''',
    ),
    (
        '''      id: `${idPrefix}.${template.id}`,''',
        '''      id: childElementId(idPrefix, template.id),''',
    ),
    (
        '''      ...(template.children?.length ? { items: worldProjectionTemplatesToTreeItems(template.children, windowKindId, `${idPrefix}.${template.id}`) } : {}),''',
        '''      ...(template.children?.length ? { items: worldProjectionTemplatesToTreeItems(template.children, windowKindId, childElementId(idPrefix, template.id)) } : {}),''',
    ),
    (
        '''          id: `framework.display.windows.${kind.id}`,''',
        '''          id: childElementId(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, kind.id),''',
    ),
    (
        '''                  ...worldProjectionTemplatesToTreeItems(createWorldProjectionTemplates({ controllerId: kind.id }), kind.id, `framework.display.windows.${kind.id}.projection`),
                  { id: `framework.display.windows.${kind.id}.kind`,''',
        '''                  ...worldProjectionTemplatesToTreeItems(createWorldProjectionTemplates({ controllerId: kind.id }), kind.id, childElementId(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, kind.id, "projection")),
                  { id: childElementId(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, kind.id, "kind"),''',
    ),
    (
        '''                  {
                    id: `framework.display.windows.${kind.id}.kind`,''',
        '''                  {
                    id: childElementId(FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, kind.id, "kind"),''',
    ),
]

PANELS_LAW_EDITS = [
    (
        '''import { Tree, type PanelTabLeaf } from "@semio-tech/ui-react";''',
        '''import { Tree, isElementId, type PanelTabLeaf } from "@semio-tech/ui-react";''',
    ),
    (
        '''    expect(ids).toEqual(fixture.displayResolvedOrder.topToBottomIds);
  });''',
        '''    expect(ids).toEqual(fixture.displayResolvedOrder.topToBottomIds);
    expect(ids.filter((id) => !isElementId(id)), "every Display row id is one element id").toEqual([]);
  });''',
    ),
]

CONTRACT_LAW_EDITS = [
    (
        '''    expect(items.some((row) => row.id === "framework.display.windows.puzzle3d-main.kind")).toBe(true);''',
        '''    expect(items.some((row) => row.id === "framework.display.windows.puzzle3dMain.kind")).toBe(true);''',
    )
]

DISPLAY_LAW_EDITS = [
    (
        '''        "framework.display.windows.main.projection.parallel.axonometric.axonometric-isometric",''',
        '''        "framework.display.windows.main.projection.parallel.axonometric.axonometricIsometric",''',
    ),
    (
        '''        assert!(keys.iter().any(|key| key.ends_with(react_id)), "🔀️ '{react_id}' is React's own nested template id, got {keys:?}");
    }''',
        '''        assert!(keys.iter().any(|key| key.ends_with(react_id)), "🔀️ '{react_id}' is React's own nested template id, got {keys:?}");
    }
    let row_ids: Vec<&str> = keys.iter().filter_map(|key| key.split_once('/').map(|(_, id)| id)).filter(|id| id.starts_with("framework.display.windows.")).collect();
    assert!(row_ids.iter().all(|id| semio_framework::is_element_id(id)), "🆔️ every Display row id is one element id, as React's `childElementId` composes it: {row_ids:?}");''',
    ),
    (
        '''        .map(|id| id.replace("puzzle3d-main", "main"))''',
        '''        .map(|id| id.replace("puzzle3dMain", "main"))''',
    ),
    (
        '''    let expected: Vec<&str> = fixture["displayResolvedOrder"]["topToBottomIds"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect();
    let mut shell = display_shell();''',
        '''    let expected: Vec<&str> = fixture["displayResolvedOrder"]["topToBottomIds"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect();
    assert!(expected.iter().all(|id| semio_framework::is_element_id(id)), "🆔️ the shared order names element ids only: {expected:?}");
    let mut shell = display_shell();''',
    ),
]

SCHEMA_EDITS = [
    (
        '''          "items": { "type": "string", "minLength": 1 }
        }
      }
    }''',
        '''          "items": { "$ref": "#/definitions/elementId" }
        }
      }
    }''',
    ),
    (
        '''            "id": { "type": "string", "minLength": 1 },
            "defaultOpen": { "const": false },
            "rows": {''',
        '''            "id": { "$ref": "#/definitions/elementId" },
            "defaultOpen": { "const": false },
            "rows": {''',
    ),
    (
        '''                  "id": { "type": "string", "minLength": 1 },
                  "branch": { "type": "boolean" },''',
        '''                  "id": { "$ref": "#/definitions/elementId" },
                  "branch": { "type": "boolean" },''',
    ),
    (
        '''  "definitions": {
    "rect": {''',
        '''  "definitions": {
    "elementId": {
      "description": "🆔️ The UI element id grammar — `ELEMENT_ID_PATTERN` (`🖱️ui/🧱️elements/🆔️ElementId/🟦️.tsx`) ≡ `is_element_id` (`🛂️manifest/🦀️.rs`): dot-separated camelCase segments, each starting with a lowercase letter.",
      "type": "string",
      "pattern": "^[a-z][a-zA-Z0-9]*(\\\\.[a-z][a-zA-Z0-9]*)*$"
    },
    "rect": {''',
    ),
]


def element_id_segment(raw: str) -> str:
    segment, capitalize = "", False
    for ch in raw:
        if ch in "-_ .":
            capitalize = True
            continue
        if not (ch.isascii() and ch.isalnum()):
            continue
        if not segment:
            segment += ch.lower()
        elif capitalize:
            segment += ch.upper()
            capitalize = False
        else:
            segment += ch
    return segment


def fixture_after(source: str) -> str:
    def rewrite(match: re.Match) -> str:
        return "framework.display.windows." + ".".join(element_id_segment(segment) for segment in match.group(1).split("."))

    after = re.sub(r"framework\.display\.windows\.([A-Za-z0-9.\-_]+)", rewrite, source)
    if after == source:
        sys.exit("the fixture's Display ids are already element ids — landed already")
    value = json.loads(after)
    order = value["displayResolvedOrder"]["topToBottomIds"]
    section = value["displayBranchPublication"]["section"]
    ids = order + [section["id"]] + [row["id"] for row in section["rows"]]
    bad = [id for id in ids if not ELEMENT_ID.match(id)]
    if bad:
        sys.exit(f"rewritten ids still outside the grammar: {bad}")
    if value["displayResolvedOrder"]["windowKindId"] != "puzzle3d-main":
        sys.exit("windowKindId is a kind id, not an element id — it must stay raw")
    return after


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


FILES = [SHELL, PANELS, PANELS_LAWS, CONTRACT_LAWS, FIXTURE, SCHEMA, DISPLAY_LAWS]


def plans():
    read = {path: path.read_text(encoding="utf-8") for path in FILES}
    schema_after = replaced(SCHEMA, read[SCHEMA], SCHEMA_EDITS)
    json.loads(schema_after)
    pattern = json.loads(schema_after)["definitions"]["elementId"]["pattern"]
    if pattern != ELEMENT_ID.pattern:
        sys.exit(f"schema pattern {pattern!r} is not ELEMENT_ID_PATTERN")
    return [
        (SHELL, read[SHELL], replaced(SHELL, read[SHELL], SHELL_EDITS)),
        (PANELS, read[PANELS], replaced(PANELS, read[PANELS], PANELS_EDITS)),
        (PANELS_LAWS, read[PANELS_LAWS], replaced(PANELS_LAWS, read[PANELS_LAWS], PANELS_LAW_EDITS)),
        (CONTRACT_LAWS, read[CONTRACT_LAWS], replaced(CONTRACT_LAWS, read[CONTRACT_LAWS], CONTRACT_LAW_EDITS)),
        (FIXTURE, read[FIXTURE], fixture_after(read[FIXTURE])),
        (SCHEMA, read[SCHEMA], schema_after),
        (DISPLAY_LAWS, read[DISPLAY_LAWS], replaced(DISPLAY_LAWS, read[DISPLAY_LAWS], DISPLAY_LAW_EDITS)),
    ]


def main():
    if "--revert" in sys.argv:
        for path in FILES:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print(f"REVERTED: {len(FILES)} files restored from backups")
        return
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=0))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files (crate: semio-framework-os-renderer-wgpu; TS package: React renderer engine 📌️ChromePanels + laws; shared fixture + schema)")


if __name__ == "__main__":
    main()
