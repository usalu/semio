"""🪟 Staged snapshots whose repo path Windows cannot create at ``C:\\git\\semio``.

https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation
"""

import hashlib
import json
import os

CLONE_PREFIX = 13
USER_PREFIX = 37
FILE_LIMIT = 259
DIRECTORY_LIMIT = 247
USER_LIMIT = 260
TICKET_ROOT = ".🧬semio/🦑️repo/🎫️tickets/"


def units(text: str) -> int:
    """📏 UTF-16 code units, which is what Win32 ``MAX_PATH`` counts."""
    return len(text.encode("utf-16-le")) // 2


def overflows(repo_relative: str) -> bool:
    """🚫 True when ``git checkout`` at ``C:\\git\\semio`` cannot create the path."""
    relative = repo_relative.replace("\\", "/")
    parent = relative.rsplit("/", 1)[0] if "/" in relative else ""
    file_length = CLONE_PREFIX + units(relative)
    directory_length = CLONE_PREFIX + units(parent) if parent else 0
    if file_length > FILE_LIMIT or (parent and directory_length > DIRECTORY_LIMIT):
        return True
    return relative.startswith(TICKET_ROOT) and USER_PREFIX + units(relative) > USER_LIMIT


def repository(start: str) -> str:
    """📦 Nearest directory that contains ``.git``."""
    current = os.path.abspath(start)
    while not os.path.isdir(os.path.join(current, ".git")):
        parent = os.path.dirname(current)
        if parent == current:
            raise RuntimeError("repository root not found")
        current = parent
    return current


def load(staged_root: str) -> dict[str, str]:
    """🗺️ Logical snapshot path to the short file name under ``w/``."""
    path = os.path.join(staged_root, "w", "map.json")
    if not os.path.exists(path):
        return {}
    with open(path, encoding="utf-8") as handle:
        payload = json.load(handle)
    files = payload.get("files", payload)
    return {str(key): str(value) for key, value in files.items()}


def save(staged_root: str, mapping: dict[str, str]) -> None:
    """💾 Write the alias map next to the short files."""
    folder = os.path.join(staged_root, "w")
    os.makedirs(folder, exist_ok=True)
    path = os.path.join(folder, "map.json")
    with open(path, "w", encoding="utf-8") as handle:
        json.dump({"files": {key: mapping[key] for key in sorted(mapping)}}, handle, ensure_ascii=False, indent=1)
        handle.write("\n")


def alias_name(logical: str, taken: set[str]) -> str:
    """🏷️ Short file name derived from the logical path."""
    digest = hashlib.sha256(logical.encode("utf-8")).hexdigest()
    suffix = os.path.splitext(logical)[1]
    if not suffix.isascii() or len(suffix) > 8:
        suffix = ".bin"
    name = digest[:12] + suffix
    serial = 0
    while name in taken:
        serial += 1
        name = f"{digest[:12]}-{serial}{suffix}"
    return name


def resolve(staged_root: str, logical: str) -> str:
    """🔎 Filesystem path for a logical snapshot, including a Windows alias."""
    logical = logical.replace("\\", "/")
    direct = os.path.join(staged_root, *logical.split("/"))
    if os.path.exists(direct):
        return direct
    name = load(staged_root).get(logical)
    if name:
        aliased = os.path.join(staged_root, "w", name)
        if os.path.exists(aliased):
            return aliased
    return direct


def iter_logical(staged_root: str) -> list[str]:
    """📂 Logical snapshot paths, with ``w/`` aliases expanded back to those paths."""
    rows: list[str] = []
    hidden = os.path.abspath(os.path.join(staged_root, "w"))
    for base, dirs, files in os.walk(staged_root):
        dirs[:] = [name for name in dirs if os.path.abspath(os.path.join(base, name)) != hidden]
        for name in files:
            rows.append(os.path.relpath(os.path.join(base, name), staged_root).replace("\\", "/"))
    rows.extend(load(staged_root))
    return sorted(set(rows))


def place(staged_root: str, logical: str, data: bytes) -> str:
    """📥 Store snapshot bytes, aliasing the file when the direct path cannot be checked out."""
    logical = logical.replace("\\", "/")
    full = os.path.relpath(os.path.join(staged_root, *logical.split("/")), repository(staged_root)).replace("\\", "/")
    if not overflows(full):
        dest = os.path.join(staged_root, *logical.split("/"))
        os.makedirs(os.path.dirname(dest), exist_ok=True)
        with open(dest, "wb") as handle:
            handle.write(data)
        return dest
    mapping = load(staged_root)
    name = mapping.get(logical) or alias_name(logical, set(mapping.values()))
    folder = os.path.join(staged_root, "w")
    os.makedirs(folder, exist_ok=True)
    dest = os.path.join(folder, name)
    with open(dest, "wb") as handle:
        handle.write(data)
    mapping[logical] = name
    save(staged_root, mapping)
    return dest
