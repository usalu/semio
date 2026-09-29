"""Close the ticket on disk (repo MCP down): status, summary and every created or changed file from git status."""
import json
import pathlib
import subprocess

root = pathlib.Path(r"C:\git\semio")
ticket = root / ".🧬semio" / "🦑️repo" / "🎫️tickets" / "🎆️26" / "🌙️09" / "☀️28" / "QUIZ-PRODUCT-AND-TEACHING-PROCTOR"
scopes = [
    "🧰️framework/🛍️products/❓️quiz",
    "🎓️teaching",
    "🧰️framework/🛍️products/🔣️.json",
    "🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority",
    "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🏷️Label/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🛡️protection/🟦️.ts",
    "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts",
    "package.json",
    "bun.lock",
    "Cargo.toml",
    "Cargo.lock",
    ".gitignore",
    ".dockerignore",
    ".vscode/launch.json",
    ".vscode/🧩️launch.seed.jsonc",
    ".claude/launch.json",
    ticket.relative_to(root).as_posix(),
]
status = subprocess.run(
    ["git", "-c", "core.quotepath=false", "status", "--porcelain=v1", "--untracked-files=all", "--", *scopes],
    cwd=root, capture_output=True, text=True, encoding="utf-8", check=True,
).stdout.splitlines()
created, updated, removed = [], [], []
for line in status:
    code, path = line[:2], line[3:]
    (removed if "D" in code else created if code in ("??", "A ") else updated).append(path)
data = json.loads((ticket / "🎫️ticket.json").read_text(encoding="utf-8"))
data["status"] = "closed"
data["summary"] = (
    "Built the render-independent quiz product (🧰️framework/🛍️products/❓️quiz: JSON Schema contract, TS + Rust twins of randomness, "
    "sheet, validation, scoring, badges, lifecycle and views; React renderer at 🎯️targets/⚛️react with local-first proctor client), the "
    "proctor server (🎓️teaching/🛂️proctor: Rust ServerInstance, CQRS + event sourcing over one SQLite file, static hosting, production "
    "gating) and the site quizze.architektur-und-technologie.de (🎓️teaching/🏛️architecture/❓️quiz: catalog, 7 badges, Vite site, "
    "Dockerfile/compose/Caddy) with four sourced energy quizzes (physics, heating, cooling, demand). Scoring gives partial credit by "
    "magnitude-weighted pair concordance and profile similarity. Verified: nx tests of all 5 projects green, Protocol v2 parity 75/75 "
    "(numpy/scipy oracle vs TS vs Rust), taxonomy clean, Docker image build + check green, browser walk across two devices. Audits "
    "(spec, rules, content) found and fixed: leaderboard no longer publishes learner ids (tag), multi-tab data loss, cancel race, "
    "stale cross-device state, GEG 2024 labelling, score rounding, clipped radar labels. Shared fixes: framework CommandBus rehydrates "
    "actors after restart; ui-react i18n port split (site main chunk 1,691 kB → 414 kB); stale stash-pop conflicts in the repo library "
    "resolved. See 📓️closing-summary.md."
)
data["files"] = {"created": created, "updated": updated, "removed": removed}
data["note"] = data.get("note", "") + " Closed manually on 2026-09-28 (repo MCP still down)."
(ticket / "🎫️ticket.json").write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(len(created), "created,", len(updated), "updated,", len(removed), "removed")
