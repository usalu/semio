"""Task 1 layer B: mutation layer (payload enum, ChangeKFiDeclared removed everywhere), edit rules, text io, DSL assets."""
from lib import *
import shutil, glob
RCM = "semio_s_artifact_norm_en1990_model::ReliabilityClass"
RC = MUT + "/🎯change-reliability-class"

# payload twins
def pj(t):
    old = '''    "newReliabilityClass": {
      "type": "integer",
      "minimum": 0,
      "x-semio-ui": {
        "label": {
          "en": "Reliability class RC",
          "de": "Zuverlässigkeitsklasse RC"
        },
        "description": {
          "en": "1 = RC1, 2 = RC2, 3 = RC3 (EN 1990 Annex B).",
          "de": "1 = RC1, 2 = RC2, 3 = RC3 (EN 1990 Anhang B)."
        },
        "widget": "stepper",
        "step": 1,
        "precision": 0
      }
    }'''
    new = '''    "newReliabilityClass": {
      "type": "string",
      "enum": [
        "RC1",
        "RC2",
        "RC3"
      ],
      "x-semio-ui": {
        "label": {
          "en": "Reliability class RC",
          "de": "Zuverlässigkeitsklasse RC"
        },
        "description": {
          "en": "RC1, RC2 or RC3 (EN 1990 Annex B); K_FI follows from the class.",
          "de": "RC1, RC2 oder RC3 (EN 1990 Anhang B); K_FI folgt aus der Klasse."
        },
        "widget": "select",
        "role": "value",
        "group": "value",
        "order": 20
      }
    }'''
    return rep(t, old, new, guard='"RC3"')
edit(RC + "/🧬️schema/🔣️.json", pj)
def pts(t):
    t = rep(t, 'import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";', 'import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";\nimport { type En1990ReliabilityClass, parseEn1990ReliabilityClass } from "../../../📸️snapshot/🟦️.ts";', guard="En1990ReliabilityClass")
    t = rep(t, "  newReliabilityClass: number;", "  newReliabilityClass: En1990ReliabilityClass;", guard="newReliabilityClass: En1990")
    t = rep(t, 'newReliabilityClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0}))', "newReliabilityClass: normWireRequired(parseEn1990ReliabilityClass)", guard="parseEn1990ReliabilityClass)")
    return t
edit(RC + "/🧬️schema/🟦️.ts", pts)

# mutation tests
for p in (MUT + "/🧪️tests/🔬️unit/🦀️.rs", TXT + "/🧬️mutations/🦀️.rs"):
    edit(p, lambda t: rep(t, "ChangeReliabilityClass { new_reliability_class: 3 }", "ChangeReliabilityClass { new_reliability_class: %s::RC3 }" % RCM, guard=RCM + "::RC3"))

# ChangeKFiDeclared removal
KFI = MUT + "/🏛️change-k-fi-declared"
if os.path.isdir(KFI):
    shutil.rmtree(KFI); print("  removed", KFI.replace(SUB, ""))
FX = SUB + "/🧪️tests/🧫️fixtures/🧬️mutations/🏛️change-k-fi-declared"
if os.path.isdir(FX):
    shutil.rmtree(FX); print("  removed", FX.replace(SUB, ""))

def drop_lines(t, needles, count=1):
    lines = t.split("\n")
    keep = [l for l in lines if not any(n in l for n in needles)]
    assert len(lines) - len(keep) == count, (needles, len(lines) - len(keep))
    return "\n".join(keep)

edit(MUT + "/🦀️.rs", lambda t: t if "change_k_fi_declared" not in t else drop_lines(t, ["change_k_fi_declared", "ChangeKFiDeclared("], 2))
edit(MUT + "/🏛️design-basis-structure/🦀️.rs", lambda t: rep(rep(t, '#[path = "../🏛️change-k-fi-declared/🦀️.rs"]\npub mod change_k_fi_declared;\n', "", guard="@@") if "change_k_fi_declared" in t else t, "k fi declared, structure kind", "structure kind") if "k fi declared" in t else t)
edit(MUT + "/🧭️edit-rules/🦀️.rs", lambda t: t if "k_fi_declared" not in t and "kFiDeclared" not in t else drop_lines(t, ["kFiDeclared"]))
edit(TXT + "/🧬️mutations/🦀️.rs", lambda t: t if "k_fi_declared" not in t else drop_lines(t, ["change_k_fi_declared", "ChangeKFiDeclared"], 3))
edit(MUT + "/🔣️.json", lambda t: t if "change-k-fi-declared" not in t else rep(t, '    {\n      "$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1990/mutation/change-k-fi-declared/schema.json"\n    },\n', ""))
edit(MUT + "/🟦️.ts", lambda t: t if "KFiDeclared" not in t else drop_lines(t, ["ChangeKFiDeclared", "parseChangeKFiDeclared"], 3))
edit(SUB + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", lambda t: t if "change-k-fi-declared" not in t else rep(t, "record change-k-fi-declared tag=89\nfield payload bytes\n", ""))
edit(SUB + "/🧪️tests/⚖️mutate-en1990-1/🥒️.feature", lambda t: t if "change-k-fi-declared" not in t else drop_lines(t, ["change-k-fi-declared"], 2))

def manifest(t):
    if "change-k-fi-declared" not in t:
        return t
    blk = '''        {
          "mutationId": "change-k-fi-declared",
          "sourceMutationDirectoryName": "🏛️change-k-fi-declared",
          "mutationDirectoryName": "🏛️change-k-fi-declared",
          "scenarios": [
            {
              "id": "apply",
              "directoryName": "✅apply"
            }
          ]
        },
'''
    return rep(t, blk, "")
edit(SUB + "/🔮️oracles/🔣️.json", manifest)

def taxonomy(t):
    if "change-k-fi-declared" not in t:
        return t
    return re.sub(r'    \{\n      "aggregateVariant": "ChangeKFiDeclared",.*?\n    \},\n', "", t, count=1, flags=re.S)
edit(SEMIO + "/✏️s/🔌️plugins/📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json", taxonomy)

# DSL assets and hub composition: reliability-class=N -> rcN, k-fi-declared removed
def dsl(t):
    t = re.sub(r"reliability-class=(\d)", r"reliability-class=rc\1", t)
    t = re.sub(r" k-fi-declared=[0-9.]+", "", t)
    return t
for p in glob.glob(SUB + "/🖼️assets/*/*/🗣️.dsl.semio"):
    edit(p, dsl)
edit(SEMIO + "/🌎️hub/🧩️compositions/📕️norm/🧩️component/🔣️.json", lambda t: dsl(t) if "k-fi-declared" in t else t)

# schema-document-index: the removed mutation schema
def docindex(t):
    return re.sub(r'\n      "https://json\.schemas\.assets\.semio-tech\.com/s/norm/en1990/mutation/change-k-fi-declared/schema\.json": [^\n]*', "", t, count=1) if "en1990/mutation/change-k-fi-declared" in t else t
edit(SEMIO + "/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json", docindex)
