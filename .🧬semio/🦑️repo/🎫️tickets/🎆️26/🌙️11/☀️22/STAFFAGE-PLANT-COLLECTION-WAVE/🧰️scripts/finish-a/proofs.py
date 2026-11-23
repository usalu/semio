"""🧾 Finisher A proof table: reads the final lines of the proof runs in `🗑️generated/finish-a/` (`<id>.validate.txt`, `<id>.check.txt` or `check-<id>.txt`) and prints one markdown row per plant with the validation summary, the remaining volume findings counted by rule and scale, and the check CLI verdict with its four scale lines.

Usage: `python3 proofs.py [--json] <id>...`.
See `./sweep.sh` and `./check.sh`.
"""
import collections, json, pathlib, re, sys

G = pathlib.Path(__file__).resolve().parent.parent.parent / "🗑️generated" / "finish-a"


def proof(ident: str) -> dict:
    validate = (G / f"{ident}.validate.txt")
    lines = validate.read_text("utf8").splitlines() if validate.is_file() else []
    summary = next((line for line in reversed(lines) if line.startswith(("VALID", "INVALID"))), "not run")
    rules = collections.Counter()
    for line in lines:
        found = re.match(r"VOLUMES (volumes-[a-z-]+) 1:(\d+)", line)
        if found:
            rules[f"{found.group(1).removeprefix('volumes-')} 1:{found.group(2)}"] += 1
    asset_issues = [line for line in lines if line.startswith(("ASSET", "SCHEMA", "PRINT"))]
    check = next((p for p in (G / f"{ident}.check.txt", G / f"check-{ident}.txt") if p.is_file()), None)
    check_lines = check.read_text("utf8").splitlines() if check else []
    verdict = next((line for line in check_lines if line.startswith(("✓", "✗"))), "not run")
    scales = [line.strip() for line in check_lines if line.strip().startswith("1:")]
    return {"id": ident, "summary": summary, "rules": dict(rules), "assetIssues": asset_issues, "check": verdict, "scales": scales}


def main() -> None:
    proofs = [proof(ident) for ident in sys.argv[1:] if not ident.startswith("--")]
    if "--json" in sys.argv:
        print(json.dumps(proofs, ensure_ascii=False, indent=1))
        return
    print("| Plant | Whole-asset validation | Remaining volume findings | Check CLI |")
    print("|---|---|---|---|")
    for p in proofs:
        rules = ", ".join(f"{name} ×{count}" for name, count in sorted(p["rules"].items())) or "none"
        print(f"| `{p['id']}` | `{p['summary']}` | {rules} | `{p['check']}` |")


if __name__ == "__main__":
    main()
