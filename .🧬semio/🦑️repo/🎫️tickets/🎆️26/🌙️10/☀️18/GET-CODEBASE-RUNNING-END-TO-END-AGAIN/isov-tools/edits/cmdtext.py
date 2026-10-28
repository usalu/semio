"""⌨️ [isov] batch `cmdtext`: the compliance language batch tokenised the first refusal of `requireCapturedBuildCommandV1` (`invalid build.closure.command-entry-authentication-required`) and its tests, but left the pinned branch throwing the English text, so the command ingress law and the SQLite oracle disagree (5 red). The pinned branch throws the same token, the one ingress assertion that still expected the English prefix follows, and the language allowance of the file drops by the converted literal (2 -> 1). Re-run = rebase onto the current live bytes (exit 2 names a lost anchor)."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mkbatch import build, live_bytes, replace_once
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
SRC = L + "🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts"; TEST = L + "🏗️build/⌨️command/🧪️tests/🟦️.ts"; ALLOW = L + "⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json"
rows = {}
base = live_bytes(SRC); text = base.decode("utf-8")
text = replace_once(text, 'throw implementationPinnedV1()?Error("Authenticated captured build command entry required"):staleImplementationV1(', 'throw implementationPinnedV1()?refuseBuildV1("invalid","closure.command-entry-authentication-required"):staleImplementationV1(', "pinned branch")
rows[SRC] = (base, text.encode("utf-8"))
base = live_bytes(TEST); text = base.decode("utf-8")
text = replace_once(text, 'rejects.toThrow("Authenticated captured")', 'rejects.toThrow("invalid build.closure.command-entry-authentication-required")', "ingress assertion")
rows[TEST] = (base, text.encode("utf-8"))
base = live_bytes(ALLOW); text = base.decode("utf-8")
text = replace_once(text, '"' + SRC + '": 2,', '"' + SRC + '": 1,', "allowance row")
rows[ALLOW] = (base, text.encode("utf-8"))
build("cmdtext", "⌨️ Staged batch `cmdtext` (lane isov): the command-entry authentication refusal is one machine token on every branch.", rows, {TEST: {"mode": "plain", "pass": 6, "fails": []}})
