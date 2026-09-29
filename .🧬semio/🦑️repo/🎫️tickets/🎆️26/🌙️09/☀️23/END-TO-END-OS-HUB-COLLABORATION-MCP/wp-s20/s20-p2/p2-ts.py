"""🧬️ S20 pass 2 (P2-F1) on the p2 overlay — the TS twins: `MutationMessage` loses `message` (kernel), the React shell's
conflict text is the report's localized label by code (never prose). Idempotent. Usage: python3 p2-ts.py [--dry-run]"""
import sys
from pathlib import Path

OVERLAY = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2/🧰️framework")
KERNEL = "🔨️modules/🎠️kernel/🟦️.ts"
SHELL_HOST = "🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
EDITS = [
    (KERNEL, " * frozen seven `mutation.*` codes (contract-freeze §C2 — no per-plugin codes, ever); `message` is\n * English prose (UI localizes by `code`, never by parsing `message`); `target`/`opIndex` are\n",
     " * frozen seven `mutation.*` codes (contract-freeze §C2 — no per-plugin codes, ever) or a catalogued framework code; a\n * report carries no prose (UI shows the text of `code`); `target`/`opIndex` are\n"),
    (KERNEL, "  readonly code: string;\n  readonly message: string;\n  readonly target?: readonly string[];\n  readonly opIndex?: number;\n};",
     "  readonly code: string;\n  readonly target?: readonly string[];\n  readonly opIndex?: number;\n};"),
    (SHELL_HOST, "    return `${shellLabel(mutationCodeLabelKey(worst.code))} — ${worst.message}`;",
     "    return worst.target?.length ? `${shellLabel(mutationCodeLabelKey(worst.code))} — ${worst.target.join(\"/\")}` : shellLabel(mutationCodeLabelKey(worst.code));"),
    (SHELL_HOST, "      const body = worst ? `${shellLabel(mutationCodeLabelKey(worst.code))} — ${worst.message}` : undefined;",
     "      const body = worst ? shellLabel(mutationCodeLabelKey(worst.code)) : undefined;"),
]
dry = "--dry-run" in sys.argv[1:]
for rel, old, new in EDITS:
    path = OVERLAY / rel
    text = path.read_text()
    if new in text and old not in text:
        continue
    assert text.count(old) == 1, (rel, old[:80])
    print("apply", rel.split("/")[-2], new.strip()[:80])
    if not dry:
        path.write_text(text.replace(old, new))
