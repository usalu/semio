#!/usr/bin/env python3
"""Migrate all .repo JSON files to use the repo ID system for resource references.

Conversions:
- Goal references (ticket.json "goal", goal.json "parent"): path → 🎯flat1🎯flat2...
- Contributor references (interaction "author", agent "contributor"): github → 🧑‍💻flatgithub
"""

import glob
import json
import os
import re
import sys
import unicodedata

EMOJI_GOAL = "🎯"
EMOJI_CONTRIBUTOR = "🧑‍💻"

# Text-default emojis that need variation selectors (from emojiText in main.go)
TEXT_DEFAULT_EMOJIS = [
    "\U0001f3d7",
    "\u2328",
    "\U0001f5b1",
    "\U0001f5c3",
    "\u2699",
    "\u2696",
    "\U0001f3f7",
    "\U0001f6e0",
    "\u2702",
    "\U0001f6e1",
    "\U0001f5d1",
    "\u2600",
    "\u23f1",
    "\u270f",
    "\U0001f46e",
]


def emoji_text(emoji: str) -> str:
    """Python port of emojiText() from main.go."""
    stripped = emoji.replace("\ufe0e", "")
    base = stripped.replace("\ufe0f", "")
    for td in TEXT_DEFAULT_EMOJIS:
        if td in base:
            return base.replace(td, td + "\ufe0f")
    return base


def flat(text: str) -> str:
    """Python port of Flat() from main.go."""
    buf = []
    for ch in text:
        if ch.isascii() and ch.isalpha():
            buf.append(ch)
        elif ch.isascii() and ch.isdigit():
            buf.append(ch)
        elif ord(ch) > 0x7F:
            buf.append(ch)
    return "".join(buf).lower()


def goal_path_to_semio_id(path: str) -> str:
    """Convert a goal filesystem path to a repo ID.

    e.g. "AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI"
      → "🎯aioptimizedrepo🎯repoclient🎯repobinary🎯repocli"
    """
    if not path:
        return ""
    goal_emoji = emoji_text(EMOJI_GOAL)
    parts = path.split("/")
    return "".join(goal_emoji + flat(p) for p in parts)


def contributor_to_semio_id(github_handle: str) -> str:
    """Convert a contributor reference to a repo ID.

    e.g. "usalu" → "🧑‍💻usalu"
    """
    if not github_handle:
        return ""
    contrib_emoji = emoji_text(EMOJI_CONTRIBUTOR)
    return contrib_emoji + flat(github_handle)


def is_already_semio_id(value: str, emoji: str) -> bool:
    """Check if a value is already in repo ID format."""
    return value.startswith(emoji_text(emoji))


# Build contributor lookup from contributor.json files
def build_contributor_lookup(semio_repo_dir: str) -> dict:
    """Build a mapping of all identifiable strings to github handles."""
    lookup = {}
    contrib_dir = os.path.join(semio_repo_dir, "🧑‍💻")
    if not os.path.exists(contrib_dir):
        return lookup

    for entry in os.listdir(contrib_dir):
        contrib_file = os.path.join(contrib_dir, entry, "contributor.json")
        if not os.path.exists(contrib_file):
            continue
        try:
            with open(contrib_file) as f:
                data = json.load(f)
            github = data.get("github", "")
            if not github:
                continue
            # Map github handle
            lookup[github.lower()] = github
            # Map alternative github handles
            for g in data.get("githubs", []):
                lookup[g.lower()] = github
            # Map name
            name = data.get("name", "")
            if name:
                lookup[name.lower()] = github
            # Map alternative names
            for n in data.get("names", []):
                lookup[n.lower()] = github
            # Map email
            email = data.get("email", "")
            if email:
                lookup[email.lower()] = github
            # Map alternative emails
            for e in data.get("emails", []):
                lookup[e.lower()] = github
            # Map "Name <email>" format
            if name and email:
                lookup[f"{name} <{email}>".lower()] = github
            for n in data.get("names", []):
                if email:
                    lookup[f"{n} <{email}>".lower()] = github
                for e in data.get("emails", []):
                    lookup[f"{n} <{e}>".lower()] = github
        except Exception as e:
            print(f"  Warning: Could not parse {contrib_file}: {e}", file=sys.stderr)

    return lookup


def resolve_contributor(value: str, lookup: dict) -> str:
    """Resolve a contributor reference to a github handle."""
    if not value or value in ("unknown", "", "Migration Script", "GitHub Copilot"):
        return value

    # Already a semio ID
    if is_already_semio_id(value, EMOJI_CONTRIBUTOR):
        return value

    # Try direct lookup
    key = value.lower()
    if key in lookup:
        return lookup[key]

    # Try parsing "Name <email>" format
    match = re.match(r"^(.+?)\s*<(.+?)>$", value)
    if match:
        name, email = match.group(1).strip(), match.group(2).strip()
        if email.lower() in lookup:
            return lookup[email.lower()]
        if name.lower() in lookup:
            return lookup[name.lower()]

    return value


def convert_goal_ref(value: str) -> str:
    """Convert a goal reference to repo ID format."""
    if not value:
        return value

    # Already in semio ID format
    goal_emoji = emoji_text(EMOJI_GOAL)
    if value.startswith(goal_emoji):
        return value

    # Path format (with /): split and convert each segment
    return goal_path_to_semio_id(value)


def convert_contributor_ref(value: str, lookup: dict) -> str:
    """Convert a contributor reference to repo ID format."""
    if not value or value in ("unknown", "", "Migration Script", "GitHub Copilot"):
        return value

    # Already in semio ID format
    contrib_emoji = emoji_text(EMOJI_CONTRIBUTOR)
    if value.startswith(contrib_emoji):
        return value

    # Resolve to github handle first
    github = resolve_contributor(value, lookup)
    if github and github not in ("unknown", "Migration Script", "GitHub Copilot"):
        return contributor_to_semio_id(github)

    return value


def process_ticket_json(filepath: str, lookup: dict, dry_run: bool = False) -> list:
    """Process a ticket.json file and convert resource references."""
    changes = []
    try:
        with open(filepath) as f:
            data = json.load(f)
    except Exception as e:
        print(f"  Warning: Could not parse {filepath}: {e}", file=sys.stderr)
        return changes

    modified = False

    # Convert goal field
    if "goal" in data and data["goal"]:
        old = data["goal"]
        new = convert_goal_ref(old)
        if old != new:
            data["goal"] = new
            changes.append(f"  goal: {old!r} → {new!r}")
            modified = True

    # Convert author in interactions
    for i, inter in enumerate(data.get("interactions", [])):
        if "author" in inter and inter["author"]:
            old = inter["author"]
            new = convert_contributor_ref(old, lookup)
            if old != new:
                data["interactions"][i]["author"] = new
                changes.append(f"  interactions[{i}].author: {old!r} → {new!r}")
                modified = True

    # Convert contributor in agents
    for i, agent in enumerate(data.get("agents", [])):
        if "contributor" in agent and agent["contributor"]:
            old = agent["contributor"]
            new = convert_contributor_ref(old, lookup)
            if old != new:
                data["agents"][i]["contributor"] = new
                changes.append(f"  agents[{i}].contributor: {old!r} → {new!r}")
                modified = True

    if modified and not dry_run:
        with open(filepath, "w") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
            f.write("\n")

    return changes


def process_goal_json(filepath: str, lookup: dict, dry_run: bool = False) -> list:
    """Process a goal.json file and convert resource references."""
    changes = []
    try:
        with open(filepath) as f:
            data = json.load(f)
    except Exception as e:
        print(f"  Warning: Could not parse {filepath}: {e}", file=sys.stderr)
        return changes

    modified = False

    # Convert parent field
    if "parent" in data and data["parent"]:
        old = data["parent"]
        new = convert_goal_ref(old)
        if old != new:
            data["parent"] = new
            changes.append(f"  parent: {old!r} → {new!r}")
            modified = True

    # Convert author in interactions
    for i, inter in enumerate(data.get("interactions") or []):
        if "author" in inter and inter["author"]:
            old = inter["author"]
            new = convert_contributor_ref(old, lookup)
            if old != new:
                data["interactions"][i]["author"] = new
                changes.append(f"  interactions[{i}].author: {old!r} → {new!r}")
                modified = True

    if modified and not dry_run:
        with open(filepath, "w") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
            f.write("\n")

    return changes


def process_event_json(filepath: str, lookup: dict, dry_run: bool = False) -> list:
    """Process an event JSON file and convert resource references."""
    changes = []
    try:
        with open(filepath) as f:
            data = json.load(f)
    except Exception as e:
        return changes

    modified = False

    # Check for contributor field in event
    evt = data.get("event", {})
    if evt and "contributor" in evt and evt["contributor"]:
        old = evt["contributor"]
        new = convert_contributor_ref(old, lookup)
        if old != new:
            data["event"]["contributor"] = new
            changes.append(f"  event.contributor: {old!r} → {new!r}")
            modified = True

    if modified and not dry_run:
        with open(filepath, "w") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
            f.write("\n")

    return changes


def main():
    dry_run = "--dry-run" in sys.argv
    if dry_run:
        print("DRY RUN - no files will be modified")

    semio_repo_dir = "/workspaces/semio/.repo"

    print("Building contributor lookup...")
    lookup = build_contributor_lookup(semio_repo_dir)
    print(f"  Found {len(lookup)} contributor mappings")

    total_changes = 0

    # Process ticket.json files
    print("\nProcessing ticket.json files...")
    ticket_files = glob.glob(
        os.path.join(semio_repo_dir, "🎫", "**", "ticket.json"), recursive=True
    )
    for f in sorted(ticket_files):
        changes = process_ticket_json(f, lookup, dry_run)
        if changes:
            rel = os.path.relpath(f, semio_repo_dir)
            print(f"\n{rel}:")
            for c in changes:
                print(c)
            total_changes += len(changes)

    # Process goal.json files
    print("\nProcessing goal.json files...")
    goal_files = glob.glob(
        os.path.join(semio_repo_dir, "🎯", "**", "goal.json"), recursive=True
    )
    for f in sorted(goal_files):
        changes = process_goal_json(f, lookup, dry_run)
        if changes:
            rel = os.path.relpath(f, semio_repo_dir)
            print(f"\n{rel}:")
            for c in changes:
                print(c)
            total_changes += len(changes)

    print(f"\n{'Would make' if dry_run else 'Made'} {total_changes} total changes")


if __name__ == "__main__":
    main()
