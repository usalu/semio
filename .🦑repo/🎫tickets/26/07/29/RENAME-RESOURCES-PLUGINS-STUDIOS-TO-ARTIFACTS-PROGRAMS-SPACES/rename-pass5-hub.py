#!/usr/bin/env python3
"""Pass 5: hub directory studio -> space rename."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
HUB = ROOT / "framework/product/os/hub"

REPLACEMENTS = [
    ("StudioRecord", "SpaceRecord"),
    ("StudioRole", "SpaceRole"),
    ("create_studio", "create_space"),
    ("list_studios_for_user", "list_spaces_for_user"),
    ("list_studios", "list_spaces"),
    ("studio_from_node", "space_from_node"),
    ("studio_role", "space_role"),
    ("studioRole", "spaceRole"),
    ("hub_studio_membership", "hub_space_membership"),
    ("hub_studio", "hub_space"),
    ("(s:Studio", "(s:Space"),
    ("(:Studio", "(:Space"),
    ("//#region Studios", "//#region Spaces"),
    ("//#endregion Studios", "//#endregion Spaces"),
    ("Users/Studios/", "Users/Spaces/"),
    ("users, studios,", "users, spaces,"),
    ("default studio", "default space"),
    ("A studio:", "A space:"),
    ("scoped to a studio", "scoped to a space"),
    ("tenant/workspace", "tenant/workspace"),
    ("placeholder studio", "placeholder space"),
    ("Seeds a default studio", "Seeds a default space"),
    ("name: 'Studio'", "name: 'Space'"),
    ("'Studio'", "'Space'"),
    ("Studio X", "Space X"),
    ("Studio A", "Space A"),
    ("create studio", "create space"),
    ("user_studio_membership", "user_space_membership"),
    ("seed_creates_studio", "seed_creates_space"),
    ("Studio-scoped", "Space-scoped"),
    ("studio scoping", "space scoping"),
    ("studio role", "space role"),
    ("studio's", "space's"),
    ("whole studio's", "whole space's"),
    ("studio-a", "space-a"),
    ("studio-b", "space-b"),
    ("studio role lookup", "space role lookup"),
    ("The seeded studio", "The seeded space"),
    ("Studio-scoped documents", "Space-scoped documents"),
    ("idx_node_studio_parent", "idx_node_space_parent"),
    ("isStudioPluginFilter", "isSpaceProgramFilter"),
    ("Studio mode", "Space mode"),
    ("studio_mode", "space_mode"),
    ("openStudio", "openSpace"),
    ("importStudio", "importSpace"),
    ("bindStudioFile", "bindSpaceFile"),
    ("StudioBackbonePort", "SpaceBackbonePort"),
    ("StudioPortKind", "SpacePortKind"),
    ("set_studio_name", "set_space_name"),
    ("OsStudioCatalogEntry", "OsSpaceCatalogEntry"),
]

FILES = [
    HUB / "directory/rs/lib.rs",
    HUB / "directory/neo4j/rs/lib.rs",
    HUB / "directory/postgres/rs/lib.rs",
    HUB / "directory/sqlite/rs/lib.rs",
    HUB / "rs/bin.rs",
    ROOT / "framework/product/os/core/rs/lib.rs",
    ROOT / "framework/renderer/wgpu/rs/lib.rs",
    ROOT / "framework/program/registry/📜script.ts",
    ROOT / "framework/renderer/react/index.tsx",
    ROOT / "s/program/rs/lib.rs",
]


def main() -> None:
    touched = []
    for path in FILES:
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        updated = text
        for old, new in REPLACEMENTS:
            updated = updated.replace(old, new)
        if updated != text:
            path.write_text(updated, encoding="utf-8")
            touched.append(str(path.relative_to(ROOT)))
    report = {"pass": 5, "files_touched": len(touched), "files": touched}
    out = Path(__file__).with_name("rename-pass5-report.json")
    out.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
