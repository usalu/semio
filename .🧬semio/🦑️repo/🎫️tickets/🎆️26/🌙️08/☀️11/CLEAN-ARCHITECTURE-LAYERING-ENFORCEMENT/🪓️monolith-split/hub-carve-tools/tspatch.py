#!/usr/bin/env python3
"""🧰️ Mechanical part of the script.ts re-pointing: exact-law groups and the bin source readers."""
import re, sys, json
sys.path.insert(0, __import__('os').path.dirname(__import__('os').path.abspath(__file__)))
import tsrepoint

HELPERS = '''
/** 🌳️ The crate roots that carry the `os-hub` binary's code, in dependency order; the binary's own root closes the list. */
const HUB_BIN_CRATE_ROOTS = ["⚙️runtime", "📒️ledgers", "🧱️models", "🧠️state", "🛰️session-routes", "📨️document-routes", "🛣️routes"] as const;

/** 🌳️ The module trees of every crate that carries the `os-hub` binary's code, concatenated in dependency order. */
function readHubBinSource(repoRoot: string): string {
  return [...HUB_BIN_CRATE_ROOTS.map((dir) => join(repoRoot, "🌎️hub/🏗️bootstrap", dir, "🦀️.rs")), join(repoRoot, "🌎️hub/🏗️bootstrap/🦀️.rs")].map((root) => readModuleTreeSource(root)).join("\\n");
}

/** 🧪️ The shared fixtures and the unit laws of the `os-hub` route laws crates, concatenated. */
function readHubBinTestSource(repoRoot: string): string {
  const base = join(repoRoot, "🌎️hub/🧪️tests/🔬️bin-unit");
  return ["🧰️testkit/🦀️.rs", "🚀️boot/🔬️unit/🦀️.rs", "🧭️open-plan/🔬️unit/🦀️.rs", "📇️directory/🔬️unit/🦀️.rs", "🔐️session/🔬️unit/🦀️.rs", "🎲️hostile-input/🔬️unit/🦀️.rs"].map((file) => readFileSync(join(base, file), "utf8")).join("\\n");
}
'''

READERS = [
    (re.compile(r'readModuleTreeSource\((?:join|resolve)\((repoRoot|this\.repoRoot), "🌎️hub/🏗️bootstrap/🦀️\.rs"\)\)'), r'readHubBinSource(\1)'),
    (re.compile(r'readModuleTreeSource\(join\(repoRoot, "🌎️hub", "🏗️bootstrap", "🦀️\.rs"\)\)'), 'readHubBinSource(repoRoot)'),
    (re.compile(r'readModuleTreeSource\(join\(this\.root, "\.\./\.\./🏗️bootstrap/🦀️\.rs"\)\)'), 'readHubBinSource(this.repoRoot)'),
    (re.compile(r'readFileSync\((?:join|resolve)\((repoRoot|this\.repoRoot), "🌎️hub/🧪️tests/🔬️bin-unit/🦀️\.rs"\), "utf8"\)'), r'readHubBinTestSource(\1)'),
]


def patch(s, add_helpers=True):
    n = 0
    for rx, rep in READERS:
        s, k = rx.subn(rep, s)
        n += k
    if add_helpers and 'function readHubBinSource' not in s:
        m = list(re.finditer(r'^import [^\n]*\n', s, re.M))
        end = m[-1].end()
        s = s[:end] + HELPERS + s[end:]
    return s, n


if __name__ == '__main__':
    src = open(sys.argv[1]).read()
    out, n = patch(src)
    open(sys.argv[2], 'w').write(out)
    print('reader replacements', n)
