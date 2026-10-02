"""Closes this ticket and the two quiz tickets of 2026-10-02 it finished, the way the repo MCP writes a closed ticket
(`status`, `summary`, `files`), while the MCP server is down.

    python close_tickets.py
"""
import json
import pathlib

DAY = pathlib.Path(__file__).resolve().parents[1]
HERE = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY"
NOTE = " Closed manually on 2026-10-02 (repo MCP still down)."


def close(slug: str, summary: str, files: object | None = None) -> None:
    path = DAY / slug / "🎫️ticket.json"
    ticket = json.loads(path.read_text(encoding="utf-8"))
    ticket["status"] = "closed"
    ticket["summary"] = summary
    if files is not None:
        ticket["files"] = files
    if not ticket.get("note", "").endswith(NOTE):
        ticket["note"] = ticket.get("note", "") + NOTE
    path.write_text(json.dumps(ticket, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print("closed", slug)


close(
    "TEACHING-ARCHITECTURE-DEPLOY-READY",
    "Teaching architecture is ready to deploy on the tree of checkpoint 667: the readiness gate ends with 'ready to deploy' (8 steps), the end-to-end gate passes 36/36 in both topologies, unit tests of ten projects, four type checks, parity (quiz 108/108, pets 183/183), the capacity gate (twice) and a Linux rehearsal of the workflow's site job pass, both deliverables are staged in .🧬semio/🎓️teaching/architecture-quiz-poc and the staged site was driven in Chromium with its proctor away. The work was repairing what checkpoints 665 and 667 broke under the finished features: the Nx plugin no longer loaded on Windows (top-level await), Protocol v2 adapters and the pets scripts used moved APIs, the quiz Vitest config found no files, a framework Rust test blocked every type check, the proctor image no longer built (the proctor moved into the 🎓️teaching cargo workspace), the capacity gate counted a client-side connect failure as a proctor error, and the lockfile failed the frozen install of the pinned bun. Left for the owner: the two legal URLs, DNS, upload, the university firewall and the certificate; repo-wide debt (taxonomy of workspace-root manifests, two bun versions) is named in 📓️deploy-ready-report.md.",
    {
        "created": [
            f"{HERE}/🎫️ticket.json",
            f"{HERE}/📓️deploy-ready-report.md",
            f"{HERE}/📸️staged-site-sorting-guesses.png",
            f"{HERE}/gates.sh",
            f"{HERE}/stage_deliverables.sh",
            f"{HERE}/linux_site_job.sh",
            f"{HERE}/drive_staged_site.ts",
            f"{HERE}/adapter_imports.py",
            f"{HERE}/launch_rows_in_docs.py",
            f"{HERE}/nx_plugin_load_probe.mjs",
            f"{HERE}/websocket_failure_probe.ts",
            f"{HERE}/capacity.tsconfig.json",
            f"{HERE}/close_tickets.py",
        ],
        "updated": [
            "bun.lock",
            "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs",
            "🧰️framework/🔨️modules/🕹️interaction/🧬️schema/🧪️tests/🔬️unit/🦀️.rs",
            "🧰️framework/🛍️products/❓️quiz/🧪️tests/🎚️config/🟦️.ts",
            *[f"🧰️framework/🛍️products/❓️quiz/🧪️tests/{case}/🟦️.ts" for case in ["✅️answer-validation", "🃏️sheet-assembly", "🎲️seeded-randomness", "🏅️badge-rules", "👥️shared-presence", "📊️crowd-view", "📏️sorting-concordance", "🔀️matching-concordance", "🕸️profile-similarity", "🪪️identity-shapes"]],
            *[f"🧰️framework/🛍️products/🐾️pets/🧪️tests/{case}/🟦️.ts" for case in ["🎞️animation-sampling", "🎪️stage-trace", "🎲️counter-randomness", "🏞️terrain-walking", "👀️gaze-tracking", "📐️turn-trigonometry", "🤝️bond-dynamics", "🦘️hop-ballistics", "🦴️rig-solving", "🧠️behavior-choice", "🧬️schema-conformance", "🪀️spring-settling"]],
            "🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/📜️script.ts",
            "🧰️framework/🛍️products/🐾️pets/📦️packages/🦀️rust/📜️script.ts",
            "🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts",
            "🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile",
            "🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/Dockerfile.dockerignore",
            "🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🧪️deploy/🟦️.ts",
            "🎓️teaching/🏛️architecture/❓️quiz/README.md",
            "🎓️teaching/🛂️proctor/README.md",
            "🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/🟦️.ts",
            ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-SORTING-NUMERIC-GUESSES/🎫️ticket.json",
            ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-POC-DEPLOY-BUNDLE/🎫️ticket.json",
        ],
        "removed": [f".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-SORTING-NUMERIC-GUESSES/{name}" for name in ["before.sha", "before2.sha", "after.sha", "after2.sha", "oracle1.txt", "parity1.txt", "parity2.txt", "parity3.txt"]],
    },
)
close(
    "QUIZ-SORTING-NUMERIC-GUESSES",
    "Every sorting task takes a typed numeric guess per item: the field shows the unit, reads an optional SI prefix (2 kW, 1,5 MWh, 1e3 kW) and shows what it reads as, and on commit the guessed items order themselves by their guesses among the places they occupy; moving a guessed item by hand removes its guess, guesses never change a score, and the results show a 'Your guess' column. Contract (SortingAnswer.guesses in the base unit, thinking drafts), both cores, the Python reference, the React client, the texts in English and German and the product README moved together as 📓️design.md says. The session that built it left the ticket open; ticket TEACHING-ARCHITECTURE-DEPLOY-READY verified it on 2026-10-02 (unit tests 640 and 297, parity 108/108, end-to-end 36/36 in both topologies, and the staged site in Chromium: both physics sortings ordered by typed guesses alone, 100 %, two 'Your guess' columns), removed the left-over tool output and closed it.",
)
close(
    "QUIZ-POC-DEPLOY-BUNDLE",
    "First round: the poc and poc-check verbs (one directory, one origin), see 📓️poc-summary.md. Second round (split deployment): the frontend is a static folder for quizze.architektur-und-technologie.de, the backend a Docker bundle for semio.iek.uni-hannover.de (docker-stack-bundle, a certificate of the operator's own), and the site works while the proctor is away (the deputy decides on the device, the outbox delivers every command once the proctor answers); see 📓️split-deploy-design.md, 📓️split-deploy-summary.md and 📓️deploy-retarget-report.md. The point left open there — the whole end-to-end gate and with it deploy-check did not pass — was closed by ticket TEACHING-ARCHITECTURE-DEPLOY-READY on 2026-10-02: both gates pass, the Dockerfile builds the proctor from the 🎓️teaching cargo workspace, and both deliverables were staged again from the final tree in .🧬semio/🎓️teaching/architecture-quiz-poc (STAGED.txt).",
)
