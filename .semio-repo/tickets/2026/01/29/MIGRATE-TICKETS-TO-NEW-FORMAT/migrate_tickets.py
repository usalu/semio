import os
import json
from pathlib import Path

TICKETS_ROOT = "/workspaces/semio/.semio-repo/tickets"


def migrate_ticket(ticket_dir):
    ticket_json_path = ticket_dir / "ticket.json"
    if not ticket_json_path.exists():
        return

    print(f"Processing {ticket_dir.name}...")

    try:
        with open(ticket_json_path, "r") as f:
            data = json.load(f)
    except Exception as e:
        print(f"Error reading {ticket_json_path}: {e}")
        return

    # Check if migration needed for JSON
    is_old_json = False
    if (
        "dates" in data
        or isinstance(data.get("author"), str)
        or "iterations" not in data
    ):
        is_old_json = True

    new_data = data
    if is_old_json:
        print(f"  Migrating JSON for {ticket_dir.name}")
        # Map fields
        prompt = data.get("prompt", "")
        title = data.get("title", ticket_dir.name)  # Use slug/dir name if title missing
        status = data.get("status", "open")

        # Dates
        created = None
        finished = None
        if "dates" in data:
            created = data["dates"].get("created")
            finished = data["dates"].get("finished")
        else:
            created = data.get("started")

        # Author
        author_old = data.get("author")
        author_obj = (
            {"name": author_old, "email": None}
            if isinstance(author_old, str)
            else author_old
        )

        commit = data.get("commit")
        llm = data.get("llm")
        ui = data.get("ui")

        # Construct iterations
        iteration = {
            "prompt": prompt,
            "llm": llm,
            "ui": ui,
            "author": author_obj,
            "started": created,
            "commit": commit,
        }
        if finished:
            iteration["finished"] = finished

        new_data = {
            "title": title,
            "status": status,
            "prompt": prompt,
            "started": created,
            "iterations": [iteration],
        }

        if finished:
            new_data["finished"] = finished

        if "github" in data:
            new_data["github"] = data["github"]

    # Handle Markdown Merging
    plan_path = ticket_dir / "plan.md"
    log_path = ticket_dir / "log.md"
    summary_path = ticket_dir / "summary.md"
    ticket_md_path = ticket_dir / "ticket.md"

    plan_content = ""
    log_content = ""
    summary_content = ""

    if plan_path.exists():
        with open(plan_path, "r") as f:
            plan_content = f.read().strip()

    if log_path.exists():
        with open(log_path, "r") as f:
            log_content = f.read().strip()

    if summary_path.exists():
        with open(summary_path, "r") as f:
            summary_content = f.read().strip()
    elif is_old_json and data.get("summary"):
        summary_content = data.get("summary").strip()

    # If any old content exists or we are migrating JSON (which might imply we need to ensure ticket.md structure), update ticket.md
    if plan_content or log_content or summary_content or is_old_json:
        print(f"  Updating ticket.md for {ticket_dir.name}")

        current_md = ""
        if ticket_md_path.exists():
            with open(ticket_md_path, "r") as f:
                current_md = f.read()
        else:
            current_md = "# Ticket\n\n## Todos\n\n## Changes\n\n## Log\n\n## Summary\n"

        def replace_section(md, section, content):
            if not content:
                return md
            header = f"## {section}"
            if header not in md:
                md += f"\n\n{header}\n{content}"
            else:
                start = md.find(header)
                # Find next header (starts with #)
                # We search for "\n# "
                next_header_idx = -1
                search_start = start + len(header)

                # Better approach using find with loop
                scan_pos = search_start
                while scan_pos < len(md):
                    next_hash = md.find("\n#", scan_pos)
                    if next_hash == -1:
                        next_header_idx = len(md)
                        break
                    else:
                        next_header_idx = next_hash
                        break

                if next_header_idx == -1:
                    next_header_idx = len(md)

                end = next_header_idx

                existing_segment = md[start + len(header) : end]
                existing_content = existing_segment.strip()

                if not existing_content:
                    # Replace empty section
                    md = md[: start + len(header)] + "\n" + content + "\n" + md[end:]
                else:
                    # Append if not duplicate
                    if content not in existing_content:
                        md = md[:end] + "\n\n" + content + "\n" + md[end:]
            return md

        new_md = current_md
        new_md = replace_section(new_md, "Todos", plan_content)
        new_md = replace_section(new_md, "Log", log_content)
        new_md = replace_section(new_md, "Summary", summary_content)

        with open(ticket_md_path, "w") as f:
            f.write(new_md)

        # Delete old files
        if plan_path.exists():
            plan_path.unlink()
        if log_path.exists():
            log_path.unlink()
        if summary_path.exists():
            summary_path.unlink()

    # Save JSON if changed
    if is_old_json:
        with open(ticket_json_path, "w") as f:
            json.dump(new_data, f, indent=2)
            f.write("\n")


def main():
    if not os.path.exists(TICKETS_ROOT):
        print(f"Tickets root {TICKETS_ROOT} not found.")
        return

    for root, dirs, files in os.walk(TICKETS_ROOT):
        if "ticket.json" in files:
            migrate_ticket(Path(root))


if __name__ == "__main__":
    main()
