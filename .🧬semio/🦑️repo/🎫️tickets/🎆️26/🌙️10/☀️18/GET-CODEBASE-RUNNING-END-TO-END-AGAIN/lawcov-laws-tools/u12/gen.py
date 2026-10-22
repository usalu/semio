import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
x = []


def case(id_, rule, files, expect, **kw):
    row = {"id": id_, "rule": rule, "files": files, "expect": expect}
    row.update(kw)
    cases.append(row)


def b(i, kind, prio="high", line=0):
    return [i, kind, prio, line]


BN = "taxonomy/banned-name-stem"
case("banned-name-stems-in-files-and-directories", "policyBannedNameStemBreaches", {
    "✏s/🔌plugins/🔭p/core.rs": x, "✏s/🔌plugins/🔭p/🔸utils/x": x, "🧰framework/🔨modules/🔸Helper.ts": x, "🌎hub/🔸shared/x": x, "♻mit-bestand/lib.go": x, "✏s/🔌plugins/🔭p/🔸ok/x": x,
    "✏s/🔌plugins/🔭p/🪆subsets/🔸base/x": x, "✏s/🔌plugins/🔭p/🔸base/x": x, "✏s/🔌plugins/🔭p/node_modules/core.rs": x, "other/core.rs": x,
    "🧰framework/🛍products/💻os/🧪testing/🧩jcoprobe/🌐browser-host/🪞preview2-shim/common.js": x,
}, [b("banned-name-stem-✏s/🔌plugins/🔭p/core.rs", BN), b("banned-name-stem-✏s/🔌plugins/🔭p/🔸utils", BN), b("banned-name-stem-🧰framework/🔨modules/🔸Helper.ts", BN), b("banned-name-stem-🌎hub/🔸shared", BN),
    b("banned-name-stem-♻mit-bestand/lib.go", BN), b("banned-name-stem-✏s/🔌plugins/🔭p/🔸base", BN)])
PR = "taxonomy/plugin-root-shape"
case("plugin-roots-need-a-leaf-and-a-commands-leaf", "policyPluginRootShapeBreaches", {
    "✏s/🔌plugins/🔭ok/🦀.rs": x, "✏s/🔌plugins/🔭ok/🎮commands/🦀.rs": x,
    "✏s/🔌plugins/🔭bare/x.txt": x, "✏s/🔌plugins/🔭nested/🔌plugin/x": x, "✏s/🔌plugins/🔭nested/🦀.rs": x, "✏s/🔌plugins/🔭nested/🎮commands/🦀.rs": x, "✏s/🔌plugins/loose.txt": x,
}, [b("plugin-root-leaf-✏s/🔌plugins/🔭bare", PR), b("plugin-root-child-✏s/🔌plugins/🔭bare/🎮commands", PR), b("plugin-root-nested-✏s/🔌plugins/🔭nested", PR)])
PB = "taxonomy/plugin-builder"
SDK = "🧰framework/🛍products/💻os/🔨modules/🔌plugin/🦀.rs"
mk = lambda lib, role="plugin": {"libRelPath": lib, "role": role, "pluginId": "p", "ownerRel": "o", "shape": "taxonomy", "dir": "d"}
case("plugin-builders-do-not-use-macros-or-bundles", "policyPluginBuilderBreaches", {
    "a/🦀.rs": ["semio_plugin! { x }"], "b/🦀.rs": ["let p = PluginBundle::new();", "let q = PluginBundle { a: 1 };"], "c/🦀.rs": ["Plugin::builder(1)"], "d/🦀.rs": ["semio_plugin! { x }"], SDK: ["semio_plugin! { x }"],
}, [b("plugin-builder-macro-a/🦀.rs", PB), b("plugin-builder-bundle-b/🦀.rs", PB)],
     crates=[mk("a/🦀.rs"), mk("b/🦀.rs", "extension"), mk("c/🦀.rs"), mk("d/🦀.rs", "tool"), mk(SDK), mk("missing/🦀.rs")])
SN = "taxonomy/spr-naming"
OWN = "✏s/🔌plugins/🔭p"
lib = OWN + "/📦packages/🦀rust/🦀.rs"
lib_lines = ["mod protocol_spec;", "pub struct SomeProtocolHandler;", "enum Protocols {}", "type X = u8;", "fn protocol() {}", "const Protocol: u8 = 1;"]
offsets = []
pos = 0
for n, line in enumerate(lib_lines, 1):
    m = re.search(r"\b(?:mod|struct|enum|type)\s+(\w*[Pp]rotocol\w*)\b", line)
    if m:
        offsets.append((pos + m.start(), n))
    pos += len(line) + 1
case("spr-naming-forbids-protocol-names", "policySprNamingBreaches", {
    OWN + "/🧩protocol/x": x, OWN + "/📡Protocol.rs": x, OWN + "/📡spr/x": x, lib: lib_lines,
}, [b("spr-naming-path-%s/🧩protocol" % OWN, SN, "medium"), b("spr-naming-path-%s/📡Protocol.rs" % OWN, SN, "medium")] + [b("spr-naming-ident-%s-%d" % (lib, off), SN, "medium", n) for off, n in offsets],
     crates=[{"libRelPath": lib, "role": "plugin", "pluginId": "p", "ownerRel": OWN, "shape": "taxonomy", "dir": OWN + "/📦packages/🦀rust"}])
EM = "file/name/"
FM = "folder/name/"
base = "✏s/🔌plugins/🔭probe/"
emoji_rows = [
    ("directory-without-an-emoji-prefix", {base + "plain-dir/🧾ok.txt": x}, [b("path-emoji-missing-" + base + "plain-dir", FM + "missing-emoji")]),
    ("duplicate-sibling-emoji-identities", {base + "🧭first/🧾a.txt": x, base + "🧭second/🧾a.txt": x}, [b("path-emoji-duplicate-" + base + "🧭second", FM + "emoji-not-unique")]),
    ("generic-folder-emoji", {base + "📁generic/🧾a.txt": x}, [b("path-emoji-generic-" + base + "📁generic", FM + "generic-emoji")]),
    ("whitespace-after-the-emoji", {base + "🧾 spaced.ts": x}, [b("path-emoji-spacing-" + base + "🧾 spaced.ts", EM + "whitespace-after-emoji")]),
    ("several-emojis-in-one-name", {base + "🧾🧭double.ts": x}, [b("path-emoji-multiple-" + base + "🧾🧭double.ts", EM + "multiple-emojis")]),
    ("clean-emoji-names", {base + "🧾ok.ts": x, base + "🧭dir/🧾inner.txt": x}, []),
]
for id_, files, expect in emoji_rows:
    case("emoji-prefix-" + id_, "policyEmojiPrefixBreaches", files, expect, git=True, oracle=False)

props = {"crates": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["libRelPath", "role", "pluginId", "ownerRel", "shape", "dir"], "properties": {k: {"type": "string"} for k in ("libRelPath", "role", "pluginId", "ownerRel", "shape", "dir")}}}}
rules = ["policyBannedNameStemBreaches", "policyPluginRootShapeBreaches", "policyPluginBuilderBreaches", "policySprNamingBreaches", "policyEmojiPrefixBreaches"]
schema = std_schema("Naming Law Corpus", rules, props)
refs = "(row.crates ?? []).map((c: any) => ({ dir: c.dir, libRelPath: c.libRelPath, shape: c.shape, role: c.role, ownerRel: c.ownerRel, pluginId: c.pluginId }))"
ts = std_ts(
    "🏷 The naming laws report exactly the banned-stem, plugin-root, builder, spr-naming and emoji-prefix breaches their language-agnostic corpus names, agree with an independent Python model where one is modelled, and reject a malformed corpus.",
    'import { policyBannedNameStemBreaches, policyEmojiPrefixBreaches, policyPluginBuilderBreaches, policyPluginRootShapeBreaches, policySprNamingBreaches } from "../../🟦.ts";',
    '''  policyBannedNameStemBreaches: (root) => policyBannedNameStemBreaches(root),
  policyPluginRootShapeBreaches: (root) => policyPluginRootShapeBreaches(root),
  policyPluginBuilderBreaches: (root, row) => policyPluginBuilderBreaches(root, %s),
  policySprNamingBreaches: (root, row) => policySprNamingBreaches(root, %s),
  policyEmojiPrefixBreaches: (root) => policyEmojiPrefixBreaches(root),''' % (refs, refs),
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🗺taxonomy-shape/🏷naming", "🏷naming", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
