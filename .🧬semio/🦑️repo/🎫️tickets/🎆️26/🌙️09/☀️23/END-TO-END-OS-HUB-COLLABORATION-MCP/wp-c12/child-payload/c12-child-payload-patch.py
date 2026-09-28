"""🩻️ C12 window-3 set (browser bundle, host TS under 🔌️plugin → L1's train): a guest error the component model raises as a jco
`ComponentError` says only "[object Object] (see error.payload)" — the guest's own typed reason sits in `error.payload`, and the
browser actor child dropped it (`childRejectionReason` read `.message` only). Seen three times on 7800/8010 (collab STEP 15, probe
`c12short-8010`): user2's child faulted on a resumed hub frame and nothing said why. The reason now carries the payload (JSON, bigints
as decimal, byte arrays as their length), bounded like every other field; law case added.
Idempotent; usage: python3 c12-child-payload-patch.py [--apply]"""
import sys

REPO = "/Users/ueli/Documents/semio/"
CHILD = REPO + "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/"
SCHEMA = CHILD + "🧬️schema/🟦️.ts"
LAW = CHILD + "🧬️schema/🧪️tests/🧪️browser-actor-child-rejection-carries-a-bounded-typed-reason/🟦️.ts"
HUNKS = [
    (SCHEMA,
     '  const message = text(() => (error as Error)?.message) ?? text(() => String(error)) ?? "";\n',
     '  const payload = text(() => {\n'
     '    const value = (error as { payload?: unknown })?.payload;\n'
     '    return value === undefined ? undefined : JSON.stringify(value, (_key, item: unknown) => (typeof item === "bigint" ? item.toString() : item instanceof Uint8Array ? `<${item.byteLength} bytes>` : item));\n'
     '  });\n'
     '  const message = payload ?? text(() => (error as Error)?.message) ?? text(() => String(error)) ?? "";\n'),
    (SCHEMA,
     "/** 🩻️ Names one guest failure: the phase it happened in, the export path it was reached through, and\n * the guest error's own class and message, each bounded.",
     "/** 🩻️ Names one guest failure: the phase it happened in, the export path it was reached through, and\n * the guest error's own class and message — a component-model error's typed `payload` when it has one (its `.message` is only\n * \"[object Object] (see error.payload)\") — each bounded."),
    (LAW,
     '    expect(childRejectionText(childRejectionReason("invoke", [], new Error("x")))).toContain("invoke -: ");\n',
     '    expect(childRejectionText(childRejectionReason("invoke", [], new Error("x")))).toContain("invoke -: ");\n'
     '    const component = Object.assign(new Error("[object Object] (see error.payload)"), { name: "ComponentError", payload: { tag: "fault", val: { code: "duplicate mutation id", at: 7n, bytes: new Uint8Array(3) } } });\n'
     '    expect(childRejectionReason("invoke", ["reactor", "poll"], component).message).toBe(\'{"tag":"fault","val":{"code":"duplicate mutation id","at":"7","bytes":"<3 bytes>"}}\');\n'),
]
apply = "--apply" in sys.argv
texts, problems = {}, []
for path, old, new in HUNKS:
    text = texts.get(path) or open(path, encoding="utf-8").read()
    if new in text:
        print("present", path.rsplit("/", 3)[-3:])
        texts[path] = text
        continue
    if text.count(old) != 1:
        problems.append((path, text.count(old)))
        continue
    texts[path] = text.replace(old, new)
    print("planned", path.rsplit("/", 3)[-3:])
print(f"{len(problems)} problems {problems}")
if problems:
    sys.exit(1)
if apply:
    for path, text in texts.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied")
