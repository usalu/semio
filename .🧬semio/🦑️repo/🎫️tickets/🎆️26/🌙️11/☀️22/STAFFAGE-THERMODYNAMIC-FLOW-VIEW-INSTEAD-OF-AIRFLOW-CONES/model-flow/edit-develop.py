"""One-shot editor: the model develops its flow for the one `FLOW_DEVELOP_S` of the depiction instead of a constant of its own."""
import sys, pathlib
S = pathlib.Path("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage")
def edit(rel, pairs):
    path = S / rel
    text = path.read_text(encoding="utf-8")
    for old, new, count in pairs:
        if (text.count(old) < 1) if count < 0 else (text.count(old) != count):
            sys.exit(f"{rel}: expected {count} of {old[:80]!r}, found {text.count(old)}")
        text = text.replace(old, new)
    path.write_text(text, encoding="utf-8")
edit("🔨️modules/🧊️volume/🌊️flow/🟦️.ts", [
    ("/** ⏳️ Seconds the flow of a model has developed before its static instant, as in the drawings: a pose is shown after that history, a looping clip after the whole periods that cover it. */\nexport const FLOW_MODEL_DEVELOP_S = 14;\n\n", "", 1),
])
edit("🔨️modules/🧊️volume/🟦️.ts", [
    ("import { VIEW_TITLES } from \"../🎨️depiction/🟦️.ts\";", "import { FLOW_DEVELOP_S, VIEW_TITLES } from \"../🎨️depiction/🟦️.ts\";", 1),
    ("import { FLOW_MODEL_DEVELOP_S, FLOW_TAG, flowParts } from \"./🌊️flow/🟦️.ts\";", "import { FLOW_TAG, flowParts } from \"./🌊️flow/🟦️.ts\";", 1),
    ("export { FLOW_MODEL, FLOW_MODEL_DEVELOP_S, FLOW_TAG,", "export { FLOW_MODEL, FLOW_TAG,", 1),
    ("{@link FLOW_MODEL_DEVELOP_S} seconds of history", "`FLOW_DEVELOP_S` seconds of history", 1),
    ("FLOW_MODEL_DEVELOP_S", "FLOW_DEVELOP_S", -1),
])
edit("📦️packages/🟦️typescript/🟦️.ts", [
    ("export { FLOW_MODEL, FLOW_MODEL_DEVELOP_S, FLOW_TAG,", "export { FLOW_MODEL, FLOW_TAG,", 1),
])
edit("🧪️tests/🌊️flow-model/🟦️.ts", [
    ("import { FLOW_MODEL, FLOW_MODEL_DEVELOP_S, FLOW_TAG,", "import { FLOW_MODEL, FLOW_TAG,", 1),
    ("import { FLOW_STYLE, SHARED_FLOW_EXCESS_K,", "import { FLOW_DEVELOP_S } from \"../../🔨️modules/🎨️depiction/🟦️.ts\";\nimport { FLOW_STYLE, SHARED_FLOW_EXCESS_K,", 1),
    ("FLOW_MODEL_DEVELOP_S", "FLOW_DEVELOP_S", -1),
])
print("ok")
