#!/usr/bin/env python3
"""💬️ LB2 p19 oracle: the `os-mcp` `search::long` description census and destructive-verb audit, reimplemented over
package descriptor JSON with python-jsonschema as the third-party validator of `CapabilityDescription`.

Rules (`🧰️framework/…/🌉️mcp/🗂️catalog/🦀️.rs` `description_problems` + `audit_declaration`): per app, every agent-audience
verb (window-kind actions first, then app actions; History/Clipboard/Interaction kinds and the framework view/shell ids
excluded) declares a description whose every cell validates against the manifest schema's `CapabilityDescription`, whose
German cell differs from its English cell, which does not only repeat the verb's title, and whose English cell no other
verb of the app shares. A Mutation/Shell verb whose id names the delete/replace class (or a Mutation naming a whole-document
replacement) declares `effects.destructive`; so does a Shell/View verb naming a user-path write.

usage: python3 lb2-p19-census.py <descriptor.json>… [--plugins stdio,stdio-pdf,…]
"""
import json, re, sys
from jsonschema import Draft202012Validator

MANIFEST_SCHEMA = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json"
DESTRUCTIVE = ["delete", "remove", "clear", "discard", "purge", "wipe", "erase", "truncate", "reset", "replace", "overwrite"]
DOCUMENT_REPLACE = ["setactiveexample", "setfixturejson", "setspecjson", "setsnapshot", "commitdocument", "loaddocument", "setdocument"]
USER_PATH = ["export", "download", "save"]
FRAMEWORK_VIEW_SHELL = {"setActiveUtility", "setActiveTool", "startIntroduction", "setHistoryCommandFilter", "noteShellCommand", "exportArtifactDocument", "importArtifactDocument"}


def words(verb):
    out, current = [], ""
    for character in verb:
        if character in "._-:":
            if current:
                out.append(current)
                current = ""
            continue
        if character.isupper() and current:
            out.append(current)
            current = ""
        current += character.lower()
    if current:
        out.append(current)
    return out


def lexicon(verb, entries):
    parts = words(verb)
    for start in range(len(parts)):
        run = ""
        for part in parts[start:]:
            run += part
            if run in entries:
                return run
    return None


def audience(action):
    declared = action.get("semantics", {}).get("audience")
    if declared:
        return declared
    if action["kind"] == "interaction":
        return "input"
    if action["kind"] == "view" and not action.get("inPalette", True):
        return "chrome"
    return "agent"


def validator():
    manifest = json.load(open(MANIFEST_SCHEMA, encoding="utf-8"))
    family = {key: value for key, value in manifest["$defs"].items() if key.startswith("CapabilityDescription")}
    return Draft202012Validator({"$schema": manifest["$schema"], "$defs": family, "$ref": "#/$defs/CapabilityDescription"})


def app_verbs(app):
    seen, verbs = {}, []
    for window in app.get("windowKinds", []):
        for action in window.get("actions", []):
            if action["id"] not in seen:
                seen[action["id"]] = action
                verbs.append(action)
    for action in app.get("actions", []):
        if action["id"] not in seen:
            seen[action["id"]] = action
            verbs.append(action)
    return verbs


def census(descriptor, check):
    plugin = descriptor["manifest"]["pluginId"]
    missing, schema, untranslated, repeats, shared, destructive = [], [], [], [], [], []
    for app in descriptor["manifest"]["apps"]:
        verbs = [action for action in app_verbs(app) if action["kind"] not in ("history", "clipboard", "interaction") and action["id"] not in FRAMEWORK_VIEW_SHELL]
        english = {}
        for action in verbs:
            if audience(action) != "agent":
                continue
            capability = f"{plugin}.{app['id']}.{action['id']}"
            described = action.get("semantics", {}).get("description")
            if described is None:
                missing.append(capability)
            else:
                if list(check.iter_errors(described)):
                    schema.append(capability)
                if any(row.get("de", "").strip() == row.get("en", "").strip() for row in described.values()):
                    untranslated.append(capability)
                if any(row.get(locale, "").strip().rstrip(".!?").strip().lower() == action["label"]["native"][locale].strip().lower() for row in described.values() for locale in ("en", "de")):
                    repeats.append(capability)
                english.setdefault(described.get("native", {}).get("en", "").strip(), []).append(capability)
            if action.get("semantics", {}).get("effects", {}).get("destructive"):
                continue
            replace = lexicon(action["id"], DOCUMENT_REPLACE) if action["kind"] == "mutation" else None
            discard = lexicon(action["id"], DESTRUCTIVE) if action["kind"] in ("mutation", "shell") else None
            write = lexicon(action["id"], USER_PATH) if action["kind"] in ("shell", "view") else None
            if replace or discard or write:
                destructive.append(f"{capability} [{replace or discard or write}]")
        shared.extend(capability for text, owners in english.items() if text and len(owners) > 1 for capability in owners)
    return {"missing": missing, "schema": schema, "untranslated": untranslated, "repeatsTitle": repeats, "sharedWithinApp": shared, "unmarkedDestructive": destructive}


def main():
    paths = [argument for argument in sys.argv[1:] if not argument.startswith("--")]
    check = validator()
    total = {}
    for path in paths:
        descriptor = json.load(open(path, encoding="utf-8"))
        result = census(descriptor, check)
        print(descriptor["manifest"]["pluginId"], {key: len(value) for key, value in result.items()})
        for key, value in result.items():
            total[key] = total.get(key, 0) + len(value)
            for row in value[:3]:
                print("   ", key, row)
    print("TOTAL", total)


if __name__ == "__main__":
    main()
