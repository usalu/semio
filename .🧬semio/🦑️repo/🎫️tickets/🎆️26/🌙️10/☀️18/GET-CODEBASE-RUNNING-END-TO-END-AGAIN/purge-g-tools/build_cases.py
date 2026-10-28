"""Authors the facts of the purge-generated cases; the model then writes each `expect`."""
import json, os, sys
MIB = 1 << 20
OLD = 86400
G = "🗑️generated"
OUT = sys.argv[1]
D = "🎆️26/🌙️10/☀️18/"

def f(path, size, age=OLD, tracked=False):
    return {"path": path, "kind": "file", "bytes": size, "ageSeconds": age, "tracked": tracked}

def t(slug, manifest, entries):
    return {"id": D + slug, "manifest": manifest, "entries": entries}

def case(cid, description, tickets, selection="oversized", scope="closed", ticket=None, git=True):
    request = {"scope": scope, "selection": selection}
    if ticket:
        request["ticket"] = D + ticket
    return {"id": cid, "description": description, "request": request, "git": git, "tickets": tickets}

retention = [
    f("📥️authored-inputs/large.json", 6 * MIB), f("📥️authored-inputs/snapshots/source.txt", 6 * MIB),
    f(f"📥️authored-inputs/{G}/authored.tex", 11 * MIB), f("📓️large-research.md", 6 * MIB), f("📜️script.ts", 6 * MIB),
    f("config.json", 6 * MIB), f("large.bin", 6 * MIB), f(".cache.bin", 6 * MIB),
    f(f"{G}/large.log", 5 * MIB + 1), f(f"{G}/exact.bin", 5 * MIB), f(f"{G}/huge/blob.bin", 10 * MIB + 1), f(f"{G}/huge/small.log", 10),
    f(f"{G}/small/file.bin", MIB), f(f"{G}/exact-directory/a.bin", 5 * MIB), f(f"{G}/exact-directory/b.bin", 5 * MIB),
    f(f"{G}/over-directory/a.bin", 5 * MIB), f(f"{G}/over-directory/b.bin", 5 * MIB), f(f"{G}/over-directory/c.bin", 1),
]
cases = [
    case("open-ticket-keeps-a-tracked-subtree-and-a-fresh-generated-folder",
         "The 17:16 loss: an open ticket with a tracked 11 MiB subtree and a fresh generated folder loses nothing.",
         [t("OPEN-WITH-TRACKED-SUBTREE", "open", [f("🪓️monolith-split/a/one.bin", 6 * MIB, OLD, True), f("🪓️monolith-split/a/two.bin", 6 * MIB, OLD, True), f(f"{G}/lane/log.bin", 11 * MIB, 5)])]),
    case("open-ticket-keeps-old-oversized-generated-output",
         "An open ticket is never purged, however old and large its generated output.",
         [t("OPEN-OLD-GENERATED", "open", [f(f"{G}/big/blob.bin", 12 * MIB), f(f"{G}/huge.log", 6 * MIB)])]),
    case("closed-ticket-oversized-removes-only-oversized-generated-output",
         "A closed ticket loses generated files above 5 MiB and generated folders above 10 MiB, never authored inputs.",
         [t("CLOSED-RETENTION", "closed", retention)]),
    case("closed-ticket-selection-generated-removes-the-whole-folder",
         "Selection generated (repo clean) removes the whole generated folder and still keeps everything outside it.",
         [t("CLOSED-RETENTION", "closed", retention)], selection="generated"),
    case("closed-ticket-keeps-non-generated-oversized-subtrees",
         "Tracked and untracked oversized subtrees outside the generated folder survive a closed ticket purge.",
         [t("CLOSED-AUTHORED", "closed", [f("📥️inputs/a.bin", 8 * MIB), f("📥️inputs/b.bin", 8 * MIB), f("🪓️monolith-split/x.bin", 11 * MIB, OLD, True)])]),
    case("live-writer-window-boundary",
         "A generated file younger than 600 s keeps the whole folder; exactly 600 s old does not.",
         [t("YOUNG-WRITER", "closed", [f(f"{G}/old.bin", 6 * MIB), f(f"{G}/new.log", 1, 599)]),
          t("EXACT-WINDOW", "closed", [f(f"{G}/old.bin", 6 * MIB), f(f"{G}/new.log", 1, 600)]),
          t("FUTURE-MTIME", "closed", [f(f"{G}/old.bin", 6 * MIB), f(f"{G}/skewed.log", 1, -30)])]),
    case("tracked-files-are-never-removed",
         "A tracked oversized file stays; its untracked oversized sibling goes; a folder holding a tracked file is kept whole.",
         [t("TRACKED-INSIDE", "closed", [f(f"{G}/tracked.bin", 6 * MIB, OLD, True), f(f"{G}/untracked.bin", 6 * MIB),
                                          f(f"{G}/mixed/tracked.txt", 20, OLD, True), f(f"{G}/mixed/big.bin", 11 * MIB), f(f"{G}/mixed/sub/blob.bin", 11 * MIB), f(f"{G}/mixed/small.txt", 10)])]),
    case("tracked-files-block-selection-generated-folders",
         "With selection generated a folder holding a tracked file is kept, every other sibling and subfolder goes.",
         [t("TRACKED-INSIDE", "closed", [f(f"{G}/mixed/tracked.txt", 20, OLD, True), f(f"{G}/mixed/loose.txt", 10), f(f"{G}/mixed/sub/blob.bin", 3), f(f"{G}/other/file.txt", 1)])], selection="generated"),
    case("boundary-sizes",
         "Exactly 5 MiB and exactly 10 MiB stay; one byte more goes.",
         [t("BOUNDARIES", "closed", [f(f"{G}/file-exact.bin", 5 * MIB), f(f"{G}/file-over.bin", 5 * MIB + 1),
                                      f(f"{G}/dir-exact/a.bin", 5 * MIB), f(f"{G}/dir-exact/b.bin", 5 * MIB),
                                      f(f"{G}/dir-over/a.bin", 5 * MIB), f(f"{G}/dir-over/b.bin", 5 * MIB), f(f"{G}/dir-over/c.bin", 1)])]),
    case("generated-root-shapes",
         "A missing generated folder is nothing to do; a regular file or a symbolic link in its place is skipped and its target stays.",
         [t("NO-GENERATED", "closed", [f("📓️report.md", 11 * MIB)]),
          t("FILE-ROOT", "closed", [f(G, 11 * MIB)]),
          t("SYMLINK-ROOT", "closed", [{"path": G, "kind": "symlink", "target": "external"}]),
          t("EMPTY-ROOT", "closed", [{"path": G, "kind": "directory"}])]),
    case("symbolic-links-inside-generated-are-never-followed",
         "A link to an outside folder and one to an outside file are not followed, counted or removed through.",
         [t("INNER-LINKS", "closed", [{"path": f"{G}/link-dir", "kind": "symlink", "target": "external"}, {"path": f"{G}/link-file.bin", "kind": "symlink", "target": "external/blob.bin"}, f(f"{G}/real.bin", 6 * MIB)])]),
    case("manifest-states",
         "Only status closed qualifies: open, an unknown status, an unreadable document and a missing document are all skipped.",
         [t("STATUS-CLOSED", "closed", [f(f"{G}/a.bin", 6 * MIB)]), t("STATUS-OPEN", "open", [f(f"{G}/a.bin", 6 * MIB)]),
          t("STATUS-CLOSING", "closing", [f(f"{G}/a.bin", 6 * MIB)]), t("MANIFEST-INVALID", "invalid", [f(f"{G}/a.bin", 6 * MIB)]),
          t("MANIFEST-MISSING", "missing", [f(f"{G}/a.bin", 6 * MIB)])]),
    case("scope-ticket-purges-only-the-named-closed-ticket",
         "A named closed ticket is purged and its closed neighbour is left alone.",
         [t("NAMED", "closed", [f(f"{G}/a.bin", 6 * MIB)]), t("NEIGHBOUR", "closed", [f(f"{G}/a.bin", 6 * MIB)])], scope="ticket", ticket="NAMED"),
    case("scope-ticket-refuses-an-open-ticket",
         "Naming an open ticket explicitly still removes nothing.",
         [t("NAMED-OPEN", "open", [f(f"{G}/a.bin", 6 * MIB)])], scope="ticket", ticket="NAMED-OPEN"),
    case("hidden-ticket-folders-are-not-discovered",
         "A dot-prefixed folder is no ticket for the all-closed scope.",
         [t(".hidden", "closed", [f(f"{G}/a.bin", 6 * MIB)]), t("VISIBLE", "closed", [f(f"{G}/a.bin", 6 * MIB)])]),
    case("two-tickets-one-live-writer",
         "A live writer in one closed ticket does not hold back another closed ticket.",
         [t("BUSY", "closed", [f(f"{G}/a.bin", 6 * MIB), f(f"{G}/lane.log", 1, 3)]), t("QUIET", "closed", [f(f"{G}/a.bin", 6 * MIB)])]),
    case("unknown-tracking-fails-closed",
         "Where git cannot say what is tracked, nothing is removed.",
         [t("NO-GIT", "closed", [f(f"{G}/a.bin", 6 * MIB)])], git=False),
    case("nested-generated-names-are-authored-content",
         "A folder named generated deeper inside the ticket is never the generated output root.",
         [t("NESTED-NAME", "closed", [f(f"📥️inputs/{G}/a.bin", 12 * MIB), f(f"📥️inputs/{G}/b.bin", 12 * MIB)])]),
]
json.dump({"policy": {"generatedDirectory": G, "oversizedFileBytes": 5 * MIB, "oversizedFolderBytes": 10 * MIB, "liveWriterSeconds": 600}, "cases": cases}, open(OUT, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
