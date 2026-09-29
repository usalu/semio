"""🪡️ S20 faults overlay rebase (session 15): hand resolutions of the conflicts `s20-overlay-land.py rebase` reported. Each
entry resolves one conflict file hunk by hunk (`live`, `overlay`, `both` = live then overlay lines, or literal text), then
applies whole-file substitutions (old → new, each must match exactly once) that convert the live side's raise sites.
Writes `…/s14-s20-rebase/resolved/<rel>` + the live sha1 it answers into `resolved.json` (a later live change re-opens it).
Usage: python3 s20-rebase-resolve.py"""
from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path

LIVE = Path("/Users/ueli/Documents/semio")
HUB = LIVE / ".🧬semio/🌐hub"
OVERLAY = HUB / "s14-s20-overlay-faults"
REBASE = HUB / "s14-s20-rebase"
JACK = "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
PDF_PAGE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/🦀️.rs"
MANIFEST_TS = "🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts"
HUNK = re.compile(r"^<<<<<<< live\n(.*?)^\|\|\|\|\|\|\| base\n(.*?)^=======\n(.*?)^>>>>>>> overlay\n", re.S | re.M)

PDF_IMPORT = (
    "use semio_framework_plugin::plugin_app_close_prelude as ui;\n"
    "use semio_framework_plugin::{app_fault, ActionArgDef, ActionDefinition, ActionId, ActionKind, Buildable, Canvas2dScene, Fault, HasBase, HasChildren, Locale, LocalizedLabel, PluginAssemblyError, SurfaceKind, UiAssemblyResult, UiMapBuilder, UiText, UiValue, WindowKindDefinition, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowOptions};\n"
)
PDF_REQ_TEXT = (
    "    if directed_field(args) == key {\n"
    "        return committed_argument(args, key).ok_or_else(|| app_fault(\"stdio.window-kit.argument-required\").with_parameter(\"argument\", key));\n"
    "    }\n"
    "    semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, key)\n"
)

RESOLUTIONS: dict[str, dict] = {
    f"{JACK}/🎭️modes/✏️edit/🪟️windows/📝️editor/🎚️config/🦀️.rs": {"deleted": True},
    f"{JACK}/🎮️commands/▶️run-query/🧵️job/🦀️.rs": {"hunks": ["live"]},
    f"{JACK}/🎮️commands/✨️format-document/🦀️.rs": {"hunks": ["live", "live"]},
    f"{JACK}/🎮️commands/✏️text-edit/🦀️.rs": {
        "hunks": ["use semio_framework_plugin::{app_fault, Emit, Fault, NoConfigMutation};\n", "live"],
        "subs": [(
            'Fault::new(FaultOrigin::App, FaultCode::new("jack.query-too-large"), format!("a Jack query holds at most {} bytes", crate::JACK_QUERY_MAXIMUM_BYTES))',
            'app_fault("jack.query-too-large").with_parameter("maximum", crate::JACK_QUERY_MAXIMUM_BYTES.to_string())',
        )],
    },
    f"{JACK}/🦀️.rs": {
        "hunks": ["live", "live", "live"],
        "subs": [
            ('_ => return Err(Fault::from("jack-retained-document-route-mismatch")),', '_ => return Err(app_fault("jack-retained-document-route-mismatch")),'),
            ('return Err(Fault::from("text selection requires its retained transient operation owner")),', 'return Err(app_fault("jack.text-selection.owner-required")),'),
        ],
    },
    PDF_PAGE: {"hunks": [PDF_IMPORT, PDF_REQ_TEXT]},
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts": {"hunks": ["both", "both"]},
    MANIFEST_TS: {"whole": "manifest-ts"},
}


def sha(data: bytes) -> str:
    return hashlib.sha1(data).hexdigest()


def resolve_hunks(text: str, choices: list[str]) -> str:
    hunks = list(HUNK.finditer(text))
    assert len(hunks) == len(choices), f"{len(hunks)} hunks, {len(choices)} choices"
    out, cursor = [], 0
    for match, choice in zip(hunks, choices):
        live, _, overlay = match.group(1), match.group(2), match.group(3)
        out.append(text[cursor:match.start()])
        out.append({"live": live, "overlay": overlay, "both": live + overlay}.get(choice, choice))
        cursor = match.end()
    out.append(text[cursor:])
    return "".join(out)


def manifest_ts() -> str:
    """🪪️ The generated manifest TS = live's generator output + F1's `AppDefinition.faults` field and `FaultDefinition` type
    (spliced where the generator puts them: the field after `io`, the type before `GranularityDefinition`)."""
    live = (LIVE / MANIFEST_TS).read_text()
    ours = (OVERLAY / MANIFEST_TS).read_text()
    field = ours[ours.index("io: AppIo,\n/**\n * 🧯️"):ours.index("faults: Array<FaultDefinition>, };\n") + len("faults: Array<FaultDefinition>, };\n")]
    start = ours.rindex("/**", 0, ours.index("export type FaultDefinition"))
    kind = ours[start:ours.index("};\n\n", ours.index("export type FaultDefinition")) + len("};\n\n")]
    assert live.count("io: AppIo, };\n") == 1 and "FaultDefinition" not in live
    live = live.replace("io: AppIo, };\n", field)
    anchor = live.rindex("/**", 0, live.index("export type GranularityDefinition"))
    return live[:anchor] + kind + live[anchor:]


def main() -> None:
    recorded = json.loads((REBASE / "resolved.json").read_text()) if (REBASE / "resolved.json").exists() else {}
    for rel, resolution in RESOLUTIONS.items():
        if resolution.get("deleted"):
            assert not (LIVE / rel).exists(), rel
            recorded[rel] = "deleted"
            print("deleted ", rel)
            continue
        theirs = (LIVE / rel).read_bytes()
        if resolution.get("whole") == "manifest-ts":
            text = manifest_ts()
        else:
            text = resolve_hunks((REBASE / "conflicts" / rel).read_text(), resolution["hunks"])
        for old, new in resolution.get("subs", []):
            assert text.count(old) == 1, (rel, old)
            text = text.replace(old, new)
        assert "<<<<<<<" not in text and ">>>>>>>" not in text, rel
        target = REBASE / "resolved" / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text)
        recorded[rel] = sha(theirs)
        print("resolved", rel)
    (REBASE / "resolved.json").write_text(json.dumps(recorded, ensure_ascii=False, indent=1))


if __name__ == "__main__":
    main()
