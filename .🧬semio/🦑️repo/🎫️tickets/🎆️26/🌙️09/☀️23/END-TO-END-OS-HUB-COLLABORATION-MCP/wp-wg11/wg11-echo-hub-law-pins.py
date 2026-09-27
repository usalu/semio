#!/usr/bin/env python3
"""🔁️ WG11 session 14 — re-apply ONLY the bin-unit law pins of WG9's hub echo-suppression set.

The bootstrap half of `wp-wg9/s13-echo-suppression-hub-patch.py` (`HUB_CATCH_UP_ORIGIN` on the hello tail and the
`FrontierAdvertise` catch-up) landed 2026-09-26 19:20; its two law pins in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` were reverted
2026-09-27 06:32 (unverifiable in the convoys). This applies exactly WG9's `LAW_EDITS` (every anchor asserted once).

Dry run by default; `--apply` writes; `--revert` restores the pre-landing lines.
"""

import difflib
import importlib.util
import sys
from pathlib import Path

WG9 = Path(__file__).resolve().parent.parent / "wp-wg9" / "s13-echo-suppression-hub-patch.py"
spec = importlib.util.spec_from_file_location("wg9_hub", WG9)
wg9 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(wg9)


def main():
    revert = "--revert" in sys.argv
    apply = "--apply" in sys.argv or revert
    edits = [(new, old) for old, new in wg9.LAW_EDITS] if revert else wg9.LAW_EDITS
    before = wg9.LAWS.read_text(encoding="utf-8")
    after = wg9.replaced(wg9.LAWS, before, edits)
    sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), wg9.LAWS.name, wg9.LAWS.name + " (patched)", n=1))
    if apply:
        wg9.LAWS.write_text(after, encoding="utf-8")
    print(f"\n{'REVERTED' if revert else 'APPLIED' if apply else 'DRY RUN'}: 1 file")


if __name__ == "__main__":
    main()
