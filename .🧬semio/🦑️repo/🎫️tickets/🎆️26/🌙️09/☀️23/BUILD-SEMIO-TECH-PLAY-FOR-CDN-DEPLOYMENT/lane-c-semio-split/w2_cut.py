#!/usr/bin/env python3
"""✂️ Wave 2 (split plan S2): the semio monolith becomes 47 feature-free crates. Rewrites every crate root, maps every
`crate::` path to the crate that owns it after the cut, and writes the new manifests and the workspace manifest.
All writes are staged in memory and applied at the end, so a failure leaves the tree untouched."""
import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from ownership import CRATE, rust_name, crate_roots, owners
from use_rewrite import rewrite_uses, rewrite_linear, _mask

pending = {}


def get(p):
    return pending[p] if p in pending else read(p)


def put(p, s):
    pending[p] = s


own = owners()
roots = crate_roots()
HEADER = ("#![allow(async_fn_in_trait)]\n#![allow(long_running_const_eval)]\n\n"
          "extern crate semio_framework_os_kernel as dsl;\nextern crate semio_framework_os_kernel as protocol;\nextern crate semio_framework_os_kernel as store;\n"
          "extern crate semio_framework_schema as framework_schema;\nextern crate semio_framework_value_derive as value_derive;\n")


def with_header(src, extra=""):
    lines = src.split("\n")
    k = 0
    while k < len(lines) and lines[k].startswith("//!"):
        k += 1
    return "\n".join(lines[:k]) + "\n\n" + HEADER + extra + "\n" + "\n".join(lines[k:]).lstrip("\n")


ALIAS_RX = re.compile(r"(?m)^[ \t]*use crate::standards::v1::subsets(?: as (\w+))?;[ \t]*\n")


def drop_subset_alias(src):
    names = set()
    for m in ALIAS_RX.finditer(src):
        names.add(m.group(1) or "subsets")
    src = ALIAS_RX.sub("", src)
    src = re.sub(r";[ \t]*use crate::standards::v1::subsets as (\w+);", lambda m: (names.add(m.group(1)), ";")[1], src)
    for n in names:
        src = re.sub(r"(?<![\w:])%s::base::" % n, "crate::", src)
        src = re.sub(r"(?<![\w:])%s::(%s)::" % (n, "|".join(NAMES)), lambda m: rust_name(m.group(1)) + "::", src)
    return src


# 🌳️ aggregator root
root = get(ROOT_RS)
root = root.replace('#[path = "🏅️standards/🔖️v1/🪆️subsets/✉️base/🦀️.rs"]\npub mod base_foundation;\n\n', "")
lines = root.split("\n")
s0 = lines.index("pub mod standards {") - 1
s1 = block_end(lines, s0 + 1)
std_block = lines[s0:s1 + 1]
bi = std_block.index("            pub mod base {")
be = block_end(std_block, bi)
base_body = [l[16:] if l.startswith(" " * 16) else l.lstrip() for l in std_block[bi + 1:be]]
lines[s0:s1 + 1] = base_body
root = "\n".join(lines)
root = drop_subset_alias(root)
a = root.index("/// 🔗️ The conversions this build's features select")
b = root.index("];\n", a) + 3
root = root[:a] + root[b:]
root = root.replace("pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<", "pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition, conversions: &[semio_s_artifact_stdio_semio_base::SemioConversion]) -> Result<")
root = root.replace("CONVERSIONS.iter().fold(builder, |builder, conversion| (conversion.declare)(builder)).try_build()", "conversions.iter().fold(builder, |builder, conversion| (conversion.declare)(builder)).try_build()")
root = root.replace("pub fn register() {", "pub fn register(conversions: &[semio_s_artifact_stdio_semio_base::SemioConversion]) {")
root = root.replace("    for conversion in CONVERSIONS {\n", "    for conversion in conversions {\n")
for fn in ("contribution", "assembly", "native_codecs"):
    m = re.search(r"(?m)^(?:///[^\n]*\n|//[^\n]*\n)*pub fn %s\(" % fn, root)
    a = m.start()
    b = root.index("\n}\n", a) + 3
    root = root[:a] + root[b:]
root = re.sub(r"(?m)^/// 🎯️ The central applier, re-exported[^\n]*\n(?:///[^\n]*\n)*pub use protocol::apply_diff;\n", "", root)
root = re.sub(r"\n*pub use crate::standards::v1::subsets::\w+::io::\{Semio\w*BuilderConstruction[^\n]*\n", "\n", root)
root = re.sub(r"pub use standards::v1::subsets::base::schema::", "pub use schema::", root)
root = root.replace("//#region 🧪️Tests\n", "//#region 🧪️Tests\n#[cfg(test)]\n#[path = \".\"]\nmod subset_sqlite_snapshot_tests {\n" + "".join('    #[path = "🏅️standards/🔖️v1/🪆️subsets/%s/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]\n    pub(crate) mod %s;\n' % (EMOJI[s], s) for s in NAMES) + "}\n", 1)
def root_alias_mapper(seg):
    if seg[:1] == ["subsets"] and len(seg) > 1 and seg[1] in NAMES:
        return [rust_name(seg[1])] + seg[2:]
    return None


root, _ = rewrite_uses(root, root_alias_mapper)
put(ROOT_RS, root.rstrip("\n") + "\n")

for p, k in own.items():
    if k == "agg" and p != ROOT_RS:
        s = get(p)
        s2 = drop_subset_alias(s)
        if s2 != s:
            put(p, s2)
union_tests = SUBSETS + "/✉️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"
put(union_tests, re.sub(r"crate::standards::v1::subsets::(\w+)::io::sqlite::snapshot::tests::", r"crate::subset_sqlite_snapshot_tests::\1::", get(union_tests)))
for s in NAMES:
    p = SUBSETS + "/" + EMOJI[s] + "/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"
    src = get(p)
    src2 = re.sub(r"#\[cfg\(test\)\]\s*#\[path\s*=\s*\"🧪️tests/🦀️.rs\"\]\s*(?:pub(?:\([a-z]+\))?\s*)?mod tests;\n?", "", src)
    assert src2 != src, s
    put(p, src2)

# 🪆️ subset, conversion, foundation and catalog roots
CONV_MOUNT_RX = re.compile(r'(?m)^[ \t]*#\[cfg\(feature = "conversion-[a-z]+"\)\]\n[ \t]*#\[path = "(?:🚪️io/)?(?:📥️import|📤️export)/🦀️.rs"\]\n[ \t]*pub mod (?:import|export);\n')
for s in NAMES:
    p = roots[s]
    src = CONV_MOUNT_RX.sub("", get(p)).replace("crate::semio_member_owner!(", "semio_s_artifact_stdio_semio_base::semio_member_owner!(")
    put(p, with_header(src))
for s in ("video", "audio", "animation"):
    p = SUBSETS + "/" + EMOJI[s] + "/🚪️io/🦀️.rs"
    put(p, CONV_MOUNT_RX.sub("", get(p)))
for s in CONVERSION_SUBSETS:
    for d in ("import", "export"):
        put(roots[s + "-" + d], with_header(get(roots[s + "-" + d])))
base = get(roots["base"]).replace("$crate::base_foundation::", "$crate::")
put(roots["base"], with_header(base))
CATALOG = roots["catalog"]
put(CATALOG, "//! 📇️ The full semio catalog: the aggregator's artifact with every subset conversion of the family declared, the\n//! contribution the stdio hub selects for its full catalog.\n\n"
    "/// 🔗️ Every conversion of the semio family, each subset's import before its export.\n"
    "pub const ALL: &[semio_s_artifact_stdio_semio_base::SemioConversion] = &[\n"
    + "".join("    semio_s_artifact_stdio_semio_%s_import::CONVERSION,\n    semio_s_artifact_stdio_semio_%s_export::CONVERSION,\n" % (s, s) for s in CONVERSION_SUBSETS)
    + "];\n\n"
    "/// 🧾️ The semio artifact declared with every conversion of the family.\n"
    "pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {\n"
    "    semio_s_artifact_stdio_semio::declaration(definition, ALL)\n}\n\n"
    "/// 🗂️ The runtime assembly of the full semio artifact.\n"
    "pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {\n"
    "    semio_s_artifact_stdio_contract::runtime_assembly(\"semio\", semio_s_artifact_stdio_semio::definition()?, declaration)\n}\n\n"
    "/// 🧵️ The native codec factories of the full catalog; semio publishes none.\n"
    "pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {\n    Vec::new()\n}\n\n"
    "/// 📇️ The full-catalog contribution of the semio artifact.\n"
    "pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {\n"
    "    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: \"semio\", schema: semio_s_artifact_stdio_semio::ARTIFACT_DEFINITION_SCHEMA, definition: semio_s_artifact_stdio_semio::definition, assembly, formats: semio_s_artifact_stdio_semio::formats, native_codecs }\n}\n")
own[CATALOG] = "catalog"

# ➡️ path codemod, per owning crate
def target_of(seg_after_crate):
    """🎯️ (crate key, rest) for a path written as `crate::<seg_after_crate>` in the monolith."""
    s = seg_after_crate
    if s[:1] == ["base_foundation"]:
        return "base", s[1:]
    if s[:3] == ["standards", "v1", "subsets"] and len(s) > 3:
        sub = s[3]
        if sub == "base":
            return "agg", s[4:]
        if sub in NAMES:
            if s[4:6] in (["io", "import"], ["io", "export"]):
                return sub + "-" + s[5], s[6:]
            return sub, s[4:]
    return None, None


def mapper_for(key):
    def m(seg):
        if seg[:1] != ["crate"]:
            return None
        t, rest = target_of(seg[1:])
        if t is None:
            return None
        return (["crate"] if t == key else [rust_name(t)]) + rest
    return m


LIN = re.compile(r"(?<![\w$])crate::(base_foundation|standards::v1::subsets::(\w+)(?:::io::(import|export)\b)?)(?=::|;|\}|,|\s)")


def map_linear(src, key):
    masked = _mask(src)
    out, last = [], 0
    for mm in LIN.finditer(masked):
        if mm.group(1) == "base_foundation":
            t = "base"
        elif mm.group(2) == "base":
            t = "agg"
        elif mm.group(2) in NAMES:
            t = mm.group(2) + ("-" + mm.group(3) if mm.group(3) else "")
        else:
            continue
        out.append(src[last:mm.start()])
        out.append("crate" if t == key else rust_name(t))
        last = mm.end()
    out.append(src[last:])
    return "".join(out)


for p, key in own.items():
    s = get(p)
    if key == "agg":
        s = s.replace("crate::declaration(crate::definition().unwrap())", "crate::declaration(crate::definition().unwrap(), &[])")
    s1, _ = rewrite_uses(s, mapper_for(key))
    s2 = map_linear(s1, key)
    s3 = re.sub(r'(?m)^[ \t]*#\[cfg\(feature = "conversion-[a-z0-9-]+"\)\][ \t]*\n', "", s2)
    if s3 != s:
        put(p, s3)

for p, c in pending.items():
    write(p, c)
json.dump({p: k for p, k in own.items()}, open(__file__.rsplit("/", 1)[0] + "/ownership.json", "w"), ensure_ascii=False, indent=0)
print("staged", len(pending), "files")
