#!/usr/bin/env python3
"""Apply 🎆️YY/🌙️MM/☀️DD date dirs + emoji prefixes on meta JSON filenames."""
from __future__ import annotations

import os
import re
from pathlib import Path

ROOT = Path(__file__).resolve()
while not (ROOT / ".git").is_dir():
    if ROOT.parent == ROOT:
        raise SystemExit("repo root not found")
    ROOT = ROOT.parent
os.chdir(ROOT)

FE0E = "\ufe0e"
EY = "🎆️" + FE0E
EM = "🌙️" + FE0E
ED = "☀️" + FE0E
ET = "🎫️" + FE0E
EG = "🎯️" + FE0E
EC = "🧑️" + FE0E + "\u200d" + "💻️" + FE0E
EI = "📌️" + FE0E
EF = "📁️" + FE0E

GO_MAIN = None
for m in ROOT.rglob("main.go"):
    if m.stat().st_size > 100_000 and "🦑️" in str(m) and "💻️" in str(m) and "⌨️" in str(m):
        GO_MAIN = m
        break
assert GO_MAIN is not None, "main.go not found"
GO_TEST = GO_MAIN.with_name("main_test.go")

META = next(ROOT.glob(".🦑️*repo"))
TICKETS = next(META.glob("*tickets"))
CACHE = next(META.glob("*cache"), None)


def year_dir(y: int | str) -> str:
    return f"{EY}{int(y):02d}"


def month_dir(m: int | str) -> str:
    return f"{EM}{int(m):02d}"


def day_dir(d: int | str) -> str:
    return f"{ED}{int(d):02d}"


def patch_go(text: str) -> str:
    # Insert helpers after PadNumber if missing
    if "func FormatYearDir" not in text:
        pad_fn = text.find("func PadNumber")
        assert pad_fn >= 0
        # find end of PadNumber function
        end = text.find("\n}", pad_fn)
        end = text.find("\n", end + 1)
        helpers = f'''

// 🎆️FormatYearDir returns the emoji-prefixed year directory segment.
func FormatYearDir(year int) string {{
	return EmojiYear + PadNumber(year, 2)
}}

// 🌙️FormatMonthDir returns the emoji-prefixed month directory segment.
func FormatMonthDir(month int) string {{
	return EmojiMonth + PadNumber(month, 2)
}}

// ☀️FormatDayDir returns the emoji-prefixed day directory segment.
func FormatDayDir(day int) string {{
	return EmojiDay + PadNumber(day, 2)
}}

// 🎫️FormatTicketRelPath returns YY/MM/DD/SLUG with emoji date segments.
func FormatTicketRelPath(year, month, day int, slug string) string {{
	return FormatYearDir(year) + "/" + FormatMonthDir(month) + "/" + FormatDayDir(day) + "/" + slug
}}
'''
        text = text[:end] + helpers + text[end:]

    text = text.replace(
        'return filepath.Join(GetTicketsDir(), PadNumber(year, 2), PadNumber(month, 2), PadNumber(day, 2), slug)',
        'return filepath.Join(GetTicketsDir(), FormatYearDir(year), FormatMonthDir(month), FormatDayDir(day), slug)',
    )
    text = text.replace(
        'return filepath.Join(GetTicketPath(year, month, day, slug), "important.md")',
        f'return filepath.Join(GetTicketPath(year, month, day, slug), "{EI}important.md")',
    )
    text = text.replace(
        'return filepath.Join(GetTicketPath(year, month, day, slug), "ticket.json")',
        f'return filepath.Join(GetTicketPath(year, month, day, slug), "{ET}ticket.json")',
    )
    text = text.replace('"goal.json"', f'"{EG}goal.json"')
    text = text.replace('"contributor.json"', f'"{EC}contributor.json"')
    text = text.replace('"files.json"', f'"{EF}files.json"')

    # filesystem sprintf under tickets
    text = text.replace(
        'ticketPath = fmt.Sprintf(".🦑️repo/🎫️tickets/%02d/%02d/%02d/%s", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)',
        'ticketPath = ".🦑️repo/🎫️tickets/" + FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)',
    )

    # logical ticket IDs
    for a, b in [
        (
            'fmt.Sprintf("%d/%02d/%02d/%s", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)',
            'FormatTicketRelPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)',
        ),
        (
            'fmt.Sprintf("%d/%02d/%02d/%s", year, month, day, slug)',
            'FormatTicketRelPath(year, month, day, slug)',
        ),
    ]:
        text = text.replace(a, b)

    # cache date dirs
    old_cache_block = '''yy := fmt.Sprintf("%02d", now.Year()%100)
	mm := fmt.Sprintf("%02d", int(now.Month()))
	dd := fmt.Sprintf("%02d", now.Day())'''
    new_cache_block = '''yy := FormatYearDir(now.Year() % 100)
	mm := FormatMonthDir(int(now.Month()))
	dd := FormatDayDir(now.Day())'''
    text = text.replace(old_cache_block, new_cache_block)

    # agent events join with %02d
    old_agent = '''filepath.Join(repoRoot, ".🦑️repo", "⚡️cache", "🤖️generated",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", int(now.Month())),
		fmt.Sprintf("%02d", now.Day()),'''
    new_agent = '''filepath.Join(repoRoot, ".🦑️repo", "⚡️cache", "🤖️generated",
		FormatYearDir(now.Year()%100),
		FormatMonthDir(int(now.Month())),
		FormatDayDir(now.Day()),'''
    text = text.replace(old_agent, new_agent)

    # parseTicketPath: strip emoji prefixes
    old_parse = '''func parseTicketPath(path string) (int, int, int, string, error) {
	parts := strings.Split(strings.TrimSpace(path), "/")
	if len(parts) < 4 {
		return 0, 0, 0, "", fmt.Errorf("invalid ticket path %q: expected format YY/MM/DD/SLUG (e.g. '26/03/27/FIX-MCP-DESCRIPTIONS')", path)
	}
	year, err := strconv.Atoi(parts[0])
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("invalid year in ticket path %q", path)
	}
	if year >= 2000 {
		year = year % 100
	}
	month, err := strconv.Atoi(parts[1])
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("invalid month in ticket path %q", path)
	}
	day, err := strconv.Atoi(parts[2])
	if err != nil {
		return 0, 0, 0, "", fmt.Errorf("invalid day in ticket path %q", path)
	}'''
    new_parse = f'''func parseTicketPath(path string) (int, int, int, string, error) {{
	parts := strings.Split(strings.TrimSpace(path), "/")
	if len(parts) < 4 {{
		return 0, 0, 0, "", fmt.Errorf("invalid ticket path %q: expected format 🎆️YY/🌙️MM/☀️DD/SLUG", path)
	}}
	yearPart := strings.TrimPrefix(parts[0], EmojiYear)
	monthPart := strings.TrimPrefix(parts[1], EmojiMonth)
	dayPart := strings.TrimPrefix(parts[2], EmojiDay)
	year, err := strconv.Atoi(yearPart)
	if err != nil {{
		return 0, 0, 0, "", fmt.Errorf("invalid year in ticket path %q", path)
	}}
	if year >= 2000 {{
		year = year % 100
	}}
	month, err := strconv.Atoi(monthPart)
	if err != nil {{
		return 0, 0, 0, "", fmt.Errorf("invalid month in ticket path %q", path)
	}}
	day, err := strconv.Atoi(dayPart)
	if err != nil {{
		return 0, 0, 0, "", fmt.Errorf("invalid day in ticket path %q", path)
	}}'''
    if old_parse not in text:
        raise SystemExit("parseTicketPath block not found for replacement")
    text = text.replace(old_parse, new_parse)
    return text


def rename_date_tree(base: Path) -> int:
    """Rename bare YY/MM/DD under base to emoji-prefixed segments (deepest first)."""
    if not base or not base.is_dir():
        return 0
    n = 0
    # collect all dirs
    dirs = sorted([p for p in base.rglob("*") if p.is_dir()], key=lambda p: len(p.parts), reverse=True)
    for d in dirs:
        name = d.name
        if re.fullmatch(r"\d{2}", name):
            # decide year/month/day by depth relative to base
            rel = d.relative_to(base)
            depth = len(rel.parts) - 1  # 0=year, 1=month, 2=day typically
            parent_names = [x.name for x in d.parents]
            # heuristic: if parent is tickets/cache root-ish year; if parent looks like year→month; if month→day
            parent = d.parent.name
            if parent in {base.name} or re.fullmatch(r"\d{2}", parent) is None and not parent.startswith(EY) and not parent.startswith(EM) and not parent.startswith(ED):
                # under tickets root: year
                if base.name.endswith("tickets") or "generated" in base.name or "diff" in base.name or base == TICKETS:
                    # only rename if at depth 0 from tickets (year) — check path length from base
                    if len(rel.parts) == 1:
                        new = d.with_name(year_dir(name))
                    elif len(rel.parts) == 2:
                        new = d.with_name(month_dir(name))
                    elif len(rel.parts) == 3:
                        new = d.with_name(day_dir(name))
                    else:
                        continue
                else:
                    continue
            elif parent.startswith(EY) or re.fullmatch(r"\d{2}", parent):
                # under year → month
                if len(rel.parts) >= 2 and (parent.startswith(EY) or (re.fullmatch(r"\d{2}", parent) and len(d.relative_to(base).parts) == 2)):
                    new = d.with_name(month_dir(name))
                elif parent.startswith(EM):
                    new = d.with_name(day_dir(name))
                else:
                    # depth-based
                    if len(rel.parts) == 2:
                        new = d.with_name(month_dir(name))
                    elif len(rel.parts) == 3:
                        new = d.with_name(day_dir(name))
                    else:
                        continue
            elif parent.startswith(EM):
                new = d.with_name(day_dir(name))
            else:
                continue
            if new.exists():
                print("skip collision", d, "->", new)
                continue
            d.rename(new)
            n += 1
            print("dir", d, "->", new)
    return n


def rename_date_tree_simple(base: Path) -> int:
    """Tickets/cache layout: base/YY/MM/DD/... → base/🎆️YY/🌙️MM/☀️DD/..."""
    if not base or not base.is_dir():
        return 0
    n = 0
    for year in list(base.iterdir()):
        if not year.is_dir() or year.name.startswith("."):
            continue
        yname = year.name
        if re.fullmatch(r"\d{2}", yname):
            target = year.with_name(year_dir(yname))
            if not target.exists():
                year.rename(target)
                print("year", year, "->", target)
                year = target
                n += 1
            else:
                year = target
        elif not yname.startswith(EY):
            continue
        for month in list(year.iterdir()):
            if not month.is_dir() or month.name.startswith("."):
                continue
            mname = month.name
            if re.fullmatch(r"\d{2}", mname):
                target = month.with_name(month_dir(mname))
                if not target.exists():
                    month.rename(target)
                    print("month", month, "->", target)
                    month = target
                    n += 1
                else:
                    month = target
            elif not mname.startswith(EM):
                continue
            for day in list(month.iterdir()):
                if not day.is_dir() or day.name.startswith("."):
                    continue
                dname = day.name
                if re.fullmatch(r"\d{2}", dname):
                    target = day.with_name(day_dir(dname))
                    if not target.exists():
                        day.rename(target)
                        print("day", day, "->", target)
                        n += 1
    return n


def rename_meta_files() -> int:
    n = 0
    mapping = {
        "ticket.json": f"{ET}ticket.json",
        "goal.json": f"{EG}goal.json",
        "contributor.json": f"{EC}contributor.json",
        "important.md": f"{EI}important.md",
        "files.json": f"{EF}files.json",
    }
    # under meta + monorepo (skip node_modules/target)
    skip = {"node_modules", "target", ".git", "dist", ".nx"}
    for dirpath, dirnames, filenames in os.walk(ROOT):
        parts = set(Path(dirpath).parts)
        if parts & skip:
            dirnames[:] = []
            continue
        dirnames[:] = [d for d in dirnames if d not in skip]
        for old, new in mapping.items():
            if old in filenames:
                src = Path(dirpath) / old
                dst = Path(dirpath) / new
                if not dst.exists():
                    src.rename(dst)
                    n += 1
                    print("file", src, "->", dst)
    return n


def patch_tests(text: str) -> str:
    text = text.replace('"ticket.json"', f'"{ET}ticket.json"')
    text = text.replace('"goal.json"', f'"{EG}goal.json"')
    text = text.replace('"contributor.json"', f'"{EC}contributor.json"')
    # bare date joins in fixtures: ".🦑️repo", "🎫️tickets", "26", "01", "20"
    # replace Join(..., "YY", "MM", "DD" patterns carefully via regex
    def repl_join(m: re.Match) -> str:
        y, mo, d = m.group(1), m.group(2), m.group(3)
        return f'{m.group(0)[:m.start(1)-m.start()]}'  # unused

    # common fixture pattern
    text = re.sub(
        r'filepath\.Join\(([^,]+),\s*"(\d{2})",\s*"(\d{2})",\s*"(\d{2})"',
        lambda m: f'filepath.Join({m.group(1)}, "{year_dir(m.group(2))}", "{month_dir(m.group(3))}", "{day_dir(m.group(4))}"',
        text,
    )
    # MkdirAll(... "🎫️tickets") alone ok
    # paths like Join(tmpDir, ".🦑️repo", "🎫️tickets", "26", ...
    return text


def patch_ts_file(path: Path) -> bool:
    if not path.is_file():
        return False
    t = path.read_text(encoding="utf-8")
    orig = t
    t = t.replace("ticket.json", f"{ET}ticket.json")
    t = t.replace("goal.json", f"{EG}goal.json")
    t = t.replace("contributor.json", f"{EC}contributor.json")
    # regex for ticket json path
    t = t.replace(
        r"/^\.🦑️repo\/🎫️tickets\/.+\/ticket\.json$/",
        rf"/^\.🦑️repo\/🎫️tickets\/.+\/{re.escape(ET)}ticket\.json$/",
    )
    t = t.replace(
        r"/^\.🦑️repo\/🎫️tickets\/.+\/ticket\.json$/",
        rf"/^\.🦑️repo\/🎫️tickets\/.+\/{re.escape(ET)}ticket\.json$/",
    )
    # pad2 date joins: common pattern path.join(root, ".🦑️repo", "🎫️tickets", pad2(y), pad2(m), pad2(d), slug)
    for a, b in [
        (
            'path.join(wsRoot, ".🦑️repo", "🎫️tickets", year, month, day, node.Data.slug)',
            f'path.join(wsRoot, ".🦑️repo", "🎫️tickets", `🎆️${{String(year).padStart(2,"0")}}`, `🌙️${{String(month).padStart(2,"0")}}`, `☀️${{String(day).padStart(2,"0")}}`, node.Data.slug)',
        ),
    ]:
        t = t.replace(a, b)
    # generic: "🎫️tickets", pad2( → emoji template — handle resolveTicketPath style
    t2 = re.sub(
        r'path\.join\(([^,]+),\s*"\.🦑️repo",\s*"🎫️tickets",\s*([^,]+),\s*([^,]+),\s*([^,]+),\s*([^)]+)\)',
        lambda m: (
            f'path.join({m.group(1)}, ".🦑️repo", "🎫️tickets", '
            f'`{EY}${{String({m.group(2)}).padStart(2,"0")}}`, '
            f'`{EM}${{String({m.group(3)}).padStart(2,"0")}}`, '
            f'`{ED}${{String({m.group(4)}).padStart(2,"0")}}`, {m.group(5)})'
            if "pad" not in m.group(2) and "🎆️" not in m.group(2)
            else m.group(0)
        ),
        t,
    )
    # If groups already use pad2(...):
    t2 = re.sub(
        r'("🎫️tickets",\s*)pad2\(([^)]+)\),\s*pad2\(([^)]+)\),\s*pad2\(([^)]+)\)',
        rf'\1`{EY}${{pad2(\2)}}`, `{EM}${{pad2(\3)}}`, `{ED}${{pad2(\4)}}`',
        t2,
    )
    t2 = re.sub(
        r'("🎫️tickets",\s*)String\(([^)]+)\)\.padStart\(2,\s*"0"\),\s*String\(([^)]+)\)\.padStart\(2,\s*"0"\),\s*String\(([^)]+)\)\.padStart\(2,\s*"0"\)',
        rf'\1`{EY}${{String(\2).padStart(2,"0")}}`, `{EM}${{String(\3).padStart(2,"0")}}`, `{ED}${{String(\4).padStart(2,"0")}}`',
        t2,
    )
    if t2 != orig:
        path.write_text(t2, encoding="utf-8")
        print("patched", path)
        return True
    return False


def main():
    print("GO_MAIN", GO_MAIN)
    go = GO_MAIN.read_text(encoding="utf-8")
    go2 = patch_go(go)
    if go2 != go:
        GO_MAIN.write_text(go2, encoding="utf-8")
        print("patched go main")
    else:
        print("go main unchanged?")

    if GO_TEST.is_file():
        tt = GO_TEST.read_text(encoding="utf-8")
        tt2 = patch_tests(tt)
        if tt2 != tt:
            GO_TEST.write_text(tt2, encoding="utf-8")
            print("patched go test")

    # TS / extension
    fw = next(ROOT.glob("*framework"))
    for p in fw.rglob("*.ts"):
        if any(x in p.parts for x in ("node_modules", "dist", "target")):
            continue
        if p.name in {"📦️index.ts", "🟦️extension.ts"} and "🦑️" in str(p):
            patch_ts_file(p)

    print("renaming date trees…")
    n = rename_date_tree_simple(TICKETS)
    if CACHE:
        for sub in CACHE.iterdir():
            if sub.is_dir() and ("generated" in sub.name or "diff" in sub.name or "🤖️" in sub.name or "🔀️" in sub.name):
                n += rename_date_tree_simple(sub)
    print("date renames", n)
    print("file renames", rename_meta_files())


if __name__ == "__main__":
    main()
