"""📝️ S10 — declares `setSnapshot`'s `snapshot` text argument on the fourteen norm editors that lack it.

S9 bridged `command_from_action` on all fifteen, so the verb now REACHES the guest — measured on
`norm-din16798` (ticket 26/09/18 S10 §5.1): the refusal changed from the trait default
(`action 'setSnapshot' is not a framework-reserved action`) to the bridge's own
`setSnapshot needs a 'snapshot' argument carrying the document's camelCase JSON`. The rail never
offers a field to put it in, because only `🧱️din4108` declares the argument: the other fourteen use
the bare `.mutation("setSnapshot", …)` builder row, which stages no form at all
(`filled: []`, `submitted: "absent"`).

This rewrites that one row into din4108's proven `ActionDefinition::new_catalog(...).with_args(...)`
shape. Anchored on each file's own `.mutation("setSnapshot", ...)` line, asserted to match exactly
once per file before anything is written, and idempotent.
"""
import io, pathlib, re, sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
EDITORS = sorted((ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts").glob("*/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"))
ANCHOR = re.compile(r'^(?P<indent>[ \t]*)\.mutation\("setSnapshot", (?P<label>LocalizedLabel::native\([^()]*\))\)\n', re.MULTILINE)

bridged, skipped = 0, []
for path in EDITORS:
    text = io.open(path, encoding="utf-8").read()
    name = str(path).split("🗿️artifacts/")[1].split("/")[0]
    if ".with_args(vec![semio_framework_plugin::ActionArgDef::text(\"snapshot\"" in text:
        skipped.append(f"{name}: already declares the argument")
        continue
    hits = ANCHOR.findall(text)
    if len(hits) != 1:
        skipped.append(f"{name}: anchor matched {len(hits)} times, not 1")
        continue
    def repl(match):
        indent, label = match.group("indent"), match.group("label")
        return (
            f"{indent}// 📝️ `setSnapshot` replaces the whole compliance document, so the shells' `{{action,args}}`\n"
            f"{indent}// channel needs somewhere to put it: one staged text argument carrying the document's own\n"
            f"{indent}// camelCase JSON — the projection the Inputs window already renders. Without it the rail\n"
            f"{indent}// stages no form and the bridge refuses `norm.set-snapshot-arg-missing` (ticket 26/09/18 S10).\n"
            f"{indent}.action_with(\n"
            f"{indent}    semio_framework_plugin::ActionDefinition::new_catalog(\"setSnapshot\", {label}, semio_framework_plugin::ActionKind::Mutation)\n"
            f"{indent}        .with_args(vec![semio_framework_plugin::ActionArgDef::text(\"snapshot\", LocalizedLabel::native(\"Document JSON\", \"Dokument-JSON\"))]),\n"
            f"{indent})\n"
        )
    io.open(path, "w", encoding="utf-8").write(ANCHOR.sub(repl, text, count=1))
    bridged += 1

print(f"{bridged} declared, {len(skipped)} skipped")
for line in skipped:
    print("  skip", line)
