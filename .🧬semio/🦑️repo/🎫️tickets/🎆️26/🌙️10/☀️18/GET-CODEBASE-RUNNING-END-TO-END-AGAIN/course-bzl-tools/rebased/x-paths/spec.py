"""Authored constants shared by apply.py (schema edits) and genfixture.py (fixture authoring).

The Bazel adapter schema receives exactly these values; the test fixture is computed from them by an
independent Python implementation and then re-checked by the TypeScript implementation and by a Python
oracle that reads the applied schema, so a drift between the three fails a test.
"""

ALIAS = {"link": "a", "hexLength": 10}

PATH_BUDGET = {
    "version": 1,
    "repositoryUnits": 256,
    "task": "t_0000000000000000",
    "file": "00000000000000000000000000000000",
    "sandboxDigits": 6,
    "platforms": {
        "darwin": {
            "unit": "utf8",
            "pathMax": 1024,
            "sandboxes": ["darwin-sandbox", "processwrapper-sandbox"],
            "configuration": {"arm64": "darwin_arm64-fastbuild", "x64": "darwin_x86_64-fastbuild"},
        },
        "linux": {
            "unit": "utf8",
            "pathMax": 4096,
            "sandboxes": ["linux-sandbox", "processwrapper-sandbox"],
            "configuration": {"arm64": "aarch64-fastbuild", "x64": "k8-fastbuild"},
        },
        "win32": {
            "unit": "utf16",
            "pathMax": 260,
            "extendedMax": 32767,
            "sandboxes": [],
            "configuration": {"arm64": "arm64_windows-fastbuild", "x64": "x64_windows-fastbuild"},
        },
    },
    "families": [
        {"name": "source", "template": "execroot/_main/{source}", "view": "alias"},
        {
            "name": "runfiles-source",
            "template": "execroot/_main/bazel-out/{configuration}/bin/{task}.runfiles/_main/{source}",
            "view": "alias",
            "needs": "runfiles",
        },
        {"name": "snapshot", "template": "execroot/_main/bazel-out/{configuration}/bin/{task}.snapshot/files/{file}"},
        {
            "name": "runfiles-snapshot",
            "template": "execroot/_main/bazel-out/{configuration}/bin/{task}.runfiles/_main/{task}.snapshot/files/{file}",
            "needs": "runfiles",
        },
        {
            "name": "sandbox-source",
            "template": "sandbox/{sandbox}/{sandboxId}/execroot/_main/{source}",
            "view": "real",
            "needs": "sandbox",
        },
    ],
}

NO_RUNFILES = {"win32"}


def utf16(text):
    return len(text.encode("utf-16-le")) // 2


def utf8(text):
    return len(text.encode("utf-8"))


def budget(platform, arch, output_base, long_paths, closure):
    """Independent evaluation of the path budget: returns the expectation block of a fixture row."""
    row = PATH_BUDGET["platforms"][platform]
    windows = platform == "win32"
    filler = {
        "configuration": row["configuration"][arch],
        "task": PATH_BUDGET["task"],
        "file": PATH_BUDGET["file"],
        "sandbox": max(row["sandboxes"], key=len) if row["sandboxes"] else "",
        "sandboxId": "0" * PATH_BUDGET["sandboxDigits"],
    }
    families = {}
    for family in PATH_BUDGET["families"]:
        if family.get("needs") == "runfiles" and platform in NO_RUNFILES:
            continue
        if family.get("needs") == "sandbox" and not row["sandboxes"]:
            continue
        view = family.get("view")
        size = closure[view] if view else {"units": 0, "bytes": 0}
        if view and size["units"] == 0:
            continue
        text = family["template"].replace("{source}", "")
        for key, value in filler.items():
            text = text.replace("{" + key + "}", value)
        families[family["name"]] = {
            "units": utf16(output_base) + 1 + len(text) + size["units"],
            "bytes": utf8(output_base) + 1 + len(text) + size["bytes"],
        }
    os_units = (PATH_BUDGET["platforms"]["win32"]["extendedMax"] - 1 if long_paths == "enabled" else row["pathMax"] - 1) if windows else None
    limit_units = min(PATH_BUDGET["repositoryUnits"], os_units) if os_units is not None else PATH_BUDGET["repositoryUnits"]
    limit_bytes = row["pathMax"] - 1 if row["unit"] == "utf8" else None
    worst_units = max(f["units"] for f in families.values())
    worst_bytes = max(f["bytes"] for f in families.values())
    slack_units = limit_units - worst_units
    slack_bytes = None if limit_bytes is None else limit_bytes - worst_bytes
    fits = slack_units >= 0 and (slack_bytes is None or slack_bytes >= 0)
    return {
        "families": families,
        "limitUnits": limit_units,
        "limitBytes": limit_bytes,
        "slackUnits": slack_units,
        "slackBytes": slack_bytes,
        "fits": fits,
        "refusals": 0 if fits else 1,
    }
