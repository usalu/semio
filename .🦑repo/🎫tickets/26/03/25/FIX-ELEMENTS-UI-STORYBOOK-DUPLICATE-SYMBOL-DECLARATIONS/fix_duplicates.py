#!/usr/bin/env python3
"""Fix duplicate symbol declarations in elements/ui/index.tsx.

This script:
1. Removes entirely duplicated sections at end of file (lines 34244-34407)
2. Removes duplicate Button story sections (lines 27445-27464, 28211-28305)
3. Prefixes generic exports (Window, Panel, Overlay, Temporary, Base) with story component name
4. Prefixes local variables (createLevelRender, defaultArgs, defaultItems, CodaThemeWrapper, WindowContent) and their references
5. Removes duplicate export default statements (handled by section removal)
"""

import re
import sys

FILE_PATH = "/workspaces/semio/elements/ui/index.tsx"


def main():
    with open(FILE_PATH, "r") as f:
        lines = f.readlines()

    print(f"[DEBUG] Original file: {len(lines)} lines")

    # === STEP 1: Remove duplicate sections ===
    # We need to remove these line ranges (1-indexed):
    # - 27445-27464: Simple duplicate Stories_Button_stories_tsx
    # - 28211-28305: Unprefixed duplicate Button_stories_tsx
    # - 34244-34407+: Duplicate sections at end (Button, tailwind, postcss, eslint, vitest)

    # Convert to 0-indexed ranges to remove
    ranges_to_remove = [
        (27445 - 1, 27464),  # Lines 27445-27464
        (28211 - 1, 28305),  # Lines 28211-28305
        (34244 - 1, len(lines)),  # Lines 34244 to end
    ]

    # Mark lines for removal
    remove_set = set()
    for start, end in ranges_to_remove:
        for i in range(start, end):
            remove_set.add(i)

    # Filter out removed lines
    filtered_lines = [line for i, line in enumerate(lines) if i not in remove_set]
    print(
        f"[DEBUG] After removing duplicate sections: {len(filtered_lines)} lines (removed {len(remove_set)})"
    )

    # === STEP 2: Rebuild and identify story regions ===
    # Now work with the filtered content as a single string
    content = "".join(filtered_lines)

    # === STEP 3: Prefix generic exports ===
    # Pattern: find story regions and prefix Window/Panel/Overlay/Temporary/Base exports
    # We need to know which region we're in to determine the prefix

    # Build a map of regions: find all non-Stories_ region markers
    # These are the ones that need fixing (lines 27465+ after filtering)
    # Region format: // #region 🔖ComponentName_stories_tsx
    # The prefix is ComponentName_stories_

    # We'll process line by line, tracking current region
    result_lines = content.split("\n")

    # Track current story region stack
    current_story_prefix = None
    region_stack = []

    generic_exports = {"Window", "Panel", "Overlay", "Temporary", "Base"}
    local_vars_to_prefix = {
        "createLevelRender",
        "defaultArgs",
        "defaultItems",
        "CodaThemeWrapper",
        "WindowContent",
    }

    # First pass: identify regions and which lines belong to which story region
    line_regions = {}  # line_index -> prefix
    for i, line in enumerate(result_lines):
        stripped = line.strip()

        # Check for region start - only non-Stories_ prefixed ones need fixing
        region_start = re.match(r"^// #region 🔖(\w+_stories_tsx)$", stripped)
        if region_start:
            region_name = region_start.group(1)
            # Extract prefix: e.g., "Accordion_stories_tsx" -> "Accordion_stories_"
            # But skip "Stories_*" prefixed ones as they're already fixed
            if not region_name.startswith("Stories_"):
                prefix = region_name.replace("_tsx", "_")
                region_stack.append(("story", prefix, region_name))
                current_story_prefix = prefix
            else:
                region_stack.append(("stories_prefixed", None, region_name))
            continue

        region_end = re.match(r"^// #endregion 🔖(\w+_stories_tsx)$", stripped)
        if region_end:
            region_name = region_end.group(1)
            # Pop matching region from stack
            if region_stack:
                region_stack.pop()
            # Update current prefix
            current_story_prefix = None
            for item in reversed(region_stack):
                if item[0] == "story":
                    current_story_prefix = item[1]
                    break
            continue

        if current_story_prefix:
            line_regions[i] = current_story_prefix

    # Second pass: do the actual replacements
    new_lines = []
    current_story_prefix = None
    region_stack = []

    for i, line in enumerate(result_lines):
        stripped = line.strip()

        # Track regions
        region_start = re.match(r"^// #region 🔖(\w+_stories_tsx)$", stripped)
        if region_start:
            region_name = region_start.group(1)
            if not region_name.startswith("Stories_"):
                prefix = region_name.replace("_tsx", "_")
                region_stack.append(("story", prefix, region_name))
                current_story_prefix = prefix
            else:
                region_stack.append(("stories_prefixed", None, region_name))
            new_lines.append(line)
            continue

        region_end = re.match(r"^// #endregion 🔖(\w+_stories_tsx)$", stripped)
        if region_end:
            if region_stack:
                region_stack.pop()
            current_story_prefix = None
            for item in reversed(region_stack):
                if item[0] == "story":
                    current_story_prefix = item[1]
                    break
            new_lines.append(line)
            continue

        # Only process lines inside non-Stories_ story regions
        if current_story_prefix:
            modified_line = line

            # Prefix generic exports: export const Window: -> export const Accordion_stories_Window:
            for name in generic_exports:
                # Match export const Window: SomeType = {
                pattern = rf"^(export const ){name}(: \w+_stories_\w+ =)"
                replacement = rf"\g<1>{current_story_prefix}{name}\2"
                modified_line = re.sub(pattern, replacement, modified_line)

            # Prefix local variable declarations
            for var_name in local_vars_to_prefix:
                # Skip if already prefixed
                if f"{current_story_prefix}{var_name}" in modified_line:
                    continue

                # Declaration: const createLevelRender = -> const Accordion_stories_createLevelRender =
                pattern = rf"^(const ){var_name}(\b)"
                if re.search(pattern, modified_line):
                    modified_line = re.sub(
                        pattern,
                        rf"\g<1>{current_story_prefix}{var_name}\2",
                        modified_line,
                    )

                # References: render: createLevelRender( -> render: Accordion_stories_createLevelRender(
                # Also: ...defaultArgs, defaultArgs.something, etc.
                # But NOT already prefixed ones (e.g., Button_stories_createLevelRender)
                # Use negative lookbehind for word char to avoid matching already-prefixed
                ref_pattern = r"(?<!\w)" + var_name + r"(?=\(|,|\.|;|\s|\}|\)|\[)"
                if re.search(ref_pattern, modified_line):
                    # Make sure it's not already prefixed
                    already_prefixed = rf"\w+_stories_{var_name}"
                    if not re.search(already_prefixed, modified_line):
                        modified_line = re.sub(
                            ref_pattern,
                            f"{current_story_prefix}{var_name}",
                            modified_line,
                        )

            new_lines.append(modified_line)
        else:
            new_lines.append(line)

    final_content = "\n".join(new_lines)

    with open(FILE_PATH, "w") as f:
        f.write(final_content)

    print(f"[DEBUG] Written file: {len(new_lines)} lines")

    # Verify no duplicate exports
    import subprocess

    result = subprocess.run(
        ["grep", "-n", "^export const Window:", FILE_PATH],
        capture_output=True,
        text=True,
    )
    if result.stdout.strip():
        print(
            f"[DEBUG] WARNING: Still have unprefixed Window exports:\n{result.stdout}"
        )
    else:
        print("[DEBUG] OK: No unprefixed Window exports remain")

    result = subprocess.run(
        ["grep", "-n", "^export const Panel:", FILE_PATH],
        capture_output=True,
        text=True,
    )
    if result.stdout.strip():
        print(f"[DEBUG] WARNING: Still have unprefixed Panel exports:\n{result.stdout}")
    else:
        print("[DEBUG] OK: No unprefixed Panel exports remain")

    result = subprocess.run(
        ["grep", "-c", "^const createLevelRender ", FILE_PATH],
        capture_output=True,
        text=True,
    )
    count = int(result.stdout.strip()) if result.stdout.strip() else 0
    if count > 0:
        print(
            f"[DEBUG] WARNING: Still have {count} unprefixed createLevelRender declarations"
        )
        result2 = subprocess.run(
            ["grep", "-n", "^const createLevelRender ", FILE_PATH],
            capture_output=True,
            text=True,
        )
        print(result2.stdout)
    else:
        print("[DEBUG] OK: No unprefixed createLevelRender declarations remain")

    result = subprocess.run(
        ["grep", "-c", "^const defaultArgs ", FILE_PATH], capture_output=True, text=True
    )
    count = int(result.stdout.strip()) if result.stdout.strip() else 0
    if count > 0:
        print(
            f"[DEBUG] WARNING: Still have {count} unprefixed defaultArgs declarations"
        )
    else:
        print("[DEBUG] OK: No unprefixed defaultArgs declarations remain")

    result = subprocess.run(
        ["grep", "-c", "^const defaultItems", FILE_PATH], capture_output=True, text=True
    )
    count = int(result.stdout.strip()) if result.stdout.strip() else 0
    if count > 0:
        print(
            f"[DEBUG] WARNING: Still have {count} unprefixed defaultItems declarations"
        )
    else:
        print("[DEBUG] OK: No unprefixed defaultItems declarations remain")

    # Check for duplicate export defaults
    result = subprocess.run(
        ["grep", "-c", "^export default ", FILE_PATH], capture_output=True, text=True
    )
    count = int(result.stdout.strip()) if result.stdout.strip() else 0
    print(f"[DEBUG] export default count: {count}")


if __name__ == "__main__":
    main()
