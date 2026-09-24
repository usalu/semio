"""🔀️ One atomic landing of the protocol outcome vocabulary in leaf descriptors (T10).

Replaces the severity vocabulary (applied/info/warning/error/fatal) in the leaf descriptor schema, the Rust
`MutationOutcomeClass`, the `MutationLeaf` derive, every bridge, the test platform's reader/scaffolder and every leaf
descriptor with the protocol vocabulary (applied/no-op/empty/disjoint/rejected). Leaf values come from the
hand-reviewed table `wp-t10/leaf-outcomes.json`; manifests are aligned to it. `--dry` reports without writing.
"""
import json, os, re, sys, collections
root = "/Users/ueli/Documents/semio/"
dry = "--dry" in sys.argv
only_plugins = [a for a in sys.argv[1:] if not a.startswith("--")]
changed, problems = [], []

def edit(rel, pairs, count=1):
    path = root + rel
    text = open(path, encoding="utf-8").read()
    new = text
    for old, rep in pairs:
        n = new.count(old)
        if n != count and not (count is None and n > 0):
            problems.append(f"{rel}: expected {count} of {old[:80]!r}, found {n}")
            return
        new = new.replace(old, rep)
    if new != text:
        changed.append(rel)
        if not dry: open(path, "w", encoding="utf-8").write(new)

REPL = "🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs"
edit(REPL, [
    ("""/// 🧷️ Schema vocabulary for one direct mutation's observable outcomes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationOutcomeClass {
    Applied,
    Info,
    Warning,
    Error,
    Fatal,
}
""", """/// 🧷️ The protocol outcome class one direct mutation can reach — the vocabulary fixtures, manifests and bridges
/// share verbatim, so no layer projects one record onto another.
///
/// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — `$defs/MutationOutcomeClass`
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MutationOutcomeClass {
    Applied,
    NoOp,
    Empty,
    Disjoint,
    Rejected,
}

impl MutationOutcomeClass {
    /// 🏷️ The wire spelling of this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            MutationOutcomeClass::Applied => "applied",
            MutationOutcomeClass::NoOp => "no-op",
            MutationOutcomeClass::Empty => "empty",
            MutationOutcomeClass::Disjoint => "disjoint",
            MutationOutcomeClass::Rejected => "rejected",
        }
    }
}
"""),
    ("""        crate::value::DslValue::String(
            match self {
                MutationOutcomeClass::Applied => "applied",
                MutationOutcomeClass::Info => "info",
                MutationOutcomeClass::Warning => "warning",
                MutationOutcomeClass::Error => "error",
                MutationOutcomeClass::Fatal => "fatal",
            }
            .to_string(),
        )""", """        crate::value::DslValue::String(self.as_str().to_string())"""),
])

DERIVE = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs"
edit(DERIVE, [
    ("enum MutationLeafOutcomeClass { Applied, Info, Warning, Error, Fatal }", "enum MutationLeafOutcomeClass { Applied, NoOp, Empty, Disjoint, Rejected }"),
    ('match value.as_str() { "applied" => Ok(MutationLeafOutcomeClass::Applied), "info" => Ok(MutationLeafOutcomeClass::Info), "warning" => Ok(MutationLeafOutcomeClass::Warning), "error" => Ok(MutationLeafOutcomeClass::Error), "fatal" => Ok(MutationLeafOutcomeClass::Fatal), _ =>',
     'match value.as_str() { "applied" => Ok(MutationLeafOutcomeClass::Applied), "no-op" => Ok(MutationLeafOutcomeClass::NoOp), "empty" => Ok(MutationLeafOutcomeClass::Empty), "disjoint" => Ok(MutationLeafOutcomeClass::Disjoint), "rejected" => Ok(MutationLeafOutcomeClass::Rejected), _ =>'),
    ("MutationLeafOutcomeClass::Applied => quote!(#contract::MutationOutcomeClass::Applied), MutationLeafOutcomeClass::Info => quote!(#contract::MutationOutcomeClass::Info), MutationLeafOutcomeClass::Warning => quote!(#contract::MutationOutcomeClass::Warning), MutationLeafOutcomeClass::Error => quote!(#contract::MutationOutcomeClass::Error), MutationLeafOutcomeClass::Fatal => quote!(#contract::MutationOutcomeClass::Fatal)",
     "MutationLeafOutcomeClass::Applied => quote!(#contract::MutationOutcomeClass::Applied), MutationLeafOutcomeClass::NoOp => quote!(#contract::MutationOutcomeClass::NoOp), MutationLeafOutcomeClass::Empty => quote!(#contract::MutationOutcomeClass::Empty), MutationLeafOutcomeClass::Disjoint => quote!(#contract::MutationOutcomeClass::Disjoint), MutationLeafOutcomeClass::Rejected => quote!(#contract::MutationOutcomeClass::Rejected)"),
])

edit("🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️tests/🔬️unit/🦀️.rs", [
    ("json_oracle(&vec![MutationOutcomeClass::Applied, MutationOutcomeClass::Info, MutationOutcomeClass::Warning, MutationOutcomeClass::Error, MutationOutcomeClass::Fatal])",
     "json_oracle(&vec![MutationOutcomeClass::Applied, MutationOutcomeClass::NoOp, MutationOutcomeClass::Empty, MutationOutcomeClass::Disjoint, MutationOutcomeClass::Rejected])"),
])
edit("🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧫️fixtures/🪪️mutation-leaf-descriptor/🔣️.json", [
    ('"outcomeClasses": ["applied", "info", "warning", "error", "fatal"],', '"outcomeClasses": ["applied", "no-op", "empty", "disjoint", "rejected"],'),
])
edit("🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧬️schema/🔣️.json", [
    ('''    "MutationOutcomeClass": {
      "enum": [
        "applied",
        "info",
        "warning",
        "error",
        "fatal"
      ]
    },''', '''    "MutationOutcomeClass": {
      "description": "🎯️ The protocol outcome class a direct mutation can reach — the same vocabulary as the repository test protocol's `MutationOutcomeClass`, carried verbatim by bridges and manifests.",
      "enum": [
        "applied",
        "no-op",
        "empty",
        "disjoint",
        "rejected"
      ]
    },'''),
])
edit("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔣️.json", [
    ('''    "outcomeClasses": {
      "type": "array",
      "minItems": 1,
      "uniqueItems": true,
      "items": {
        "enum": [
          "applied",
          "info",
          "warning",
          "error",
          "fatal"
        ]
      }
    },''', '''    "outcomeClasses": {
      "description": "🎯️ Every protocol outcome class the leaf's own diff can reach, read from its code: `applied` (a change), `no-op` (no change, no refusal), `empty`/`disjoint` (format-defined empty or disjoint results), `rejected` (an Error or Fatal message). Bridges and manifests carry these values verbatim.",
      "type": "array",
      "minItems": 1,
      "uniqueItems": true,
      "items": {
        "enum": [
          "applied",
          "no-op",
          "empty",
          "disjoint",
          "rejected"
        ]
      }
    },'''),
])

DERIVE_FIXTURE = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧫️fixtures/🔣️mutation-leaf-json/🔣️.json"
edit(DERIVE_FIXTURE, [
    ('\\"outcomeClasses\\":[\\"applied\\",\\"warning\\"]', '\\"outcomeClasses\\":[\\"applied\\",\\"no-op\\"]'),
    ('\\"outcomeClasses\\":[\\"info\\"]', '\\"outcomeClasses\\":[\\"no-op\\"]'),
    ('\\"outcomeClasses\\":[\\"error\\"]', '\\"outcomeClasses\\":[\\"rejected\\"]'),
    ('\\"outcomeClasses\\":[\\"fatal\\"]', '\\"outcomeClasses\\":[\\"disjoint\\"]'),
    ('\\"outcomeClasses\\":[\\"applied\\",\\"info\\",\\"warning\\",\\"error\\",\\"fatal\\"]', '\\"outcomeClasses\\":[\\"applied\\",\\"no-op\\",\\"empty\\",\\"disjoint\\",\\"rejected\\"]'),
], count=None)

TEST = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts"
edit(TEST, [
    ("""  outcomeClasses: readonly string[];
  composition: "atomic" | "composite";
  requiredLanguageSurfaces: readonly string[];
}>;""", """  outcomeClasses: readonly MutationOutcomeClass[];
  composition: "atomic" | "composite";
  requiredLanguageSurfaces: readonly string[];
}>;"""),
    ("""/** 🎯️ Maps the implementation's outcome vocabulary onto the protocol's declared classes. */
export function outcomeClassesOf(descriptor: MutationLeafDescriptor): MutationOutcomeClass[] {
  // 🎯️`Info`/`Warning` are DIAGNOSTIC severities on an outcome, not outcome classes — a mutation that
  // applies with a warning still applied. `Error`/`Fatal` are the refusal. Collapsing them here is the
  // one piece of vocabulary translation between the two records, and it is done in one place.
  const mapped = new Set<MutationOutcomeClass>();
  for (const raw of descriptor.outcomeClasses) {
    const value = raw.toLowerCase();
    if (value === "applied" || value === "info" || value === "warning") mapped.add("applied");
    else if (value === "error" || value === "fatal" || value === "rejected") mapped.add("rejected");
    else if ((MUTATION_OUTCOME_CLASSES as readonly string[]).includes(value)) mapped.add(value as MutationOutcomeClass);
  }
  if (mapped.size === 0) mapped.add("applied");
  return [...mapped];
}""", """/** 🎯️ The protocol outcome classes a leaf descriptor declares — carried verbatim, refusing any value outside the protocol vocabulary. */
export function outcomeClassesOf(descriptor: MutationLeafDescriptor): MutationOutcomeClass[] {
  const foreign = descriptor.outcomeClasses.filter((value) => !(MUTATION_OUTCOME_CLASSES as readonly string[]).includes(value));
  if (foreign.length > 0) throw new Error(`${descriptor.owner}: outcomeClasses ${JSON.stringify(foreign)} are outside the protocol vocabulary ${MUTATION_OUTCOME_CLASSES.join("|")}`);
  return [...descriptor.outcomeClasses];
}"""),
    ("""      if (/MutationOutcome::error\\s*\\(/.test(line)) {
        outcomes.add("error");
        cite.push(`error@${index + 1}`);
      }
      if (/MutationOutcome::empty\\s*\\(/.test(line)) {
        outcomes.add("info");
        cite.push(`empty@${index + 1}`);
      }""", """      if (/MutationOutcome::(?:error|fatal)\\s*\\(/.test(line)) {
        outcomes.add("rejected");
        cite.push(`rejected@${index + 1}`);
      }
      if (/MutationOutcome::empty\\s*\\(|"mutation\\.no-op"/.test(line)) {
        outcomes.add("no-op");
        cite.push(`no-op@${index + 1}`);
      }"""),
    ("""          const mapped = status === "applied" ? "applied" : status === "rejected" || status === "error" ? "error" : status === "no-op" || status === "empty" ? "info" : "";
          if (mapped.length > 0) {
            outcomes.add(mapped);""", """          if ((MUTATION_OUTCOME_CLASSES as readonly string[]).includes(status)) {
            outcomes.add(status);"""),
])

BRIDGE_DOC = re.compile(r"\n(?:///[^\n]*\n)+fn protocol_outcomes\(classes: &\[protocol::MutationOutcomeClass\]\) -> Vec<&'static str> \{\n.*?\n\}\n", re.S)
BRIDGE_OLD = re.compile(r"\n/// 🎯️ Production outcome severities as protocol outcome classes[^\n]*\nfn protocol_outcomes\(classes: &\[MutationOutcomeClass\]\) -> Vec<&'static str> \{\n.*?\n\}\n", re.S)
for dp, ds, fs in os.walk(root):
    ds[:] = [d for d in ds if d not in ("node_modules", "target", ".git", "🗑️generated", ".tmp-ticket", ".tmp-ticket-0918", ".🧬semio", "dist")]
    if os.path.basename(dp) != "🏭️bridge" or "🦀️.rs" not in fs: continue
    rel = os.path.join(dp, "🦀️.rs")[len(root):]
    if only_plugins and not any(p in rel for p in only_plugins): continue
    text = open(root + rel, encoding="utf-8").read()
    if "protocol_outcomes" not in text: continue
    new = BRIDGE_OLD.sub("\n", text, count=1)
    new = BRIDGE_DOC.sub("\n", new, count=1)
    new = re.sub(r"protocol_outcomes\(([a-z_]+)\.outcome_classes\)\.into_iter\(\)\.map\(pack::JsonValue::from\)", r"\1.outcome_classes.iter().map(|class| pack::JsonValue::from(class.as_str()))", new)
    new = new.replace("use protocol::{Mutation, MutationLeafDescriptor, MutationOutcomeClass};", "use protocol::{Mutation, MutationLeafDescriptor};")
    if "protocol_outcomes" in new or "MutationOutcomeClass::Info" in new: problems.append(f"{rel}: bridge still projects"); continue
    changed.append(rel)
    if not dry: open(root + rel, "w", encoding="utf-8").write(new)

TEMPLATE = root + ".tmp-ticket/wp-t5/bridges.py"
ttext = open(TEMPLATE, encoding="utf-8").read()
tnew = re.sub(r"/// 🎯️ Production outcome severities as protocol outcome classes[^\n]*\nfn protocol_outcomes\(classes: &\[MutationOutcomeClass\]\) -> Vec<&'static str> \{\{\n.*?\n\}\}\n\n", "", ttext, count=1, flags=re.S)
tnew = tnew.replace("use protocol::{{Mutation, MutationLeafDescriptor, MutationOutcomeClass}};", "use protocol::{{Mutation, MutationLeafDescriptor}};")
tnew = tnew.replace("pack::json_array(protocol_outcomes(descriptor.outcome_classes).into_iter().map(pack::JsonValue::from))", "pack::json_array(descriptor.outcome_classes.iter().map(|class| pack::JsonValue::from(class.as_str())))")
if "protocol_outcomes" in tnew: problems.append("wp-t5/bridges.py: template still projects")
elif tnew != ttext:
    changed.append(".tmp-ticket/wp-t5/bridges.py")
    if not dry: open(TEMPLATE, "w", encoding="utf-8").write(tnew)

HAND = json.load(open(root + ".tmp-ticket/wp-t10/hand-descriptors.json", encoding="utf-8"))
for rel, spec in HAND.items():
    if only_plugins and not any(p in rel for p in only_plugins): continue
    edit(rel, [(o, n) for o, n in spec], count=None)

TABLE = json.load(open(root + ".tmp-ticket/wp-t10/leaf-outcomes.json", encoding="utf-8"))
ORDER = ["applied", "no-op", "empty", "disjoint", "rejected"]
for rel, classes in TABLE.items():
    if only_plugins and not any(p in rel for p in only_plugins): continue
    path = root + rel + "/🔣️.json"
    text = open(path, encoding="utf-8").read()
    m = re.search(r'("outcomeClasses"\s*:\s*)\[[^\]]*\]', text)
    if not m: problems.append(f"{rel}: no outcomeClasses"); continue
    indent = re.search(r'\n([ \t]*)"outcomeClasses"', text)
    pad = indent.group(1) if indent else "  "
    ordered = [c for c in ORDER if c in classes]
    inline = "\n" not in m.group(0)
    body = "[" + ", ".join(json.dumps(c) for c in ordered) + "]" if inline else "[\n" + ",\n".join(pad + "  " + json.dumps(c) for c in ordered) + "\n" + pad + "]"
    new = text[:m.start()] + m.group(1) + body + text[m.end():]
    if new != text:
        changed.append(rel + "/🔣️.json")
        if not dry: open(path, "w", encoding="utf-8").write(new)

print(f"{'would change' if dry else 'changed'} {len(changed)} file(s); {len(problems)} problem(s)")
for p in problems: print("PROBLEM", p)
json.dump(changed, open(root + ".tmp-ticket/wp-t12/generated/switch-changed.json", "w"), ensure_ascii=False, indent=1)
