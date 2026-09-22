#!/usr/bin/env python3
"""⚠️ CA1 declaration codemod — inserts one `.action_destructive("<id>")` or
`.action_audience("<id>", …Input)` chain step immediately after the line that DECLARES the verb, for
every `semio-os-mcp audit` finding of ticket 26/09/18 (capture `🗑️generated/ca1-audit-before.txt`).

Guards (a name-keyed edit without a region guard hit production code on this repo before):
  * the anchor line must contain the expected fragment, verbatim;
  * the file must not already carry the inserted declaration for that id;
  * per file the insertions are applied in descending line order so earlier anchors do not move;
  * nothing is written unless every anchor in the plan matches, and a per-file diffstat is printed.

Usage: 🐍️ca1-declare.py [--dry-run] [latent]
"""
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[7]
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"


def editor(plugin: str, artifact: str) -> str:
    return f"✏️s/🔌️plugins/{plugin}/🗿️artifacts/{artifact}/{SUBSET}"


# (file, anchor line, fragment that must be on the anchor line, inserted line)
PLAN = [
    (editor("🏛️architect", "🏛️program"), 1606, 'ActionDefinition::new("setActiveExample"', '            .action_destructive("setActiveExample")'),
    (editor("🌍️gis", "🗺️gismap"), 1240, '.mutation("deleteFeature"', '            .action_destructive("deleteFeature")'),
    (editor("📏️layout", "📏️layout"), 1406, 'bounded_catalog("deleteSelection"', '            .action_destructive("deleteSelection")'),
    (editor("💠️lowpoly", "💠️lowpoly"), 2263, '.mutation("deleteSelection"', '            .action_destructive("deleteSelection")'),
    (editor("📖️playbook", "📖️playbook"), 689, '.mutation("removeStep"', '        .action_destructive("removeStep")'),
    (editor("📖️playbook", "📖️playbook"), 692, '.mutation("removeBlock"', '        .action_destructive("removeBlock")'),
    (editor("🎬️sequence", "🎬️sequence"), 3676, 'ActionDefinition::new("setActiveExample"', '            .action_destructive("setActiveExample")'),
    (editor("🌿️vcs", "🌿️vcs"), 1091, 'ActionDefinition::new("setActiveExample"', '            .action_destructive("setActiveExample")'),
    (editor("➗️mathematical", "➗️equation"), 1503, 'ActionDefinition::new("setActiveExample"', '        .action_destructive("setActiveExample")'),
    (editor("🔱️trinity", "♻️rewriting"), 1067, 'ActionDefinition::new("setActiveExample"', '            .action_destructive("setActiveExample")'),
    (editor("📕️norm", "🧱️din4108"), 249, ')', '            .action_destructive("setSnapshot")'),
    (editor("🀄️wfc", "◻️2d"), 1099, '.action_args(WFC_2D_SET_ACTIVE_EXAMPLE', '        .action_destructive(WFC_2D_SET_ACTIVE_EXAMPLE)'),
    (editor("🀄️wfc", "◻️2d"), 1099, '.action_args(WFC_2D_SET_ACTIVE_EXAMPLE', '        .action_destructive("delete-slot")'),
    (editor("🀄️wfc", "🧊️3d"), 1045, '.action_destructive("setActiveExample")', '        .action_destructive("delete-slot")'),
    (editor("🀄️wfc", "🖼️bitmap"), 983, '.action_destructive("setActiveExample")', '        .action_destructive("remove-palette-color")'),
    (editor("🧩️puzzle", "🧊️3d"), 8488, '.mutation("engagementSubmit"', '            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)'),
    (editor("🧩️puzzle", "🧊️3d"), 8526, 'ActionDefinition::new("engagementInput"', '            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)'),
    (editor("🧩️puzzle", "🧊️3d"), 8527, 'ActionDefinition::new("engagementAbort"', '            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)'),
    (editor("🧩️puzzle", "🧊️3d"), 8546, 'ActionDefinition::new("worldPointerDown"', '            .action_audience("worldPointerDown", semio_framework_plugin::CapabilityAudience::Input)'),
]

# 🕵️ `--plan latent` — `🧩️puzzle`'s committed descriptor does not decode, so these carry no audit
# finding today; they become findings the moment a describe of it succeeds (CE3's). Found by
# 🐍️ca1-source-lexicon-scan.py run per ARTIFACT rather than per plugin, which is what un-masks an id
# a SIBLING artifact of the same plugin already declares.
LATENT_PLAN = [
    (editor("🧩️puzzle", "◻️2d"), 5340, 'bounded_catalog("deleteEdge"', '            .action_destructive("deleteEdge")'),
    (editor("🧩️puzzle", "◻️2d"), 5356, 'ActionDefinition::new("engagementInput"', '            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)'),
    (editor("🧩️puzzle", "◻️2d"), 5362, 'puzzle2d_internal_action("engagementSubmit"', '            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)'),
    (editor("🧩️puzzle", "◻️2d"), 5363, 'ActionDefinition::new("engagementAbort"', '            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)'),
    (editor("🧩️puzzle", "◻️2d"), 5376, 'bounded_catalog("deleteTargetRegion"', '            .action_destructive("deleteTargetRegion")'),
    (editor("🧩️puzzle", "🖐️5d"), 9839, '.mutation("engagementSubmit"', '            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)'),
    (editor("🧩️puzzle", "🖐️5d"), 9869, 'ActionDefinition::new("engagementInput"', '            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)'),
    (editor("🧩️puzzle", "🖐️5d"), 9870, 'ActionDefinition::new("engagementAbort"', '            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)'),
]


def main() -> int:
    dry = "--dry-run" in sys.argv
    plan = LATENT_PLAN if "latent" in sys.argv else PLAN
    by_file: dict[str, list[tuple[int, str]]] = {}
    for relative, line_number, fragment, inserted in plan:
        path = REPO / relative
        if not path.exists():
            print(f"REFUSED {relative}: missing")
            return 1
        lines = path.read_text(encoding="utf8").splitlines()
        anchor = lines[line_number - 1]
        if fragment not in anchor:
            print(f"REFUSED {relative}:{line_number}: anchor does not carry {fragment!r}")
            print(f"   {anchor.strip()[:200]}")
            return 1
        if inserted.strip() in (candidate.strip() for candidate in lines):
            print(f"SKIP {relative}:{line_number}: {inserted.strip()} already declared")
            continue
        by_file.setdefault(relative, []).append((line_number, inserted))

    for relative, rows in by_file.items():
        path = REPO / relative
        lines = path.read_text(encoding="utf8").splitlines(keepends=True)
        for line_number, inserted in sorted(rows, reverse=True):
            lines.insert(line_number, inserted + "\n")
        if not dry:
            path.write_text("".join(lines), encoding="utf8")
        print(f"+{len(rows)} {relative}")
    if not dry:
        print(subprocess.run(["git", "diff", "--stat", "--", *[str(REPO / relative) for relative in by_file]], cwd=REPO, capture_output=True, text=True).stdout.strip())
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
