#!/usr/bin/env python3
"""🖊️ Source readers whose Rust files lost items to the crates they were carved into: they read the carved crate roots together."""
import re, sys
sys.path.insert(0, __import__('os').path.dirname(__import__('os').path.abspath(__file__)))
from tsmanual import sub_once

HELPERS = '''
/** 🔏️ The trusted-catalog crate root together with the contract and descriptor roots it was carved into. */
function readTrustedCatalogSource(repoRoot: string): string {
  return ["🦀️.rs", "📜️contract/🦀️.rs", "🧾️descriptor/🦀️.rs"].map((file) => readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog", file), "utf8")).join("\\n");
}

/** 🏃️ The inference runtime module trees together with the committer and contract crates it was carved into. */
function readHubInferenceRuntimeSource(repoRoot: string): string {
  return ["🦀️.rs", "🔐️committer/🦀️.rs", "📜️contract/🦀️.rs"].map((file) => readModuleTreeSource(join(repoRoot, "🌎️hub/💡️inference/🏃️runtime", file))).join("\\n");
}

/** 🌐️ The neo4j directory backend together with the row decoders it was carved into. */
function readNeo4jDirectorySource(repoRoot: string): string {
  return ["🦀️.rs", "🧰️rows/🦀️.rs"].map((file) => readFileSync(join(repoRoot, "🌎️hub/📇️directory/🌐️neo4j", file), "utf8")).join("\\n");
}

'''


def patch(s):
    s = sub_once(s, "    !startup.includes('#[path = \"../🧪️tests/🔬️bin-unit/🦀️.rs\"]') ||\n", '')
    s = sub_once(s, "    !hub.includes('#[path = \"../🧪️tests/🔬️bin-unit/🦀️.rs\"]') ||\n", '')
    s = sub_once(s, '"🧰️testkit/🦀️.rs", "🚀️boot/🔬️unit/🦀️.rs", "🧭️open-plan/🔬️unit/🦀️.rs", "📇️directory/🔬️unit/🦀️.rs", "🔐️session/🔬️unit/🦀️.rs"', '"🧰️testkit/🦀️.rs", "🚀️boot/🔬️unit/🦀️.rs", "🧭️open-plan/🔬️unit/🦀️.rs", "📇️directory/🔬️unit/🦀️.rs", "👥️presence/🔬️unit/🦀️.rs", "🔐️session/🔬️unit/🦀️.rs"')
    tc = re.compile(r'readFileSync\((join|resolve)\((repoRoot|this\.repoRoot), "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️\.rs"\), "utf8"\)')
    s, n1 = tc.subn(lambda m: 'readTrustedCatalogSource(%s)' % m.group(2), s)
    rt = re.compile(r'readModuleTreeSource\(join\(repoRoot, "🌎️hub", "💡️inference", "🏃️runtime", "🦀️\.rs"\)\)')
    s, n2 = rt.subn('readHubInferenceRuntimeSource(repoRoot)', s)
    n4 = re.compile(r'readFileSync\(join\(repoRoot, "🌎️hub/📇️directory/🌐️neo4j/🦀️\.rs"\), "utf8"\)')
    s, n3 = n4.subn('readNeo4jDirectorySource(repoRoot)', s)
    print('trusted-catalog reads', n1, 'inference runtime reads', n2, 'neo4j reads', n3)
    anchor = '/** 🧪️ The shared fixtures and the unit laws'
    assert s.count(anchor) == 1
    return s.replace(anchor, HELPERS.lstrip('\n') + anchor, 1)


if __name__ == '__main__':
    s = open(sys.argv[1]).read()
    open(sys.argv[2], 'w').write(patch(s))
