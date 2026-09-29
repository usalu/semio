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
    "🧰️framework/🛍️products/🖥️server",
    "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n",
    "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🪟️chrome",
    "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌓️appearance",
    "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json",
    "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🏷️Label/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🔨️modules/👂️dom-event-binding",
    "🧰️framework/🔨️modules/🖱️ui/🔨️modules/📝️form-control-presentation/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/🔨️modules/👥️presence-presentation",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/🔮️oracles/🔣️.json",
    "🏢️semio-tech/🎡️play/⚛️play-card.tsx",
    "🏢️semio-tech/🎡️play/🎨️globals.css",
    "♻️mit-bestand/🧺️demonstrator/⚛️demonstrator-card.tsx",
    "♻️mit-bestand/🧺️demonstrator/🎨️globals.css",
    ".github/workflows/architecture-quiz.yml",
    "♻️mit-bestand/🧺️demonstrator/🧪️tests",
    "🏢️semio-tech/🎡️play/🧪️tests",
    "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🥞️layered-overview",
    "♻️mit-bestand/🧺️demonstrator/🟦️.tsx",
    "🏢️semio-tech/🎡️play/🟦️.tsx",
    "🧰️framework/🔨️modules/🖱️ui/🔨️modules/🥞️layered-overview-geometry",
    "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview",
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
base = "dfe2687f7db"
git = ["git", "-c", "core.quotepath=false"]
run = lambda *args: subprocess.run([*git, *args], cwd=root, capture_output=True, text=True, encoding="utf-8", check=True).stdout.splitlines()
created, updated, removed = [], [], []
for line in run("diff", "--name-status", "--no-renames", base, "--", *scopes):
    code, path = line.split("\t", 1)
    (removed if code == "D" else created if code == "A" else updated).append(path)
created += run("ls-files", "--others", "--exclude-standard", "--", *scopes)
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
    "resolved. Revision 2026-09-29: the UI is a card grid in the semio-tech play / mit-bestand demonstrator language "
    "(WindowChrome cards, leaderboard as the central card, shared 🃏️OverviewCard for quiz, play and demonstrator, "
    "@semio-tech/ui-react/chrome); the site is a static CDN build for quizzes.architektur-und-technologie.de with the proctor "
    "origin baked in; the proctor is API-only and zero-touch on Docker (proctor + Caddy compose, baked production defaults, "
    "GHCR publish, manual GitHub Pages/GHCR workflow) at proctor.quizzes.architektur-und-technologie.de; shared presence: "
    "framework presence WebSocket with coalesced batches, quiz presence/cursor states in both cores, online roster, "
    "learning-now counts and live cursors anchored to shared cards. Verified: nx tests of 7 projects, parity 84/84, taxonomy "
    "clean (teaching, quiz, server), Docker image/stack checks incl. presence through Caddy, hub compiles against the framework "
    "changes, two-device browser check. Evening revision: the home is a layered overview like semio-tech play (shared "
    "🥞️LayeredOverview element used by quiz, play and demonstrator; nine real pages behind one glass veil, hovering a card "
    "shows its page clear). See 📓️closing-summary.md."
)
data["files"] = {"created": created, "updated": updated, "removed": removed}
data["note"] = data.get("note", "") + " Closed manually on 2026-09-29 (repo MCP still down)."
(ticket / "🎫️ticket.json").write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(len(created), "created,", len(updated), "updated,", len(removed), "removed")
