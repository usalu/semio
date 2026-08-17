#!/usr/bin/env python3
"""
Rewind Deleted Edits From Session Transcripts

Parses JSONL transcript files from VS Code Copilot sessions, extracts all
replace_string_in_file / multi_replace_string_in_file operations, and lets
you replay or rewind (reverse) edits that were lost.

Usage:
    python3 rewind.py --list                         # List all replacements
    python3 rewind.py --list --file <path>           # List replacements for a file
    python3 rewind.py --replay --file <path>         # Re-apply edits for a file
    python3 rewind.py --rewind --file <path>         # Reverse edits for a file
    python3 rewind.py --diff --file <path>           # Show what would change
    python3 rewind.py --replay --after <timestamp>   # Replay edits after a timestamp
    python3 rewind.py --replay --session <id>        # Replay edits from a session
    python3 rewind.py --check --file <path>          # Check which edits are missing
"""

import argparse
import glob
import json
import os
import sys
from dataclasses import dataclass, field
from typing import Optional


@dataclass
class Replacement:
    """A single string replacement operation extracted from a transcript."""

    session_id: str
    timestamp: str
    tool_name: str
    file_path: str
    old_string: str
    new_string: str
    index: int = 0  # index within a multi-replace batch
    explanation: str = ""

    def __repr__(self):
        return (
            f"Replacement(ts={self.timestamp}, "
            f"file={os.path.basename(self.file_path)}, "
            f"old={len(self.old_string)}→new={len(self.new_string)})"
        )


def parse_transcript(jsonl_path: str) -> list[Replacement]:
    """Parse a JSONL transcript file and extract all replacement operations."""
    session_id = os.path.basename(jsonl_path).replace(".jsonl", "")
    replacements = []

    with open(jsonl_path, "r", encoding="utf-8") as f:
        for line_num, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            try:
                entry = json.loads(line)
            except json.JSONDecodeError:
                continue

            if entry.get("type") != "assistant.message":
                continue

            data = entry.get("data", {})
            timestamp = entry.get("timestamp", "")

            for req in data.get("toolRequests", []):
                name = req.get("name", "")
                if "replace" not in name.lower():
                    continue

                try:
                    args = json.loads(req.get("arguments", "{}"))
                except json.JSONDecodeError:
                    continue

                if "replacements" in args:
                    # multi_replace_string_in_file
                    explanation = args.get("explanation", "")
                    for i, r in enumerate(args["replacements"]):
                        replacements.append(
                            Replacement(
                                session_id=session_id,
                                timestamp=timestamp,
                                tool_name=name,
                                file_path=r.get("filePath", ""),
                                old_string=r.get("oldString", ""),
                                new_string=r.get("newString", ""),
                                index=i,
                                explanation=explanation,
                            )
                        )
                else:
                    replacements.append(
                        Replacement(
                            session_id=session_id,
                            timestamp=timestamp,
                            tool_name=name,
                            file_path=args.get("filePath", ""),
                            old_string=args.get("oldString", ""),
                            new_string=args.get("newString", ""),
                            explanation=args.get("explanation", ""),
                        )
                    )

    return replacements


def find_transcripts(transcript_dir: Optional[str] = None) -> list[str]:
    """Find all JSONL transcript files."""
    if transcript_dir:
        return sorted(glob.glob(os.path.join(transcript_dir, "*.jsonl")))

    default_dirs = [
        "/home/vscode/.vscode-server/data/User/workspaceStorage/10f09a20a540f520512a4aa0af089115/GitHub.copilot-chat/transcripts",
        os.path.expanduser(
            "~/.vscode-server/data/User/workspaceStorage/*/GitHub.copilot-chat/transcripts"
        ),
    ]
    for d in default_dirs:
        files = sorted(glob.glob(os.path.join(d, "*.jsonl")))
        if files:
            return files
    return []


def load_all_replacements(
    transcript_dir: Optional[str] = None,
    file_filter: Optional[str] = None,
    session_filter: Optional[str] = None,
    after_timestamp: Optional[str] = None,
    before_timestamp: Optional[str] = None,
) -> list[Replacement]:
    """Load and filter replacements from all transcripts."""
    all_replacements = []
    for jsonl_path in find_transcripts(transcript_dir):
        sid = os.path.basename(jsonl_path).replace(".jsonl", "")
        if session_filter and session_filter not in sid:
            continue
        all_replacements.extend(parse_transcript(jsonl_path))

    # Sort by timestamp
    all_replacements.sort(key=lambda r: r.timestamp)

    # Apply filters
    if file_filter:
        all_replacements = [r for r in all_replacements if file_filter in r.file_path]
    if after_timestamp:
        all_replacements = [
            r for r in all_replacements if r.timestamp > after_timestamp
        ]
    if before_timestamp:
        all_replacements = [
            r for r in all_replacements if r.timestamp < before_timestamp
        ]

    return all_replacements


def check_missing(replacements: list[Replacement], file_path: str) -> list[Replacement]:
    """Check which replacement results (newStrings) are missing from the current file."""
    if not os.path.exists(file_path):
        print(f"File not found: {file_path}", file=sys.stderr)
        return []

    with open(file_path, "r", encoding="utf-8") as f:
        content = f.read()

    missing = []
    # Walk through replacements forward. For each replacement, if newString
    # is NOT in the file, it's "missing" (was lost/deleted).
    # But if a later replacement modifies the same region, we need to
    # track the latest state.

    # Build a map of replacement chains: track for each oldString→newString
    # whether later replacements consumed the newString as their oldString
    new_strings_consumed = set()
    for i, r in enumerate(replacements):
        if r.file_path != file_path:
            continue
        # Check if this replacement's old_string is a new_string from a previous one
        for j in range(i):
            prev = replacements[j]
            if prev.file_path == file_path and prev.new_string == r.old_string:
                new_strings_consumed.add(j)

    for i, r in enumerate(replacements):
        if r.file_path != file_path:
            continue
        if i in new_strings_consumed:
            # This replacement's newString was consumed by a later replacement
            continue
        # Check if the newString is present in the current file
        if r.new_string not in content:
            missing.append(r)

    return missing


def replay_replacements(
    replacements: list[Replacement], dry_run: bool = False
) -> dict[str, int]:
    """Re-apply replacements to their target files. Returns counts of applied per file."""
    counts: dict[str, int] = {}
    for r in replacements:
        if not os.path.exists(r.file_path):
            print(f"  SKIP (file not found): {r.file_path}", file=sys.stderr)
            continue

        with open(r.file_path, "r", encoding="utf-8") as f:
            content = f.read()

        if r.new_string in content:
            # Already applied
            continue

        if r.old_string not in content:
            print(
                f"  SKIP (oldString not found): {r.timestamp} "
                f"{os.path.basename(r.file_path)} "
                f"old={len(r.old_string)} new={len(r.new_string)}",
                file=sys.stderr,
            )
            continue

        new_content = content.replace(r.old_string, r.new_string, 1)

        if dry_run:
            print(f"  WOULD APPLY: {r.timestamp} {os.path.basename(r.file_path)}")
        else:
            with open(r.file_path, "w", encoding="utf-8") as f:
                f.write(new_content)
            print(f"  APPLIED: {r.timestamp} {os.path.basename(r.file_path)}")

        counts[r.file_path] = counts.get(r.file_path, 0) + 1

    return counts


def rewind_replacements(
    replacements: list[Replacement], dry_run: bool = False
) -> dict[str, int]:
    """Reverse replacements (swap old/new) in reverse chronological order."""
    counts: dict[str, int] = {}
    for r in reversed(replacements):
        if not os.path.exists(r.file_path):
            print(f"  SKIP (file not found): {r.file_path}", file=sys.stderr)
            continue

        with open(r.file_path, "r", encoding="utf-8") as f:
            content = f.read()

        if r.new_string not in content:
            print(
                f"  SKIP (newString not in file): {r.timestamp} "
                f"{os.path.basename(r.file_path)}",
                file=sys.stderr,
            )
            continue

        new_content = content.replace(r.new_string, r.old_string, 1)

        if dry_run:
            print(f"  WOULD REWIND: {r.timestamp} {os.path.basename(r.file_path)}")
        else:
            with open(r.file_path, "w", encoding="utf-8") as f:
                f.write(new_content)
            print(f"  REWOUND: {r.timestamp} {os.path.basename(r.file_path)}")

        counts[r.file_path] = counts.get(r.file_path, 0) + 1

    return counts


def main():
    parser = argparse.ArgumentParser(
        description="Rewind or replay session transcript edits"
    )
    parser.add_argument(
        "--list", action="store_true", help="List all replacement operations"
    )
    parser.add_argument(
        "--check", action="store_true", help="Check which edits are missing from files"
    )
    parser.add_argument(
        "--replay", action="store_true", help="Re-apply edits (forward)"
    )
    parser.add_argument(
        "--rewind", action="store_true", help="Reverse edits (backward)"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Show what would change without modifying files",
    )
    parser.add_argument(
        "--file", type=str, help="Filter by file path (substring match)"
    )
    parser.add_argument(
        "--session", type=str, help="Filter by session ID (substring match)"
    )
    parser.add_argument(
        "--after", type=str, help="Only include replacements after this ISO timestamp"
    )
    parser.add_argument(
        "--before", type=str, help="Only include replacements before this ISO timestamp"
    )
    parser.add_argument(
        "--transcript-dir", type=str, help="Directory containing transcript JSONL files"
    )
    parser.add_argument(
        "--verbose",
        "-v",
        action="store_true",
        help="Show full oldString/newString content",
    )
    parser.add_argument(
        "--json-output", action="store_true", help="Output as JSON instead of text"
    )

    args = parser.parse_args()

    if not any([args.list, args.check, args.replay, args.rewind]):
        parser.print_help()
        sys.exit(1)

    replacements = load_all_replacements(
        transcript_dir=args.transcript_dir,
        file_filter=args.file,
        session_filter=args.session,
        after_timestamp=args.after,
        before_timestamp=args.before,
    )

    if not replacements:
        print("No replacements found matching the filters.", file=sys.stderr)
        sys.exit(0)

    if args.list:
        if args.json_output:
            output = []
            for r in replacements:
                output.append(
                    {
                        "timestamp": r.timestamp,
                        "session": r.session_id[:8],
                        "tool": r.tool_name,
                        "filePath": r.file_path,
                        "oldStringLength": len(r.old_string),
                        "newStringLength": len(r.new_string),
                        "explanation": r.explanation,
                        "oldString": r.old_string
                        if args.verbose
                        else r.old_string[:80],
                        "newString": r.new_string
                        if args.verbose
                        else r.new_string[:80],
                    }
                )
            print(json.dumps(output, indent=2))
        else:
            print(f"Found {len(replacements)} replacement(s):\n")
            for i, r in enumerate(replacements):
                fname = os.path.basename(r.file_path)
                print(
                    f"  [{i:3d}] {r.timestamp} | {r.session_id[:8]}.. | {fname} | old:{len(r.old_string)} → new:{len(r.new_string)}"
                )
                if r.explanation:
                    print(f"        {r.explanation[:100]}")
                if args.verbose:
                    print(f"        OLD: {repr(r.old_string[:200])}")
                    print(f"        NEW: {repr(r.new_string[:200])}")
            print()

    if args.check:
        # Group by file
        files = set(r.file_path for r in replacements)
        total_missing = 0
        for fp in sorted(files):
            file_replacements = [r for r in replacements if r.file_path == fp]
            missing = check_missing(file_replacements, fp)
            if missing:
                fname = os.path.basename(fp)
                print(f"\n{fname} - {len(missing)} missing edit(s):")
                for m in missing:
                    print(
                        f"  {m.timestamp} | {m.session_id[:8]}.. | old:{len(m.old_string)} → new:{len(m.new_string)}"
                    )
                    if args.verbose:
                        print(f"    OLD: {repr(m.old_string[:200])}")
                        print(f"    NEW: {repr(m.new_string[:200])}")
                total_missing += len(missing)
            else:
                print(f"\n{os.path.basename(fp)} - all edits present ✓️")

        if total_missing > 0:
            print(f"\nTotal missing: {total_missing}")
            print("Use --replay to re-apply missing edits.")
        else:
            print("\nAll edits are present in the current files.")

    if args.replay:
        print("Replaying replacements...")
        counts = replay_replacements(replacements, dry_run=args.dry_run)
        if counts:
            print(
                f"\n{'Would apply' if args.dry_run else 'Applied'} {sum(counts.values())} replacement(s) to {len(counts)} file(s)."
            )
        else:
            print("\nNo replacements needed (all already applied or not applicable).")

    if args.rewind:
        print("Rewinding replacements...")
        counts = rewind_replacements(replacements, dry_run=args.dry_run)
        if counts:
            print(
                f"\n{'Would rewind' if args.dry_run else 'Rewound'} {sum(counts.values())} replacement(s) in {len(counts)} file(s)."
            )
        else:
            print("\nNo replacements to rewind.")


if __name__ == "__main__":
    main()
