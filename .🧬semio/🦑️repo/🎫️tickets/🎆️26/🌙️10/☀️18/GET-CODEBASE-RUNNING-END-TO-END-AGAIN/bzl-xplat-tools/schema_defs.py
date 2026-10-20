import json

PLATFORM_RULES = {
    "const": {
        "version": 1,
        "limits": {"posix": 4095, "windows": 259, "windowsExtended": 32766},
        "trackedPathUnits": 256,
        "execroot": {"posix": "/execroot/_main/s/", "win32": "\\execroot\\_main\\s\\"},
        "platforms": {
            "darwin": {"separator": "/", "launcherExtension": "", "sourceLink": "dir", "startup": [], "command": ["--nobuild_runfile_links"], "environment": {"LANG": "en_US.UTF-8", "LC_ALL": "en_US.UTF-8"}, "emojiLabels": "verified"},
            "linux": {"separator": "/", "launcherExtension": "", "sourceLink": "dir", "startup": [], "command": ["--nobuild_runfile_links"], "environment": {}, "emojiLabels": "verified"},
            "win32": {"separator": "\\", "launcherExtension": ".exe", "sourceLink": "junction", "startup": ["--nowindows_enable_symlinks"], "command": ["--nobuild_runfile_links", "--noenable_runfiles"], "environment": {}, "emojiLabels": "unverified"},
        },
    }
}

STRINGS = {"type": "array", "items": {"type": "string", "pattern": "^--[a-z_]+(=\\S+)?$"}, "uniqueItems": True}
ENVIRONMENT = {"type": "object", "additionalProperties": {"type": "string", "minLength": 1}}

PLATFORM_INPUT = {
    "type": "object",
    "additionalProperties": False,
    "required": ["platform", "arch", "container", "outputBase", "bun", "longPaths"],
    "properties": {
        "platform": {"enum": ["win32", "darwin", "linux"]},
        "arch": {"enum": ["x64", "arm64"]},
        "container": {"type": "boolean"},
        "outputBase": {"type": "string", "minLength": 1, "maxLength": 256},
        "bun": {"type": "string", "minLength": 1, "maxLength": 1024},
        "longPaths": {"enum": ["enabled", "disabled", "unprobed"]},
    },
}

PLATFORM_PROFILE = {
    "type": "object",
    "additionalProperties": False,
    "required": ["version", "platform", "arch", "startup", "command", "environment", "launcherExtension", "sourceLink", "shell", "emojiLabels", "pathBudget", "refusals", "warnings"],
    "properties": {
        "version": {"const": 1},
        "platform": {"enum": ["win32", "darwin", "linux"]},
        "arch": {"enum": ["x64", "arm64"]},
        "startup": STRINGS,
        "command": STRINGS,
        "environment": ENVIRONMENT,
        "launcherExtension": {"enum": ["", ".exe"]},
        "sourceLink": {"enum": ["dir", "junction"]},
        "shell": {"const": "none"},
        "emojiLabels": {"enum": ["verified", "unverified"]},
        "pathBudget": {
            "type": "object",
            "additionalProperties": False,
            "required": ["outputBase", "execroot", "tracked", "total", "limit", "fits"],
            "properties": {key: {"type": "integer", "minimum": 0} for key in ["outputBase", "execroot", "tracked", "total", "limit"]} | {"fits": {"type": "boolean"}},
        },
        "refusals": {"type": "array", "items": {"type": "string", "minLength": 1}},
        "warnings": {"type": "array", "items": {"type": "string", "minLength": 1}},
    },
}

LINUX_HOST_PATHS = {
    "guarded": ["install"],
    "pattern": "^[\\u0020-\\u007e]+$",
    "reason": "The Linux JVM resolves the Bazel server classpath from the install base through its byte-wise path strings, so a non-ASCII install base crashes the server at startup (Could not find or load main class, ClassNotFoundException com.google.devtools.build.lib.bazel.Bazel) under every locale tried: unset, POSIX, C, C.UTF-8 and en_US.UTF-8. Output bases, sandbox directories, execroot, source labels, disk cache and workspace paths accept non-ASCII names (an emoji output base built and cached on Linux arm64).",
}


def apply(schema):
    definitions = schema["definitions"]
    if "PlatformRules" in definitions:
        return schema
    rebuilt = {}
    for key, value in definitions.items():
        if key == "HostPaths":
            rebuilt["PlatformRules"] = PLATFORM_RULES
            rebuilt["PlatformInput"] = PLATFORM_INPUT
            rebuilt["PlatformProfile"] = PLATFORM_PROFILE
        rebuilt[key] = value
    host = rebuilt["HostPaths"]["const"]
    host["platforms"]["linux"] = LINUX_HOST_PATHS
    policy = rebuilt["HostPathPolicy"]["properties"]["platforms"]["additionalProperties"]
    policy["properties"]["guarded"] = {"type": "array", "minItems": 1, "uniqueItems": True, "items": {"enum": ["install", "outputs", "server"]}}
    schema["definitions"] = rebuilt
    return schema


if __name__ == "__main__":
    print(json.dumps(PLATFORM_PROFILE)[:200])
